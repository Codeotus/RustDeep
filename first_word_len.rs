/*
Returns the length of the first word in a string.

INPUT:
    s : &str
    - A borrowed string slice (does NOT take ownership)
    - Example:
        let s1 = String::from("Rust language");
        print_first_word_len(&s1);

OUTPUT:
    usize
    - Unsigned integer type used for indexes and lengths
*/

/*
IMPORTANT IDEA:
Strings in Rust cannot be indexed directly like arrays.
So we convert the string into bytes and iterate over them.
*/

pub fn print_first_word_len(s: &str) -> usize {
    /*
    s.bytes():
        - Gives ONE BYTE at a time from the string
        - Each byte is of type u8

    Example:
        let x = "Rust".bytes();
        println!("{:?}", x);

    Output:
        Bytes(Copied { it: Iter([82, 117, 115, 116]) })

    Meaning:
        82  -> b'R'
        117 -> b'u'
        115 -> b's'
        116 -> b't'

    So the string "Rust" becomes:
        [b'R', b'u', b's', b't']

    BUT WAIT!
    Every array / list has an index that tells the position
    of each value.

    Example (like Python):
        index : value
        0     : b'R'
        1     : b'u'
        2     : b's'
        3     : b't'

    How do we get this index in Rust?
    → Use .enumerate()

    .enumerate():
        - Adds a counter (index) to each byte
        - Returns a tuple: (index, value)

    Example:
        "Rust".bytes().enumerate()

    Becomes:
        [(0, b'R'), (1, b'u'), (2, b's'), (3, b't')]

    We can think of it as:
        [(index, value)]
        or , more smartly -->
        [(i, item)]
    */
    for (i, item) in s.bytes().enumerate() {
        /*
        i:
            - Index (position)
            - Type: usize

        item:
            - One byte from the string
            - Type: u8

        b' ':
            - Byte representation of a space character
            - b means "byte literal"

        ' '   -> char
        b' '  -> byte (u8)
        */

        /*
        LOGIC:
        If the current byte is a space,
        then the first word has ended.
        */
        if item == b' ' {
            /*
            i is the index of the space.
            That index is exactly the length
            of the first word.
            */
            return i;
        }
    }

    /*
    If the loop finishes and no space is found:
        - The string has only ONE word

    Example:
        "Rust"

    So we return the full length of the string.
    */
    s.len() // no semicolon means it is being returned.
}
