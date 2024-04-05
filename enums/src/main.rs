
// #[derive(Debug)]
// enum IpAddrKind {
//     V4(u8, u8, u8, u8),
//     V6(String)
// }
//
// #[derive(Debug)]
// struct IpAddress {
//     kind: IpAddrKind,
//     address: String
// }
//
// enum Message {
//     Quit,
//     Move { x: i32, y: i32 },
//     Write(String),
//     ChangeColor( i32, i32, i32)
// }
//
//
// fn main() {
//     let localhost = IpAddrKind::V4(127, 0, 0, 1);
//
//
//
//     println!("The IP address for the localhost is {:?}", localhost)
// }

// fn main() {
//     let x = 10;
//     let y = Some(20);
//
//     let result = x + y.unwrap_or(10);
//     println!("The result is {}", result);
// }
enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(EgyptionGovernorate)
}
#[derive(Debug)]
enum EgyptionGovernorate {
    Sohag, // I am from here
    Cairo,
    Alex,
}

fn main() {
   let coin = Coin::Quarter(EgyptionGovernorate::Cairo);
   println!("The coin is {}", value_in_cents(coin));
}


fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(governorate) => {
            println!("Governorate quarter from {:?}", governorate);
            25
        }
    }
}



