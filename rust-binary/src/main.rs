fn main() {
    println!("Hello, world!");
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test() {
        assert_eq!(1 == 1, true);
    }
}
