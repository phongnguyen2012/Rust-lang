pub fn factors(mut n: u64) -> Vec<u64> {
       let mut v: Vec<u64> = Vec::new();
    let mut count = 2..;
    while n >1 {
        let x = count.next().unwrap();
        while n % x == 0 {
            n /= x;
            v.push(x);
            
        }
    }
    v
}
