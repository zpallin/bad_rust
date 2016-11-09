
//!
//! thanks to reddit user [dbrgn](https://www.reddit.com/user/dbrgn)
//! this causes a stack overflow, but can build on 1.12.1
//!


/// this could be prevented by tail call optimization, but it's not implemented in the compiler atm
fn recurse(i: u32) {
    if i == 1_000_000 {
        println!("Bottom reached");
    } else {
        recurse(i + 1);
    }
}
fn main() {
    recurse(0);
}

