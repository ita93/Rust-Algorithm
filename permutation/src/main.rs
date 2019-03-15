fn permute<T>(mut data: &mut Vec<T>, l: usize, r: usize)
    where T: std::fmt::Debug{
    if l == r{
        println!("{:?}", data);
    }else{
        for i in 1..=r {
            data.swap(l, i);
            permute(&mut data, l + 1, r);
            data.swap(l,i);
        }
    }
}

fn main() {
    let mut input:Vec<u32> = vec![1,2,3];
    let data_len = input.len();
    permute(&mut input, 0, data_len-1);
}
