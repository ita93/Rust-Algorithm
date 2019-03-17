//Solution: THe number of close parentheses are always less than or equal to the number of
//open one.
fn generate(open: u32, close: u32, res: String){
    if open == 0 && close == 0 {
        println!("{:?}", res);
    }
    if open > close {
        return;
    } else {
        if open > 0 {
            generate(open - 1, close, format!("{}(", res));
        }
        if close > 0 {
            generate(open, close - 1, format!("{})", res));
        }
    }
}

fn main() {
    generate(5,5,String::new());
}
