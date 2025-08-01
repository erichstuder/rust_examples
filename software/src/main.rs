//! This is a simple program beginning with the `main` function.
//!
//! To run the program type `cargo run` in the terminal.

mod tasks;
mod enums;

fn main() {
    println!("\n*** rust examples ***\n");

    #[allow(unused)]
    let text: String = String::from("Hello, world!");

    my_print(text);
    // my_print(text);

    // my_print_ref(&text);
    //my_print_ref(&text);

    //my_print_mut_ref(&mut text);
    //my_print_mut_ref(&mut text);

    //tasks::run();

    // let data = [1, 2, 3, 4, 5, 6];
    // // let data: [u8; 10];
    // print_slice(&data);


    // let option = enums::maybe_get_a_value(true);
    // if option.is_some() {
    //     println!("there is some with {}", option.unwrap());
    // }

    // if enums::maybe_get_a_value(false).is_none() {
    //     println!("there is none!")
    // }


    // println!("{:?}", enums::check_value(-5));
    // println!("{:?}", enums::check_value(42));
    // println!("{:?}", enums::check_value(999));
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

#[allow(unused)]
fn print_slice(slice: &[u8]) {
    for a in slice {
        println!("{:?}", a);
    }

    // Die länge von einem slice (array) ist bekannt! Whaaaaaaaat?!?!?
    // for n in 0..slice.len() {
    // for n in 0..100 {
    //     println!("{:?}", slice[n]);
    // }
}

#[allow(unused)]
fn get_number(reference: &mut u8) -> u8 {
    *reference = 33;
    return 42;
}

// #[allow(unused)]
// fn try_get() -> u8 {

// }
