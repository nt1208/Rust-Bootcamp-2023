fn main() {
    // sử dụng closure
    println!("-----------------CLASS-2---------------");
    println!("CLOSURE");
    let x = |y: &str| -> String { y.to_string() }; 
    let res = x("VBI");
    println!("Res: {}", res);

    // for , iter, into_iter , iter_mut
    println!("----------------------------");
    println!("FOR, INTO_ITER, ITER");
    let vec1 = vec![1, 2, 3, 4, 5];

    // for value in vec1 {
    //     println!("Value:{}",value);
    // }

    let vec2 = vec![1, 2, 3, 4, 5];
    vec2.iter().enumerate().for_each(|x|{
        println!("Index:{ }, value: {}",x.0,x.1)

    });
    let res: Vec<i32> = vec2.iter().map(|x| x * 2).collect();
    println!("res: {:?}", res);

    let mut vec_new = vec![];

    for value in vec1 {
        vec_new.push(value * 2);
    }
    println!("Cach 2: {:?}", vec_new);

    let vec3: Vec<i32> = vec![1,2,3,4,5];
    let res = vec3.iter().max();
    println!("Max: {:?}", res);

    assert_eq!(count_char_occurrences("Rust is fun", 'u'), 2);
    assert_eq!(count_char_occurrences("Mississippi", 's'), 4);


    //let s2 = &s1[..];
    // gabage collector
    println!("----------------------------");
    println!("OWNERSHIP");
    let s1 = String::from("Hello world");
    // s1 đang sở hữu dữ liệu "Hello world"
    let s2 = s1;
    //s1 gán cho s2 và đồng thời s2  sở hữu dữ liệu "Hello world"
    // theo nguyên tắc ownership của Rust
    // 1 biến chỉ sở hữu 1 giá trị tương ứng ***
    // xoá s1: drop s1 ***
    // s1 move value to s2 
    println!("S2: {}", s2);
    //println!("S1:{}",s1);

    // Rust primitive : u32,u64, ..
    // Collections: vector, string,

    // Stack và Heap

    // Biến
    // là tên để lưu giá trị -> lưu ở 1 vùng nhớ nào đó
    // Memory Management

    // Pointer : trỏ địa chỉ của biến đó

    //let s3 = s1;
    // vi phạm nguyên tắc của ownership;
    // owernship: sở hữu

    //let s3 = s1;

    // ownership
    // {} : scope

    let x = 10;
    // x : global
    {
        //y: local
        let y = "Hello";
        println!("Y:{}", y);
    }
    println!("y:{}", x);
    
    let mut test = 9;
    {
        let mut tmp = test;
        let mut change = 10;
        test = change;
        change = tmp;
        println!("test in scope: {}", test);
    }
    println!("test out of scope: {}", test); 
    // Dù có thay đổi giá trị global bên trong scope thì giá trị đó vẫn giữ nguyên khi ra khỏi scope
    // Rust: out of scope -> value drop

    function_a();
    //print!("z:{}",z);

    println!("----------------------------");
    println!("POINTER");
    let x = 42;
    let y = 43;
    let var1 = &x;
    // pointer là 1 con trỏ -> trỏ tới biến 
    // mang địa chỉ 
    println!("var1(Value): {}", var1);
    println!("var1(Address): {:p}",var1); //Thêm :p để in ra địa chỉ

    let s3 = String::from("Hello "); //bộ nhớ động, luôn thay đổi địa chỉ sau mỗi lần run
    println!("s3(Address): {:p}", s3.as_ptr());
    let s4 = &s3;


    let s5 = s4;
    println!("s3 address:{:?}",s3.as_ptr());
    println!("s4 address:{:?}",s4.as_ptr());
    println!("s3: {}", s3); // vẫn có thể in ra s3 mà không vi phạm tính chát ownership vì s4 chỉ trỏ đến địa chỉ của s3 đang trỏ (s4 = &s3) mà
                            // làm thay đổi giá trị hoặc gán s3 vào s4.
    
    println!("----------------------------");
    println!("STACK AND HEAP");
    //println!("S3:{}",s3);
    // tham chiếu 
    // Stack và Heap 
    // Stack : địa chỉ ô nhớ để lưu giá trị cố định 
    // Heap: ô nhớ  động 

    // compile time: thời gian chuyển đổi từ mã ban đầu sang mã thực thi
    // runtime: thời gian chạy mã thực thi 

    // Primitive: default là stack -> biết size 
    // collections (string, vec) có bỏ vào stack dc hay ko ?
    // ko biết size ở compile time 
    // lưu ở HEAP 
    // Stack nhanh hơn heap

    // Phân biệt len và capacity, len là size của kiểu dữ liệu, capacity là khoảng ước chừng mà hệ thống cấp phát cho kiểu dữ liệu đó
    let mut s6 =String::from("Hell");
    println!("len:{}, capacity:{}", s6.len(),s6.capacity());
    s6.push_str("o");
    println!("len:{}, capacity:{}", s6.len(),s6.capacity());
    s6.push_str("World");
    println!("len:{}, capacity:{}", s6.len(),s6.capacity());
    s6.push_str("Worldabcdcd");
    println!("len:{}, capacity:{}", s6.len(),s6.capacity());


    println!("----------------------------");
    println!("SOLVING OWNERSHIP PROBLEM");
    println!("Solution 1 (Using clone()):");
    let mut s7 = String::from("Hell");
    let s8 =s7.clone(); // tạo ra bản sao s7 

    println!("s7 address:{:?}",s7.as_ptr());
    println!("s8 address:{:?}",s8.as_ptr());


    println!("***************");
    println!("Solution 2 (Using reference/borrowing):");
    println!("S7:{}",s7);
    s7.push_str("World");
    println!("s7: {}", s7);
    let s9 = &s7;
    // shared reference
    // immutable

    println!("s9: {}", s9);
    println!("s7: {}", s7);
    //let s10 = &s7;
    //let s11 = &s7;
    // có thể shared reference nhiều lần 


    // mutable reference 
    // có thể thay đổi giá trị
    // nhưng owner phải share quyền thay đổi mut 

    let s12 = &mut s7; //s7 cho phép s16 co quyền thay đổi giá trị
    s12.push_str(" World!");
    // println!("s7: {}",s7); Không thể print s7 trực tiếp vì đã trao quyền mutable cho s12
    println!("s12: {}",s12);
    //Note

    let s13 = &s7; //tại thời điểm này s12 kết thúc việc mutable với s7, thay vào đó là s13 thực hiện shared reference
    //println!("s12: {}",s12); //không thể thực hiện dòng này được vì s12 đã hết quyền mutable

    let s14 = &mut s7; //s13 dừng việc shared, thay vào đó là s14 mutable
    s14.push_str(" World!"); // không thể thực hiện s13 vì trong 1 thời điểm chỉ có thể có 1 reference(shared hoặc mutable)
                                    // Trường hợp trên do s13 không được sử dụng nên bị drop, chuyển mutable sang s14
    println!("s14: {}",s14);



    // s12.push_str("World");
    // println!("S7:{}",s7);
    // let s13 = &s7;
    // let s14 = &s13;






    //println!("s9:{}",s9);


    let mut s = String::from("hello");

    change(&mut s);
    //println!("s2:{}",s2);
    println!("s:{}",s);
}

fn change(some_string: &mut String) {
    
    some_string.push_str(", world");
}
fn function_a() {
    println!("Hello");
    let z = 10;
}

fn count_char_occurrences(string: &str, ch: char) -> usize {
    let res = string.chars().into_iter().filter(|c| c == &ch).count();
    res
}

// borrow checker 




