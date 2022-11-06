#[derive(Debug)]
enum IpAddr {
    V4(u8, u8, u8, u8),
    V6(String),
}

impl IpAddr {
    fn print(&self) {
        println!("{:?}", self)
    }
}

fn route(ip_kind: IpAddr) {}

fn main() {
    let four = IpAddr::V4(127, 0, 0, 1);
    let six = IpAddr::V6(String::from("::1"));

    println!("Hello, world!");

    let test = IpAddr::V6(String::from("This is V6"));
    test.print();

    let some_number = Some(6);
    let some_char = Some('a');
    let absent_number: Option<i32> = None;

    value_in_cents(Coin::Quarter(UsState::Alaska));

    let five = Some(5);
    let six = plus_one(five);
    let test_none = None;
    let none_plus_one = plus_one(test_none);

    println!("six: {:?}", six);
    println!("none_plus_one: {:?}", none_plus_one);
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

#[derive(Debug)]
enum UsState {
    Alabama,
    Alaska,
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => {
            println!("This is lucky penny!");

            1
        }
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("State quarter from {:?}!", state);

            25
        }
    }
}

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(number) => Some(number + 1),
    }
}
