use std::net::TcpListener;

const TCP_ADDRESS: &'static str = "127.0.0.1";
const PORT: i32 = 7878;

fn main() {
    let address = format!("{TCP_ADDRESS}:{PORT}");
    let listener = TcpListener::bind(address).unwrap();

    println!("App is running, listening on the port {PORT}");

    for stream in listener.incoming() {
        let _stream = stream.unwrap();

        println!("Connection established!");
    }
}
