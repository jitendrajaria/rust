fn main() {
    // let mut counter = 0;
    // let result = loop {
    //     counter += 1;
    //     if counter == 10 {
    //         break counter * 2;
    //     }
    // };
    // println!("Result is {result}");
    // let mut count = 0;
    // 'counting_up: loop {
    //     println!("Count = {count}");
    //     let mut remaining = 10;

    //     loop {
    //         println!("remaining = {remaining}");
    //         if remaining == 9 {
    //             break;
    //         }
    //         if count == 2 {
    //             break 'counting_up;
    //         }
    //         remaining -= 1;
    //     }
    //     count += 1;
    // }
    // println!("End of count is {count}");

    // let mut number = 3;
    // while number > 0 {
    //     println!("Count {number}");
    //     number -= 1;
    // }
    // println!("Liftoff");

    // for i in (1..=3).rev() {
    //     println!("Count {i}");
    // }
    // println!("Liftoff");
    println!("Excercise 1");
    println!(
        "Temp conversion from celcius to fahernite {}",
        convert_temp(25.0, String::from("F")),
    );
    println!(
        "Temp conversion from Fahernite to celcius {}",
        convert_temp(77.0, String::from("C"))
    );
    println!(
        "Inadequate conversion {}",
        convert_temp(77.0, String::from("D"))
    );

    for i in 1..0 {
        println!("nth fib {}", generate_n_fibonacci(i));
    }
}

// excercises:

// Convert temperature between Fahrenhites and celsius
fn convert_temp(temp: f64, to_unit: String) -> f64 {
    return if to_unit == String::from("C") {
        (temp - 32.0) * 5.0 / 9.0
    } else if to_unit == String::from("F") {
        (temp * 9.0 / 5.0) + 32.0
    } else {
        println!("Cannot convert the inadequate temp");
        0.0
    };
}

fn generate_n_fibonacci(n: u32) -> i32 {
    let mut b = 1;
    let mut res = b;

    for _i in 2..n {
        let temp = res;
        res = res + b;
        b = temp;
    }
    return res;
}
