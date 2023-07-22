
fn solve(k: u32, c: u32, x: u32, q: u32) -> u32{
    let s = (k as i32 - (c * x) as i32) % (q as i32);
    
    if s >= 0{
        s as u32
    }else{
        assert!(q as i32 + s >= 0, "Not!");
        (q as i32 + s) as u32
    }
}

fn main() {
    let array = [1.0, 3.0, 4.0];
    println!("array: {}, {}", array[0], array[1]);

    let tuple1 = (2,3, 4.0, "s");
    println!("tuple element: {}", tuple1.2);   
 
    type A = u32; // alias of u32
    
    let a = 32; // bien a khong the bi thay doi (immutable) (vi du a = a + b)
    let mut a = a + 12; // co the dinh nghia lai bien a thanh mutable
    println!("a: {}", a);
    
    let x = 1;
    let mut b = 10; // bien b co the duoc thay doi (mutable)
    b = b + 10;
    println!("b(mutable variable): {}", b);

    const PI: f64 = 3.14; // khong the thay doi trong suot chuong trinh, nen viet hoa ten bien const
    
    let mut s = String::new(); // cap phat cho s 1 chuoi rong, kich thuoc dong
    println!("S is empty:? {}", s.is_empty());
    s.push('h');
    println!("s: {}", s);
    s = s + "h";
    println!("s: {}", s);

    let mut f = String::from("Hello World!");
    println!("f: {}", f);

    //reference string: chi co the doc gia tri duoc khoi tao ban dau ma khong the sua doi 
    let s2 = "Hello World!";
    println!("result is: {}", &s2[0..5]);

    let s3 = "123";
    //s3 = s3 + "456"; khong thuc hien duoc 
    
    //conversion &str -> String
    let convert_str_to_String = "Ho Ngoc Thien".to_string(); // chuyen tu kieu &str sang String
    let convert_String_to_str = convert_str_to_String.as_str(); // chuyen String sang &str
    let test_byte = convert_String_to_str.as_bytes();
    println!("byte: {:?}", test_byte);
    let str_format = format!("{}", "Hello World!");
    println!("Format string: {}", str_format);

    let check = true;
    if check{
        println!("If-else structue: That's correct!");
    }
    else{
        println!("If-else structue: Oops..It's false!");
    }
    
    //patter matching giong switch case
    match check {
        true => println!("That's correct!"),
        false => println!("Oops..It't false")
    }

    let number = 10;
    match number {
        7 | 8 => println!("Match structure: That's correct!"), //co the su dung dau hoac
        10 | 12 => println!("Match structure: Oops..It't false"),
        _ => println!("Match structure: Gruhh..It's empty")
    }

    let vec = vec![1,2,3,4,5,6];
    let mut index = 0;
    let max = vec.iter().max().unwrap();
    println!("Max of vec: {}", max);

    // for value in vec{
    //     println!("Value of {}: {}", index, value);
    //     index = index + 1;
    // }

    // **phan biet for, iter(), iter_mut(), into_iter()**
    // iter duoc su dung nhieu hon 
    
    for run in 4..vec.len(){
        println!("Check index: {}", run);
    }

    for value in vec.iter(){
        println!("Value of {}: {}", index, value);
        index = index + 1; 
    }
    func_no_parameters();
    func_have_parameters(s.clone()); // muon bien s
    println!("Returned value: {}",func_have_parameters_and_returned_value(s));
} 

fn func_no_parameters(){
    println!("Sorry, I don't have parameter and returned value.");
}

fn func_have_parameters(s:String){
    println!("Yeah i have parameters, but i don't return value.");
}
fn func_have_parameters_and_returned_value(s:String) -> String{
    println!("Yeah i have parameters and returned value.");
    let a = String::from("Returned value!");
    a
}