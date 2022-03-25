pub fn ends_with_any(string: &str, ends: Vec<&str>) -> bool {
    ends.into_iter().any(|end| string.ends_with(end))
}
