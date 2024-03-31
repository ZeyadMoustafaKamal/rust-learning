/*
struct User {
    username: String,
    email: String,
    sign_in_count: u32,
    active: bool
}

fn main() {
    let user1 = User {
        username: String::from("Zeyad"),
        email: String::from("Zeyad20062018@gmail.com"),
        sign_in_count: 3,
        active: true
    };

    let username = user1.username;
    // println!("{username}");
    println!("{}", user1.email);
    let user2 = build_user(
        String::from("zeyad2@gmail.com"),
        String::from("Zeyad2")
        );
    // println!("{}", user1.email);
}

fn build_user(email: String, username: String) -> User {
    User{
        email,
        username,
        sign_in_count: 1,
        active: true
    }
}*/

#[derive(Debug)]
struct Recatangle {
    width: u32,
    height: u32
}

impl Recatangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn can_hold(&self, other:  &Recatangle) -> bool {
        self.width > other.width && self.height > other.height 
    }
}

impl Recatangle {
    fn square(size: u32) -> Recatangle {
        Recatangle{
            width: size,
            height: size
        }
    }
}

fn main() {
    let rectangle = Recatangle {
        width: 30,
        height: 50
    };
   
    let rectangle2 = Recatangle {
        width: 40,
        height: 30
    };
    let rectangle3 = Recatangle {
        width: 20,
        height: 40
    };

    let rectangle4 = Recatangle::square(25);

    println!("The square is : {:#?}", rectangle4);
    
    println!("The rectangle1 can hold the rectangle2 ?? {}", rectangle.can_hold(&rectangle2));
    println!("The rectangle 1 can hold the rectangle 2 ?? {}", rectangle.can_hold(&rectangle3));
    

    println!("Our rectangle is : {:#?}", rectangle);
    println!("The are of the rectangle is {}", rectangle.area())
}


