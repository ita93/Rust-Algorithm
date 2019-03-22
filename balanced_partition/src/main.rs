/*SOLUTION:
 * 1) Calculate sum of the array. If sum is odd, there can not be two subsets with equal sum, so return false.
 * 2) If sum of array elements is even, calculate sum/2 and find a subset of array with sum equal to sum/2.
*/
fn check_balanced(input: &Vec<u32>) -> bool{
    let input_sum = input.iter().fold(0, |sum, x| sum + x);
    if input_sum % 2 == 1 {
        return false;
    }
    let half_sum = (input_sum / 2) as usize;
    //Subset sum
    let mut mem_table = vec![vec![false;half_sum+1];input.len()+1];
    for i in 0..=input.len(){
        mem_table[i][0] = true;
    }

    for i in 1..=input.len(){
        for j in 1..=half_sum {
            if j < input[i-1] as usize{
                mem_table[i][j] = mem_table[i-1][j];
            }else{
                mem_table[i][j] = mem_table[i-1][j] || mem_table[i-1][j-input[i-1] as usize];
            }
        }
    }
    mem_table[input.len()][half_sum]
}

#[test]
fn test_odd_sum() {
    let input: Vec<u32> =  vec![1, 5, 11, 5,1];
    assert!(check_balanced(&input) == false);
}

#[test]
fn test_valid_set_1(){
    let input: Vec<u32> =  vec![1, 5, 11, 5];
    assert!(check_balanced(&input) == true);
}

#[test]
fn test_invalid_set_1(){
    let input: Vec<u32> =  vec![1, 5, 2];
    assert!(check_balanced(&input) == false);
}

#[test]
fn test_valid_set_2(){
    let input: Vec<u32> =  vec![1, 4, 6, 9];
    assert!(check_balanced(&input) == true);
}

#[test]
fn test_valid_set_3(){
    let input: Vec<u32> =  vec![1, 4, 6, 9, 10];
    assert!(check_balanced(&input) == true);
}

#[test]
fn test_invalid_set_2(){
    let input: Vec<u32> =  vec![1, 4, 6, 9,4];
    assert!(check_balanced(&input) == false);
}

#[test]
fn test_invalid_set_3(){
    let input: Vec<u32> =  vec![1, 4, 6, 9, 7];
    assert!(check_balanced(&input) == false);
}