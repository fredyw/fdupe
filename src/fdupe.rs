fn edit_distance(str1: &str, str2: &str) -> usize {
    // TODO
    1
}

#[cfg(test)]
mod test {
    use fdupe;

    #[test]
    fn test_edit_distance() {
        assert_eq!(1, fdupe::edit_distance("hello", "holla"));
    }
}