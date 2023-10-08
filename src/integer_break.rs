use crate::Solution;

impl Solution {
    /// Wrong.
    /// TODO: fix this.
    pub fn integer_break(n: i32) -> i32 {
        assert!(n >= 2);

        let mut max_prod = 1;

        for k in 2..n {
            let div = n / k;
            let rem = n % k;

            let prod = if rem == 0 {
                div.pow(k as u32)
            } else {
                if rem == 1 {
                    div.pow(k as u32 - 1) * (k + rem)
                } else {
                    div.pow(k as u32) * rem
                }
            };

            max_prod = max_prod.max(prod);
        }

        max_prod
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tests() {
        let input = 2;
        let expected = 1;

        let actual = Solution::integer_break(input);

        assert_eq!(actual, expected);
    }

    #[test]
    fn tests2() {
        let input = 10;
        let expected = 36;

        let actual = Solution::integer_break(input);

        assert_eq!(actual, expected);
    }

    #[test]
    fn tests3() {
        let input = 4;
        let expected = 4;

        let actual = Solution::integer_break(input);

        assert_eq!(actual, expected);
    }

    #[test]
    fn tests4() {
        let input = 8;
        let expected = 18;

        let actual = Solution::integer_break(input);

        assert_eq!(actual, expected);
    }

    #[test]
    fn tests5() {
        let input = 8;
        let expected = 18;

        let actual = Solution::integer_break(input);

        assert_eq!(actual, expected);
    }
}
