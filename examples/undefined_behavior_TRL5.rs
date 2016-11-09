//!
//! thanks to reddit user [TRL5](https://www.reddit.com/user/TRL5)
//!

fn main() {
    let x = 3;
    let y = &x;
    unsafe {
        let z: &mut i32 = ::std::mem::transmute(y as *const i32);
        *z = 4;
    }
    println!("{}", x)
}
