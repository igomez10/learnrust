use std::{
    io::{Read, Write},
    net::TcpStream,
};

// import echo.rs
mod echo;
mod ls;
mod pwd;

fn main() {
    // parse flag name
    let flags: Vec<String> = std::env::args().collect();

    // if flags is empty then return
    if flags.len() < 2 {
        println!("No flag name");
        return;
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
        _ => println!("Unknown flag name: \"{}\"", command_name),
    }
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

// enum enumname {

// }

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
    let mut stream: TcpStream;
    let address = _flags[0].as_str();
    let result = TcpStream::connect(address);
    match result {
        Ok(tcp_stream) => {
            println!("connected succesfully");
            stream = tcp_stream
        }
        Err(_e) => {
            println!("failed to connect");
            std::process::exit(1);
        }
    }

    // write to socket in two functions to show that we can write multiple times
    // even after move
    let host_header = format!("Host: {}\r\n", address);
    write_to_stream(&mut stream, "GET / HTTP/1.1\r\n");
    write_to_stream(&mut stream, &host_header);
    write_to_stream(&mut stream, "User-Agent: curl/7.64.1\r\n");
    write_to_stream(&mut stream, "Accept: */*\r\n");
    write_to_stream(&mut stream, "\r\n");

    // read from socket
    read_from_stream(&mut stream);
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

fn read_from_stream(stream: &mut TcpStream) {
    let mut buffer = [0; 1024];
    let res = stream.read(&mut buffer);
    match res {
        Ok(num_bytes) => {
            println!("successfully read {} bytes from socket", num_bytes);
            println!("{}", String::from_utf8_lossy(&buffer));
        }
        Err(_e) => {
            println!("failed to read from socket")
        }
    }
}
