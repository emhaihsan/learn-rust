// Definisi Struct Biasa
#[derive(Debug)]
struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

#[derive(Debug)]
struct Rectangle{
    width: u32,
    height: u32,
}

// Implementasi (impl) block untuk Rectangle
impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }

    fn square(size: u32) -> Self {
        Self {
            width: size,
            height: size,
        }
    }

    fn width(&self) -> bool {
        self.width > 0
    }
}

// mendefinisakn tuple struct
struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

struct AlwaysEqual;

fn main() {
    println!("--- Chapter 5: Structs ---");

    // --- 1. Structs ---
    println!("\n--- 1. Structs ---");

    // Struct Biasa
    let mut user1 = User {
        active: true,
        username: String::from("user1"),
        email: String::from("user1@example.com"),
        sign_in_count: 1,
    };
    println!("User: {:?}", user1);

    user1.email = String::from("user1@example.com");
    println!("User 1 (email updated): {:?}", user1);

    fn build_user(email: String, username: String) -> User {
        User {
            active: true,
            username,
            email,
            sign_in_count: 1,
        }
    }

    let user2 = build_user(String::from("user2@example.com"), String::from("user2"));
    println!("User 2: {:?}", user2);

    let user3 = User {
        email: String::from("user3@example.com"),
        ..user2
    };
    println!("User 3: {:?}", user3);

    println!("\n -- Contoh Struct Biasa (rectangle) --");
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };
    let rect2 = Rectangle {
        width: 10,
        height: 40,
    };
    let rect3 = Rectangle {
        width: 60,
        height: 40,
    };
    println!("\nArea of the rectangle: {} square pixels", rect1.area());
    
    // memanggil method can_hold
    println!("\nCan rect1 hold rect2? {}", rect1.can_hold(&rect2));
    println!("\nCan rect1 hold rect3? {}", rect1.can_hold(&rect3));

    // memanggil associated function square
    let square = Rectangle::square(25);
    println!("\nSquare: {:?}", square);
    println!("\nArea of square: {}", square.area());

    // Membedakan field dan method dengan nama yang sama
    if rect1.width() {
        println!("\nRectangle 1 has non zero width");
    }
    
    println!("\n -- Contoh Debugging dengan dbg! --");
    let scale = 2;
    let rect_dbg = Rectangle {
        width: dbg!(30 * scale),
        height: 50,
    };
    dbg!(&rect_dbg);

    println!("\n -- Contoh Tuple Struct --");
    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);
    println!("Black Color: ({}, {}, {})", black.0, black.1, black.2);
    println!("Origin Point: ({}, {}, {})", origin.0, origin.1, origin.2);

    // Tuple struct destructuring
    let Color(r,g,b) = black;
    let Point(x,y,z) = origin;
    println!("\n -- Contoh Tuple Struct Destructuring --");
    println!("Black Color: ({}, {}, {})", r, g, b);
    println!("Origin Point: ({}, {}, {})", x, y, z);

    println!("\n -- Contoh Unit Struct --");
    let subject = AlwaysEqual;
    println!("Unit like struct instance created");
}