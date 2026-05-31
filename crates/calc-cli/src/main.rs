//! Tiny REPL for poking at calc-core from the terminal.
//!
//! Each line is a key-event sequence using a compact mini-language so you
//! can drive the calculator without a UI:
//!
//!   5 ft 6 in + 2 ft 7 in =
//!   6 pitch 10 ft run rise
//!   convert m
//!
//! Tokens:
//!   digits like 12 or 0.5 or 3/8 → typed verbatim into the entry buffer
//!   `ft`, `in`, `yd`, `m` → unit commit
//!   `+`, `-`, `*`, `/` → operators
//!   `=` → equals
//!   `pitch`, `rise`, `run`, `diag` → rafter functions
//!   `clear`, `ce`, `bs` → clear, clear-all, backspace
//!   `convert <unit>` → change display mode

use calc_core::calculator::{
    BinaryOp, Calculator, FunctionKey, KeyEvent, LengthUnitKey, WeightUnitKey,
};
use calc_core::format::{AngleFormat, AreaFormat, LengthFormat, VolumeFormat, WeightFormat};
use std::io::{self, BufRead, Write};

fn main() {
    let mut calc = Calculator::new();
    let stdin = io::stdin();
    let stdout = io::stdout();
    let mut out = stdout.lock();

    println!("calc-cli — type `help` for the mini-language, `quit` to exit");
    print_prompt(&calc, &mut out);

    for line in stdin.lock().lines() {
        let line = match line {
            Ok(l) => l,
            Err(_) => break,
        };
        let trimmed = line.trim();
        if trimmed.is_empty() {
            print_prompt(&calc, &mut out);
            continue;
        }
        if trimmed == "quit" || trimmed == "exit" {
            break;
        }
        if trimmed == "help" {
            print_help();
            print_prompt(&calc, &mut out);
            continue;
        }

        if let Err(e) = run_line(&mut calc, trimmed) {
            eprintln!("error: {e}");
        } else {
            println!("= {}", calc.display_string());
        }
        print_prompt(&calc, &mut out);
    }
}

fn print_prompt(_c: &Calculator, out: &mut impl Write) {
    let _ = write!(out, "> ");
    let _ = out.flush();
}

fn print_help() {
    println!("Tokens:");
    println!("  numbers: 12  0.5  3/8  5-1/2");
    println!("  units:   ft in yd m");
    println!("  weight:  lb kg ton tonne  (tag the entered number as a weight)");
    println!("  ops:     + - * / =");
    println!("  rafter:  pitch rise run diag");
    println!("  state:   clear (= CE), clearall (= AC), bs (backspace)");
    println!("  display: convert <feet|inch|m|yd>  (feet rounds to 1/4, 1/8, or 1/16)");
    println!("  area:    area <sqin|sqft|sqyd|sqm|acre>  (how an area result shows)");
    println!("  volume:  vol <cuin|cuft|cuyd|cum|gal|l>  (how a volume result shows)");
    println!("  angfmt:  afmt <dms|dd>  (angle display: D°M'S\" or decimal degrees)");
    println!("  wgtfmt:  wfmt <lb|kg|ton|tonne>  (how a weight result shows)");
    println!("  angle:   angle <deg|rad>  (how trig keys read a plain number)");
    println!("  cost:    <qty> cost <price> =  (price per the shown unit -> $total)");
    println!("Example: 5 ft 6 in + 2 ft 7 in =");
}

