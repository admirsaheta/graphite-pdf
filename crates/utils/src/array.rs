#[derive(Clone, Debug, PartialEq, Eq)]
pub enum OneOrMany<T> {
    One(T),
    Many(Vec<T>),
}

impl<T> From<T> for OneOrMany<T> {
    fn from(value: T) -> Self {
        Self::One(value)
    }
}

impl<T> From<Vec<T>> for OneOrMany<T> {
    fn from(value: Vec<T>) -> Self {
        Self::Many(value)
    }
}

pub fn cast_array<T>(input: impl Into<OneOrMany<T>>) -> Vec<T> {
    match input.into() {
        OneOrMany::One(value) => vec![value],
        OneOrMany::Many(values) => values,
    }
}

pub fn adjust<T, F>(index: isize, mut adjuster: F, values: &[T]) -> Vec<T>
where
    T: Clone,
    F: FnMut(T) -> T,
{
    let mut result = values.to_vec();
    let len = result.len();

    if let Some(index) = normalize_index(index, len) {
        result[index] = adjuster(result[index].clone());
    }

    result
}

pub trait DropLast {
    type Output;

    fn drop_last(self) -> Self::Output;
}

pub fn drop_last<T>(value: T) -> T::Output
where
    T: DropLast,
{
    value.drop_last()
}

impl<T: Clone> DropLast for &[T] {
    type Output = Vec<T>;

    fn drop_last(self) -> Self::Output {
        self[..self.len().saturating_sub(1)].to_vec()
    }
}

impl<T> DropLast for Vec<T> {
    type Output = Vec<T>;

    fn drop_last(mut self) -> Self::Output {
        let _ = self.pop();
        self
    }
}

impl DropLast for &str {
    type Output = String;

    fn drop_last(self) -> Self::Output {
        let mut result = self.to_string();
        let _ = result.pop();
        result
    }
}

impl DropLast for String {
    type Output = String;

    fn drop_last(mut self) -> Self::Output {
        let _ = self.pop();
        self
    }
}

pub trait Last {
    type Output;

    fn last_value(self) -> Option<Self::Output>;
}

pub fn last<T>(value: T) -> Option<T::Output>
where
    T: Last,
{
    value.last_value()
}

impl<T: Clone> Last for &[T] {
    type Output = T;

    fn last_value(self) -> Option<Self::Output> {
        self.last().cloned()
    }
}

impl<T> Last for Vec<T> {
    type Output = T;

    fn last_value(self) -> Option<Self::Output> {
        self.into_iter().last()
    }
}

impl Last for &str {
    type Output = char;

    fn last_value(self) -> Option<Self::Output> {
        self.chars().next_back()
    }
}

impl Last for String {
    type Output = char;

    fn last_value(self) -> Option<Self::Output> {
        self.chars().next_back()
    }
}

pub fn repeat<T>(value: T, count: usize) -> Vec<T>
where
    T: Clone,
{
    vec![value; count]
}

pub trait Reverse {
    type Output;

    fn reverse_value(self) -> Self::Output;
}

pub fn reverse<T>(value: T) -> T::Output
where
    T: Reverse,
{
    value.reverse_value()
}

impl<T: Clone> Reverse for &[T] {
    type Output = Vec<T>;

    fn reverse_value(self) -> Self::Output {
        self.iter().cloned().rev().collect()
    }
}

impl<T> Reverse for Vec<T> {
    type Output = Vec<T>;

    fn reverse_value(mut self) -> Self::Output {
        self.reverse();
        self
    }
}

pub fn without<T>(excluded: &[T], values: &[T]) -> Vec<T>
where
    T: Clone + PartialEq,
{
    values
        .iter()
        .filter(|value| !excluded.iter().any(|excluded| excluded == *value))
        .cloned()
        .collect()
}

fn normalize_index(index: isize, len: usize) -> Option<usize> {
    if len == 0 {
        return None;
    }

    if index >= 0 {
        let index = usize::try_from(index).ok()?;
        return (index < len).then_some(index);
    }

    let len = isize::try_from(len).ok()?;
    let adjusted = len.checked_add(index)?;

    if adjusted < 0 {
        None
    } else {
        usize::try_from(adjusted).ok()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn casts_single_values_and_vectors() {
        assert_eq!(cast_array("foo"), vec!["foo"]);
        assert_eq!(cast_array::<&str>(vec!["foo"]), vec!["foo"]);
    }

    #[test]
    fn adjusts_values_using_positive_and_negative_indexes() {
        assert_eq!(adjust(1, |value| value * 2, &[1, 2, 3]), vec![1, 4, 3]);
        assert_eq!(adjust(-1, |value| value + 10, &[1, 2, 3]), vec![1, 2, 13]);
    }

    #[test]
    fn drops_last_for_slices_and_strings() {
        assert_eq!(drop_last(&[1, 2, 3][..]), vec![1, 2]);
        assert_eq!(drop_last("hello"), "hell");
    }

    #[test]
    fn gets_last_for_slices_and_strings() {
        assert_eq!(last(&[1, 2, 3][..]), Some(3));
        assert_eq!(last("abc"), Some('c'));
        assert_eq!(last(&[] as &[i32]), None);
    }

    #[test]
    fn repeats_reverses_and_filters_values() {
        assert_eq!(repeat("a", 3), vec!["a", "a", "a"]);
        assert_eq!(reverse(&[1, 2, 3][..]), vec![3, 2, 1]);
        assert_eq!(without(&[2, 4], &[1, 2, 3, 4, 5]), vec![1, 3, 5]);
    }
}
