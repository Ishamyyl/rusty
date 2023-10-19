#![allow(unused)]

fn main() {
    let mut s0 = "t";
    let s1: String = s0.into();
    let s2 = String::from(s0);
    let s3 = s0.to_string();
    s0 = "y";
    println!("{} {} {} {}", &s0, &s1, &s2, &s3);

    let mut s4 = s1 + &s2;
    s4.push_str(s0);
    println!("{} {} {} {}", &s0, &s2, &s3, &s4);
}
