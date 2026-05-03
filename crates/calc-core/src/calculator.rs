//! Calculator state machine.
//!
//! The calculator is event-driven: each [`KeyEvent`] mutates state and
//! optionally appends to the [`crate::tape::Tape`]. The state machine is
//! pure — no I/O, no time, no randomness — which makes it trivial to
//! property-test and to drive from any frontend (WASM, CLI, native).

use num_rational::Rational64;
use num_traits::Zero;

use crate::error::CalcError;
use crate::format::LengthFormat;
use crate::length::Length;
use crate::operations::compound_miter_state::{MiterField, PartialCompoundMiter};
use crate::operations::rafter::{PartialRafter, RafterField};
use crate::tape::{Tape, TapeEntry};
use crate::value::Value;

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
    Millimeters,
    Centimeters,
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
    Convert(LengthFormat),
    Memory(MemoryOp),
    Backspace,
    Clear,
    ClearAll,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct Mode {
    pub default_length_format: LengthFormat,
    pub angle_in_degrees: bool,
}

impl Default for Mode {
    fn default() -> Self {
        Self {
            default_length_format: LengthFormat::FeetInchFraction { denom: 16 },
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
        let mut r = if self.whole.is_empty() {
            Rational64::zero()
        } else {
            crate::length::parse::parse_decimal_or_fraction(&self.whole)
                .map_err(CalcError::Parse)?
        };
        if let (Some(num), Some(den)) = (&self.numerator, &self.denominator) {
            if num.is_empty() || den.is_empty() {
                return Err(CalcError::Invalid("incomplete fraction".into()));
            }
            let n: i64 = num
                .parse()
                .map_err(|_| CalcError::Invalid(format!("bad numerator '{}'", num)))?;
            let d: i64 = den
                .parse()
                .map_err(|_| CalcError::Invalid(format!("bad denominator '{}'", den)))?;
            if d == 0 {
                return Err(CalcError::DivByZero);
            }
            r += Rational64::new(n, d);
        } else if let Some(num) = &self.numerator {
            if !num.is_empty() && self.whole.is_empty() {
                let n: i64 = num
                    .parse()
                    .map_err(|_| CalcError::Invalid(format!("bad numerator '{}'", num)))?;
                r = Rational64::from_integer(n);
            }
        }
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
            KeyEvent::Convert(fmt) => self.convert_display(fmt)?,
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
            LengthUnitKey::Feet => Length::from_inches(
                n * Rational64::from_integer(crate::length::consts::IN_PER_FT),
            ),
            LengthUnitKey::Inch => Length::from_inches(n),
            LengthUnitKey::Yards => Length::from_inches(
                n * Rational64::from_integer(crate::length::consts::IN_PER_YD),
            ),
            LengthUnitKey::Millimeters => Length::from_mm(n),
            LengthUnitKey::Centimeters => Length::from_cm(n),
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
                self.display = Value::Scalar(rational_from_f64(v));
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
            FunctionKey::Corner | FunctionKey::Spring | FunctionKey::Miter | FunctionKey::Bevel => {
                if matches!(f, FunctionKey::Corner) {
                    self.compound_miter
                        .set_field(MiterField::Corner, current)?;
                } else if matches!(f, FunctionKey::Spring) {
                    self.compound_miter
                        .set_field(MiterField::Spring, current)?;
                }
                if let Some(soln) = self.compound_miter.try_solve()? {
                    // Append a note describing the cut.
                    self.tape.push(TapeEntry::Note(format!(
                        "Compound miter — corner {}, spring {} → miter {}, bevel {}",
                        soln.corner_angle,
                        soln.spring_from_wall,
                        soln.miter_angle,
                        soln.bevel_angle
                    )));
                    self.display = match f {
                        FunctionKey::Corner | FunctionKey::Miter => Value::Angle(soln.miter_angle),
                        FunctionKey::Spring | FunctionKey::Bevel => Value::Angle(soln.bevel_angle),
                        _ => unreachable!(),
                    };
                }
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
        match m {
            MemoryOp::Store(i) => self.memory[i as usize] = self.display,
            MemoryOp::Recall(i) => self.display = self.memory[i as usize],
            MemoryOp::AddTo(i) => {
                let cur = self.memory[i as usize];
                self.memory[i as usize] = cur.add(self.display)?;
            }
            MemoryOp::Clear(i) => self.memory[i as usize] = Value::zero_scalar(),
            MemoryOp::ClearAll => {
                for slot in self.memory.iter_mut() {
                    *slot = Value::zero_scalar();
                }
            }
        }
        Ok(())
    }

    pub fn display_string(&self) -> String {
        match self.display {
            Value::Scalar(r) => crate::format::rational_to_decimal_string(r, 6),
            Value::Length(l) => crate::format::format_length(&l, self.mode.default_length_format),
            Value::Area(r) => format!(
                "{} sq in",
                crate::format::rational_to_decimal_string(r, 4)
            ),
            Value::Volume(r) => format!(
                "{} cu in",
                crate::format::rational_to_decimal_string(r, 4)
            ),
            Value::Angle(a) => a.to_string(),
        }
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

fn rational_to_f64(r: num_rational::Rational64) -> f64 {
    *r.numer() as f64 / *r.denom() as f64
}

fn rational_from_f64(x: f64) -> num_rational::Rational64 {
    let n = (x * 1_000_000.0).round() as i64;
    num_rational::Rational64::new(n, 1_000_000)
}

fn sqrt_value(v: Value) -> Result<Value, CalcError> {
    match v {
        Value::Scalar(r) => {
            let f = rational_to_f64(r);
            if f < 0.0 {
                return Err(CalcError::Domain("sqrt of negative".into()));
            }
            Ok(Value::Scalar(rational_from_f64(f.sqrt())))
        }
        Value::Area(r) => {
            let f = rational_to_f64(r);
            if f < 0.0 {
                return Err(CalcError::Domain("sqrt of negative area".into()));
            }
            let inches = (f.sqrt() * 64.0).round() as i64;
            Ok(Value::Length(crate::length::Length::from_inches(
                num_rational::Rational64::new(inches, 64),
            )))
        }
        Value::Length(l) => {
            let f = rational_to_f64(l.inches());
            if f < 0.0 {
                return Err(CalcError::Domain("sqrt of negative".into()));
            }
            Ok(Value::Scalar(rational_from_f64(f.sqrt())))
        }
        _ => Err(CalcError::TypeMismatch),
    }
}
