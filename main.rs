mod better_first_word;
mod channel_func_for_refrence;
mod first_word;
mod fn_without_return;
mod multi_borrowing;
fn main() {
    let mut s1 = String::from("Rust");
    let s2 = String::from("Rust is Good");
    println!("{}", channel_func_for_refrence::safe_returning(&mut s1));
    multi_borrowing::multi_borrowing();
    println!("{}", fn_without_return::refee());
    println!(
        "The length of first word is : {}",
        first_word::print_first_word_len(&s1)
    );

    // let s = String::from("Good Life");

    let s = String::from("é🍎String Hello");
    let space_index = s.find(' ').unwrap();
    let first_word_ = &s[..space_index];
    println!("{}", first_word_);

    let slice = &s[0..];
    println!("{}", slice);

    println!("{}", better_first_word::better_fst_(&s2));

    let s3 = String::from("Hello");
    let bytes = s3.bytes();
    println!("{:?}", bytes);
}
