/*
 * Input: 
 *  items: list of items (value, weight)
 *  limit: The max weight that the bag can be.
 */
fn knapsack(items: &Vec<(u32, u32)>, limit: usize) -> u32{
    let mut mem_table = vec![vec![0;limit+1]; items.len()+1];
    for i in 1..=items.len(){
        for j in 1..=limit{
            if j < items[i-1].1 as usize { //items[i-1].1
                mem_table[i][j] = mem_table[i][j]; //J < w[i-1] so we cannot take i.
            } else {
                mem_table[i][j] = max_of_two(mem_table[i-1][j], mem_table[i-1][ j - items[i-1].1 as usize] + items[i-1].0);
                //
            }
        }
    }
    mem_table[items.len()][limit]
}

fn max_of_two(a: u32, b: u32) -> u32{
    if a > b {
        return a;
    }
    b
}

#[test]
fn test_first_case() {
    let items = vec![(60, 10), (100, 20), (120, 30)];
    assert!(knapsack(&items, 50) == 220);
}
