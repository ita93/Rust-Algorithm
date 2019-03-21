//Define a 2-d table, named Memroy Table
/*
 * Return true if there are a subset that can sum up to target value
 */
fn subset_sum(input: Vec<u32>, target: usize) -> bool{
    if target == 0 {return true;}
    if input.len() == 0 {return false;}
    let mut mem_table = vec![vec![false; target+1]; input.len()+1];
    //Init base subproblem
    for i in 0..=input.len() {
        mem_table[i][0] = true;
    }
    
    for i in 1..=input.len() {
        for j in 1..=target {
            if j < input[i-1] as usize {
                mem_table[i][j] = mem_table[i-1][j];
            }else{
                mem_table[i][j] = mem_table[i-1][j] || mem_table[i-1][j-input[i-1] as usize];
            }
        }
    }
    mem_table[input.len()][target]
}

#[test]
fn test_valid_sumable_set() {
  assert!(subset_sum(vec![3, 34, 4, 12, 5, 2], 9 as usize))
}

#[test]
fn test_valid_sumable_set_2() {
  assert!(subset_sum(vec![1, 2, 3, 6],  12 as usize))
}


#[test]
fn test_valid_sumable_set_3() {
  assert!(subset_sum(vec![3, 2, 7, 1],  6 as usize))
}

#[test]
fn test_invalid_sumable_set_1() {
  assert!(!subset_sum(vec![1, 2, 3, 6],  13 as usize))
}

#[test]
fn test_invalid_sumable_set_2() {
  assert!(!subset_sum(vec![3, 34, 4, 12, 5, 2],  13 as usize))
}