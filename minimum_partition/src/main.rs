fn partition(input: &Vec<u32>) -> u32 {
    let input_sum = input.iter().fold(0, |sum, x| sum + x)  as usize;
    let half_sum = input_sum / 2;

    let mut mem_table = vec![vec![false; half_sum + 1];input.len() + 1];
    let mut max = 0;
    for i in 0..=input.len() {
        mem_table[i][0] = true;
    }

    for i in 1..=input.len() {
        for j in 1..=half_sum{
            if j < input[i-1] as usize{
                mem_table[i][j] = mem_table[i-1][j];
            } else {
                mem_table[i][j] = mem_table[i-1][j] || mem_table[i-1][j-input[i-1] as usize];
            }
            if mem_table[i][j] == true && j > max {
                max = j;
            }
        }
    }
    let result = (input_sum as i32 - 2*max as i32) as i32;
    result.abs() as u32
}

#[test]
fn test_set_1() {
    let input = vec![1, 6, 11, 5];
    assert_eq!(partition(&input), 1);
}

#[test]
fn test_set_2() {
    let input = vec![3, 1, 4, 2, 2, 1];
    assert_eq!(partition(&input), 1);
}

#[test]
fn test_set_3() {
    let input = vec![1,3,7,9,4];
    assert_eq!(partition(&input), 0);
}

#[test]
fn test_set_4() {
    let input = vec![1, 5, 11, 5];
    assert_eq!(partition(&input), 0);
}

#[test]
fn test_set_5() {
    let input = vec![3,1,1,2,2,1];
    assert_eq!(partition(&input), 0);
}
