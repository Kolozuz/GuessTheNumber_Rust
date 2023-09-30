use std::{io::{self}, time::Duration, thread::sleep};
use rand::prelude::*;
use std::cmp;
//use std::collections::HashMap;
use indicatif::{ProgressBar, ProgressStyle};


fn main() {
    test();
}

pub struct User{
    name: String,
    id: u32,
    score: u32
}

fn test () {

    cls();
    println!("  
    +---------------------------------------+
    |   Welcome to Guess the Number Game    |
    +---------------------------------------+  
    ");

    //let mut _enter: String = input(); 

    let input = input(true);

    print!("{:?}",input);

    // while _enter != "" {
    //     println!("Press Enter to continue ");
    //     _enter = input();
    // }

    if input == "" {
        menu();
    }
}

fn menu (){

    println!("
    +---------------+
    |   Options     |
    +---------------+ 
    | 1. Play       |
    | 2. Scoreboard |
    | 0. Exit       |
    +---------------+ 
    ");

    let selection: String = input(false);
    
    
    match selection.as_str(){
        "1" => { println!("Selected option -> {} - Play", selection); game() },
        "2" => { println!("Selected option -> {} - Scoreboards", selection)},
        "0" => { println!("Selected option -> {} - Exit", selection); finish() },
        _ =>   { println!("Invalid Option")}
    }
}

fn game(){
    let user_name: String;
    let mut user_guess = String::new();
    let user_id:u32 = random();
    let mut user_score:u32 = 0;


    println!("To start please input your username:");
    user_name = input(false);

    println!("+-------------------------+");
    println!(" Welcome {}", user_name.trim());
    println!(" Your ID is: {}", user_id);
    println!("+-------------------------+");

    input(true);

    println!("+---------------------------------------------------+");
    println!(" Guess a number between 1 and 10");
    println!(" The closer your guess is, the more points you'll earn");
    println!(" (type help for more info about the scores)");
    println!("+---------------------------------------------------+");

    user_guess = input(false);
    let user_guess_int: i32 = user_guess.trim().parse().unwrap();

    let pb: ProgressBar = ProgressBar::new(100);

    pb.set_message("\nGenerating random number");
    pb.set_style(ProgressStyle::with_template(" {msg} {spinner} \n[{eta}] {bar:40.cyan/blue} {pos:>1}/{len:7}")
        .unwrap()
    );     

    for _ in 0..100 {
        sleep(Duration::from_millis(25));
        pb.inc(1);
    }
    pb.finish();

    let rand_number:i32 = rand::thread_rng().gen_range(1..=10);

    let max_num = cmp::max(user_guess_int, rand_number);
    let min_num = cmp::min(user_guess_int, rand_number);

    if (max_num - min_num) == 0 {
        user_score += 20;
    }else if (max_num - min_num) == 1 {
        user_score += 10;
    }else {
        user_score += 2;
    }

    cls();
    println!("\n> Random number -> {} ", rand_number);
    println!("> Your guess -> {}", user_guess);
    println!("> Your were off by {} numbers", rand_number-user_guess_int);
    println!("> Earned +{} points 🌟 \n", user_score);        


    if user_guess == "help" { score::help() }

    let user:User = User {
        name : user_name,
        id : user_id,
        score : user_score
    };

    let user_info = score::add(user);
    print!("{:?}", user_info);


}

fn input(enter:bool) -> String {
    let mut user_input: String = String::default();
    // println!("default input value -> {:?}", user_input);
    
    if enter == true {
        println!("Press Enter to continue ");
        
        io::stdin().read_line(&mut user_input).expect("Failed to read line");
        user_input = trim_with_carriagereturn(user_input);
        
        // println!("default input value after trimming -> {:?}", user_input);

        while user_input != "" {
            println!("Press Enter to continue ");
            //User Input with \r\n trimming
            io::stdin().read_line(&mut user_input).expect("Failed to read line");
            user_input = trim_with_carriagereturn(user_input);
        }

        return user_input;
    }

    //User Input
    io::stdin().read_line(&mut user_input).expect("Failed to read line");
    user_input = trim_with_carriagereturn(user_input);

    user_input
}

fn cls(){
    print!("\x1B[2J\x1B[1;1H");
}

mod score {
    use std::collections::HashMap;

    use crate::User;
    
    pub fn help(){
        println!("
        +-------------------+
        |   Points Chart    |
        +-------------------+-----------+-----------+
        | Exact same number             |   20pts   |
        | Off by one number             |   10pts   |
        | Off by more tha 1 number      |   2pts    |
        +-------------------------------+-----------+ 
        ");    
    }
    
    pub fn add(entry:User) -> HashMap<u32, [String; 2]>{

        //scores_map = [scores_map, entry].join("~");
        let user_info:[String; 2] = [entry.name, entry.score.to_string()];
        let mut scores_map = HashMap::new();
        scores_map.insert(entry.id, user_info);

        scores_map
    }

    // pub fn show(){
    //     println!("
    //     +-------------------+
    //     |   ScoreBoards     |
    //     +-------------------+
    //     {}
    //     +-------------------+
        
    //     ", scores_map);
    // }

}

// pub trait Test {

//     fn trim_with_carriagereturn(self) -> String {
//         self
//             .replace("\r", "")
//             .trim()
//             .to_string()
//     }
// }

pub fn trim_with_carriagereturn(input:String) -> String {
    input
        .replace("\r", "")
        .trim()
        .to_string()
}

fn finish(){
    let exit_code: i32 = 202;
    std::process::exit(exit_code);
}