// fn find_avg(a: i32, b: f64, c: f32, value: f64) -> f64 {
//     let sum = a as f64 + b + c as f64;

//     let avg = sum / 3.0;

//     assert_eq!(avg, value);
//     return avg;
// }

// fn array_example() {
//     let mut arr = [1, 2, 3, 4, 5];
//     println!("Array is {:#?}", arr);
//     println!("Array is {}", arr.len());
//     arr[1] += 10;
//     println!("Array is {:#?}", arr);
//     let arr2: [u32; 5] = [1; 5];
//     println!("Array is {:#?}", arr2);

//     let two_d_arr = [[1, 2, 3], [4, 5, 6], [7, 8, 9]];
//     println!("2D Array is {:#?}", two_d_arr);
//     println!("2D Array is {}", two_d_arr.len());
//     println!("2D Array is {}", two_d_arr[1][2]);
//     println!("2D Array is {}", two_d_arr[2][2]);
//     println!("2D Array is {}", two_d_arr[2][2]);

//     let two_d_arr2: [[u32; 3]; 2] = [[1; 3]; 2];
//     println!("2D Array is {:#?}", two_d_arr2);

//     let two_d_arr3: [[u32; 3]; 3] = [[101; 3]; 3];
//     println!("2D array is {:#?}", two_d_arr3);
// }

// fn convert_to_f(temp: f64) -> f64 {
//     return (temp * 9.0 / 5.0) + 32.0;
// }

// fn loop_example() {
//     for a in 1..4 {
//         print!("(a {})", a)
//     }
//     let arr = [1, 2, 34, 5];
//     let mut i = 0;
//     while true {
//         print!("{}", arr[i]);
//         i += 1;

//         if i == 3 {
//             break;
//         }
//     }
//     loop {
//         if i >= arr.len() {
//             break;
//         }
//         println!("{}", arr[i]);
//         i += 1;
//     }
//     for item in arr {
//         println!("item print in item loop {}", item);
//     }
//     for (index, item) in arr.iter().enumerate() {
//         println!("item at index {index} is {item}")
//     }
// }

// fn loop_excercise() {
//     let numbers = [1, 9, -2, 0, 23, 20, -7, 13, 37, 20, 56, -18, 20, 3];
//     let mut max: i32 = i32::MIN;
//     let mut min: i32 = i32::MAX;
//     let mut mean: f64 = 0.0;

//     /* YOUR CODE GOES HERE */
//     for &item in numbers.iter() {
//         max = if max < item { item } else { max };
//         min = if min > item { item } else { min };
//         mean = mean + item as f64;
//     }
//     mean = mean / numbers.len() as f64;
//     println!("Max is {max}, Min is {min}, Mean is {mean}");
//     assert_eq!(max, 56);
//     assert_eq!(min, -18);
//     assert_eq!(mean, 12.5);
//     println!("Tests passed!");
// }

// fn ownership_example() {
//     let s = "Hello";
//     let s2: &str;

//     let s3 = s.as_bytes()[0];
//     println!("s3 is {s3}");
//     // let mut rng = rand::thread_rng()
//     // println!("s is {s} and s2 is {s2}");
// }

// fn ownership_excercise(input_string: &str) -> &str {
//     // remove leading and trailing whitespace from a string without using trim()
//     let mut start = 0;
//     let mut end = input_string.len();

//     let mut start_index_without_whitespace = 0;
//     let mut end_index_without_whitespace = input_string.len();
//     while start < end && input_string.as_bytes()[start] == b' ' {
//         start_index_without_whitespace += 1;
//         start += 1;
//     }

//     while end >= start && input_string.as_bytes()[end - 1] == b' ' {
//         end -= 1;
//         end_index_without_whitespace -= 1
//     }
//     return &input_string[start_index_without_whitespace..end_index_without_whitespace + 1];
// }
// fn main() {
//     // let a = 2.0;
//     // let b = 3;
//     // let c: i32 = a as i32 + b;
//     // //  let multi = a as f64 * b as f64;
//     // //  let div = a as f64 / b as f64;
//     // //  let sub = a as f64 - b as f64;
//     // println!("Value of c is: {c}");

//     // //  String formatting in Rust
//     // let name = "John";
//     // let money = 10.45;
//     // let age = 30;
//     // println!("Name is {} and age is {}", name, age);
//     // println!("Age is {age}");
//     // println!("Name is {0} and age is {1}", name, age);
//     // println!("{} has {1:18.2} dollars", name, money);
//     // println!("{n} has {m:1>7.2}$ dollars", n = name, m = money);
//     // let name2 = "John";
//     // let width = 10;
//     // let score = 85;
//     // let precision = 2;
//     // println!("{name2}:{score:width$.precision$}");
//     // let avg = find_avg(13, 2.3, 120.0, 45.1);
//     // println!("Average is {avg}");
//     // println!("Temperature in Fahrenheit is {}", convert_to_f(0.0));
//     // loop_example();
//     // loop_excercise();
//     let s = "   Hello World   ";
//     println!(
//         "Trimmed string is {}, original string is {} test",
//         ownership_excercise(s),
//         s
//     );
// }

// understanding move semantics
fn test(s: String) {
    println!("Passed string is {s}");
}

// String s resides in heap
//
fn main() {
    let s = String::from("Hello world");
    test(s);
    println!("String in main {}", s)
}
