pub mod common {
    pub fn greet(name: String, lang: String) {
        match lang.as_str() {
            "Es" => println!("Hola Sinore {name}!"),
            "En" => println!("Hello There! {name}"),
            _ => println!("What?"),
        }
    }

    pub fn read_user_input(user_input: &mut String) {
        loop {
            std::io::stdin()
                .read_line(user_input)
                .expect("Failed to read line");

            if user_input.trim().len() == 0 {
                println!("You didnt enter anything ");
                continue;
            } else {
                break;
            }
        }
    }
}
