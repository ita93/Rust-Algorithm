fn cut_rod(input: &Vec<u32>) -> u32{
    let mut val = vec![0; input.len() + 1];
    for i in 1..=input.len(){
        let mut max_val = 0;
        for j in 0..i {
            max_val = std::cmp::max(max_val, input[j] + val[i-j-1]); 
        }
        val[i] = max_val;
    }
    val[input.len()]
}

#[test]
fn test_case_1() {
    let input = vec![1,5,7,9,10,17,17,20];
    assert_eq!(cut_rod(&input), 22);
}

#[test]
fn test_case_2() {
    let input = vec![3, 5, 8, 9, 10, 17, 17, 20];
    assert_eq!(cut_rod(&input), 24);
}
