use super::process;
use super::types::FormatSpec;
use super::types::DECIMAL_CHAR;
use super::types::PREFIXES;

/// Format a number to a specific human readable form defined by the format spec pattern.
/// The method takes in a string specifier and a number and returns the string representation
/// of the formatted number.
pub fn format<T: Into<f64>>(pattern: &str, input: T) -> String {
    let format_spec: FormatSpec = pattern.into();

    let input_f64: f64 = input.into();
    let mut value_is_negative: bool = input_f64.is_sign_negative();

    let mut decimal_part = String::new();
    let mut si_prefix_exponent: &str = "";
    let unit_of_measurement: &str = match format_spec.format_type {
        Some("%") => "%",
        _ => "",
    };

    let mut value = match format_spec.format_type {
        Some("%") => format!(
            "{:.1$}",
            input_f64.abs() * 100_f64,
            format_spec.precision.unwrap() as usize
        ),
        Some("b") => format!("{:#b}", input_f64.abs() as i64)[2..].into(),
        Some("o") | Some("O") => format!("{:#o}", input_f64.abs() as i64)[2..].into(),
        Some("x") => format!("{:#x}", input_f64.abs() as i64)[2..].into(),
        Some("X") => format!("{:#X}", input_f64.abs() as i64)[2..].into(),
        Some("f") if format_spec.symbol.unwrap_or_default() == "#" => {
            let maybe_decimal = if format_spec.precision.unwrap() == 0 {
                DECIMAL_CHAR.to_string()
            } else {
                "".to_string()
            };
            format!(
                "{:.2$}{}",
                input_f64.abs(),
                maybe_decimal,
                format_spec.precision.unwrap() as usize
            )
        }
        Some("e") => process::get_formatted_exp_value(
            "e",
            input_f64.abs(),
            format_spec.precision.unwrap() as usize,
            format_spec.symbol.unwrap_or_default() == "#",
        ),
        Some("E") => process::get_formatted_exp_value(
            "E",
            input_f64.abs(),
            format_spec.precision.unwrap() as usize,
            format_spec.symbol.unwrap_or_default() == "#",
        ),
        Some("s") => {
            let (val, si_prefix) =
                process::format_si_prefix(input_f64.abs(), format_spec.precision);
            si_prefix_exponent = PREFIXES[(8 + si_prefix) as usize];
            val
        }
        _ => format!(
            "{:.1$}",
            input_f64.abs(),
            format_spec.precision.unwrap() as usize
        ),
    };

    // If a negative value rounds to zero after formatting, and no explicit positive sign is requested, hide the sign.
    if format_spec.format_type != Some("x")
        && format_spec.format_type != Some("X")
        && value_is_negative
        && value.parse::<f64>().unwrap() == 0_f64
        && format_spec.sign.unwrap_or("+") != "+"
    {
        value_is_negative = false;
    }

    let sign_prefix = process::get_sign_prefix(value_is_negative, &format_spec);

    let leading_part = match format_spec.symbol {
        Some("#") => match format_spec.format_type {
            Some("b") => "0b",
            Some("o") => "0o",
            Some("x") => "0x",
            Some("O") => "0O",
            Some("X") => "0x",
            _ => "",
        },
        _ => "",
    };

    // Split the integer part of the value for grouping purposes and attach the decimal part as suffix.
    for (i, c) in value.chars().enumerate() {
        if "0123456789".find(c).is_none() {
            decimal_part = value[i..].to_owned();
            value = value[..i].to_owned();
            break;
        }
    }

    // Compute the prefix and suffix.
    let prefix = format!("{}{}", sign_prefix, leading_part);
    let suffix = format!(
        "{}{}{}",
        decimal_part, si_prefix_exponent, unit_of_measurement
    );

    // If should group and filling character is different than "0",
    // group digits before applying padding.
    if format_spec.grouping.is_some() && !format_spec.zero {
        value = process::group_value(&value, 0)
    }

    // Compute the padding.
    let length = prefix.len() + value.to_string().len() + suffix.len();
    let mut padding = if length < format_spec.width.unwrap() {
        vec![format_spec.fill.unwrap(); format_spec.width.unwrap() - length].join("")
    } else {
        "".to_owned()
    };

    // If "0" is the filling character, grouping is applied after computing padding.
    if format_spec.grouping.is_some() && format_spec.zero {
        value = process::group_value(
            format!("{}{}", &padding, value).as_str(),
            if !padding.is_empty() {
                format_spec.width.unwrap() - suffix.len()
            } else {
                0
            },
        );
        padding = "".to_owned();
    };

    match format_spec.align {
        Some("<") => format!("{}{}{}{}", prefix, value, suffix, padding),
        Some("=") => format!("{}{}{}{}", prefix, padding, value, suffix),
        Some("^") => format!(
            "{}{}{}{}{}",
            &padding[..padding.len() / 2],
            prefix,
            value,
            suffix,
            &padding[padding.len() / 2..]
        ),
        _ => format!("{}{}{}{}", padding, prefix, value, suffix),
    }
}
