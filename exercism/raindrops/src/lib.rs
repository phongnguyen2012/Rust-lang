pub fn raindrops(n: u32) -> String {
    let mut ketqua = String::new();
    if n % 3 == 0 {
        ketqua += "Pling";
    }
    if n % 5 == 0{
        ketqua += "Plang";
    }
    if n % 7 == 0{
        ketqua += "Plong";
    }
    if ketqua ==""{
        return n.to_string();
    }
    else {
        return ketqua;
    }
    
}