
pub fn nth(n: u32) -> u32 {
    let nn = n +1;
    let mut count = 0;
    let mut i =2;
    let mut res = n;
    while count != nn {
        println!("{}", i);
        if is_prime(i) {
            count += 1;
            println!("found:{}", count);
            println!("found index:{}", i);
        }
        res = i;
        i += 1;
    };  
    res
}
fn is_prime(n: u32) -> bool {
    for a in 2..n {
        println!("hhhhhh:{}", n);
        if n % a == 0 {
            return  false;
        }
    }
    return  true;
}
