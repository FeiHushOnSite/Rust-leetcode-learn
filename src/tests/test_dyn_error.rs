use std::error::Error;
use std::fmt;
use std::fmt::Formatter;

#[derive(Debug)]
struct MyCustomerError {
    message: String,
}
impl fmt::Display for MyCustomerError {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl Error for MyCustomerError {

}

fn create_dynamic_error() -> Box<dyn Error> {
    let error = MyCustomerError { message: "Something went wrong".to_string() };
    Box::new(error) as Box<dyn Error>
}

#[test]
pub fn test_dyn_error() {
    let dynamic_error: &dyn Error = &*create_dynamic_error();
    println!("{}", dynamic_error)
}
