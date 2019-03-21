/*
 * Input: 
 *  v: list of values (of items)
 *  w: list of weights (of items)
 *  limit: The max weight that the bag can be.
 */
fn knapsack(v: &Vec<u32>, w: &Vec<u32>, limit: usize) -> u32{
    if v.len() != w.len() {
        panic!("W and V must be the same size");
    }
    let mut mem_table = vec![vec![0;limit+1]; v.len()+1];
    for i in 1..=v.len(){
        for j in 1..=limit{
            if j < w[i-1] as usize {
                mem_table[i][j] = mem_table[i][j]; //J < w[i-1] so we cannot take i.
            } else {
                mem_table[i][j] = max_of_two(mem_table[i-1][j], mem_table[i-1][ j - w[i-1] as usize] + v[i-1]);
            }
        }
    }
    mem_table[v.len()][limit]
}

fn max_of_two(a: u32, b: u32) -> u32{
    if a > b {
        return a;
    }
    b
}

#[test]
fn test_first_case() {
    let values = vec![60, 100, 120];
    let weights = vec![10, 20, 30];
    assert!(knapsack(&values, &weights, 50) == 220);
}
