pub fn better_fst_(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }
    s
}

/*

&str => refrence or & to str i.e the string data
So this function will return a data refrence to that string.

as_bytes converts string slices to bytes slices for e.g :

let s = String::from("Hello");
=> String_Slices = H e l l o

let bytes = s.bytes();
=> String_Slices ==> 72, 101, 108, 108, 111


Now we have the index and byte tuple and we can write
=> [(0,b'H'), (1, b'e'), (2, b'l'), (3, b'l'), (4,b'o')]
Now this tuple will be containg diffrent bytes as per character inserted by user and it will once be having a space (x , b' ') and if it has then we will directly return that refrence to string data i.e &str.

Now to reach that (x, b' ') we write a iteration to get to x wherevalue will be equal to b' ' & we will return that much data to console.

The &s is just the refrence of s data and [0..i] is just the range from byte 0 to byte till the last iteration possible. Hence in simple words &s[0..i] is just the refrence of the string from bhte 0 to all the way to the last iterated byte.


*/
