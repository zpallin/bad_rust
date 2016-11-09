
//!
//! Thanks to reddit user [steveklabnik1](https://www.reddit.com/user/steveklabnik1)
//!


/// creates a dangling pointer
fn main() {
    let p;
    {
        let v = vec![1];
        p = &v[0] as *const i32;
    }

    unsafe {
        println!("{}", *p);
    }
}
