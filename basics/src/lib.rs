pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[cfg(test)]
mod basics {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
