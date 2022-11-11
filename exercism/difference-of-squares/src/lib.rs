pub fn square_of_sum(n: u32) -> u32 {
    //unimplemented!("square of sum of 1...{}", n)
    let mut ketqua: u32 = 0;
    for i in 1..=n{
        ketqua += i;
    }
   return ketqua * ketqua;
}

pub fn sum_of_squares(n: u32) -> u32 {
    //unimplemented!("sum of squares of 1...{}", n)
    let mut ketqua: u32 = 0;
    for i in 1..=n {
        ketqua += i * i;
    }
   return ketqua;
}


pub fn difference(n: u32) -> u32 {
    return square_of_sum(n) - sum_of_squares(n);
}
