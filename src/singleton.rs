use lazy_static::lazy_static;
use std::sync::Mutex;

static mut SINGLETON_1: Vec<usize> = vec![];

lazy_static!{
    static ref SINGLETON_2: Mutex<Vec<usize>> = Mutex::default();
}
//SINGLETON_2 is lazy-initialized, Meaning that
// it will be initialized on first access
// lazy_static gives us a shared reference of the value
// so, to mutate it, we had to use interior-mutability {Mutex}

//Since Rust 1.63, Mutex::new is const
//so, another safe option is to easily use static with Mutex::new
static SINGLETON_3: Mutex<Vec<usize>> = Mutex::new(vec![]);
#[test]
fn execute() {
    // Accessing or mutating static mut variables is unsafe
    // and can cause concurrency issues
    unsafe {
        SINGLETON_1.push(10);
        println!("Singleton1: {:?}", SINGLETON_1);
    }

    {
        let mut singleton2 = SINGLETON_2.lock().unwrap();
        singleton2.push(10);
        println!("Singleton2: {:?}", singleton2);
    }

    {
        let mut singleton3 = SINGLETON_3.lock().unwrap();
        singleton3.push(10);
        println!("Singleton2: {:?}", singleton3);
    }
}
