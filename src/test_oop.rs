use std::error;

// trait isvalid
trait IsValid {
    fn is_valid(&self) -> Result<(), Box<dyn error::Error>>;
}

#[derive(Debug)]
struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

#[derive(Debug)]
struct ReaderRole {
    read: bool,
}

impl IsValid for ReaderRole {
    fn is_valid(&self) -> Result<(), Box<dyn error::Error>> {
        if self.read == false {
            return Err("user is not active".into());
        }
        Ok(())
    }
}

impl User {
    // function is_valid validates the user and returns an error if invalid
    fn is_valid(&self) -> Result<(), Box<dyn error::Error>> {
        // validate active is true
        if self.active == false {
            return Err("user is not active".into());
        }

        // validate username is not empty, and smaller than 64 chars
        if self.username.len() == 0 || self.username.len() > 64 {
            return Err("username is invalid".into());
        }

        // validate email contains @ and smaller than 128 chars
        if !self.email.contains("@") || self.email.len() > 128 {
            return Err("email is invalid".into());
        }

        // validate email only contains 1 @
        let mut at_count = 0;
        for c in self.email.chars() {
            if c == '@' {
                at_count += 1;
            }
            if at_count > 1 {
                return Err("email is invalid: found more than 1 @".into());
            }
        }

        // validate sign_in_count is smaller than 1000
        if self.sign_in_count > 1000 {
            return Err("sign_in_count is invalid".into());
        }

        Ok(())
    }
}

// function pwd
pub fn test_oop(_flags: &[String]) {
    //print current path
    let demouser = User {
        active: dbg!(true),
        username: String::from("demo"),
        email: String::from("demo@demo.com"),
        sign_in_count: 0,
    };
    if demouser.is_valid().is_err() {
        println!("Invalid user {}", demouser.is_valid().unwrap_err());
        std::process::exit(1);
    }
    println!("{:#?}", demouser)
}
