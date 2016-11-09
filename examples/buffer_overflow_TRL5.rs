//!
//! thanks to reddit user [TRL5](https://www.reddit.com/user/TRL5)
//!

fn main() {
    let x = 3;
    let y = &x;
    unsafe {
        let z = y as *const i32;
        println!("{}", *z.offset(1)); 
    } 
}
