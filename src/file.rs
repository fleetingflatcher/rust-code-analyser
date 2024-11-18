use crate::print;

use std::io::Error;

static TAG: &str = "FILE";

pub fn contents(filepath: &str) -> Result<String, Error> {
    let ret_val = std::fs::read_to_string(filepath);
    match ret_val {
        Ok(_) => {
            print::debug(TAG, "File contents read.");
        }
        Err(_) => {
            print::error(TAG, "Error reading from provided filepath.");
        }
    }
    ret_val
}
