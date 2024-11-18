mod utils;
use utils::print;

mod file;

static TAG: &str = "MAIN";

fn count_variables(contents: String) -> i32 {
    let mut count = 0;
    for _line in contents.lines() {
        if _line.contains("let") {
            count += 1;
        }
    }
    count
}

fn count_lines(contents: String) -> i32 {
    
    let mut count = 0;
    for _line in contents.lines() {
        count += 1;
    }
    count
}

fn main() {
    let filename: &str = "src/main.r";

    let file_contents;

    match file::contents(filename) {
        Ok(s) => {
            file_contents = s;
        }
        Err(e) => {
            file_contents = String::new();
            print::error(TAG, &format!("{}", e)[..]);
        }
    }
    
    let count = count_lines(file_contents);
    print::out(TAG, &format!("The file has {} lines.", count)[..]);


}
