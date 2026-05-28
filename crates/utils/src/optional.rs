pub fn is_nil<T>(value: &Option<T>) -> bool {
    value.is_none()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn detects_none_as_nil() {
        assert!(is_nil(&None::<i32>));
        assert!(!is_nil(&Some(0)));
    }
}
