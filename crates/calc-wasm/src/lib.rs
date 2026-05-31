//! Web assembly bindings for the calculator.

use calc_core::calculator::{BinaryOp, Calculator, FunctionKey, KeyEvent, LengthUnitKey, MemoryOp};
use calc_core::format::{AreaFormat, LengthFormat, VolumeFormat};
use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct WasmCalculator {
    inner: Calculator,
}

#[derive(Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "camelCase")]
enum WasmKey {
    Digit {
        value: u8,
    },
    Decimal,
    Slash,
    Negate,
    Op {
        op: String,
    },
    Equals,
    Unit {
        unit: String,
    },
    Function {
        fun: String,
    },
    Convert {
        format: String,
        denom: Option<u32>,
        precision: Option<u8>,
    },
    ConvertArea {
        format: String,
        precision: Option<u8>,
    },
    ConvertVolume {
        format: String,
        precision: Option<u8>,
    },
    SetAngleMode {
        degrees: bool,
    },
    Memory {
        op: String,
        slot: Option<u8>,
    },
    Backspace,
    Clear,
    ClearAll,
    Note {
        text: String,
    },
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct Snapshot {
    display: String,
    /// Dimensional category of the display value ("scalar" / "length" /
    /// "area" / "volume" / "angle") so the UI can show the matching format
    /// controls without parsing `display`.
    dimension: String,
    tape: serde_json::Value,
    error: Option<String>,
}

impl Default for WasmCalculator {
    fn default() -> Self {
        Self::new()
    }
}

#[wasm_bindgen]
impl WasmCalculator {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Self {
        #[cfg(feature = "console_error_panic_hook")]
        console_error_panic_hook::set_once();
        Self {
            inner: Calculator::new(),
        }
    }

    #[wasm_bindgen]
    pub fn handle(&mut self, json: JsValue) -> Result<JsValue, JsError> {
        let wk: WasmKey = serde_wasm_bindgen::from_value(json)
            .map_err(|e| JsError::new(&format!("bad event: {e}")))?;
        let ev = decode_key(wk).map_err(|e| JsError::new(&e))?;
        let mut error = None;
        if let Err(e) = self.inner.handle(ev) {
            error = Some(format!("{}", e));
        }
        let snap = Snapshot {
            display: self.inner.display_string(),
            dimension: self.inner.display_dimension().as_str().to_string(),
            tape: serde_json::to_value(&self.inner.tape).unwrap_or(serde_json::Value::Null),
            error,
        };
        serde_wasm_bindgen::to_value(&snap).map_err(|e| JsError::new(&format!("ser: {e}")))
    }

    #[wasm_bindgen(js_name = displayString)]
    pub fn display_string(&self) -> String {
        self.inner.display_string()
    }

    #[wasm_bindgen(js_name = exportMarkdown)]
    pub fn export_markdown(&self) -> String {
        self.inner.tape.to_markdown()
    }

    #[wasm_bindgen(js_name = exportJson)]
    pub fn export_json(&self) -> Result<String, JsError> {
        serde_json::to_string_pretty(&self.inner.tape)
            .map_err(|e| JsError::new(&format!("export: {e}")))
    }

    #[wasm_bindgen(js_name = loadJsonTape)]
    pub fn load_json_tape(&mut self, json: &str) -> Result<(), JsError> {
        let tape: calc_core::tape::Tape =
            serde_json::from_str(json).map_err(|e| JsError::new(&format!("parse: {e}")))?;
        self.inner.tape = tape;
        Ok(())
    }

