//how to read user input in rust
//how to convert user input into integer in rust
use std::io;
fn convert_to_int(year_input:& String)->i32{
    let x=year_input.trim().parse::<i32>().unwrap();
    x
}
fn main(){
    println!("please enter the number");
    let mut year=String::new();
    io::stdin().read_line(&mut year).expect("got error while reading");
    println!("numer is {}",convert_to_int(& year)*2);
    println!("{}",year);
}