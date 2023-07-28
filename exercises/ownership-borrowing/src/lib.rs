// Exercise 1
// Make it compile
fn exercise1() {
    // Use as many approaches as you can to make it work
    // có thể sử dụng clone(), tuy nhiên không nên sử dụng clone() nhiều vì tốn bộ nhớ (clone() sao chép ra một vùng nhớ mới)
    let x = String::from("hello, world");
    let y = &x; // borrowing, tham chiếu đến vùng nhớ của x 
    let z = &x;
}

// 

// Exercise 2
// Make it compile
// Don't modify code in exercise2 function!
fn exercise2() {
    let s1 = String::from("hello, world");
    let s2 = take_ownership(s1); // có thể truyền vào s1.clone() hoặc &s1 để giải quyết vấn đề ownership
    println!("{}", s2);
}
// Only modify the code below!

fn take_ownership(s: String) -> String {
    s
}

// Exercise 3
// Make it compile
// Dont care about logic
fn exercise3() {
    let values: Vec<f64> = vec![
        2817.42, 2162.17, 3756.57, 2817.42, -2817.42, 946.9, 2817.42, 964.42, 795.43, 3756.57,
        139.34, 903.58, -3756.57, 939.14, 828.04, 1120.04, 604.03, 3354.74, 2748.06, 1470.8,
        4695.71, 71.11, 2391.48, 331.29, 1214.69, 863.52, 7810.01,
    ];

    let values_number = values.len();

    let additions: Vec<usize> = vec![0];

    println!("{:?}", values_number);

    while additions.len() > 0 {
        let mut addition: f64 = 0.0;

        // Sumar valores en additions
        // Solution 1
        for element_index in &additions{ // Khi duyệt qua từng phần tử trong addition, element_index sẽ mượn phần tử addition
            let addition_aux = values[*element_index]; // phải dereference element_index 
            addition = addition_aux + addition;
        }
        //Solution 2
        for element_index in additions.iter(){ // Khi duyệt qua từng phần tử trong addition, element_index sẽ mượn phần tử addition vì iter() cũng trả về kiểu reference
            let addition_aux = values[*element_index]; // phải dereference element_index 
            addition = addition_aux + addition;
        }
    }
}

// Exercise 4
// Make it compile
fn exercise4(value: u32) -> String {
    let str_value = value.to_string(); // Convert u32 to String
    let str_ref: &str = &str_value; // Obtain a reference to the String
    str_ref.to_string() // Return the reference to the String
}

fn exercise4_2(value: u32) -> &'static str {
    let str_value = value.to_string(); // Convert u32 to String
    let static_str = Box::leak(str_value.into_boxed_str()); // Convert String to &'static str
    static_str // Return a reference to the &'static str
}

// Exercise 5
// Make it compile
use std::collections::HashMap;
fn exercise5() {
    let mut my_map = HashMap::from([(1, "1.0".to_string()), (2, "2.0".to_string())]);

    let key = 3;
    let value = "3.0".to_string();
    let res = match my_map.get(&key) {
        Some(child) => child, // khi trả về child thì child là 1 slice của my_map (1 phần của my_map), ví dụ như &[1..4]
        None => {                               // sẽ bị giải phóng khi kết thúc phạm vi của hàm
            my_map.insert(key, value.clone()); //tương tự với child 
            //&value -->Không thể sử dụng tham chiếu ở đây vì value là biến cục bộ trong None, ra khỏi phạm vi sẽ bị giải phóng
            &value // bỏ dấu tham chiếu đi và chỉ trả về giá trị 
            // my_map.get(&key).unwrap()
        }
    };

    println!("{}", res);
}

// Exercise 6
// Make it compile

use std::io;

fn exercise6() {
    let mut prev_key: String = String::from(""); // vì prev_key là slice string (tham chiếu đến 1 vùng nhớ chứa dữ liệu) nên không thể thay đổi giá trị

    for line in io::stdin().lines() {
        let s = line.unwrap();

        let data: Vec<&str> = s.split("\t").collect();
        if prev_key.len() == 0 {
            prev_key = data[0].to_string();
        }
    }
}


// Exercise 7
// Make it compile
fn exercise7() {
    let mut v: Vec<String> = Vec::new();
    {
        let chars = [b'x', b'y', b'z'];
        let s = std::str::from_utf8(&chars).unwrap().to_string();
        v.push(s);
    }
    println!("{:?}", v);
}

// Exercise 8
// Make it compile
fn exercise8() {
    let mut accounting = vec!["Alice".to_string(), "Ben".to_string()];
    
    loop {
        let mut add_input = String::from("");

        io::stdin()
            .read_line(&mut add_input)
            .expect("Failed to read line");

        let add_vec: Vec<&str> = add_input.trim()[..].split_whitespace().collect();

        if add_vec.len() < 1 {
            println!("Incorrect input, try again");
            continue;
        }

        let person = add_vec[0].to_string(); 
        accounting.push(person); // person bị dropped khi ra khỏi scope, mà accounting lại tham chiếu đến vùng nhớ đã bị drop --> lỗi
    }
}