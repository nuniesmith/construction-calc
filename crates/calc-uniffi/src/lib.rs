//! UniFFI bindings for `calc-core`.
//!
//! Exposes a Swift-friendly interface so the iOS app can drive the same
//! calculator engine that the WASM + Svelte web app uses. Kotlin gets the
//! same bindings for the future Android port.
//!
//! ## Design notes
//!
//! - The `Calculator` object is `Send + Sync` via an internal `Mutex`, so
//!   Swift can pass it across actor boundaries safely (e.g. updating from
//!   a `@MainActor` view model while a background task reads it).
//! - We mirror `calc-core::KeyEvent` with our own UniFFI-shaped enum
//!   rather than re-exporting it directly, for two reasons: (1) UniFFI
//!   enums need named fields, not tuple variants; (2) decoupling lets us
//!   evolve the FFI surface without churning the engine.
//! - Errors come back inside [`Snapshot::error`] rather than as Rust
//!   `Result`s, because every event still produces a displayable state —
//!   even a divide-by-zero leaves the display intact and just sets an
//!   error message.

use std::sync::Mutex;

use calc_core::Dimension as CoreDimension;
use calc_core::calculator::{
    BinaryOp as CoreBinaryOp, Calculator as CoreCalculator, FunctionKey as CoreFn,
    KeyEvent as CoreKeyEvent, LengthUnitKey as CoreUnit, MemoryOp as CoreMemOp,
    WeightUnitKey as CoreWeightUnit,
};
use calc_core::format::{
    AngleFormat as CoreAngleFormat, AreaFormat as CoreAreaFormat, LengthFormat as CoreLengthFormat,
    VolumeFormat as CoreVolumeFormat, WeightFormat as CoreWeightFormat,
};

// The UniFFI namespace IS the generated Swift module name — it must match the
// Swift app's `import CalcEngine` and the `CalcEngine*` filenames that
// `scripts/build-ios.sh` moves into the xcframework. (The crate is still
// `calc_uniffi` / `libcalc_uniffi.a`; the namespace is independent of that.)
uniffi::setup_scaffolding!("CalcEngine");

// ---------------------------------------------------------------------------
// Event enums — flat, Swift-friendly mirrors of the core types.
// ---------------------------------------------------------------------------

#[derive(uniffi::Enum, Clone, Copy, Debug)]
pub enum Op {
    Add,
    Sub,
    Mul,
    Div,
}

#[derive(uniffi::Enum, Clone, Copy, Debug)]
pub enum Unit {
    Feet,
    Inch,
    Yards,
    Meters,
}

/// Weight input unit keys. Pressing one tags the entered number as a weight.
#[derive(uniffi::Enum, Clone, Copy, Debug)]
pub enum WeightUnit {
    Pounds,
    Kilograms,
    Tons,
    Tonnes,
}

#[derive(uniffi::Enum, Clone, Copy, Debug)]
pub enum FunctionKey {
    // Rafter family
    Pitch,
    Rise,
    Run,
    Diagonal,
    HipValley,
    Jack,
    // Trig
    Sin,
    Cos,
    Tan,
    Asin,
    Acos,
    Atan,
    // Plain math
    Sqrt,
    Square,
    Reciprocal,
    Percent,
    // Material counts
    Studs,
    // Compound miter
    Corner,
    Spring,
    Miter,
    Bevel,
}

#[derive(uniffi::Enum, Clone, Copy, Debug)]
pub enum MemoryOp {
    Store { slot: u8 },
    Recall { slot: u8 },
    AddTo { slot: u8 },
    Clear { slot: u8 },
    ClearAll,
}

/// Display format for length results. `denom` on `FeetInchFraction` is
/// the rounding resolution and must be one of 2, 4, 8, 16.
#[derive(uniffi::Enum, Clone, Copy, Debug)]
pub enum LengthFormat {
    FeetInchFraction { denom: u32 },
    DecimalFeet { precision: u8 },
    DecimalInches { precision: u8 },
    Yards { precision: u8 },
    Meters { precision: u8 },
    Centimeters { precision: u8 },
    Millimeters { precision: u8 },
}

/// Display format for area results.
#[derive(uniffi::Enum, Clone, Copy, Debug)]
pub enum AreaFormat {
    SquareInches { precision: u8 },
    SquareFeet { precision: u8 },
    SquareYards { precision: u8 },
    SquareMeters { precision: u8 },
    Acres { precision: u8 },
}

