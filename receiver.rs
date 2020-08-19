use std::net::UdpSocket;
use std::thread;
use std::time;

fn main() {
    let socket = UdpSocket::bind("127.0.0.1:8002").unwrap();
    let mut buf = [0; 10];
    let mut count = 0;
    loop {
        match socket.recv(&mut buf) {
            Ok(_) => {
                count += 1;
                thread::sleep(time::Duration::from_millis(100));
                println!("{}", count);
            },
            Err(e) => println!("recv function failed: {:?}", e),
        }
    }
}
