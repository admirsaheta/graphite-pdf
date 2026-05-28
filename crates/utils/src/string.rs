#[derive(Clone, Copy, Debug, PartialEq)]
pub struct PercentMatch {
    pub value: f32,
    pub percent: f32,
}

pub fn upper_first(input: &str) -> String {
    let mut chars = input.chars();

    match chars.next() {
        Some(first) => {
            let mut result = String::new();
            result.extend(first.to_uppercase());
            result.push_str(chars.as_str());
            result
        }
        None => String::new(),
    }
}

pub fn capitalize(input: &str) -> String {
    let mut result = String::with_capacity(input.len());
    let mut should_capitalize = true;

    for ch in input.chars() {
        if ch.is_whitespace() {
            should_capitalize = true;
            result.push(ch);
            continue;
        }

        if should_capitalize {
            result.extend(ch.to_uppercase());
            should_capitalize = false;
        } else {
            result.push(ch);
        }
    }

    result
}

pub fn match_percent(input: &str) -> Option<PercentMatch> {
    let trimmed = input.trim();

    if !trimmed.ends_with('%') {
        return None;
    }

    let numeric = trimmed[..trimmed.len().saturating_sub(1)].trim();
    let value = numeric.parse::<f32>().ok()?;

    Some(PercentMatch {
        value,
        percent: value / 100.0,
    })
}

pub fn parse_float(input: &str) -> Option<f32> {
    let trimmed = input.trim();
    let mut consumed = 0_usize;
    let mut seen_digit = false;
    let mut seen_decimal = false;

    for (index, ch) in trimmed.char_indices() {
        let allowed = if index == 0 && (ch == '+' || ch == '-') {
            true
        } else if ch.is_ascii_digit() {
            seen_digit = true;
            true
        } else if ch == '.' && !seen_decimal {
            seen_decimal = true;
            true
        } else {
            false
        };

        if !allowed {
            break;
        }

        consumed = index + ch.len_utf8();
    }

    if !seen_digit || consumed == 0 {
        return None;
    }

    trimmed[..consumed].parse::<f32>().ok()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn capitalizes_words_and_first_letter() {
        assert_eq!(capitalize("hello world"), "Hello World");
        assert_eq!(upper_first("hello"), "Hello");
    }

    #[test]
    fn parses_percentages() {
        assert_eq!(
            match_percent("50%"),
            Some(PercentMatch {
                value: 50.0,
                percent: 0.5,
            })
        );
        assert_eq!(match_percent("abc"), None);
    }

    #[test]
    fn parses_leading_floats() {
        assert_eq!(parse_float("3.14"), Some(314.0_f32 / 100.0));
        assert_eq!(parse_float("10px"), Some(10.0));
        assert_eq!(parse_float("abc"), None);
    }
}
