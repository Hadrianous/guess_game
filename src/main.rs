use std::cmp::Ordering;
use std::io;
use rand::Rng;

fn main() {
    println!("Guess the number!");

    let secret_number: u32 = rand::thread_rng().gen_range(1..=21);

    let mut card1 = gen_card();
    let mut card2 = gen_card();

    println!("You cards are {card1} and {card2}");
    println!("Wanna change ? Type 1 for card 1, 2 for card 2, both for changing both, none to change none");

    let mut change: String = String::new();

    io::stdin()
        .read_line(&mut change)
        .expect("Failed to read line");

    match change.trim() {
        "1" => card1 = gen_card(),
        "2" => card2 = gen_card(),
        "both" => {
            card1 = gen_card();
            card2 = gen_card()
        },
        _ => (),
    };

    let sum: u32 = card1 + card2;
    println!("You cards are {card1} and {card2} : {sum}");

    let mut result_message: String = "secret is ".to_owned();
    result_message += secret_number.to_string().as_str();
    match (sum).cmp(&secret_number) {
        Ordering::Less => println!("Too small, {result_message}"),
        Ordering::Greater => println!("You win, {result_message}"),
        Ordering::Equal => println!("You lose, {result_message}"),
    }
}

fn gen_card() -> u32 {
    return rand::thread_rng().gen_range(1..=10);
}
