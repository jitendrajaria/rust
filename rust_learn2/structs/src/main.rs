// struct User {
//     username: String,
//     active: bool,
//     email: String,
//     sign_in_count: u64,
// }

// struct Color(i32, i32, i32);
// struct Point(i32, i32, i32);
// fn main() {
//     let mut user1 = User {
//         username: String::from("Jitendra"),
//         active: true,
//         email: String::from("jj@jj.com"),
//         sign_in_count: 10,
//     };
//     user1.email = String::from("aj@aj.com");

//     let user2 = User {
//         email: String::from("email.aemal@com"),
//         ..user1
//     };
//     // println!("User 1 email: {} {}", user1.email, user2.email, user1);

//     // Struct tupples
//     let color = Color(0, 0, 0);
//     let point = Point(0, 0, 0);
//     let color2 = (0, 0, 0);

//     let Color(r, g, b) = color;
//     let (r1, g1, b1) = color2;

//     println!(
//         "Point is {:?}, and color is {:?},{}",
//         point.0, color.0, color2.0
//     )

// }

// fn build_user(email: String, username: String) -> User {
//     User {
//         active: true,
//         sign_in_count: 1,
//         username,
//         email,
//     }
// }
#[derive(Debug)]
struct Rectangle {
    width: i32,
    height: i32,
}
impl Rectangle {
    fn area(self: &Self) -> i32 {
        return self.width * self.height;
    }
    fn width(&self) -> bool {
        return self.width > 0;
    }
    fn can_hold(&self, rect: &Rectangle) -> bool {
        if self.width > rect.width && self.height > rect.height {
            return true;
        }
        return false;
    }
    fn square(size: i32) -> Self {
        Self {
            width: size,
            height: size,
        }
    }
}
fn main() {
    let rect1 = Rectangle {
        width: 10,
        height: 20,
    };

    let rect2 = Rectangle {
        width: 5,
        height: 5,
    };
    if rect1.width() {
        println!("The rectangle has a positive length {}", rect1.width);
    }
    println!("Area of the rectange is {}", rect1.area());
    println!("Widht of the rectangle was {:?}", rect1);

    println!(
        "Checking if rect 1 can hold rect2 ,{}",
        if rect1.can_hold(&rect2) { "yes" } else { "no" }
    );
    println!("lets check square {:?}", Rectangle::square(10))
}
