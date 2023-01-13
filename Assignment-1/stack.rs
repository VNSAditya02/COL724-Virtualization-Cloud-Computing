use std::env::args;
pub struct Stack {}

impl Stack {
    pub fn calculate(s: String) -> i32 {
        let mut ans_stack: Vec<i32> = Vec::new();
        let mut op_stack: Vec<String> = Vec::new();
        let split = s.split(" ");
        for x in split {
            if x == "(" || x == "+" || x == "-" {
                op_stack.push((*x).to_string());
                continue;
            }
            if x == ")" {
                op_stack.pop();
            }
            else {
                ans_stack.push(x.parse::<i32>().unwrap());
            }
            if op_stack.len() == 0 {
                continue;
            }
            let top = &op_stack[op_stack.len() - 1];
            if top == "(" {
                continue;
            }
            let n2 = ans_stack[ans_stack.len() - 1];
            ans_stack.pop();
            let n1 = ans_stack[ans_stack.len() - 1];
            ans_stack.pop();
            if top == "+" {
                ans_stack.push(n1 + n2);
            }
            else{
                ans_stack.push(n1 - n2)
            }
            op_stack.pop();
        }
        return ans_stack[ans_stack.len() - 1];
    }
}

fn main() {
    let args: Vec<String> = args().collect();
    let expr: String = (&args[1..]).join(" ");

    println!("{:?}", Stack::calculate(expr));
}