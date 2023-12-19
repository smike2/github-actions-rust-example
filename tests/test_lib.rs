use trust::add;
use trust::div;
use trust::mul;
use trust::sub;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add() {
        assert_eq!(add(2, 3), 5);
        assert_eq!(add(-2, 3), 1);
        assert_eq!(add(0, 0), 0);
    }

    #[test]
    fn test_sub() {
        assert_eq!(sub(5, 3), 2);
        assert_eq!(sub(10, 5), 5);
        assert_eq!(sub(0, 0), 0);
    }

    #[test]
    fn test_mul() {
        assert_eq!(mul(2, 3), 6);
        assert_eq!(mul(-2, 3), -6);
        assert_eq!(mul(0, 5), 0);
    }

    #[test]
    fn test_div() {
        assert_eq!(div(6, 3), 2);
        assert_eq!(div(10, 2), 5);
        assert_eq!(div(0, 5), 0);
    }
}
