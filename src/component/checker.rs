pub trait Checker{
    fn check_these(&self, name: String);
    fn get_name(&self) -> &String;
}

#[derive(Debug)]
pub struct Sajjan{
    pub name: String,
    pub age: i32
}

#[macro_export]
macro_rules! sajjan_macro {
    ($expression:expr) => { println!("hello here --- {}",$expression)};
}

impl Sajjan {}

pub fn new(name: String) -> impl Checker{
    sajjan_macro!("test--hello");
    Sajjan{
        name: name,
        age: 0,
    }
}

impl Drop for Sajjan {
    fn drop(&mut self) {
        println!("sajjan dropping-- hooo hooo");
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