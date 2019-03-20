//Define a 2-d table, named Memroy Table
/*
 * Return true if there are a subset that can sum up to target value
 */
fn subset_sum(input: Vec<u32>, target: u32) -> bool{
    if target == 0 {return true;}
    if input.len() == 0 {return false;}
    let mut mem_table = vec![vec![bool; target]; input.len()];
    //Init base subproblem
    for i in 1..input.len() {
        mem_table[i][0] = false;
    }
    //When target = 0 
    for i in 0..target {
        mem_table[0][i] = true;
    }
    
    for i in 0..
}

fn main() {
    println!("Hello, world!");
}
