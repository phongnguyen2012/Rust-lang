pub fn convet_to_vec(num: u32) -> Vec<u32> {
    let mut v = vec![];
    for i in num.to_string().chars() {
        v.push(i.to_digit(10).unwrap());
    }
    v
}
pub fn is_armstrong_number(num: u32) -> bool {
    let v = convet_to_vec(num);
    let mut sum = 0;
    let len = v.len();
    for i in v.iter() {
        sum += i.pow(len as u32);
    }
    sum == num
   
}