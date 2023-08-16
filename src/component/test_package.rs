#[derive(Debug)]
#[allow(unused)]
pub enum HomeError {
    Good,
    Bad(i32,bool),
}

/// say hello is a simple function
pub fn say_hello() -> String {
    String::from("sajjan")
}

pub fn check_value() -> Result<bool, HomeError> {
    // Err(HomeError::Good)
    Err(HomeError::Bad(35,false))
    //Ok((true))
}

pub fn check_borrow(val: &String) {
    println!("{}", val);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_say_hello() {
        assert_eq!(say_hello(), "sajjan")
    }
}
