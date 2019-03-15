//Solution: use include/exclude 
struct Combination<T>{
    data: Vec<T>,
    com_size: usize, //Number of elements in a combination.
}

impl <T: Clone> Combination<T>
where T: std::fmt::Debug + std::default::Default+std::cmp::PartialEq{
    fn new(data: Vec<T>, com_size: usize) -> Self{
        Combination{
            data,
            com_size,
        }
    }
    
    fn print_combinations(&self){
        let mut res = vec![Default::default(); self.com_size];
        self.com_util(&mut res, 0, 0);
    }

    fn com_util(&self, mut res: &mut Vec<T>, start: usize, index: usize){
        if index == self.com_size  {
            println!("{:?}", res);
        } else {
            let mut i = start;
            while self.data.len() + index - self.com_size + 1 > i {
                res[index] = self.data[i].clone();
                //this while loop will prevent duplicated result when there are duplicate elements in input array
                //Require: Array elements are sorted (increasing)
                while i<self.data.len() -1  && self.data[i] == self.data[i+1]{
                    i += 1;
                }
                self.com_util(&mut res, i+1, index + 1);
                i += 1;
            }
        }
    }
}

fn main() {
    let v1 = vec![1, 2, 3, 5];
    let combo = Combination::new(v1, 3);
    combo.print_combinations();
}
