/*

We can't return refrence to the value that is inside a function.

Reason:
The value of s1 will be dropped as it finishes and after that it is impossible in rust to return any refrence to it.

pub fn _dangling_returning() -> &mut String {
    let mut s1 = String::from("Retuening s1");
    let s2 = &mut s1;
    s2
}

*/

// The what we can do ?
// Do this as Rust functions can act like channels for extrenal refrences:

pub fn safe_returning(s1: &mut String) -> &mut String {
    *s1 = String::from("Rust From Refrence");
    s1
}
