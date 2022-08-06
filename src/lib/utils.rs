use std::io::stdin;




pub fn get_user_input() -> Option<String> {
    let mut buffer = String::from("");
    while stdin().read_line(&mut buffer).is_err() {
        println!("wrong input! Please insert again:");
    }

    let input = buffer.trim().to_lowercase().to_owned();
    if input == "" {
        None
    } else {
        Some(input)
    }
}