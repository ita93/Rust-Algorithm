struct Notes{
    notes: Vec<u32>,
    target: u32,
    non: u32, //Number of Notes
}

impl Notes{
    fn new(notes: Vec<u32>, target: u32) -> Self{
        let len = notes.len();
        Notes{notes, target, non: len as u32}
    }

    fn get_notes(&mut self,mut res: &mut Vec<usize>, index: usize, current_sum: u32, current_non: u32){
        if current_sum == self.target{
            if current_non <= self.non {
                self.non = current_non;
               println!("{:?}", res);
            }
            return
        }
        for j in 0..=1 {
           if current_sum < self.target && index < self.notes.len() && current_non <= self.non{
               let new_sum = current_sum + self.notes[index]*j;
               res[index] = j as usize;
               self.get_notes(&mut res, index + 1, new_sum, current_non + j);
           }
        }
    }
}

fn main() {
    let input = vec![200, 10, 20, 20, 50, 50, 50,50, 100, 100];
    let len = input.len();
    let mut note = Notes::new(input, 390);
    let mut checker: Vec<usize> = vec![0; len];
    note.get_notes(&mut checker, 0, 0, 0);
}
