use super::CONFIG;

pub fn info(msg: &String) {
    println!("{}", msg);
}

pub fn debug(msg: &String) {
    if CONFIG.debug == true {
        eprintln!("{}", msg);
    }
}