/// Display format for volume results.
#[derive(uniffi::Enum, Clone, Copy, Debug)]
pub enum VolumeFormat {
    CubicInches { precision: u8 },
    CubicFeet { precision: u8 },
    CubicYards { precision: u8 },
    CubicMeters { precision: u8 },
    Gallons { precision: u8 },
    Liters { precision: u8 },
    BoardFeet { precision: u8 },
}

/// Display format for angle results: sexagesimal D°M'S" or decimal degrees.
#[derive(uniffi::Enum, Clone, Copy, Debug)]
pub enum AngleFormat {
    DegMinSec,
    DecimalDegrees { precision: u8 },
}

/// Display format for weight results.
#[derive(uniffi::Enum, Clone, Copy, Debug)]
pub enum WeightFormat {
    Pounds { precision: u8 },
    Kilograms { precision: u8 },
    Tons { precision: u8 },
    Tonnes { precision: u8 },
}

/// Dimensional category of the display value, so the iOS UI can show the
/// matching unit controls (length vs. area vs. volume).
#[derive(uniffi::Enum, Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dimension {
    Scalar,
    Length,
    Area,
    Volume,
    Angle,
    Money,
    Weight,
}

#[derive(uniffi::Enum, Clone, Debug)]
pub enum KeyEvent {
    Digit {
        value: u8,
    },
    Decimal,
    Slash,
    Negate,
    Op {
        op: Op,
    },
    Equals,
    Unit {
        unit: Unit,
    },
    /// Tag the entered number as a weight in the given unit.
    WeightUnit {
        unit: WeightUnit,
    },
    Function {
        function: FunctionKey,
    },
    /// Begin a cost-per-unit calculation: the current value is the quantity,
    /// the next entered number is the price per its current display unit, and
    /// `Equals` yields the total as money.
    CostPerUnit,
    Convert {
        format: LengthFormat,
    },
    /// Switch how an area result is displayed.
    ConvertArea {
        format: AreaFormat,
    },
    /// Switch how a volume result is displayed.
    ConvertVolume {
        format: VolumeFormat,
    },
    ConvertAngle {
        format: AngleFormat,
    },
    /// Switch how a weight result is displayed.
    ConvertWeight {
        format: WeightFormat,
    },
    /// Set whether trig keys use degrees (`true`) or radians (`false`).
    SetAngleMode {
        degrees: bool,
    },
    Memory {
        op: MemoryOp,
    },
    Backspace,
    Clear,
    ClearAll,
    /// Append a free-form labelled note to the tape without other state
    /// change. Used by the EZ Calc forms layer.
    Note {
        text: String,
    },
}

#[derive(uniffi::Record, Clone, Debug)]
pub struct Snapshot {
    /// The display register, already formatted for the user's chosen mode.
    pub display: String,
    /// Dimensional category of the display value, so the UI can show the
    /// matching unit controls (length vs. area vs. volume).
    pub dimension: Dimension,
    /// Set when the last event failed (e.g. divide by zero). The display
    /// is still valid — Swift can show this in a toast / banner.
    pub error: Option<String>,
    /// The full tape rendered as Markdown. Cheap to render and easy to
    /// show in a `Text` view. Use `Calculator::export_json()` if you need
    /// structured access.
    pub tape_markdown: String,
}

// ---------------------------------------------------------------------------
// Calculator object.
// ---------------------------------------------------------------------------

#[derive(uniffi::Object)]
pub struct Calculator {
    inner: Mutex<CoreCalculator>,
}

#[uniffi::export]
impl Calculator {
    #[uniffi::constructor]
    pub fn new() -> std::sync::Arc<Self> {
        std::sync::Arc::new(Self {
            inner: Mutex::new(CoreCalculator::new()),
        })
    }

    pub fn handle(&self, event: KeyEvent) -> Snapshot {
        let mut calc = self.inner.lock().expect("calc-uniffi mutex poisoned");
        let core_ev = into_core_event(event);
        let error = calc.handle(core_ev).err().map(|e| e.to_string());
        Snapshot {
            display: calc.display_string(),
            dimension: dimension_from_core(calc.display_dimension()),
            error,
            tape_markdown: calc.tape.to_markdown(),
        }
    }

