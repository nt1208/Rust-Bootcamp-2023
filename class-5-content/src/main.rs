use std::{error::Error, fmt::write};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // let res = foo(-10).unwrap();
    // let res = foo(-10).map_err(|_| "Wrong");
    // println!("{:?}", res);
    // let res = foo(-10);
    // match &res{
    //     Ok(value) => println!("Result: {}", value),
    //     Err(error) => println!("Error: {}", error)
    // }
    
    // // su dung if let cung tuong tu nhu match
    // if let Ok(value) = res{
    //     println!("Result with let: {}", value);
    // }
    // else{
    //     println!("Wrong input");
    // }
    
    // su dung kieu du lieu enum 
    //let res1 = foo_with_enum(-10);
    // match res1{
    //     Ok(value) => println!("Result: {}", value),
    //     Err(error) => println!("Error: {:?}", error)
    // }
    // let res2 = divide(10, 0)?;
   // let res3 = divide_2(10, 0)?; 
    //call_foo_2()?;
    // println!("Quotient: {:?}", res3);
    // println! ("Yeu Thao Nguyen");
    // let vec1 = vec![1,2,3,4];
    
    let bad = Level::default();
    println!("{:?}", bad );
    let a: Student = Student { grade: 8, girl_friend: vec!("Lan".to_string(), "Huong".to_string()) };
    println!("{:?}", a);
    Ok(())
    
}

fn foo(i: i32) -> Result<i32, String>{
    if i > 0{
        Ok(i)
    }
    else{
        Err("Wrong input".to_string())
    }
}

fn foo_2(i: i32) -> Result<i32, ErrorWithThisError>{
    if i > 0{
        Ok(i)
    }
    else{
        Err(ErrorWithThisError::WrongAnswer)
    }
}
fn foo_with_enum(i: i32) -> Result<i32, ErrorList>{
    if i > 0{
        Ok(i)
    }
    else{
        return Err(ErrorList::WrongInput)
    }
}

fn call_foo_2() -> Result<(), ErrorWithThisError>{
     foo_2(-10)?;
     Ok(())
}
impl std::fmt::Display for ErrorList{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self)
    }
}
#[derive(Debug)]
pub enum ErrorList{
    WrongInput
}

#[derive(Debug)]
pub struct CustomError{
    message: String
}
impl std::error::Error for CustomError{

}

impl std::fmt::Display for CustomError{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.message)
    }
}
fn divide(a: i32, b: i32) -> Result<i32, Box<dyn std::error::Error>>{
    if b == 0 {
        Err(Box::new(CustomError{message:"Denominator can not be zero!".to_string()}))
    }
    else {
        Ok(a / b)
    } 
}

//use anyhow
// rut gon duoc phan Result
use anyhow::{Result, anyhow}; 
fn divide_2(a: i32, b: i32) -> Result<i32>{
    if b == 0 {
            Err(anyhow!("Can not divide"))
    }
    else {
        Ok(a/b)
    }
}

// this error
#[derive(thiserror::Error, Debug)]

pub enum ErrorWithThisError {
    #[error("Wrong answer")] //convert tu enum sang reference string
    WrongAnswer,
}

// macro 
#[macro_export]
macro_rules! test {
    () => {
        println!("Ok");
    };
}
#[derive(Debug)]
pub struct Student{
    grade: u8,
    girl_friend: Vec<String>
}

#[derive(Debug)]
pub enum Level{
    FAIL,
    GOOD,
    OUTSTANDING
}

impl Default for Level{
    fn default() -> Self {
        Self::FAIL
    }
}