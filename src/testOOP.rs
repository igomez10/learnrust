#[derive(Debug)]
struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

impl User {
    fn is_valid(&self) -> bool {
        // validate active is true
        if self.active == false {
            return false;
        }

        // validate username is not empty, and smaller than 64 chars
        if self.username.len() == 0 || self.username.len() > 64 {
            return false;
        }

        // validate email contains @ and smaller than 128 chars
        if !self.username.contains("@") || self.email.len() > 128 {
            return false;
        }

        // validate email only contains 1 @
        let mut found = false;
        for c in self.email.chars() {
            if c == '@' {
                if found == true {
                    return false;
                }
                found = true;
            }
        }

        // validate sign_in_count is not negative

        return true;
    }
}

// function pwd
pub fn testOOP(_flags: &[String]) {
    //print current path
    let demouser = User {
        active: dbg!(true),
        username: String::from("demo"),
        email: String::from("demo@demo.com"),
        sign_in_count: 0,
    };
    // println!("{}", user_to_string(demouser))
    println!("{:#?}", demouser)
}
