# Ownership 

Ownership in rust is similar to someone owning a particular land part and by law it is the only legal owner to that land. Think of it like you are owning a property and there is no 2nd owner but yes there are your heirs(the one who will recieve the property after you).

Well! If you have a property you probably be wanting to rent it but :

- Would you want tenants(People in rented property) to live without paying the rents ?
- Would you wnat them to be the owner wihout purchasing the property ?

Probably not !

That is what ownership in rust is. 

Example :

```Rust
fn main(){
    let s1 = String::from("Hello"); // s1 is immutable by default 
    let go = &s1; // refrence of s1 is held by go 
    let py = &s1; // refrence of s2 is held by py 
    let cpp = &s1;
    let julia = &s1;

    println!{"The value of s1: {}, Other borrowed values :  {}, {}, {}, {}", go, py, cpp, julia};
}
```

But this e.g is of immutable Refrence and if we try to mutate the value of s1 via the below methods ... : 

```Rust
// Note : Don't try to copy paste pick a method and use yourself.
s1.push('_'); // pushes a single character
s1.push_str("world!"); // pushes a string 

s1.insert(0, 'H'); // inserts a character at a specific byte 
s2.insert_str(0, "Hello"); // inserts a string at byte 0 

s1.pop(); // remove last character 
s1.remove(x) // remove the character at given index x.

s1.truncate(x); // Shorten the string to length x in bytes.

s1.clear(); // remove everything.
```

... we will directly get :
```Error
1. cannot mutate immutable variable `s2` [E0384]
2. cannot borrow `s2` as mutable, as it is not declared as mutable cannot borrow as mutable [E0596]
```
Remember we had s1 and its data type is String as it is itself owning the string. What if we do take a refrence from it but making it mutable ?

```Rust 
fn main(){
    let mut s1 = String::form("Some String");
    // mutable borrowing
    let r1 = &mut s1;
    // with pointer (*) variable r1 we change the data :
    *r1 = String::from("I am r1");
    // We use it to get rid of borrow checker :
    println!("{}", r1); // using it 

    // New refrence 
    // Rust suddenly detected this is a brand new borrowing and checks whether r1 was used or not ? 
    // If it recieves not/false then borrow checker will produce error or ...
    // if it is true it will think as a totally new refrence..
    let r2 = &mut s1; 
    *r2 = String::from("I am r2"); 
    println!("{}", r2);
}
```

Hence, borrowing in Rust is not much then taking a refrence of the data on the heap.
Is borrowing = copying or cloning ?
Answer : Obviously not , Borrowing means borrowing it and it comes under a single ownership while copying or cloning produces multiple data , and each data will have an owner so it is like having a multiple pdfs of a paid single writer book rather than being an owner.




