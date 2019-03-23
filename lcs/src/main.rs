fn lcs(input_a: &str,input_b: &str) ->u32{
    let m = input_a.len();
    let n = input_b.len();
    let mut mem_table = vec![vec![0 as usize;m+1];n+1];
    for i in 1..=m {
        for j in 1..=n {
            if input_a.chars().nth(i-1).unwrap() == input_b.chars().nth(j-1).unwrap() {
                mem_table[i][j] = mem_table[i-1][j-1] + 1;
            }else{
                mem_table[i][j] = std::cmp::max(mem_table[i-1][j], mem_table[i][j-1]);
            }
        }
    }
    mem_table[m][n] as u32
}

fn main() {
    let input1 = String::from("ABCDGH");
    let input2 = String::from("AEDFHR");
    println!("{}",lcs(&input1, &input2));
}
