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
    /// Convert a Value to a number
    ///
    /// # Return value
    /// The value converted to a number
    pub fn num_value(&self) -> f32 {
        match self {
            Value::Number(val) => *val,
            Value::String(val) | Value::StrNum(val) => {
                let mut end_num_index = 0;
                let mut scientific_notation_flag = false;
                let mut plus_minus_prefix_flag = false;
                let mut decimal_flag = false;

                for char in val.chars() {
                    let is_scientific = char.to_ascii_lowercase() == 'e';
                    let is_one_time_prefix = char == '+' || char == '-';
                    let is_decimal = char == '.';

                    if char.is_numeric()
                        || (!decimal_flag && is_decimal)
                        || (!scientific_notation_flag && is_scientific)
                        || (!plus_minus_prefix_flag && is_one_time_prefix)
                    {
                        decimal_flag |= is_decimal;
                        scientific_notation_flag |= is_scientific;
                        plus_minus_prefix_flag |= is_one_time_prefix;

                        end_num_index += 1;
                    } else {
                        break;
                    }
                }

                val[0..end_num_index].parse().unwrap_or(0.0)
            }
        }
    }

    /// Convert a Value to a string
    ///
    /// # Return value
    /// The value converted to a string
    pub fn str_value(&self) -> String {
        match self {
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
            Value::Number(val) => write!(f, "{}", val.to_string()),
            Value::String(val) => write!(f, "{}", val.as_str()),
            Value::StrNum(val) => write!(f, "{}", val.as_str()),
        }
    }
}

#[cfg(test)]
mod value {
    use crate::value::Value;

    #[test]
    fn it_displays_a_number_with_no_decimal_points() {
        assert_eq!(Value::Number(4.0).to_string(), "4");
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

    #[test]
    fn num_value_returns_the_same_number() {
        let expected_number = 9.09;

        assert_eq!(Value::Number(expected_number).num_value(), expected_number);
    }

    #[test]
    fn num_value_converts_a_number_represented_as_string() {
        assert_eq!(Value::String(String::from("20")).num_value(), 20.0);
    }

    #[test]
    fn num_value_converts_a_float_represented_as_string() {
        assert_eq!(Value::String(String::from("20.21")).num_value(), 20.21);
    }

    #[test]
    fn num_value_converts_a_float_with_too_many_decimals_represented_as_string() {
        assert_eq!(Value::String(String::from("20.21.22")).num_value(), 20.21);
    }

    #[test]
    fn num_value_converts_a_string_with_numbers_and_chars() {
        assert_eq!(Value::String(String::from("20Hello87")).num_value(), 20.0);
    }

    #[test]
    fn num_value_supports_scientific_notation_strings() {
        assert_eq!(Value::String(String::from("1e2")).num_value(), 1e2);
    }

    #[test]
    fn num_value_supports_capitalized_scientific_notation_strings() {
        assert_eq!(Value::String(String::from("1E2")).num_value(), 1e2);
    }

    #[test]
    fn num_value_supports_leading_plus_prefix() {
        assert_eq!(Value::String(String::from("+1.2")).num_value(), 1.2);
    }

    #[test]
    fn num_value_supports_leading_minus_prefix() {
        assert_eq!(Value::String(String::from("-3.4")).num_value(), -3.4);
    }

    #[test]
    fn num_value_defaults_to_zero_with_two_leading_plus_prefixes() {
        assert_eq!(Value::String(String::from("++1.2")).num_value(), 0.0);
    }

    #[test]
    fn num_value_defaults_to_zero_with_two_leading_minus_prefixes() {
        assert_eq!(Value::String(String::from("--3.4")).num_value(), 0.0);
    }

    #[test]
    fn num_value_defaults_to_zero_with_leading_plus_and_minus_prefix() {
        assert_eq!(Value::String(String::from("+-5.6")).num_value(), 0.0);
    }

    #[test]
    fn num_value_defaults_to_zero_with_leading_minus_and_plus_prefix() {
        assert_eq!(Value::String(String::from("-+7.8")).num_value(), 0.0);
    }

    #[test]
    fn str_value_parses_a_number_to_string() {
        assert_eq!(Value::Number(3.21).str_value(), String::from("3.21"));
    }

    #[test]
    fn str_value_parses_a_string_to_string() {
        assert_eq!(
            Value::String(String::from("3.21")).str_value(),
            String::from("3.21")
        );
    }

    #[test]
    fn str_value_parses_a_string_number_to_string() {
        assert_eq!(
            Value::StrNum(String::from("3.21")).str_value(),
            String::from("3.21")
        );
    }
}
