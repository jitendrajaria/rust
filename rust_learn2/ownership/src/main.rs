fn main() {
    let mut s = String::from("Hello");
    s = take_and_give_ownership(s);

    println!("Getting ownership back to the main fun {s}");
    let (s2, length) = calc_length(s);
    println!("The String s2 is {s2} and length is {length} ");
    // s.push_str("World");

    calculate_length(&s2);
    let x = 101;
    make_copy(x);

    // Mutable references
    let mut test_str = String::from("Test");
    change_str(&mut test_str);

    let r2 = &test_str;
    println!("{r2}");
    let r1 = &mut test_str;
    println!("{r1}"); // No error is thrown becaue r1,r2 does not exists simuntaneously
    // let r3 = &test_str;
    // let r4 = &mut test_str;
    // println!("{r1} {r2}"); //Will throw error of mutable ref

    // Slice type
    let test_string = String::from("Hello world");
    println!("First word is {}", first_word(&String::from("hello world")));
    println!(
        "First word using slice is {}",
        first_word_via_slice(&String::from("Hello world"))
    );
    println!(
        "First word using slice is {}",
        first_word_slice(&test_string[..])
    );

    //other slices
    let a = [1, 2, 3, 4];
    let slice = &a[1..3];

    println!("Slice of array is {:?}", slice);
}

fn take_and_give_ownership(mut s: String) -> String {
    s.push_str("World");
    return s;
}
fn make_copy(x: i32) {
    println!("Value of x is {x}");
}
fn take_ownership(s: String) {
    println!("String is {s}")
}

fn calc_length(s: String) -> (String, usize) {
    let length = s.len();
    (s, length)
}

// Using a value without transferring ownership and it is known as references
fn calculate_length(s: &String) -> usize {
    return s.len();
}

fn change_str(s: &mut String) {
    s.push_str("World");
}

// fn dangle() -> &String {
//     let s = String::from("Hello");
//     &s
// }

fn first_word(s: &String) -> String {
    let bytes = s.as_bytes();
    let mut res: String = String::from("");
    for (_i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return res;
        }
        res.push(char::from(item));
    }
    return s.to_string();
}

fn first_word_via_slice(s: &String) -> &str {
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }
    return &s[..];
}

fn first_word_slice(s: &str) -> &str {
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }
    return &s[..];
}
