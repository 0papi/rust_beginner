use rand::Rng;
use std::io;

fn main() {
    println!("Hello there, let's play a little rock paper scissors");

    let mut plays = 0;
    let mut player_wins = 0;
    let mut computer_wins = 0;


    loop {
        if plays == 3 {
            println!("GAME OVER");
            println!("SCORELINE:\n Computer {computer_wins} - Player {player_wins}");
            break;
        }

        const COMPUTER_CHOICES: [&str; 3] = ["rock", "paper", "scissors"];
        let choice_length = COMPUTER_CHOICES.len();
        let random_number = rand::thread_rng().gen_range(0..choice_length);
        let computer_choice = COMPUTER_CHOICES[random_number];

        println!("Please select your choice from rock, paper and scissors");
        let mut player_choice = String::new();

        io::stdin().read_line(&mut player_choice).expect("an error occurred");

        println!("Computer choice: {}", computer_choice);
        println!("Your choice: {}", player_choice);

        if computer_choice.trim() == player_choice.trim() {
            player_wins += 1;
            println!("YOU WIN!");
        } else {
            computer_wins += 1;
            println!("YOU LOSE! BETTER LUCK NEXT ROUND")
        }

        plays += 1;
    }


}
