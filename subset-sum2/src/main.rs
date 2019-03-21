//Require input is sorted set
//You can see that the greatest RESULT is: input.sum() + 1
fn smallest_unsum(input: Vec<u32>) -> u32{
    let mut temp = input.clone();
    temp.sort(); //Sort vector elements before we continue
    let mut res = 1;
    for i in 0..input.len(){
        if res < input[i]{
            return res;
        }
        res += input[i];
    }
    res
}

#[test]
fn test_valid_1(){
    assert!(smallest_unsum(vec![1,2,3,6]) == 13);
}

#[test]
fn test_valid_2(){
    assert!(smallest_unsum(vec![1, 3, 4, 5]) == 2);
}

#[test]
fn test_valid_3(){
    assert!(smallest_unsum(vec![1, 2, 6, 10, 11, 15]) == 4);
}

#[test]
fn test_valid_4(){
    assert!(smallest_unsum(vec![1, 3, 4, 5]) == 2);
}

#[test]
fn test_valid_5(){
    assert!(smallest_unsum(vec![1, 1, 3, 4]) == 10);
}