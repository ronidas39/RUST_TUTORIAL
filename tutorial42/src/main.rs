//if it is divided by 4 and 100 and 400 it is leap year
//if it is divided by 4 and 100 but not divided by 400 then it not leap year
//if it is divided by 4 only then it is leap year
//else not leap year

use std::io;

fn check_leap_year(year_input:i32)->bool{
if year_input%4==0{
    if year_input%100==0{

        if year_input%400==0{
            true
        }
        else{
            false
        }

    }
    else{
        true
    }

}
else {
    false
}

}

fn main(){
    println!("please enter a year");
    let mut year=String::new();
    io::stdin().read_line(&mut year).expect("error occured while reading");
    let year_int:i32=year.trim().parse::<i32>().unwrap();
    let x:bool=check_leap_year(year_int);
    if x{
        println!("{} is leap year",year_int);
    }
    else{
        println!("{} is not leap year",year_int);

    }


}
