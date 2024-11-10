pub struct Solution;
mpl Solution {
    /// Solution to [leetcode 3024](https://leetcode.com/problems/type-of-triangle/).
    /// You are given a 0-indexed integer array nums of size 3 which can form the sides of a triangle.
    /// A triangle is called equilateral if it has all sides of equal length.
    /// A triangle is called isosceles if it has exactly two sides of equal length.
    /// A triangle is called scalene if all its sides are of different lengths.
    ///
    /// Return a string representing the type of triangle that can be formed or "none" if it cannot form a triangle.
    ///
    /// # Arguments
    ///
    /// * `nums` - a vec of integers with length 3
    ///
    /// # Returns
    ///
    /// * `string` - A string that is one of 'equilateral', 'isosceles', 'scalene', or 'none'
    ///
    /// # Examples
    /// ```rust
    /// use leetcode::t3024::Solution;
    ///
    /// assert_eq!(Solution::triangle_type(vec![3,3,3]), "equilateral");
    /// assert_eq!(Solution::triangle_type(vec![3,4,5]), "scalene");
    /// assert_eq!(Solution::triangle_type(vec![2,2,3]), "isosceles");
    /// assert_eq!(Solution::triangle_type(vec![2,2,5]), "none");
    /// assert_eq!(Solution::triangle_type(vec![2,1,5]), "none");
    /// ```
    ///
    /// # Constraints
    /// ```md
    ///     nums.length == 3
    ///     1 <= nums[i] <= 100
    /// ```
    /// # Solution learnings
    ///
    pub fn triangle_type(nums: Vec<i32>) -> String {
        let (a, b, c) = (nums[0], nums[1], nums[2]);
        if a == b && b == c { return "equilateral".to_string() }
        if a == b || b == c || c == a {
            if Self::is_legal_triangle(a,b,c) {
                return "isosceles".to_string();
            }
            return "none".to_string();
        }
        if Self::is_legal_triangle(a, b, c) {
            return "scalene".to_string();
        }
        "none".to_string()
    }

    fn is_legal_triangle(a: i32, b: i32, c: i32) -> bool {
        (a + b > c)
        && (a + c > b)
        && (b + c > a)
    }
}
