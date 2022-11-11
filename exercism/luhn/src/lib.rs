pub fn is_valid(number_card: &str) -> bool {
    let mut count = 0;
    let mut total_sum = 0;
    for (i, c) in number_card.chars().rev().filter(|c| !c.is_whitespace()).enumerate() {
        match c.to_digit(10) {
            None => { println!("plop"); return false },
            Some(mut n) => {
                if i % 2 == 1 {
                    n *= 2;
                    if n > 9 {
                        n -= 9;
                    }
                }
                total_sum += n;
                count += 1;
            },
        }
    }
    println!("plip");
    count > 1 && total_sum % 10 == 0
}