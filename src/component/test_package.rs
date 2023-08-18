#[derive(Debug)]
#[allow(unused)]
pub enum CheckError {
    Good,
    Bad(i32,bool),
}

/// say hello is a simple function
pub fn say_hello() -> String {
    "sajjan".to_string()
}

pub fn check_value() -> Result<bool, CheckError> {
    // Err(HomeError::Good)
    Err(CheckError::Bad(35, false))
    //Ok((true))
}

pub fn check_borrow(val: &String) {
    println!("{}", val);
}

//functional test
#[test]
fn test_say_hello() {
    assert_eq!(say_hello(), "sajjan")
}

//modular tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_say_hello() {
        assert_eq!(say_hello(), "sajjan")
    }
}
