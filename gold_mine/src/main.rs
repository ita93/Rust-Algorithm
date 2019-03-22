fn get_max_gold(input: &Vec<Vec<u32>>)-> u32{
    if input.len() == 0{
        return 0;
    }
    let width = input[0].len();
    let height = input.len();

    let mut mem_table = input.clone();
    //iterate through COLUMNS first, then ROWS (quite difference to normal strategy)
    for j in 1..width {
        for i in 0..height {
            let right_up = if i == 0 {0} else {mem_table[i-1][j-1]};
            let right_down = if i == height - 1 {0} else {mem_table[i+1][j-1]};
            let right = mem_table[i][j-1];
            let max_move = max_move((right_up, right_down, right));
            mem_table[i][j] += max_move;
        }
    }
    //get the max value in the very last col
    let mut max  = mem_table[0][width - 1];
    for i in 1..height{
        if max <  mem_table[i][width - 1]{
            max = mem_table[i][width -1];
        }
    }
    max
}

fn max_move(input: (u32, u32, u32)) -> u32{
    let mut max  = input.0;
    if max < input.1 {
        max = input.1;
    }
    if max < input.2 {
        max = input.2;
    }
    max
}

#[test]
fn test_set_1() {
    let gold = vec![vec![1,3,1,5], vec![2,2,4,1], vec![5,0,2,3], vec![0,6,1,2]];
    assert_eq!(get_max_gold(&gold), 16);
}

#[test]
fn test_set_2() {
    let gold = vec![vec![10,33,13,15], vec![22,21,4,1], vec![5,0,2,3], vec![0,6,14,2]];
    assert_eq!(get_max_gold(&gold), 83);
}

#[test]
fn test_set_3() {
    let gold = vec![vec![1,3,3], vec![2,1,4], vec![0,6,4]];
    assert_eq!(get_max_gold(&gold), 12);
}
