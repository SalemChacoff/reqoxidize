pub fn minus(left: u64, right: u64) -> u64 {
    left - right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = minus(2, 2);
        assert_eq!(result, 0);
    }
}
