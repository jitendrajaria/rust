// // #[derive(Debug)]
// // struct IPAddr {
// //     kind: IpAddrKind,
// //     address: String,
// // }

// // #[derive(Debug)]
// // enum IpAddrKind {
// //     V4(String),
// //     V6(String),
// // }
// // enum IpAddrEnum {
// //     V4(u8, u8, u8, u8),
// //     V6(String),
// // }
// // fn main() {
// //     // let four = IpAddrKind::V4;
// //     // let six: IpAddrKind = IpAddrKind::V6(String::from("128.0.0.0.0.01"));
// //     // let home = IPAddr {
// //     //     address: String::from("0.0.0.0"),
// //     //     kind: IpAddrKind::V4(String::from("0.0.0.0")),
// //     // };
// //     // println!("Home info is {:?}", home);
// //     // route(four);
// //     let home = IpAddrKind::V4(String::from("0.0.0.0"));
// //     route(home);

// //     // We can not do this by using struct;

// //     // struct IpStruct{
// //     //     kind:IpAddrEnum,
// //     //     address: String|(u8,u8,u8,u8)
// //     // }

// //     // Above will throw error

// //     let home= IpAddrEnum::V4(0,0,0,0);
// //     let loopback = IpAddrEnum::V6(String::from("::1"))
// // }

// // fn route(ip_addr: IpAddrKind) {
// //     // println!("IP address is {:?}", ip_addr);
// // }

// #[derive(Debug)]
// struct Ipv4Addr {
//     address: (u8, u8, u8, u8),
// }
// #[derive(Debug)]
// struct Ipv6Addr {
//     address: String,
// }

// #[derive(Debug)]
// enum IpAddr {
//     V4(Ipv4Addr),
//     V6(Ipv6Addr),
// }
// #[derive(Debug)]
// enum Message {
//     Quit,
//     Move { x: i32, y: i32 },
//     Write(String),
//     ChangeColor(i32, i32, i32),
// }
// impl Message {
//     fn call(&self) {
//         println!("This is call for self {:?}", self);
//     }
// }

// fn main() {
//     let four = IpAddr::V4(Ipv4Addr {
//         address: (1, 2, 3, 4),
//     });
//     let six = IpAddr::V6(Ipv6Addr {
//         address: String::from("::1"),
//     });

//     println!("The value of four is {:?}", four);
//     println!("The value of six is {:?}", six);

//     let m = Message::Write(String::from("Value"));
//     m.call();
//     let m2 = Message::Quit.call();
// }

/**
 * Option Enum
 */

enum Coin {
    Penny,
    Nickle,
    Dime,
    Quarter(State),
}

fn value_in_cents(coin: Coin) -> u8 {
    return match coin {
        Coin::Penny => 1,
        _x => 5,
    };
}

fn sum_value(value: Option<(u8, Option<u8>)>) -> Option<u8> {
    match value {
        Some((x, y)) => match y {
            //Binding Some to value
            None => Some(x),
            Some(y) => Some(x + y),
        },
        x => None,
    }
}

#[derive(Debug)]
enum State {
    Alabama,
    Alaska,
}

fn main() {
    let some_number = Some(5);
    let absent_number: Option<i32> = None;

    let x = 5;

    // println!("x+y is {}", some_number + x);
    let coin = Coin::Penny;
    println!(
        "Value in cents: {}",
        value_in_cents(Coin::Quarter(State::Alabama))
    );

    println!("Sum value is x,y is  {:?}", sum_value(Some((4, None))));
    println!("Sum value is x,y is  {:?}", sum_value(Some((4, Some(4)))));

    let config_max = Some(3u8);
    match config_max {
        Some(valy) => println!("The mx value to be configured is {}", valy),
        _ => (),
    }

    if let Some(max) = config_max {
        println!("The max value to be config {}", max);
    }
}
