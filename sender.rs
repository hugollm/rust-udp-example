use std::net::UdpSocket;

fn main() {
    let socket = UdpSocket::bind("127.0.0.1:8001").unwrap();
    let mut count = 0;
    socket.connect("127.0.0.1:8002").unwrap();
    loop {
        match socket.send(&[1, 2, 3]) {
            Ok(_) => {
                count += 1;
                println!("{}", count);
            },
            Err(err) => println!("{}", err),
        }
    }
}
