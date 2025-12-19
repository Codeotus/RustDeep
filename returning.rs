/*

We can't return refrence to the value that is inside a function.

Reason:
The value of s1 will be dropped as it finishes and after that it is impossible in rust to return any refrence to it.

pub fn _dangling_returning() -> &mut String {
    let mut s1 = String::from("Returning s1");
    let s2 = &mut s1;
    s2
}

*/

// The what we can do ?
// Do this as Rust functions can act like channels for extrenal refrences:

pub fn channel_returning(s1: &mut String) -> &mut String {
    // since it is a &mut String ..
    // we will need pointer to variable to modify it.
    *s1 = String::from("Rust From Refrence");
    s1 // returning s1
}

/*

Go to main and do :

println!(
    " The bytes of Hello are : {:?}",
    bytes_slices::bytes_printer()
);


// This will print What is inside println strings with () at the end because it returns nothing.


// To make it be used make sure that it returns what you want or ...
// Do call the function.




*/
