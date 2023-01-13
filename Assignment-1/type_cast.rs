use std::env::args;
pub struct Typecast {}

impl Typecast {
    pub fn product(num1: String, num2: String) -> String {
        let cnt1 = num1.len();
        let cnt2 = num2.len();
        if cnt1 == 0 || cnt2 == 0 {
            let mut s: String = String::new();
            s.push('0');
            return s;
        }

        let mut v: Vec<i32> = Vec::new();
        let xs = num1.as_bytes();
        let ys = num2.as_bytes();
        for _i in 0..cnt1 + cnt2 {
            v.push(0);
        }
        let mut idx1: usize = 0;
        for i in 0..cnt1 {
            let mut carry = 0;
            let digit1 = (xs[cnt1 - 1 - i] - 48) as i32;
            let mut idx2: usize = 0;
            for j in 0..cnt2 {
                let digit2 = (ys[cnt2 - 1 - j] - 48) as i32;
                let sum = digit1*digit2 + v[idx1 + idx2] + carry;
                v[idx1 + idx2] = sum % 10;
                carry = sum / 10;
                idx2 += 1;
            }
            if carry > 0 {
                v[idx1 + idx2] += carry;
            }
            idx1 += 1;
        }
        let mut x: usize = 0;
        for i in 0..cnt1 + cnt2 {
            if v[cnt1 + cnt2 - 1 - i] == 0 {
                x += 1;
            }
            else{
                break;
            }
        }
        let mut ans: String = String::new();
        for i in x..cnt1 + cnt2 {
            let c = v[cnt1 + cnt2 - 1 - i].to_string();
            ans.push_str(&c);
        }
        if ans.len() == 0 {
            let c = "0";
            ans.push_str(&c);
        }
        return ans;
    }
}

fn main() {
    let s: String = args().nth(1).unwrap();
    let t: String = args().nth(2).unwrap();
    println!("{}", Typecast::product(s,t));
}