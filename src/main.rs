use std::io;
use rand::Rng;

fn main(){
    let hangman_list: [&str; 8] = ["Mercure", "Venus", "Terre", "Mars", "Jupiter", "Saturne", "Uranus", "Neptune"];
    println!("Welcome to hangman, The roles are simple, write a letter and if it's correct, \nthe chosen letter is write on the word. Else if you lose a life, 0 is game over");

    loop {
        let rand_number = rand::thread_rng().gen_range(0..8);

        let mut chosen_word = String::from(hangman_list[rand_number]);
        let init_chosen_word = chosen_word.clone();
        let mut masked_word = "_".repeat(chosen_word.len());
        let mut life = 10;

        println!("\nWrite \"play\" to launch!, you have {} life", life);

        let mut launch_input: String = String::new();
        io::stdin().read_line(&mut launch_input).expect("Err");

        if launch_input.trim() == "play" {

            println!("\nThe game begins!\n");
            println!("Your word: {}", masked_word);

            loop {
                let mut found:bool = false;
                let mut word_input:String = String::new();
                io::stdin().read_line(&mut word_input).expect("Err");
        
                if word_input.trim().to_lowercase() == init_chosen_word.to_lowercase() {
                    println!("\nYou won!");
                    break;
                } 

                for (i, c) in chosen_word.to_lowercase().chars().enumerate(){
                    //println!("indice:{} char:{} masked_word:{} chosen_word:{} init_chosen_word: {}", i, c, masked_word, chosen_word, init_chosen_word);

                    if c == word_input.to_lowercase().chars().next().unwrap() {
                        masked_word.replace_range(i..i+1, &c.to_string());
                        chosen_word.replace_range(i..i+1, " ");

                        found = true;
                    }
                }
                if !found {
                    life -= 1;
                    println!("too bad, you lose a life, you have {} life remmaining\n", life);
                }

                if life < 1 {
                    println!("you lose, you maybe win next time!\n");
                    break;
                }

                println!("Your word: {}", masked_word);

                if init_chosen_word.to_lowercase() == masked_word.to_lowercase(){
                    println!("\nYou won!");
                    break;
                }
            }
        } else {
            break;
        }
    }
    print!("good bye !!!"); 
}
