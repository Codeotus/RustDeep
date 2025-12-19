pub fn multi_borrowing() {
    let mut s1 = String::from("Simple Mutable String");

    /*
        borrow checker works here because those two mutable refrence declaration are not used and ...

        can cause a data race where one will be saying "Hello , World!" & other will be saying "Mellow , World" to the same data on heap.

                 let r2 = &mut s1;
                 let r3 = &mut s1;
                 println!("{} {}", r2, r3);
    */
    // Here I just used curly brace to crate a spearate scope {}
    {
        // seperate scope begins
        let r3 = &mut s1; // borrowing a mut refrence !
        *r3 = String::from("I am r3"); // chnaging it with using pointer to the refrence data.
        println!("{}", r3); // using it

        let r4 = &mut s1;
        // Rust suddenly detected this is a brand new borrowing ...
        // and checks whether r3 was used or not ?
        // If it recieves not/false then borrow checker will produce error or ...
        // if it is true it will allow new mutable refrence.

        *r4 = String::from("I am r4"); // chnaged the data
        println!("{}", r4); // used bt printing it.
        let r1 = &s1; // immutable refrence
        let r2 = &s1; // immutable refrence

        println!("{} , {}", r1, r2);
    } // seperate scope ends.

    // New scope
    // Upside down

    {
        let r1 = &s1;
        let r2 = &s1;
        println!("{} , {}", r1, r2);

        let r3 = &mut s1;
        *r3 = String::from("Downflow of r3");
        println!("{}", r3);

        let r4 = &mut s1;
        *r4 = String::from("Downflow of r4");
        println!("{}", r4);
    }
}
