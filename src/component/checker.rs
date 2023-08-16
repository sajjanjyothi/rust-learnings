pub trait Checker{
    fn check_these(&self, name: String);
    fn get_name(&self) -> &String;
}

#[derive(Debug)]
pub struct Sajjan{
    pub name: String,
    pub age: i32
}

impl Sajjan {}

pub fn new(name: String) -> impl Checker{
    Sajjan{
        name: name,
        age: 0,
    }
}

impl Checker for Sajjan {
    fn check_these(&self, name: String) {
        eprintln!("{}",name);
    }

    fn get_name(&self) -> &String {
        let string = &self.name;
        return string
    }
}