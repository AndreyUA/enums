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
}
