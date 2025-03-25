use std::io;
fn main() {
    let mut text = Box::new(String::from("Message initial: "));
    println!("Entrez un texte : ");
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Erreur de lecture");
    text.push_str(&input);
    println!("Texte final : {}", text);
}

