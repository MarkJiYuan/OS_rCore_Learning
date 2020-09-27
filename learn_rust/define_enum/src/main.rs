enum IpAddrKind {
    V4,
    V6
}

fn route(ip_type: IpAddrKind) {}

fn main() {
    enum Message {
        Quit,
        Move { x: i32, y: i32 },
        Write(String),
        ChangeColor(i32, i32, i32),
    }
}