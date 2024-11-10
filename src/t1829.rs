pub struct Solution;
impl Solution {
    /// Solution to [leetcode 1829](https://leetcode.com/problems/maximum-xor-for-each-query/description/?envType=daily-question&envId=2024-11-08).
    /// You are given a sorted array nums of n non-negative integers and an integer maximumBit.
    /// 
    /// You want to perform the following query n times:
    ///     Find a non-negative integer k < 2maximumBit such that nums[0] XOR nums[1] XOR ... 
    ///         XOR nums[nums.length-1] XOR k is maximized. k is the answer to the ith query.
    ///     Remove the last element from the current array nums.
    ///
    /// Return an array answer, where answer[i] is the answer to the ith query.
    ///
    /// # Arguments
    ///
    /// * `nums` - an array of integers
    /// * `maximum_bit` - the max bit (see constraints)
    ///
    /// # Returns
    ///
    /// An array of the largest complement of the bitwise of all arguments
    ///
    /// # Examples
    /// ```rust
    /// use leetcode::t1829::Solution;
    ///
    /// assert_eq!(Solution::get_maximum_xor(vec![0,1,1,3], 2), vec![0,3,2,3]);
    /// assert_eq!(Solution::get_maximum_xor(vec![2,3,4,7], 3), vec![5,2,6,5]);
    /// assert_eq!(Solution::get_maximum_xor(vec![0,1,2,2,5,7], 3), vec![4,3,6,4,6,7]);
    /// ```
    ///
    /// # Constraints
    /// ```md
    ///     nums.length == n
    ///     1 <= n <= 105
    ///     1 <= maximumBit <= 20
    ///     0 <= nums[i] < 2maximumBit
    ///     nums is sorted in ascending order.
    /// ```
    /// # Solution learnings
    ///
    /// Bitwise xor operations are communicative and their own opposite. Meaning if you calculate the
    /// xor of an array and remove an element, calculating the xor of that element and your previous 
    /// sum will be the same as calculating the entire sum again. This means we can get the sum once
    /// then maintain it.
    ///
    /// I think there might be a way to solve this problem backwards and get the right result, but I
    /// kept getting the wrong one when I tried it.
    ///
    /// There's this thing called a mask in binary problems, ((1 << maximum_bit) - 1), we use that
    /// mask on all the operations to get all the results. But we cheat by doing the mask on the one
    /// K value and then using that K to get all the others.
    /// We prepopulate a results array because its free realestate. Better than popping a bunch and
    /// having capacity increase calls.
    pub fn get_maximum_xor(nums: Vec<i32>, maximum_bit: i32) -> Vec<i32> {
        let mut k = nums.iter().fold(0, |acc, &x| acc ^ x) ^ ((1 << maximum_bit) - 1);
        let mut results = vec![0; nums.len()];

        for (i, &num) in nums.iter().rev().enumerate() {
            results[i] = k;
            k = k ^ num;
        }

        results
    }
}
