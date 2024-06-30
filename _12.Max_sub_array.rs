//Find the maximum subarray sum in Rust

impl Solution {
    pub fn max_sub_array(nums: Vec<i32>) -> i32 {
        
         let mut max_sum = i32::MIN;
    let mut curmax = 0;
    
    for i in nums{
        curmax = i32::max(curmax+i,i);
        max_sum =  i32::max(max_sum,curmax)
    }
    max_sum
    }
}