fn run_line(calc: &mut Calculator, line: &str) -> Result<(), String> {
    let mut tokens = line.split_whitespace().peekable();
    while let Some(tok) = tokens.next() {
        match tok {
            "+" => calc.handle(KeyEvent::Op(BinaryOp::Add)).map_err(s)?,
            "-" => calc.handle(KeyEvent::Op(BinaryOp::Sub)).map_err(s)?,
            "*" | "x" => calc.handle(KeyEvent::Op(BinaryOp::Mul)).map_err(s)?,
            "/" => calc.handle(KeyEvent::Op(BinaryOp::Div)).map_err(s)?,
            "=" => calc.handle(KeyEvent::Equals).map_err(s)?,
            "ft" | "feet" => calc
                .handle(KeyEvent::Unit(LengthUnitKey::Feet))
                .map_err(s)?,
            "in" | "inch" => calc
                .handle(KeyEvent::Unit(LengthUnitKey::Inch))
                .map_err(s)?,
            "yd" => calc
                .handle(KeyEvent::Unit(LengthUnitKey::Yards))
                .map_err(s)?,
            "m" => calc
                .handle(KeyEvent::Unit(LengthUnitKey::Meters))
                .map_err(s)?,
            "lb" | "lbs" => calc
                .handle(KeyEvent::WeightUnit(WeightUnitKey::Pounds))
                .map_err(s)?,
            "kg" => calc
                .handle(KeyEvent::WeightUnit(WeightUnitKey::Kilograms))
                .map_err(s)?,
            "ton" => calc
                .handle(KeyEvent::WeightUnit(WeightUnitKey::Tons))
                .map_err(s)?,
            "tonne" => calc
                .handle(KeyEvent::WeightUnit(WeightUnitKey::Tonnes))
                .map_err(s)?,
            "pitch" => calc
                .handle(KeyEvent::Function(FunctionKey::Pitch))
                .map_err(s)?,
            "rise" => calc
                .handle(KeyEvent::Function(FunctionKey::Rise))
                .map_err(s)?,
            "run" => calc
                .handle(KeyEvent::Function(FunctionKey::Run))
                .map_err(s)?,
            "diag" | "diagonal" => calc
                .handle(KeyEvent::Function(FunctionKey::Diagonal))
                .map_err(s)?,
            "hipv" | "hip" => calc
                .handle(KeyEvent::Function(FunctionKey::HipValley))
                .map_err(s)?,
            "jack" => calc
                .handle(KeyEvent::Function(FunctionKey::Jack))
                .map_err(s)?,
            "corner" => calc
                .handle(KeyEvent::Function(FunctionKey::Corner))
                .map_err(s)?,
            "spring" => calc
                .handle(KeyEvent::Function(FunctionKey::Spring))
                .map_err(s)?,
            "miter" => calc
                .handle(KeyEvent::Function(FunctionKey::Miter))
                .map_err(s)?,
            "bevel" => calc
                .handle(KeyEvent::Function(FunctionKey::Bevel))
                .map_err(s)?,
            "sin" => calc
                .handle(KeyEvent::Function(FunctionKey::Sin))
                .map_err(s)?,
            "cos" => calc
                .handle(KeyEvent::Function(FunctionKey::Cos))
                .map_err(s)?,
            "tan" => calc
                .handle(KeyEvent::Function(FunctionKey::Tan))
                .map_err(s)?,
            "asin" => calc
                .handle(KeyEvent::Function(FunctionKey::Asin))
                .map_err(s)?,
            "acos" => calc
                .handle(KeyEvent::Function(FunctionKey::Acos))
                .map_err(s)?,
            "atan" => calc
                .handle(KeyEvent::Function(FunctionKey::Atan))
                .map_err(s)?,
            "sqrt" => calc
                .handle(KeyEvent::Function(FunctionKey::Sqrt))
                .map_err(s)?,
            "sq" | "square" => calc
                .handle(KeyEvent::Function(FunctionKey::Square))
                .map_err(s)?,
            "recip" | "1/x" => calc
                .handle(KeyEvent::Function(FunctionKey::Reciprocal))
                .map_err(s)?,
            "%" | "pct" => calc
                .handle(KeyEvent::Function(FunctionKey::Percent))
                .map_err(s)?,
            "cost" => calc.handle(KeyEvent::CostPerUnit).map_err(s)?,
            "clear" | "ce" => calc.handle(KeyEvent::Clear).map_err(s)?,
            "clearall" | "ac" => calc.handle(KeyEvent::ClearAll).map_err(s)?,
            "bs" => calc.handle(KeyEvent::Backspace).map_err(s)?,
            "convert" => {
                let target = tokens
                    .next()
                    .ok_or_else(|| "convert needs unit".to_string())?;
                let fmt = match target {
                    "feet" | "fif" => LengthFormat::FeetInchFraction { denom: 16 },
                    "feet8" => LengthFormat::FeetInchFraction { denom: 8 },
                    "feet4" => LengthFormat::FeetInchFraction { denom: 4 },
                    "inch" => LengthFormat::DecimalInches { precision: 4 },
                    "m" => LengthFormat::Meters { precision: 4 },
                    "yd" => LengthFormat::Yards { precision: 4 },
                    other => return Err(format!("unknown convert target {other}")),
                };
                calc.handle(KeyEvent::Convert(fmt)).map_err(s)?
            }
            "area" => {
                let target = tokens
                    .next()
                    .ok_or_else(|| "area needs sqin|sqft|sqyd|sqm|acre".to_string())?;
                let fmt = match target {
                    "sqin" => AreaFormat::SquareInches { precision: 2 },
                    "sqft" => AreaFormat::SquareFeet { precision: 2 },
                    "sqyd" => AreaFormat::SquareYards { precision: 2 },
                    "sqm" => AreaFormat::SquareMeters { precision: 4 },
                    "acre" | "acres" => AreaFormat::Acres { precision: 4 },
                    other => return Err(format!("unknown area target {other}")),
                };
                calc.handle(KeyEvent::ConvertArea(fmt)).map_err(s)?
            }
            "vol" | "volume" => {
                let target = tokens
                    .next()
                    .ok_or_else(|| "vol needs cuin|cuft|cuyd|cum|gal|l".to_string())?;
                let fmt = match target {
                    "cuin" => VolumeFormat::CubicInches { precision: 2 },
                    "cuft" => VolumeFormat::CubicFeet { precision: 2 },
                    "cuyd" => VolumeFormat::CubicYards { precision: 2 },
                    "cum" => VolumeFormat::CubicMeters { precision: 4 },
                    "gal" | "gallons" => VolumeFormat::Gallons { precision: 2 },
                    "l" | "liters" => VolumeFormat::Liters { precision: 2 },
                    other => return Err(format!("unknown volume target {other}")),
                };
                calc.handle(KeyEvent::ConvertVolume(fmt)).map_err(s)?
            }
            "afmt" => {
                let target = tokens
                    .next()
                    .ok_or_else(|| "afmt needs dms|dd".to_string())?;
                let fmt = match target {
                    "dms" => AngleFormat::DegMinSec,
                    "dd" | "deg" => AngleFormat::DecimalDegrees { precision: 4 },
                    other => return Err(format!("unknown angle format {other}")),
                };
                calc.handle(KeyEvent::ConvertAngle(fmt)).map_err(s)?
            }
            "wfmt" => {
                let target = tokens
                    .next()
                    .ok_or_else(|| "wfmt needs lb|kg|ton|tonne".to_string())?;
                let fmt = match target {
                    "lb" => WeightFormat::Pounds { precision: 2 },
                    "kg" => WeightFormat::Kilograms { precision: 2 },
                    "ton" => WeightFormat::Tons { precision: 3 },
                    "tonne" => WeightFormat::Tonnes { precision: 3 },
                    other => return Err(format!("unknown weight format {other}")),
                };
                calc.handle(KeyEvent::ConvertWeight(fmt)).map_err(s)?
            }
            "angle" => {
                let target = tokens
                    .next()
                    .ok_or_else(|| "angle needs deg|rad".to_string())?;
                let degrees = match target {
                    "deg" | "degrees" => true,
                    "rad" | "radians" => false,
                    other => return Err(format!("unknown angle mode {other}")),
                };
                calc.handle(KeyEvent::SetAngleMode(degrees)).map_err(s)?
            }
            tok => type_number(calc, tok)?,
        }
    }
    Ok(())
}

fn s<E: std::fmt::Display>(e: E) -> String {
    format!("{e}")
}

/// Drive the entry buffer to type a literal number token like "5", "0.5",
/// "3/8", or "5-1/2".
fn type_number(calc: &mut Calculator, tok: &str) -> Result<(), String> {
    for ch in tok.chars() {
        match ch {
            '0'..='9' => calc
                .handle(KeyEvent::Digit(ch.to_digit(10).unwrap() as u8))
                .map_err(s)?,
            '.' => calc.handle(KeyEvent::Decimal).map_err(s)?,
            '/' => calc.handle(KeyEvent::Slash).map_err(s)?,
            '-' => {
                // dash is the mixed-number separator: insert a slash if a
                // fraction is coming, otherwise toggle sign.
                // For simplicity here: treat dash inside numeric tokens as
                // a "now switch to fraction" hint, which is what the buffer
                // does anyway when it hits a slash. Easiest: ignore — it's
                // visual only.
            }
            _ => return Err(format!("unexpected char in number: {ch}")),
        }
    }
    Ok(())
}
