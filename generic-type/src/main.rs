///////////////////////////////////////////
// BAI 1
// Yêu cầu :
// + Sửa code liên quan tới vấn đề generic type (thay đổi ở định nghĩa struct)
///////////////////////////////////////////


// struct Point<T> {
//     x: T,
//     y: T,
// }

// fn main() {
//     // DON'T modify this code.
//     let p = Point{x: 5, y : "hello".to_string()};

//     println!("Success!");
// }
//*********************SUA BAI 1 **************** */
struct Point<T, U> {
    x: T,
    y: U,
}




///////////////////////////////////////////
// BAI 2
// Yêu cầu :
// + Implement hàm sum dưới đây, sao cho việc kiểm tra assert_eq đúng 
///////////////////////////////////////////


// Implement the generic function below.
// fn sum

// fn main() {
//     assert_eq!(5, sum(2i8, 3i8));
//     assert_eq!(50, sum(20, 30));
//     assert_eq!(2.46, sum(1.23, 1.23));

//     println!("Success!");
// }
//+++++++++++++++++++++++SUA BAI 2 ++++++++++++++++++++++++++++++++
// use std::ops::Add;
// struct Point<T> {
//     x: T,
//     y: T,
// }
fn sum<T: std::ops::Add>(x: T, y: T) -> T::Output {
    
    x + y
}



///////////////////////////////////////////
// BAI 3
// Yêu cầu :
// + Sửa lại code sao cho compile cho đúng 
///////////////////////////////////////////


// fn main() {
//     let a = A {p: Some("p".to_string())};
//     a.a();
// }

struct A {
    p: Option<String>
}


impl A {
    fn a(self) -> Self {
        Self::b(&self.p.as_ref().unwrap());
        self
    }
    fn b(b: &str) {
        print!("b: {}", b)
    }
}

//*********************SUA BAI 3 **************** */
// fn main() {
//     let a = A {p: Some("p".to_string())};
//     a.a();
// }

// struct A {
//     p: Option<String>
// }


// impl A {
//     // fn a(self) -> Self {
//     //     Self::b(&self.p.as_ref().unwrap());
//     //     &Option<String> --> Option<&String> --> &String
//     //     self
//     // }
//     // fn b(b: &str) {
//     //     println!("b: {}", b)
//     // }
//     //*****************cach 2************ */
//     // fn a(self) {
//     //     Self::b(&self.p.unwrap());
        
//     // }
//     // fn b(b: &str) {
//     //     println!("b: {}", b)
//     // }

// }



///////////////////////////////////////////
// BAI 4
// Yêu cầu :
// + Sửa lại code sao cho compile cho đúng 
///////////////////////////////////////////






// #[derive(Debug, Clone)]
// struct MyData {
//     val1: i32,
//     val2: String,
// }



// fn main() {
//     let d = MyData {
//         val1: 35,
//         val2: String::from("Hello World"),
//     };

//     let both = d.get_both();
//     let x = d.get_val1();
//     let y = d.get_val2();
// }


// impl MyData {
//     pub fn get_val1(self) -> i32 {
//         return self.val1.clone();
//     }

//     pub fn get_val2(self) -> String {
//         return self.val2.clone();
//     }

//     pub fn get_both(self) -> (i32, String) {
//         return (self.val1, self.val2);
//     }
// }

// fn main() {
//     let d = MyData {
//         val1: 35,
//         val2: String::from("Hello World"),
//     };

//     let both = d.get_both();
//     let x = d.get_val1();
//     let y = d.get_val2();
// }

struct MyData {
    val1: i32,
    val2: String,
}
impl MyData {
    pub fn get_val1(&self) -> i32 {
        return self.val1;
    }

    pub fn get_val2(&self) -> String {
        return self.val2.to_string();
    }

    pub fn get_both(&self) -> (i32, String) {
        return (self.val1, self.val2.to_string());
    }
}


fn main() {

    //******************Bai tap 1********************** */
    let p = Point{x: 5, y : "hello".to_string()};

    //******************Bai tap 2********************** */
    assert_eq!(5, sum(2i8, 3i8));
    assert_eq!(50, sum(20, 30));
    assert_eq!(2.46, sum(1.23, 1.23));

    println!("Success!");

    //******************Bai tap 3********************** */
    
        let a = A {p: Some("p".to_string())};
    a.a();

    let d = MyData {
        val1: 35,
        val2: String::from("Hello World"),
    };
    //******************Bai tap 4********************** */

    let both = d.get_both();
    let x = d.get_val1();
    let y = d.get_val2();
}

