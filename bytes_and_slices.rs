// {:?} and {:#?} are format specifiers for debugging.
// TODO: Use any of the above fmt specifier in println!() to print soemthing that already has a Display trait
//
// :NOTE: Display Trait : This will format text to be displayed in console
// :NOTE: Debug Trait : This will help in debugging code.

pub fn bytes_printer() {
    // s contains String
    let s = String::from("Hello");

    // created a bytes variable a
    let bytes = s.as_bytes();

    // We use {:?} because &[u8] implements Debug, not Display
    println!("{:?}", bytes);
}

pub fn _byte_slices() {
    // a byte slice is a refrence to [u8] hence is &[u8]
    // Is indexible and represents raw bytes.

    // b stands for bytes hence , b"String" means bytes of String

    // Think of b"Byte" as a row of numbered boxes:

    // index:  0    1    2    3
    // bytes: 'B'  'y'  't'  'e'
    let bytes = b"Byte";
    println!("{}", bytes[0]); // print b'B'
}

pub fn _easy_first_word() {
    let s = String::from("é🍎String Hello");
    // We find space index (' ') with .find(x) and
    let space_index = s.find(' ').unwrap();

    // We say first word starts from the origin char ...
    // and ends with our defined space_index
    // println!("{}", first_word_)
    let first_word_ = &s[..space_index];
    println!("{}", first_word_);
}

pub fn _string_slices() {
    // é is 2 bytes and apple is 4 bytes hence both are 6 bytes
    let s = String::from("é🍎String Hello");
    // the range i.e refrence of s from.byte 0 to (..) byte 5 is ...
    // mathematically smaller and this would cause a sudden faliure.
    // WARN: Replace 5 with 6
    // TODO: Replace 5 with 1000 or any big number and note the error.
    let slice = &s[0..5];
    println!("{}", slice);
}
