use crate::Solution;

impl Solution {
    /// Timing out.
    /// TODO: Implement non-naive solution.
    pub fn max_area(height: Vec<i32>) -> i32 {
        let mut max_area = 0;

        for window_start in 0..height.len() {
            let window = &height[window_start..];

            let height_1 = window[0];

            for (pos, height_2) in window[1..].iter().enumerate() {
                let distance = pos + 1;
                let cube_height = height_1.min(*height_2);
                let area = cube_height * (distance as i32);

                max_area = max_area.max(area);
            }
        }

        max_area as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tests() {
        let input = [1, 8, 6, 2, 5, 4, 8, 3, 7];
        let expected = 49;

        let actual = Solution::max_area(input.to_vec());

        assert_eq!(actual, expected);
    }

    #[test]
    fn tests2() {
        let input = [1, 1];
        let expected = 1;

        let actual = Solution::max_area(input.to_vec());

        assert_eq!(actual, expected);
    }

    #[test]
    fn tests3() {
        let input = [2, 3, 4, 5, 18, 17, 6];
        let expected = 17;

        let actual = Solution::max_area(input.to_vec());

        assert_eq!(actual, expected);
    }
}
