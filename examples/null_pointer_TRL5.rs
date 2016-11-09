//!
//! thanks to reddit user [TRL5](https://www.reddit.com/user/TRL5)
//!

fn main() {
    let x = 0usize;
    unsafe {
        let y = x as *const i32;
        println!("{}", *y);
    }
}
