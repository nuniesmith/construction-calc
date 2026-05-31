//! Calculator state machine.
//!
//! The calculator is event-driven: each [`KeyEvent`] mutates state and
//! optionally appends to the [`crate::tape::Tape`]. The state machine is
//! pure — no I/O, no time, no randomness — which makes it trivial to
//! property-test and to drive from any frontend (WASM, CLI, native).

use num_rational::Rational64;
use num_traits::Zero;

use crate::error::CalcError;
use crate::format::{AngleFormat, AreaFormat, LengthFormat, VolumeFormat};
use crate::length::Length;
use crate::numeric::{rational_from_f64, rational_from_f64_on_grid, rational_to_f64};
use crate::operations::compound_miter_state::{MiterField, PartialCompoundMiter};
use crate::operations::rafter::{PartialRafter, RafterField};
use crate::tape::{Tape, TapeEntry};
use crate::value::{Dimension, Value};

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub enum BinaryOp {
    Add,
    Sub,
    Mul,
    Div,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub enum LengthUnitKey {
    Feet,
    Inch,
    Yards,
    Meters,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub enum FunctionKey {
    // -- Rafter family --
    Pitch,
    Rise,
    Run,
    Diagonal,
    HipValley,
    Jack,
    // -- Trig --
    Sin,
    Cos,
    Tan,
    Asin,
    Acos,
    Atan,
    // -- Plain math --
    Sqrt,
    Square,
    Reciprocal,
    Percent,
    // -- Compound miter family --
    /// Set the corner angle for compound-miter solving.
    Corner,
    /// Set the spring angle for compound-miter solving.
    Spring,
    /// Show the miter (saw rotation) angle.
    Miter,
    /// Show the bevel (blade tilt) angle.
    Bevel,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub enum MemoryOp {
    Store(u8),
    Recall(u8),
    AddTo(u8),
    Clear(u8),
    ClearAll,
}

#[derive(Clone, Debug, PartialEq)]
pub enum KeyEvent {
    Digit(u8),
    Decimal,
    Slash,
    Negate,
    Op(BinaryOp),
    Equals,
    Unit(LengthUnitKey),
    Function(FunctionKey),
    /// Begin a cost-per-unit calculation: the current value becomes the
    /// quantity, the next entered number is the price per the quantity's
    /// current display unit, and `=` yields the total as money.
    CostPerUnit,
    Convert(LengthFormat),
    /// Switch how an area result is displayed (sq in / ft / yd / m, acres).
    ConvertArea(AreaFormat),
    /// Switch how a volume result is displayed (cu in / ft / yd / m, gal, L).
    ConvertVolume(VolumeFormat),
    /// Switch how an angle result is displayed (D°M'S" vs decimal degrees).
    ConvertAngle(AngleFormat),
    /// Set whether trig keys interpret/emit angles in degrees (`true`) or
    /// radians (`false`). Mirrors a user preference; persists in `Mode`.
    SetAngleMode(bool),
    Memory(MemoryOp),
    Backspace,
    Clear,
    ClearAll,
    /// Append a free-form labelled note to the tape (no other state change).
    /// Used by the EZ Calc forms layer to record a real entry instead of
    /// faking it with digit-typed values.
    Note(String),
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct Mode {
    pub default_length_format: LengthFormat,
    pub default_area_format: AreaFormat,
    pub default_volume_format: VolumeFormat,
    pub angle_format: AngleFormat,
    pub angle_in_degrees: bool,
}

impl Default for Mode {
    fn default() -> Self {
        Self {
            default_length_format: LengthFormat::FeetInchFraction { denom: 16 },
            // Square / cubic feet are the most useful defaults for builders;
            // `17_280 sq in` reads as `120 sq ft`. The strip offers the rest.
            default_area_format: AreaFormat::SquareFeet { precision: 2 },
            default_volume_format: VolumeFormat::CubicFeet { precision: 2 },
            // DMS is the long-standing display default; the strip offers
            // decimal degrees. This is the *display* axis — independent of
            // `angle_in_degrees`, which is trig input interpretation.
            angle_format: AngleFormat::DegMinSec,
            angle_in_degrees: true,
        }
    }
}

#[derive(Clone, Debug, Default, PartialEq)]
struct EntryBuffer {
    whole: String,
    numerator: Option<String>,
    denominator: Option<String>,
    has_decimal: bool,
    negative: bool,
    accumulated: Option<Length>,
}

impl EntryBuffer {
    fn is_empty(&self) -> bool {
        self.whole.is_empty()
            && self.numerator.is_none()
            && self.denominator.is_none()
            && self.accumulated.is_none()
    }

    fn push_digit(&mut self, d: u8) {
        let ch = (b'0' + d) as char;
        match (&mut self.numerator, &mut self.denominator) {
            (_, Some(s)) => s.push(ch),
            (Some(s), None) => s.push(ch),
            (None, None) => self.whole.push(ch),
        }
    }

    fn push_decimal(&mut self) {
        if self.numerator.is_some() || self.denominator.is_some() {
            return;
        }
        if !self.has_decimal {
            if self.whole.is_empty() {
                self.whole.push('0');
            }
            self.whole.push('.');
            self.has_decimal = true;
        }
    }

    fn push_slash(&mut self) {
        if self.has_decimal {
            return;
        }
        if self.numerator.is_none() {
            self.numerator = Some(String::new());
        } else if self.denominator.is_none() {
            self.denominator = Some(String::new());
        }
    }

    fn negate(&mut self) {
        self.negative = !self.negative;
    }

    fn backspace(&mut self) {
        if let Some(s) = self.denominator.as_mut() {
            if s.pop().is_some() {
                if s.is_empty() {
                    self.denominator = None;
                }
                return;
            }
        }
        if let Some(s) = self.numerator.as_mut() {
            if s.pop().is_some() {
                if s.is_empty() {
                    self.numerator = None;
                }
                return;
            }
        }
        if let Some(c) = self.whole.pop() {
            if c == '.' {
                self.has_decimal = false;
            }
        }
    }

    fn reset(&mut self) {
        *self = Self::default();
    }

    fn current_rational(&self) -> Result<Rational64, CalcError> {
        // Buffer states the user can produce by typing digits + slashes:
        //   `3`           → whole="3"                      → 3
        //   `3 /`         → whole="3", num=Some("")        → 3 (mid-typing, slash not yet meaningful)
        //   `3 / 8`       → whole="3", num=Some("8")       → 3/8 (single-slash → fraction)
        //   `3 / 8 /`     → whole="3", num="8", den=Some("")
        //   `3 / 1 / 2`   → whole="3", num="1", den="2"    → 3 + 1/2 (mixed)
        //   `/ 5`         → whole="",  num=Some("5")       → 5 (leading slash treated as a no-op)
        let parse_whole = |w: &str| -> Result<Rational64, CalcError> {
            if w.is_empty() {
                Ok(Rational64::zero())
            } else {
                crate::length::parse::parse_decimal_or_fraction(w).map_err(CalcError::Parse)
            }
        };
        let parse_int = |label: &str, s: &str| -> Result<i64, CalcError> {
            s.parse()
                .map_err(|_| CalcError::Invalid(format!("bad {} '{}'", label, s)))
        };

        let mut r = match (&self.numerator, &self.denominator) {
            (Some(num), Some(den)) => {
                if num.is_empty() || den.is_empty() {
                    return Err(CalcError::Invalid("incomplete fraction".into()));
                }
                let whole_part = parse_whole(&self.whole)?;
                let n = parse_int("numerator", num)?;
                let d = parse_int("denominator", den)?;
                if d == 0 {
                    return Err(CalcError::DivByZero);
                }
                whole_part + Rational64::new(n, d)
            }
            (Some(num), None) => {
                if num.is_empty() {
                    // Slash typed but no denominator digits yet — show the whole portion only.
                    parse_whole(&self.whole)?
                } else if self.whole.is_empty() {
                    // Leading-slash entry: treat the typed value as a plain integer.
                    Rational64::from_integer(parse_int("numerator", num)?)
                } else {
                    // Single-slash entry like `3 / 8` → 3/8 (whole was the numerator).
                    let n = parse_int("numerator", &self.whole)?;
                    let d = parse_int("denominator", num)?;
                    if d == 0 {
                        return Err(CalcError::DivByZero);
                    }
                    Rational64::new(n, d)
                }
            }
            (None, _) => parse_whole(&self.whole)?,
        };
        if self.negative {
            r = -r;
        }
        Ok(r)
    }
}

#[derive(Clone, Debug)]
pub struct Calculator {
    pub display: Value,
    pub mode: Mode,
    pub memory: [Value; 4],

    entry: EntryBuffer,
    pending: Option<(Value, BinaryOp)>,
    last_op: Option<(BinaryOp, Value)>,
    /// Set by `CostPerUnit`: the quantity awaiting a price. When `=` lands
    /// with this set, the entered scalar is the price-per-display-unit.
    pending_cost: Option<Value>,

    rafter: PartialRafter,
    compound_miter: PartialCompoundMiter,

    pub tape: Tape,
}

impl Default for Calculator {
    fn default() -> Self {
        Self::new()
    }
}

impl Calculator {
    pub fn new() -> Self {
        Self {
            display: Value::zero_scalar(),
            mode: Mode::default(),
            memory: [
                Value::zero_scalar(),
                Value::zero_scalar(),
                Value::zero_scalar(),
                Value::zero_scalar(),
            ],
            entry: EntryBuffer::default(),
            pending: None,
            last_op: None,
            pending_cost: None,
            rafter: PartialRafter::default(),
            compound_miter: PartialCompoundMiter::default(),
            tape: Tape::default(),
        }
    }

    pub fn handle(&mut self, ev: KeyEvent) -> Result<(), CalcError> {
        match ev {
            KeyEvent::Digit(d) => {
                self.entry.push_digit(d);
                self.refresh_display_from_entry()?;
            }
            KeyEvent::Decimal => {
                self.entry.push_decimal();
                self.refresh_display_from_entry()?;
            }
            KeyEvent::Slash => self.entry.push_slash(),
            KeyEvent::Negate => {
                self.entry.negate();
                self.refresh_display_from_entry()?;
            }
            KeyEvent::Backspace => {
                self.entry.backspace();
                self.refresh_display_from_entry()?;
            }
            KeyEvent::Unit(u) => self.commit_unit(u)?,
            KeyEvent::Op(op) => self.commit_op(op)?,
            KeyEvent::Equals => self.commit_equals()?,
            KeyEvent::Function(f) => self.handle_function(f)?,
            KeyEvent::CostPerUnit => self.commit_cost()?,
            KeyEvent::Convert(fmt) => self.convert_display(fmt)?,
            KeyEvent::ConvertArea(fmt) => {
                self.mode.default_area_format = fmt.validate().map_err(CalcError::Parse)?
            }
            KeyEvent::ConvertVolume(fmt) => {
                self.mode.default_volume_format = fmt.validate().map_err(CalcError::Parse)?
            }
            KeyEvent::ConvertAngle(fmt) => {
                self.mode.angle_format = fmt.validate().map_err(CalcError::Parse)?
            }
            KeyEvent::SetAngleMode(degrees) => self.mode.angle_in_degrees = degrees,
            KeyEvent::Memory(m) => self.handle_memory(m)?,
            KeyEvent::Clear => {
                self.entry.reset();
                self.display = Value::zero_scalar();
            }
            KeyEvent::ClearAll => {
                *self = Self {
                    mode: self.mode,
                    ..Self::new()
                };
            }
            KeyEvent::Note(text) => {
                self.tape.push(TapeEntry::Note(text));
            }
        }
        Ok(())
    }

    fn refresh_display_from_entry(&mut self) -> Result<(), CalcError> {
        if self.entry.is_empty() {
            self.display = Value::zero_scalar();
            return Ok(());
        }
        let n = self.entry.current_rational()?;
        if let Some(acc) = self.entry.accumulated {
            self.display = Value::Length(acc + Length::from_inches(n));
        } else {
            self.display = Value::Scalar(n);
        }
        Ok(())
    }

    fn commit_unit(&mut self, u: LengthUnitKey) -> Result<(), CalcError> {
        let n = if self.entry.is_empty() {
            Rational64::zero()
        } else {
            self.entry.current_rational()?
        };
        let added = match u {
            LengthUnitKey::Feet => {
                Length::from_inches(n * Rational64::from_integer(crate::length::consts::IN_PER_FT))
            }
            LengthUnitKey::Inch => Length::from_inches(n),
            LengthUnitKey::Yards => {
                Length::from_inches(n * Rational64::from_integer(crate::length::consts::IN_PER_YD))
            }
            LengthUnitKey::Meters => Length::from_m(n),
        };
        let total = self.entry.accumulated.unwrap_or(Length::ZERO) + added;
        self.entry = EntryBuffer {
            accumulated: Some(total),
            ..EntryBuffer::default()
        };
        self.display = Value::Length(total);
        Ok(())
    }

    fn commit_current_value(&mut self) -> Result<Value, CalcError> {
        let v = self.display;
        self.entry.reset();
        Ok(v)
    }

    fn commit_op(&mut self, op: BinaryOp) -> Result<(), CalcError> {
        let current = self.commit_current_value()?;
        let new_acc = match self.pending.take() {
            None => current,
            Some((lhs, prev_op)) => apply_op(lhs, prev_op, current)?,
        };
        self.display = new_acc;
        self.pending = Some((new_acc, op));
        Ok(())
    }

    fn commit_equals(&mut self) -> Result<(), CalcError> {
        let current = self.commit_current_value()?;
        // A pending cost binds tighter than a binary op: the just-entered
        // scalar is the price, so compute the money subtotal first, then fold
        // it into any pending op (`$20 + 5 cost 3` = $35). The repeat-equals
        // (last_op) path is deliberately skipped for a cost.
        if let Some(quantity) = self.pending_cost.take() {
            let subtotal = self.apply_cost(quantity, current)?;
            let result = match self.pending.take() {
                Some((lhs, op)) => apply_op(lhs, op, subtotal)?,
                None => subtotal,
            };
            self.last_op = None;
            self.display = result;
            self.tape.push(TapeEntry::Result(result));
            return Ok(());
        }
        let result = match self.pending.take() {
            Some((lhs, op)) => {
                self.last_op = Some((op, current));
                apply_op(lhs, op, current)?
            }
            None => match &self.last_op {
                Some((op, rhs)) => apply_op(current, *op, *rhs)?,
                None => current,
            },
        };
        self.display = result;
        self.tape.push(TapeEntry::Result(result));
        Ok(())
    }

    /// Start a cost-per-unit calculation: stash the current value as the
    /// quantity and await a price. Any pending binary op is left intact and
    /// applied to the money subtotal at `=` (cost binds tighter). To price a
    /// running sum, press `=` first, then the cost key.
    fn commit_cost(&mut self) -> Result<(), CalcError> {
        let quantity = self.commit_current_value()?;
        self.display = quantity;
        self.pending_cost = Some(quantity);
        Ok(())
    }

    /// Total cost = price × the quantity expressed in its *current display
    /// unit*, so a price reads as dollars-per-what-you-see ($/ft, $/sq ft,
    /// $/cu yd, $/each). The price must be a plain scalar.
    fn apply_cost(&self, quantity: Value, price: Value) -> Result<Value, CalcError> {
        let price = match price {
            Value::Scalar(r) => r,
            _ => return Err(CalcError::Domain("price must be a plain number".into())),
        };
        let qty = self.magnitude_in_display_unit(quantity)?;
        Ok(Value::Money(qty * price))
    }

    fn magnitude_in_display_unit(&self, v: Value) -> Result<Rational64, CalcError> {
        Ok(match v {
            Value::Scalar(r) => r,
            Value::Length(l) => match self.mode.default_length_format {
                LengthFormat::FeetInchFraction { .. } | LengthFormat::DecimalFeet { .. } => {
                    l.feet()
                }
                LengthFormat::DecimalInches { .. } => l.inches(),
                LengthFormat::Yards { .. } => l.yards(),
                LengthFormat::Meters { .. } => l.meters(),
            },
            Value::Area(r) => crate::format::area_magnitude(r, self.mode.default_area_format),
            Value::Volume(r) => crate::format::volume_magnitude(r, self.mode.default_volume_format),
            Value::Angle(_) => return Err(CalcError::Domain("can't price an angle".into())),
            Value::Money(_) => return Err(CalcError::Domain("already a cost".into())),
        })
    }

    fn handle_function(&mut self, f: FunctionKey) -> Result<(), CalcError> {
        let current = self.display;
        match f {
            // -- Rafter family --
            FunctionKey::Pitch
            | FunctionKey::Rise
            | FunctionKey::Run
            | FunctionKey::Diagonal
            | FunctionKey::HipValley
            | FunctionKey::Jack => {
                if let Some(field) = rafter_field_for(f) {
                    self.rafter.set_field(field, current)?;
                }
                if let Some(soln) = self.rafter.try_solve()? {
                    self.tape
                        .push(TapeEntry::RafterSolution(Box::new(soln.clone())));
                    self.display = match f {
                        FunctionKey::Pitch => Value::Scalar(soln.pitch_ratio),
                        FunctionKey::Rise => Value::Length(soln.rise),
                        FunctionKey::Run => Value::Length(soln.run),
                        FunctionKey::Diagonal => Value::Length(soln.diagonal),
                        FunctionKey::HipValley => Value::Length(soln.hip_valley),
                        FunctionKey::Jack => Value::Length(soln.jack_difference),
                        _ => unreachable!(),
                    };
                }
            }

            // -- Forward trig --
            FunctionKey::Sin | FunctionKey::Cos | FunctionKey::Tan => {
                let angle = match current {
                    Value::Angle(a) => a,
                    Value::Scalar(r) => {
                        if self.mode.angle_in_degrees {
                            crate::angle::Angle::from_degrees(r)
                        } else {
                            crate::angle::Angle::from_radians(rational_to_f64(r))
                        }
                    }
                    _ => return Err(CalcError::TypeMismatch),
                };
                let v = match f {
                    FunctionKey::Sin => crate::operations::trig::sin(angle),
                    FunctionKey::Cos => crate::operations::trig::cos(angle),
                    FunctionKey::Tan => crate::operations::trig::tan(angle),
                    _ => unreachable!(),
                };
                self.display = Value::Scalar(rational_from_f64(v)?);
            }

            // -- Inverse trig --
            FunctionKey::Asin | FunctionKey::Acos | FunctionKey::Atan => {
                let x = match current {
                    Value::Scalar(r) => rational_to_f64(r),
                    _ => return Err(CalcError::TypeMismatch),
                };
                let a = match f {
                    FunctionKey::Asin => crate::operations::trig::asin(x)?,
                    FunctionKey::Acos => crate::operations::trig::acos(x)?,
                    FunctionKey::Atan => crate::operations::trig::atan(x),
                    _ => unreachable!(),
                };
                self.display = Value::Angle(a);
            }

            FunctionKey::Sqrt => self.display = sqrt_value(current)?,
            FunctionKey::Square => self.display = current.mul(current)?,
            FunctionKey::Reciprocal => {
                let one = Value::Scalar(num_rational::Rational64::from_integer(1));
                self.display = one.div(current)?;
            }
            FunctionKey::Percent => {
                if let Value::Scalar(r) = current {
                    let pct = Value::Scalar(r / num_rational::Rational64::from_integer(100));
                    if let Some((lhs, _)) = self.pending {
                        self.display = lhs.mul(pct)?;
                    } else {
                        self.display = pct;
                    }
                } else {
                    return Err(CalcError::TypeMismatch);
                }
            }

            // -- Compound miter family --
            //
            // Input keys (Corner, Spring) store the field and echo the entered
            // value back to the display so the user sees their input was
            // accepted. Output keys (Miter, Bevel) compute and display the
            // requested component, but only when both inputs are present.
            FunctionKey::Corner => {
                self.compound_miter.set_field(MiterField::Corner, current)?;
                if let Some(stored) = self.compound_miter.corner {
                    self.display = Value::Angle(stored);
                }
            }
            FunctionKey::Spring => {
                self.compound_miter.set_field(MiterField::Spring, current)?;
                if let Some(stored) = self.compound_miter.spring {
                    self.display = Value::Angle(stored);
                }
            }
            FunctionKey::Miter | FunctionKey::Bevel => {
                if let Some(soln) = self.compound_miter.try_solve()? {
                    self.tape.push(TapeEntry::Note(format!(
                        "Compound miter — corner {}, spring {} → miter {}, bevel {}",
                        soln.corner_angle,
                        soln.spring_from_wall,
                        soln.miter_angle,
                        soln.bevel_angle
                    )));
                    self.display = match f {
                        FunctionKey::Miter => Value::Angle(soln.miter_angle),
                        FunctionKey::Bevel => Value::Angle(soln.bevel_angle),
                        _ => unreachable!(),
                    };
                }
                // If both inputs aren't set yet, leave the display alone — the
                // user can fill in the missing field and press the result key
                // again.
            }
        }
        self.entry.reset();
        Ok(())
    }

    fn convert_display(&mut self, fmt: LengthFormat) -> Result<(), CalcError> {
        self.mode.default_length_format = fmt.validate().map_err(CalcError::Parse)?;
        Ok(())
    }

    fn handle_memory(&mut self, m: MemoryOp) -> Result<(), CalcError> {
        let slot = |i: u8| -> Result<usize, CalcError> {
            let idx = i as usize;
            if idx >= self.memory.len() {
                return Err(CalcError::Invalid(format!(
                    "memory slot {} out of range (have {})",
                    i,
                    self.memory.len()
                )));
            }
            Ok(idx)
        };
        match m {
            MemoryOp::Store(i) => self.memory[slot(i)?] = self.display,
            MemoryOp::Recall(i) => self.display = self.memory[slot(i)?],
            MemoryOp::AddTo(i) => {
                let idx = slot(i)?;
                let cur = self.memory[idx];
                self.memory[idx] = cur.add(self.display)?;
            }
            MemoryOp::Clear(i) => self.memory[slot(i)?] = Value::zero_scalar(),
            MemoryOp::ClearAll => {
                for s in self.memory.iter_mut() {
                    *s = Value::zero_scalar();
                }
            }
        }
        Ok(())
    }

    pub fn display_string(&self) -> String {
        match self.display {
            Value::Scalar(r) => crate::format::rational_to_decimal_string(r, 6),
            Value::Length(l) => crate::format::format_length(&l, self.mode.default_length_format),
            Value::Area(r) => crate::format::format_area(r, self.mode.default_area_format),
            Value::Volume(r) => crate::format::format_volume(r, self.mode.default_volume_format),
            Value::Angle(a) => crate::format::format_angle(&a, self.mode.angle_format),
            Value::Money(r) => crate::format::format_money(r),
        }
    }

    /// The dimensional category of the current display value, so a UI can
    /// show the matching unit controls (length vs. area vs. volume) without
    /// parsing [`display_string`](Self::display_string).
    pub fn display_dimension(&self) -> Dimension {
        self.display.dimension()
    }
}

fn apply_op(lhs: Value, op: BinaryOp, rhs: Value) -> Result<Value, CalcError> {
    match op {
        BinaryOp::Add => lhs.add(rhs),
        BinaryOp::Sub => lhs.sub(rhs),
        BinaryOp::Mul => lhs.mul(rhs),
        BinaryOp::Div => lhs.div(rhs),
    }
}

fn rafter_field_for(f: FunctionKey) -> Option<RafterField> {
    match f {
        FunctionKey::Pitch => Some(RafterField::Pitch),
        FunctionKey::Rise => Some(RafterField::Rise),
        FunctionKey::Run => Some(RafterField::Run),
        FunctionKey::Diagonal => Some(RafterField::Diagonal),
        _ => None,
    }
}

fn sqrt_value(v: Value) -> Result<Value, CalcError> {
    match v {
        Value::Scalar(r) => {
            let f = rational_to_f64(r);
            if f < 0.0 {
                return Err(CalcError::Domain("sqrt of negative".into()));
            }
            Ok(Value::Scalar(rational_from_f64(f.sqrt())?))
        }
        Value::Area(r) => {
            let f = rational_to_f64(r);
            if f < 0.0 {
                return Err(CalcError::Domain("sqrt of negative area".into()));
            }
            Ok(Value::Length(crate::length::Length::from_inches(
                rational_from_f64_on_grid(f.sqrt(), 64)?,
            )))
        }
        Value::Length(l) => {
            let f = rational_to_f64(l.inches());
            if f < 0.0 {
                return Err(CalcError::Domain("sqrt of negative".into()));
            }
            Ok(Value::Scalar(rational_from_f64(f.sqrt())?))
        }
        _ => Err(CalcError::TypeMismatch),
    }
}
