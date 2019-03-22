fn coin_change(input: &Vec<usize>, target: usize) -> usize{
    let  mut table = vec![vec![0; input.len()]; target + 1];
    for i in 0..input.len() {
        table[0][i] = 1;
    }

    for i in 1..=target{
        for j in 0..input.len(){
            let x = if i >= input[j] {table[i-input[j]][j]} else {0};
            let y = if j >= 1 {table[i][j-1]} else {0};
            table[i][j] = x + y;
        }
    }
    table[target][input.len()-1]
}

#[test]
fn test_set_1() {
    let input = vec![1,2,3];
    assert_eq!(coin_change(&input, 4), 4);
}

#[test]
fn test_set_2() {
    let input = vec![2,5,3,6];
    assert_eq!(coin_change(&input, 10), 5);
}
