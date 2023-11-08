use std::fs;

// function ls
pub fn ls(_flags: &[String]) {
    let path = std::env::current_dir().unwrap();
    let files = fs::read_dir(path);

    match files {
        Ok(files) => {
            for file in files {
                let file = file.unwrap();
                let file_name = file.file_name();
                let file_name = file_name.to_str().unwrap();
                println!("{}", file_name);
            }
        }
        Err(e) => {
            println!("{}", e);
            std::process::exit(1);
        }
    }
}
