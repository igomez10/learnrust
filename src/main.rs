mod api_create_user;
mod echo;
mod ls;
mod pwd;
mod test_oop;

use std::error::Error;
use std::net::ToSocketAddrs;
use std::{
    io::{Read, Write},
    net::TcpStream,
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // parse flag name
    let flags: Vec<String> = std::env::args().collect();

    // if flags is empty then return
    if flags.len() < 2 {
        println!("No flag name");
        return Ok(());
    }

    let command_name = flags[1].as_str();

    // switch command_name
    match command_name {
        "ls" => ls::ls(&flags[2..]),
        "pwd" => pwd::pwd(&flags[2..]),
        "whoami" => whooami(),
        "touch" => touch(&flags[2..]),
        "cat" => cat(&flags[2..]),
        "echo" => echo::echo(&flags[2..]),
        "mkdir" => mkdir(&flags[2..]),
        "cp" => cp(&flags[2..]),
        "curl" => curl(&flags[2..]),
        "nslookup" => nslookup(&flags[2..]),
        "test_oop" => test_oop::test_oop(&flags[2..]),
        "create_user" => api_create_user::e2e_user_lifecycle(&flags[2..]).await?,
        _ => println!("Unknown flag name: \"{}\"", command_name),
    }
    Ok(())
}
// function whoami will return the current user running the command
fn whooami() {
    let user = std::env::var("USER").unwrap();
    println!("{}", user);
}

// function touch
fn touch(_flags: &[String]) {
    let file_name = _flags[0].as_str();
    // check if file exists or not
    if std::path::Path::new(file_name).exists() {
        // exit with code 1
        println!("File already exists");
        std::process::exit(1);
    } else {
        // create file
        std::fs::File::create(file_name).unwrap();
    }
}

// function cat
fn cat(_flags: &[String]) {
    let file_name = _flags[0].as_str();
    // check if file exists or not
    if std::path::Path::new(file_name).exists() {
        // read file
        let file_content = std::fs::read_to_string(file_name).unwrap();
        println!("{}", file_content);
    } else {
        println!("File not found");
        // exit with code 1
        std::process::exit(1);
    }
}

// function mkdir
fn mkdir(_flags: &[String]) {
    let dir_name = _flags[0].as_str();
    // check if directory exists or not
    if std::path::Path::new(dir_name).exists() {
        println!("Directory already exists");
        // exit with code 1
        std::process::exit(1);
    } else {
        // create directory
        std::fs::create_dir(dir_name).unwrap();
    }
}

fn cp(_flags: &[String]) {
    let origin_file = &_flags[0];
    let destination_file = &_flags[1];
    let should_overwrite = _flags[2].as_str();

    println!("will copy {} to {}", origin_file, destination_file);

    // check if origin file exists
    if !std::path::Path::new(origin_file).exists() {
        println!("origin file {} not found", origin_file);
        std::process::exit(1);
    }

    // check if destination file exists
    if std::path::Path::new(destination_file).exists() {
        if should_overwrite == "true" {
            // If we need to overwrite, remove the file
            std::fs::remove_file(destination_file).unwrap();
        } else {
            println!(
                "destination file {} already exists and overwrite is not allowed",
                destination_file
            );
            std::process::exit(1);
        }
    }

    // read contents
    let contents = std::fs::read(origin_file).expect("Should have been able to read the file");

    // write to destination_file
    std::fs::write(destination_file, contents).expect("Should have been able to write to the file");

    println!("copy completed");
}

fn curl(_flags: &[String]) {
    let address = _flags[0].as_str();
    let method = "GET".to_string();
    http_request(&method, address);
}

// function to execute HTTP Request
fn http_request(method: &str, url: &str) {
    let mut stream: TcpStream;
    let result = TcpStream::connect(url);
    match result {
        Ok(tcp_stream) => stream = tcp_stream,
        Err(_e) => {
            std::process::exit(1);
        }
    }

    let host_header = format!("Host: {}\r\n", url);
    let method = format!("{} / HTTP/1.1\r\n", method);
    write_to_stream(&mut stream, &method);
    write_to_stream(&mut stream, &host_header);
    write_to_stream(&mut stream, "User-Agent: curl/7.64.1\r\n");
    write_to_stream(&mut stream, "Accept: */*\r\n");
    write_to_stream(&mut stream, "\r\n");

    println!();
    // read from socket
    let response = read_from_stream(&mut stream);
    match response {
        Ok(response) => {
            println!("{}", response);
        }
        Err(_e) => {
            println!("failed to read from socket");
        }
    }
}

fn write_to_stream(stream: &mut TcpStream, message: &str) {
    let res = stream.write(message.as_bytes());
    match res {
        Ok(num_bytes) => {
            println!("successfully wrote {} bytes to socket", num_bytes)
        }
        Err(_e) => {
            println!("failed to write to socket")
        }
    }
}

// function read_from_stream reads from socket and returns the result as a string
// this reads until the socket is closed
fn read_from_stream(stream: &mut TcpStream) -> Result<String, std::io::Error> {
    // vector to store all the bytes read from socket
    let mut buffer = Vec::new();

    // read from socket
    let result = stream.read_to_end(&mut buffer);

    // check if read was successful
    match result {
        Ok(_num_bytes) => {
            // convert bytes to string
            let response = String::from_utf8(buffer).unwrap();
            Ok(response)
        }
        Err(e) => Err(e),
    }
}

// nslookup command to resolve domain name to IP address
fn nslookup(_flags: &[String]) {
    let domain_name = _flags[0].as_str();
    // resolve domain name
    let result = (domain_name, 0).to_socket_addrs();
    match result {
        Ok(mut addresses) => {
            while let Some(address) = addresses.next() {
                println!("{}", address.ip());
            }
        }
        Err(_e) => {
            println!("failed to resolve domain name");
            std::process::exit(1);
        }
    }
}
