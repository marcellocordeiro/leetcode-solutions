use crate::Solution;

impl Solution {
    pub fn length_of_longest_substring(s: String) -> i32 {
        let mut max_size = 0;

        for window_start in 0..s.len() {
            let window = &s[window_start..];

            if max_size >= window.len() {
                break;
            }

            for (window_end, char) in window.char_indices() {
                let sub_window = &window[0..window_end];

                if sub_window.len() > max_size {
                    break;
                }

                if sub_window.contains(char) {
                    break;
                }

                max_size = max_size.max(sub_window.len() + 1);
            }
        }

        max_size as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tests() {
        let input = "abc".to_owned();
        let expected = 3;

        let actual = Solution::length_of_longest_substring(input);

        assert_eq!(actual, expected);
    }

    #[test]
    fn tests2() {
        let input = "pwwkew".to_owned();
        let expected = 3;

        let actual = Solution::length_of_longest_substring(input);

        assert_eq!(actual, expected);
    }

    #[test]
    fn tests3() {
        let input = "bbbbbb".to_owned();
        let expected = 1;

        let actual = Solution::length_of_longest_substring(input);

        assert_eq!(actual, expected);
    }

    #[test]
    fn test_limit() {
        let input = "hijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789hijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789hijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789hijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789hijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789hijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789";
        let expected = 55;

        let actual = Solution::length_of_longest_substring(input.to_owned());

        assert_eq!(actual, expected);
    }

    #[test]
    fn test5() {
        let input = "abcabcbb";
        let expected = 3;

        let actual = Solution::length_of_longest_substring(input.to_owned());

        assert_eq!(actual, expected);
    }
}
