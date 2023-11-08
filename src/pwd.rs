// function pwd
pub fn pwd(_flags: &[String]) {
    //print current path
    let path = std::env::current_dir().unwrap();
    println!("{}", path.display());
}
