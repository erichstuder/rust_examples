//! This is a simple program beginning with the `main` function.
//!
//! To run the program type `cargo run` in the terminal.

mod tasks;

fn main() {
    println!("\n***rust examples***\n");

    let text: String = String::from("Hello, world!");
    my_print(text);
    //my_print(text);

    //my_print_ref(&text);
    //my_print_ref(&text);

    //my_print_mut_ref(&mut text);
    //my_print_mut_ref(&mut text);

    //tasks::run();
}

// Note: No forward declaration is needed like for example in C.
// #[allow(unused)] suppresses the warning for unused function.
#[allow(unused)]
fn my_print(text: String) {
    println!("{}", text);
}

#[allow(unused)]
fn my_print_ref(text: &String) {
    //text.push_str("\n");
    println!("{}", text);
}

#[allow(unused)]
fn my_print_mut_ref(text: &mut String) {
    text.push_str("_123");
    println!("{}", text);
}
