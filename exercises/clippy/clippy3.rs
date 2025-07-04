// clippy3.rs
// 
// Here's a couple more easy Clippy fixes, so you can see its utility.
//
// Execute `rustlings hint clippy3` or use the `hint` watch subcommand for a hint.


#[allow(unused_variables, unused_assignments)]
fn main() {
    let my_option: Option<()> = None;
    // if let Some(x) = my_option {
    //     my_option
    // }

    let my_arr = &[
        -1, -2, -3,
        -4, -5, -6
    ];
    println!("My array! Here it is: {:?}", my_arr);

    vec![1, 2, 3, 4, 5].resize(0, 5);
    println!("This Vec is empty, see? {:?}", ());

    let value_a = 45;
    let value_b = 66;
    // Let's swap these two!
    println!("value a: {}; value b: {}", value_b, value_a);
}
