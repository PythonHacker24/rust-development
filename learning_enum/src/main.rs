enum Option<T> {
    None,
    Some(T),
}

enum IpAddrKind {
    V4,
    V6,
}

struct IpAddr {
    kind: IpAddrKind,
    address: String,
}

#[derive(Debug)]
enum NewIpAddr {
    V4(u8, u8, u8, u8),
    V6(String),
}

#[derive(Debug)]
enum Message {
    Quit,
    Move {x: i32, y: i32},
    Write(String),
    ChangeColor(i32, i32, i32),
}

impl Message {
    fn print_data(&self) {
        println!("{:#?}", self);
    }
}

#[derive(Debug)]
struct Rectange {
    length: i32,
    breath: i32,
}

impl Rectange {
    fn perimeter(&self) {
        println!("{}", 2*(self.length * self.breath));
    }

    fn area(&self) {
        println!("{}", self.length * self.breath);
    }
}

fn main() {
   
    let home = IpAddr {
        kind: IpAddrKind::V4,
        address: String::from("127.0.0.1"),
    };

    let loopback = IpAddr {
        kind: IpAddrKind::V6,
        address: String::from("::1"),
    };

    let new_home = NewIpAddr::V4(127, 0, 0, 1);
    let new_loopback = NewIpAddr::V6(String::from("::1"));
    
    println!("home_address: {}\nloopback_address: {}", home.address, loopback.address);
   	println!("new_home_address: {:#?}\nloopback_address: {:#?}", new_home, new_loopback);
   
    let quit_message = Message::Quit;
    let message = Message::Write(String::from("hello!"));
  	let color_change = Message::ChangeColor(100, 100, 100);
    
	message.print_data();
    color_change.print_data();
    
    let rectangle = Rectange {
        length: 20,
        breath: 30,
    };

    rectangle.perimeter();
    rectangle.area();
}
