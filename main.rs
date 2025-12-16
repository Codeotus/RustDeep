mod better_first_word;
mod bytes;
mod channel_func_for_refrence;
mod first_word;
mod fn_without_return;
mod multi_borrowing;

fn main() {
    // Using function as a channel for refrencing.
    let mut s1 = String::from("Rust");
    println!("{}", channel_func_for_refrence::safe_returning(&mut s1));

    // Prints first word.
    let s1 = String::from("Rust is Good");
    println!("{}", better_first_word::better_fst_(&s1));

    // prints first word length
    let s1 = String::from("Rust");
    println!(
        "The length of first word is : {}",
        first_word::print_first_word_len(&s1)
    );

    // Multiple borrowing
    multi_borrowing::multi_borrowing();

    // Function without return.
    println!("{}", fn_without_return::refee());

    // display bytes
    println!(" The bytes of Hello are : {:?}", bytes::bytes_printer());

    let s = String::from("é🍎String Hello");
    let space_index = s.find(' ').unwrap();
    let first_word_ = &s[..space_index];
    println!("{}", first_word_);

    let slice = &s[0..];
    println!("{}", slice);

    let x = 10;
    let y = x;
    println!("{}", y);
    println!("{}", x);
    let _x = 99; // unused variable warning
    let x = vec![1, 2, 3];
    println!("{:?}", x); // rust forces to use {:?} to display vec! 
}
