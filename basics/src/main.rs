use std::{io, str};
fn checksub(a: &[i32], b: &[i32]) -> bool {
    let mut check = false;
    let mut count =0;
    for i in a.iter(){
        for j in b.iter(){
            if i == j {
                count += 1
            }
        }
    }
    if count == b.len(){
        check = true
    }
    check
}

fn input() -> String {
    println!("Guess the number!");

    println!("Please input your guess.");

    let mut guess = String::new();

    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");

    println!("You guessed: {}", guess);

   
    guess 
    
}
fn checkstr(s: &str, c: char) -> (i32, String, Vec<char>) {
    let mut count = 0;
    let mut  s1 = String::new();
    let mut  v1 = Vec::new();
   
    for x in s.chars() {
        if x == c {
            count +=1;
            s1 = x.to_string();
        }
        else {
            v1.push(x);
            
        }
    }
   
   
   (count, s1, v1)
}

fn main() {
    println!("Hello, world!");
  
    let mut count =0;
    let a = [1, 2,3,5,6,8, 10, 11, 55];
    let b = [6,8,10,55];
    let result = checksub(&a, &b);
    if result == true {
        println!("{:?} la mang con cua {:?}", &b, &a);
    }
    else {
        println!("{:?} khong phai la mang con cua {:?}", &b, &a);
    }

   
    let s1 = "abcdAafhoiurbcbc".to_lowercase();
    println!("Chuoi chu so sau {} ", s1);
    let c = input().trim().chars().next().unwrap();
 
    let  result = checkstr(&s1, c);
    println!("ky tu {} ban nhap vao lap lai {} lan ",result.1,result.0);
    println!("Chuoi con khong co ky tu nha vao {:?}", result.2);
   
}
