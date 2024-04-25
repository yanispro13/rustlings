// move_semantics6.rs
//
// You can't change anything except adding or removing references.
//
// Execute `rustlings hint move_semantics6` or use the `hint` watch subcommand
// for a hint.


fn main() {
    let mut data = "Rust is great!".to_string();

    let last_char = get_char(&data);
    println!("Last char: {}", last_char);

    string_uppercase(data);
}

// Should take a reference
fn get_char(data: &String) -> char {
    data.chars().last().unwrap()
}

// Should take ownership
fn string_uppercase(data: String) {
    let uppercase_data = data.to_uppercase();
    println!("{}", uppercase_data);
}