    pub fn display_string(&self) -> String {
        self.inner.lock().unwrap().display_string()
    }

    pub fn export_markdown(&self) -> String {
        self.inner.lock().unwrap().tape.to_markdown()
    }

    pub fn export_json(&self) -> Result<String, CalcFfiError> {
        let calc = self.inner.lock().unwrap();
        serde_json::to_string_pretty(&calc.tape).map_err(|e| CalcFfiError::Serialize {
            message: e.to_string(),
        })
    }

    pub fn load_json_tape(&self, json: String) -> Result<(), CalcFfiError> {
        let mut calc = self.inner.lock().unwrap();
        let tape: calc_core::tape::Tape =
            serde_json::from_str(&json).map_err(|e| CalcFfiError::Parse {
                message: e.to_string(),
            })?;
        calc.tape = tape;
        Ok(())
    }

    pub fn clear_tape(&self) {
        self.inner.lock().unwrap().tape.clear();
    }
}

// ---------------------------------------------------------------------------
// Error type for fallible methods.
// ---------------------------------------------------------------------------

#[derive(Debug, thiserror::Error, uniffi::Error)]
pub enum CalcFfiError {
    #[error("serialize: {message}")]
    Serialize { message: String },
    #[error("parse: {message}")]
    Parse { message: String },
}

// ---------------------------------------------------------------------------
// Event translation.
// ---------------------------------------------------------------------------

