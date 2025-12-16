pub fn bytes_printer() {
    // s contains String
    let s = String::from("Hello");

    // created a bytes variable a
    let bytes = s.as_bytes();

    // used :? as u8 do not implements the standard format display trait.
    println!("{:?}", bytes);
}
