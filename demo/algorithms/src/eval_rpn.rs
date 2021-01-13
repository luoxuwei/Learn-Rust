use std::num::ParseIntError;

struct Solution;
impl Solution {
    pub fn eval_rpn(tokens: Vec<String>) -> i32 {
        if tokens.len() == 0 {
            return 0;
        }

        let mut stack = Vec::with_capacity(tokens.len());
        let mut test : Result<i32, ParseIntError> = Result::Ok(0);
        let mut t1 : i32 = 0;
        let mut t2 : i32 = 0;
        for i in 0..tokens.len() {
            test = tokens[i].parse::<i32>();
            match test {
                Ok(num) => {stack.push(num)}
                Err(_) => {
                    match tokens[i].as_str() {
                        "+" => {
                            t1 = stack.pop().unwrap();
                            t2 = stack.pop().unwrap();
                           stack.push(t2 + t1);
                        }
                        "-" => {
                            t1 = stack.pop().unwrap();
                            t2 = stack.pop().unwrap();
                            stack.push(t2 - t1);
                        }
                        "*" => {
                            t1 = stack.pop().unwrap();
                            t2 = stack.pop().unwrap();
                            stack.push(t2 * t1);
                        }
                        "/" => {
                            t1 = stack.pop().unwrap();
                            t2 = stack.pop().unwrap();
                            stack.push(t2 / t1);
                        }
                        _ => panic!("unknow")
                    }
                }
            }
        }
        return stack.pop().unwrap();
    }
}

#[cfg(test)]
mod test {
    use crate::eval_rpn::Solution;

    #[test]
    fn test() {
        let data = vec!["2".to_owned(),"1".to_owned(),"+".to_owned(),"3".to_owned(),"*".to_owned()];
        println!("result = {}", Solution::eval_rpn(data))
    }
}