fn minus(left: u64, right: u64) -> u64 {
    left - right
}

fn main() {
    println!("{}", minus(2, 2));
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
