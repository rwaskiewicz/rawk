//! Module to describe Values under the hood in r-awk

use std::fmt;

/// Enum whose variants are underlying data types
#[derive(Clone, Debug, PartialEq)]
pub enum Value {
    Number(f32),
    String(String),
    StrNum(String),
}

impl Value {
    pub fn is_number(&self) -> bool {
        return matches!(self, Value::Number(_));
    }

    pub fn is_string(&self) -> bool {
        return matches!(self, Value::String(_));
    }

    pub fn is_string_number(&self) -> bool {
        return matches!(self, Value::StrNum(_));
    }

    pub fn num_value(&self) -> f32 {
        match self {
            Value::Number(val) => *val,
            Value::String(val) | Value::StrNum(val) => {
                //echo 'hi' | awk '{print 2 + "-2fixx"}', echo 'hi' | awk '{print 2 + "---2fixx"}'
                // TODO: This is basic for now, need to make this much more flexible
                let mut end_num_index = 0;
                for char in val.chars() {
                    if char.is_numeric() || char == '"' {
                        end_num_index += 1;
                    } else {
                        break;
                    }
                }

                let prelim = &val[0..end_num_index];
                let result = str::replace(prelim, "\"", "");

                result.parse().unwrap()
            }
        }
    }

    pub fn str_value(&self) -> String {
        match self {
            // TODO: This isn't 100% right yet..
            Value::Number(val) => val.to_string(),
            Value::String(val) => val.clone(),
            Value::StrNum(val) => val.clone(),
        }
    }
}

/// Display trait implementation for Value
impl fmt::Display for Value {
    /// Format the Value
    ///
    /// # Arguments
    /// - `fmt` the Formatter to use to print the value
    ///
    /// # Return value
    /// The resulting formatted string
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Value::Number(val) => {
                // TODO: This is a hack as I can't understand formatter docs
                // https://stackoverflow.com/questions/37506672/convert-float-to-integer-in-rust?rq=1
                let mut num = val.to_string();
                if val.fract() == 0.0 {
                    num.push_str(".0");
                }
                write!(f, "{}", num)
            }
            Value::String(val) => write!(f, "{}", val.as_str()),
            Value::StrNum(val) => write!(f, "{}", val.as_str()),
        }
    }
}

#[cfg(test)]
mod value {
    use crate::value::Value;

    #[test]
    fn it_displays_a_number_with_one_decimal_point() {
        assert_eq!(Value::Number(4.0).to_string(), "4.0");
    }

    #[test]
    fn it_displays_a_number_with_needed_precision() {
        assert_eq!(Value::Number(4.0123).to_string(), "4.0123");
    }

    #[test]
    fn it_displays_a_string() {
        let input = "Hello World";
        assert_eq!(Value::String(String::from(input)).to_string(), input);
    }

    #[test]
    fn it_displays_a_numeric_string() {
        let input = "   +3.14";
        assert_eq!(Value::StrNum(String::from(input)).to_string(), input);
    }
}
