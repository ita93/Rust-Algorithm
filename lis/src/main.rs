fn lis(input: &Vec<u32>) -> u32 {
    let mut lis = vec![0; input.len()]; //lis[i] is the longest increase subsequence in input[0..i]
    lis[0] = 1;
    for i in 1..input.len(){
        let mut max = 0;
        for j in 0..input.len() {
            if input[j] < input[i] && lis[j] > max {
                max = lis[j];
            }
        }
        lis[i] = max + 1;
    }
    lis.pop().unwrap_or(0)
}

#[test]
fn set_1() {
    let input = vec![10, 22, 9, 33, 21, 50, 41, 60, 80];
    assert_eq!(lis(&input), 6);
}

#[test]
fn set_2() {
    let input = vec![10, 22, 9, 33, 21, 50, 41, 60];
    assert_eq!(lis(&input), 5);
}
