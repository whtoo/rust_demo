use std::process::Command;

fn main() {
    let mut cmd = Command::new("python");
    cmd.arg("hello.py");


    match cmd.output() {
        Ok(out) => {
            unsafe {
                let out_buffer = String::from_utf8_unchecked(out.stdout);
                println!("Output: {}",out_buffer);
            }
        },
        Err(e) => {
            println!("There was an error {} ",e);
        }
    }
}

#[cfg(test)]
mod unit_tests {
    #[test]
    fn test_basic() {
        assert!(1 == 1);
        panic!("Oh no!");
    }
}