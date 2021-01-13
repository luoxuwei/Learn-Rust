struct Solution;
impl Solution {

    fn isPlusAndDivide(c : char) -> bool {
        if c == '*' || c == '/' {
            return true;
        }

        return false;
    }

    fn isAddAndMinus(c : char) -> bool {
        if c == '-' || c == '+' {
            return true;
        }

        return false;
    }

    fn parse(str: &str, data: &mut Vec<i32>, help: &mut Vec<bool>) {
        let mut i = 0;
        let mut chars: Vec<char> = str.chars().collect();
        let mut flag : i32 = 1;
        let mut count : i32 = 0;
        let mut stat: Vec<char> = vec![];
        loop {
            if i == str.len() {
                //此处最开始有bug写成了return，导致最后的符号没有添加到data列表里
                //其实只要调试单步走一遍就能发现问题，不用构造什么复杂的式子
                //一定要谨记，简单的调试手段是基础，出了问题不过现象有多复杂，都不要忽视基础的调试手段
                //bug都是人的局限造成的，要依赖自动化手段和流程来规范，哪怕是简单的自动化，都是有很大的价值的。
                break;
            }
            if (chars[i] == '-' && count == 1) || (chars[i] == '-' && i == 0) {
                flag = -1;
                i = i+1;
                continue;
            }
            if chars[i].is_digit(10) {
                let mut sum = 0;
                while i < chars.len() && chars[i].is_digit(10) {
                    sum = sum*10 + (chars[i] as i32 - '0' as i32);
                    i = i + 1;
                }

                sum = sum * flag;
                count = 0;
                flag = 1;
                data.push(sum);
                help.push(true);

            } else {
                if stat.is_empty() || chars[i] == '(' {
                    stat.push(chars[i]);
                    i=i+1;
                } else if chars[i] == ')' {
                    while *stat.last().unwrap() != '(' {
                        data.push(stat.pop().unwrap() as i32);
                        help.push(false);
                    }
                    stat.pop();
                    i=i+1;
                } else if Solution::isPlusAndDivide(chars[i]) {
                    while !stat.is_empty() && Solution::isPlusAndDivide(*stat.last().unwrap()) {
                        data.push(stat.pop().unwrap() as i32);
                        help.push(false);
                    }
                    stat.push(chars[i]);
                    count = count + 1;
                    i=i+1;
                } else if Solution::isAddAndMinus(chars[i]) {
                    while !stat.is_empty() && (Solution::isPlusAndDivide(*stat.last().unwrap())
                        || Solution::isAddAndMinus(*stat.last().unwrap())) {
                        data.push(stat.pop().unwrap() as i32);
                        help.push(false);
                    }
                    stat.push(chars[i]);
                    count = count + 1;
                    i=i+1;
                }

            }
        }

        while !stat.is_empty() {
            data.push(stat.pop().unwrap() as i32);
            help.push(false);
        }
    }

    fn eval(str: &str) -> i32 {
        if str.is_empty() {
            return 0;
        }

        let mut data : Vec<i32> = vec![];
        let mut help : Vec<bool> = vec![];
        let mut stack: Vec<i32> = vec![];

        let mut t1 : i32 = 0;
        let mut t2 : i32 = 0;
        Solution::parse(str, &mut data, &mut help);

        for i in 0..data.len() {
            if (*help.get(i).unwrap()) {
                stack.push(data[i]);
            } else {
                t1 = stack.pop().unwrap();
                t2 = stack.pop().unwrap();
                match data[i] {
                    45 => {stack.push(t2 - t1)}
                    42 => {stack.push(t2 * t1)}
                    43 => {stack.push(t2 + t1)}
                    47 => {stack.push(t2 / t1)}
                    _ => {panic!("unknow")}
                }
            }
        }
        return stack.pop().unwrap();
    }



}

#[cfg(test)]
mod test {
    use crate::eval_parse::Solution;

    #[test]
    fn testEvalParse() {
        let data = "1+2*(-1)*2";
        println!("{} = {}", data, Solution::eval(data));
    }
}