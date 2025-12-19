use crate::chapter5::r#struct::Userdata;

mod borrow_checker;
mod bytes_and_slices;
mod chapter5;
mod first_word;
mod first_word_len;
mod nil_return;
mod returning;

// This main will explain you shadowing.
fn main() {
    // This will teach you how to use functions as a channel for refrencing.
    let mut s1 = String::from("Rust");
    println!("{}", returning::channel_returning(&mut s1));

    // This will print the first word.
    let s1 = String::from("Rust is Good");
    println!("{}", first_word::better_fst_(&s1));

    // This will print the length of 1st word.
    let s1 = String::from("Rust");
    println!(
        "The length of first word is : {}",
        first_word_len::print_first_word_len(&s1)
    );

    // This will expalin you borrow checker & multiple borrowing.
    borrow_checker::multi_borrowing();

    // This function do not returns anything.
    println!("{}", nil_return::refee());

    // This prints string bytes
    bytes_and_slices::bytes_printer();

    // This is assigning a structure to user1
    let mut user1 = Userdata {
        username: String::from("Vishnu_Pulkit"),
        email: String::from("Vishnu_Pulkit@gmail.com"),
        sign_in_count: 1,
        active: true,
    };

    println!("{:#?}", user1);
    println!("{:#?}", user1.username);
    println!("{:#?}", user1.email);
    println!("{:#?}", user1.sign_in_count);
    println!("{:#?}", user1.active);

    user1.email = String::from("me@gmail.com");

    let user2 = Userdata {
        email: String::from("Hello"),
        username: String::from("Hello"),
        ..user1
    };

    // We can even define structures as :
    struct Color(i32, i32, i32);
    struct Origin(i32, i32, i32);

    let black = Color(0, 0, 0);
    println!(" The first index of black : {}", black.0)

    // Despite Color and Origin are equal but we can not compare them as they are two different
    // things instead of being the same.
    // Think of it like two humans with different faces but same height & weight.

    //     if Color == Origin {
    //         println!("True");
    //     }
}

fn _build_user(email: String, username: String) -> Userdata {
    Userdata {
        email,
        username,
        active: true,
        sign_in_count: 1,
    }
}