    #[wasm_bindgen(js_name = clearTape)]
    pub fn clear_tape(&mut self) {
        self.inner.tape.clear();
    }
}

fn decode_key(k: WasmKey) -> Result<KeyEvent, String> {
    Ok(match k {
        WasmKey::Digit { value } => KeyEvent::Digit(value),
        WasmKey::Decimal => KeyEvent::Decimal,
        WasmKey::Slash => KeyEvent::Slash,
        WasmKey::Negate => KeyEvent::Negate,
        WasmKey::Op { op } => KeyEvent::Op(match op.as_str() {
            "add" => BinaryOp::Add,
            "sub" => BinaryOp::Sub,
            "mul" => BinaryOp::Mul,
            "div" => BinaryOp::Div,
            other => return Err(format!("unknown op {other}")),
        }),
        WasmKey::Equals => KeyEvent::Equals,
        WasmKey::Unit { unit } => KeyEvent::Unit(match unit.as_str() {
            "ft" | "feet" => LengthUnitKey::Feet,
            "in" | "inch" => LengthUnitKey::Inch,
            "yd" | "yards" => LengthUnitKey::Yards,
            "m" => LengthUnitKey::Meters,
            other => return Err(format!("unknown unit {other}")),
        }),
        WasmKey::Function { fun } => KeyEvent::Function(match fun.as_str() {
            "pitch" => FunctionKey::Pitch,
            "rise" => FunctionKey::Rise,
            "run" => FunctionKey::Run,
            "diag" | "diagonal" => FunctionKey::Diagonal,
            "hipv" | "hip_valley" => FunctionKey::HipValley,
            "jack" => FunctionKey::Jack,
            "sin" => FunctionKey::Sin,
            "cos" => FunctionKey::Cos,
            "tan" => FunctionKey::Tan,
            "asin" => FunctionKey::Asin,
            "acos" => FunctionKey::Acos,
            "atan" => FunctionKey::Atan,
            "sqrt" => FunctionKey::Sqrt,
            "square" => FunctionKey::Square,
            "recip" | "reciprocal" => FunctionKey::Reciprocal,
            "percent" => FunctionKey::Percent,
            "corner" => FunctionKey::Corner,
            "spring" => FunctionKey::Spring,
            "miter" => FunctionKey::Miter,
            "bevel" => FunctionKey::Bevel,
            other => return Err(format!("unknown function {other}")),
        }),
        WasmKey::Convert {
            format,
            denom,
            precision,
        } => {
            let fmt = match format.as_str() {
                "feet_inch_fraction" => LengthFormat::FeetInchFraction {
                    denom: denom.unwrap_or(16),
                },
                "decimal_feet" => LengthFormat::DecimalFeet {
                    precision: precision.unwrap_or(4),
                },
                "decimal_inches" => LengthFormat::DecimalInches {
                    precision: precision.unwrap_or(4),
                },
                "yards" => LengthFormat::Yards {
                    precision: precision.unwrap_or(4),
                },
                "m" => LengthFormat::Meters {
                    precision: precision.unwrap_or(4),
                },
                other => return Err(format!("unknown format {other}")),
            };
            KeyEvent::Convert(fmt)
        }
        WasmKey::ConvertArea { format, precision } => {
            let p = precision.unwrap_or(2);
            let fmt = match format.as_str() {
                "sq_in" | "square_inches" => AreaFormat::SquareInches { precision: p },
                "sq_ft" | "square_feet" => AreaFormat::SquareFeet { precision: p },
                "sq_yd" | "square_yards" => AreaFormat::SquareYards { precision: p },
                "sq_m" | "square_meters" => AreaFormat::SquareMeters { precision: p },
                "acres" | "acre" => AreaFormat::Acres { precision: p },
                other => return Err(format!("unknown area format {other}")),
            };
            KeyEvent::ConvertArea(fmt)
        }
        WasmKey::ConvertVolume { format, precision } => {
            let p = precision.unwrap_or(2);
            let fmt = match format.as_str() {
                "cu_in" | "cubic_inches" => VolumeFormat::CubicInches { precision: p },
                "cu_ft" | "cubic_feet" => VolumeFormat::CubicFeet { precision: p },
                "cu_yd" | "cubic_yards" => VolumeFormat::CubicYards { precision: p },
                "cu_m" | "cubic_meters" => VolumeFormat::CubicMeters { precision: p },
                "gallons" | "gal" => VolumeFormat::Gallons { precision: p },
                "liters" | "l" => VolumeFormat::Liters { precision: p },
                other => return Err(format!("unknown volume format {other}")),
            };
            KeyEvent::ConvertVolume(fmt)
        }
        WasmKey::SetAngleMode { degrees } => KeyEvent::SetAngleMode(degrees),
        WasmKey::Memory { op, slot } => {
            let s = slot.unwrap_or(0);
            KeyEvent::Memory(match op.as_str() {
                "store" => MemoryOp::Store(s),
                "recall" => MemoryOp::Recall(s),
                "add" => MemoryOp::AddTo(s),
                "clear" => MemoryOp::Clear(s),
                "clear_all" => MemoryOp::ClearAll,
                other => return Err(format!("unknown memory op {other}")),
            })
        }
        WasmKey::Backspace => KeyEvent::Backspace,
        WasmKey::Clear => KeyEvent::Clear,
        WasmKey::ClearAll => KeyEvent::ClearAll,
        WasmKey::Note { text } => KeyEvent::Note(text),
    })
}