fn into_core_event(ev: KeyEvent) -> CoreKeyEvent {
    match ev {
        KeyEvent::Digit { value } => CoreKeyEvent::Digit(value),
        KeyEvent::Decimal => CoreKeyEvent::Decimal,
        KeyEvent::Slash => CoreKeyEvent::Slash,
        KeyEvent::Negate => CoreKeyEvent::Negate,
        KeyEvent::Op { op } => CoreKeyEvent::Op(match op {
            Op::Add => CoreBinaryOp::Add,
            Op::Sub => CoreBinaryOp::Sub,
            Op::Mul => CoreBinaryOp::Mul,
            Op::Div => CoreBinaryOp::Div,
        }),
        KeyEvent::Equals => CoreKeyEvent::Equals,
        KeyEvent::Unit { unit } => CoreKeyEvent::Unit(match unit {
            Unit::Feet => CoreUnit::Feet,
            Unit::Inch => CoreUnit::Inch,
            Unit::Yards => CoreUnit::Yards,
            Unit::Meters => CoreUnit::Meters,
        }),
        KeyEvent::WeightUnit { unit } => CoreKeyEvent::WeightUnit(match unit {
            WeightUnit::Pounds => CoreWeightUnit::Pounds,
            WeightUnit::Kilograms => CoreWeightUnit::Kilograms,
            WeightUnit::Tons => CoreWeightUnit::Tons,
            WeightUnit::Tonnes => CoreWeightUnit::Tonnes,
        }),
        KeyEvent::Function { function } => CoreKeyEvent::Function(match function {
            FunctionKey::Pitch => CoreFn::Pitch,
            FunctionKey::Rise => CoreFn::Rise,
            FunctionKey::Run => CoreFn::Run,
            FunctionKey::Diagonal => CoreFn::Diagonal,
            FunctionKey::HipValley => CoreFn::HipValley,
            FunctionKey::Jack => CoreFn::Jack,
            FunctionKey::Sin => CoreFn::Sin,
            FunctionKey::Cos => CoreFn::Cos,
            FunctionKey::Tan => CoreFn::Tan,
            FunctionKey::Asin => CoreFn::Asin,
            FunctionKey::Acos => CoreFn::Acos,
            FunctionKey::Atan => CoreFn::Atan,
            FunctionKey::Sqrt => CoreFn::Sqrt,
            FunctionKey::Square => CoreFn::Square,
            FunctionKey::Reciprocal => CoreFn::Reciprocal,
            FunctionKey::Percent => CoreFn::Percent,
            FunctionKey::Studs => CoreFn::Studs,
            FunctionKey::Corner => CoreFn::Corner,
            FunctionKey::Spring => CoreFn::Spring,
            FunctionKey::Miter => CoreFn::Miter,
            FunctionKey::Bevel => CoreFn::Bevel,
        }),
        KeyEvent::Convert { format } => CoreKeyEvent::Convert(match format {
            LengthFormat::FeetInchFraction { denom } => {
                CoreLengthFormat::FeetInchFraction { denom }
            }
            LengthFormat::DecimalFeet { precision } => CoreLengthFormat::DecimalFeet { precision },
            LengthFormat::DecimalInches { precision } => {
                CoreLengthFormat::DecimalInches { precision }
            }
            LengthFormat::Yards { precision } => CoreLengthFormat::Yards { precision },
            LengthFormat::Meters { precision } => CoreLengthFormat::Meters { precision },
            LengthFormat::Centimeters { precision } => {
                CoreLengthFormat::Centimeters { precision }
            }
            LengthFormat::Millimeters { precision } => {
                CoreLengthFormat::Millimeters { precision }
            }
        }),
        KeyEvent::ConvertArea { format } => CoreKeyEvent::ConvertArea(match format {
            AreaFormat::SquareInches { precision } => CoreAreaFormat::SquareInches { precision },
            AreaFormat::SquareFeet { precision } => CoreAreaFormat::SquareFeet { precision },
            AreaFormat::SquareYards { precision } => CoreAreaFormat::SquareYards { precision },
            AreaFormat::SquareMeters { precision } => CoreAreaFormat::SquareMeters { precision },
            AreaFormat::Acres { precision } => CoreAreaFormat::Acres { precision },
        }),
        KeyEvent::ConvertVolume { format } => CoreKeyEvent::ConvertVolume(match format {
            VolumeFormat::CubicInches { precision } => CoreVolumeFormat::CubicInches { precision },
            VolumeFormat::CubicFeet { precision } => CoreVolumeFormat::CubicFeet { precision },
            VolumeFormat::CubicYards { precision } => CoreVolumeFormat::CubicYards { precision },
            VolumeFormat::CubicMeters { precision } => CoreVolumeFormat::CubicMeters { precision },
            VolumeFormat::Gallons { precision } => CoreVolumeFormat::Gallons { precision },
            VolumeFormat::Liters { precision } => CoreVolumeFormat::Liters { precision },
            VolumeFormat::BoardFeet { precision } => CoreVolumeFormat::BoardFeet { precision },
        }),
        KeyEvent::ConvertAngle { format } => CoreKeyEvent::ConvertAngle(match format {
            AngleFormat::DegMinSec => CoreAngleFormat::DegMinSec,
            AngleFormat::DecimalDegrees { precision } => {
                CoreAngleFormat::DecimalDegrees { precision }
            }
        }),
        KeyEvent::ConvertWeight { format } => CoreKeyEvent::ConvertWeight(match format {
            WeightFormat::Pounds { precision } => CoreWeightFormat::Pounds { precision },
            WeightFormat::Kilograms { precision } => CoreWeightFormat::Kilograms { precision },
            WeightFormat::Tons { precision } => CoreWeightFormat::Tons { precision },
            WeightFormat::Tonnes { precision } => CoreWeightFormat::Tonnes { precision },
        }),
        KeyEvent::CostPerUnit => CoreKeyEvent::CostPerUnit,
        KeyEvent::SetAngleMode { degrees } => CoreKeyEvent::SetAngleMode(degrees),
        KeyEvent::Memory { op } => CoreKeyEvent::Memory(match op {
            MemoryOp::Store { slot } => CoreMemOp::Store(slot),
            MemoryOp::Recall { slot } => CoreMemOp::Recall(slot),
            MemoryOp::AddTo { slot } => CoreMemOp::AddTo(slot),
            MemoryOp::Clear { slot } => CoreMemOp::Clear(slot),
            MemoryOp::ClearAll => CoreMemOp::ClearAll,
        }),
        KeyEvent::Backspace => CoreKeyEvent::Backspace,
        KeyEvent::Clear => CoreKeyEvent::Clear,
        KeyEvent::ClearAll => CoreKeyEvent::ClearAll,
        KeyEvent::Note { text } => CoreKeyEvent::Note(text),
    }
}

fn dimension_from_core(d: CoreDimension) -> Dimension {
    match d {
        CoreDimension::Scalar => Dimension::Scalar,
        CoreDimension::Length => Dimension::Length,
        CoreDimension::Area => Dimension::Area,
        CoreDimension::Volume => Dimension::Volume,
        CoreDimension::Angle => Dimension::Angle,
        CoreDimension::Money => Dimension::Money,
        CoreDimension::Weight => Dimension::Weight,
    }
}
