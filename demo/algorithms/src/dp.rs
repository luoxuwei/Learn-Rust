use std::usize::MAX;
use std::cmp::min;
use std::collections::HashMap;
use std::hash::Hash;


pub struct LeetCode120;

impl LeetCode120 {
    pub fn minimun_single_total(triangle: &Vec<Vec<i32>>, l: i32, i: usize, memo: &mut HashMap<usize, HashMap<usize, i32>>) -> i32 {

        if l < 0 {return 0;}
        if !memo.contains_key(&(l as usize)) {
            let cur = HashMap::with_capacity(triangle.get(l as usize).unwrap().len());
            memo.insert(l as usize, cur);
        }
        let cur = memo.get(&(l as usize)).unwrap();
        let mut num1: i32 = std::i32::MAX;
        let mut num2: i32 = std::i32::MAX;
        if triangle[l as usize].len()>i {
            if !memo.get(&(l as usize)).unwrap().contains_key(&(i as usize)) {
                num1 = triangle[l as usize][i] + LeetCode120::minimun_single_total(triangle, l-1, i, memo);
                memo.get_mut(&(l as usize)).unwrap().insert(i, num1);
            } else {
                num1 = memo[&(l as usize)][&(i as usize)];
            }
        };
        if i > 0 {
            if memo.get(&(l as usize)).unwrap().contains_key(&((i - 1) as usize)){
                num2 = memo[&(l as usize)][&(i-1)];
            } else {
                num2 = triangle[l as usize][i-1] + LeetCode120::minimun_single_total(triangle, l-1, i-1, memo);
                memo.get_mut(&(l as usize)).unwrap().insert(i-1, num2);
            }
        };
        return min(num1, num2);
    }

    pub fn minimum_total(triangle: Vec<Vec<i32>>) -> i32 {
        let top = triangle.len()-1;
        let mut res = std::i32::MAX;
        let mut num: i32 = 0;
        let mut memo: HashMap<usize, HashMap<usize, i32>> = HashMap::with_capacity(top+1);
        for i in 0..triangle[top].len() {
            num = triangle[top][i] + LeetCode120::minimun_single_total(&triangle, top as i32-1, i, &mut memo);
            if num < res {res = num}
        }
        return res;
    }
}

pub struct Solution;

impl Solution {

    pub fn minimum_total(triangle: Vec<Vec<i32>>) -> i32 {
        let mut memo1 = vec![std::i32::MAX; triangle.len()];
        let mut memo2 = vec![std::i32::MAX; triangle.len()];
        memo1[0] = triangle[0][0];
        let mut res = std::i32::MAX;
        let mut num1 = std::i32::MAX;
        let mut num2 = std::i32::MAX;
        for i in 1..triangle.len() {
            for j in 0..triangle[i].len() {
                num1 = if j > 0 {
                    memo1[j-1] + triangle[i][j]
               }  else { std::i32::MAX };

                num2 = if j < triangle[i-1].len() {
                    memo1[j] + triangle[i][j]
                } else {
                    std::i32::MAX
                };
                memo2[j] = min(num1, num2);
            }
            memo1.copy_from_slice(&memo2);
        }

        for i in memo1 {
            if i < res {
                res = i;
            }
        }
        return res;
    }
}

#[cfg(test)]
mod test {
    use crate::dp::{LeetCode120, Solution};


    #[test]
    fn test120() {
        let data = vec![vec![2], vec![3,4], vec![6,5,7], vec![4,1,8,3]];
        println!("minimun_total {}", LeetCode120::minimum_total(data));
    }

    #[test]
    fn testSolution120() {
        let data = vec![vec![2], vec![3,4], vec![6,5,7], vec![4,1,8,3]];
        println!("minimun_total {}", Solution::minimum_total(data));
    }
}
