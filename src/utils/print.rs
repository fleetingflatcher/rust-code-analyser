use colored::{Colorize, ColoredString};

pub fn out(tag: &str, msg: &str) {
    let c_str: ColoredString = format!("[{}]:\t{}", tag, msg).green();
    println!("{}", c_str);
}

pub fn debug(tag: &str, msg: &str) {
    let c_str: ColoredString = format!("[{}]:\t{}", tag, msg).white();
    println!("{}", c_str);
}

pub fn error(tag: &str, msg: &str) {
    let c_str: ColoredString = format!("[{}]:\t{}", tag, msg).red();
    println!("{}", c_str);
}