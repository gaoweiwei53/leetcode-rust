use std::collections::HashMap;

pub struct Solution {}
impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let lengh = nums.len();
        for i in 0..lengh{
            for j in i+1..lengh {
                if nums[i] + nums[j] == target {
                    return vec![i as i32, j as i32];
                }
            }
        }
        vec![]
    }
}

impl Solution {
    pub fn two_sum2(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut map = HashMap::new();
        for i in 0..nums.len() {
            // 为什么需要传入引用？
            // if let 是一种模式匹配的写法
            if let Some(k) = map.get(&(target - nums[i])) {
                return vec![*k as i32,  i as i32];
            }
            map.insert(nums[i], i);
        }
        panic!("not found")
    }
}

#[cfg(test)]

// tests名字任意
mod tests {
    // 必须有
    use super::*;
    #[test]
    fn test_two_sum(){
        assert_eq!(vec![0,2], Solution::two_sum2(vec![3,4,6,7,10], 9));
    }
}