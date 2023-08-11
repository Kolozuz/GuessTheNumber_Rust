
use std::{io::{self}, time::Duration, thread::sleep, cmp::Ordering};
use rand::prelude::*;
//use std::collections::HashMap;
use indicatif::{ProgressBar, ProgressStyle};


fn main() {
    test();
}

fn test () {

    println!("  
    +---------------------------------------+
    |   Welcome to Guess the Number Game    |
    +---------------------------------------+  
    Press Enter to continue 
    ");

    let mut _enter: String = input(); 

    while _enter != "" {
        println!("Press Enter to continue ");
        _enter = input();
    }

    if _enter == "" {
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

    let selection: String = input();
    
    
    match selection.as_str(){
        "1" => { println!("Selected option -> {} - Play", selection); game() },
        "2" => { println!("Selected option -> {} - Scoreboards", selection); score::show() },
        "0" => { println!("Selected option -> {} - Exit", selection); finish() },
        _ =>   { println!("Invalid Option")}
    }
}

fn game(){
    let mut user_name: String = String::from("");
    let mut user_guess: String = String::from("");
    let user_id:u32 = random();

    println!("To start please input your username:");
    io::stdin().read_line(&mut user_name).expect("Failed to read line");

    println!("+-------------------------+");
    println!(" Welcome {}", user_name.trim());
    println!(" Your ID is: {}", user_id);
    println!("+-------------------------+");

    input();

    println!("+---------------------------------------------------+");
    println!(" Guess a number between 1 and 10");
    println!(" The closer your guess is, the more points you'll earn");
    println!(" (type help for more info about the scores)");
    println!("+---------------------------------------------------+");

    user_guess = input();
    let user_guess_int = user_guess.parse::<i32>().unwrap();

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

    let rand_number:i32 = rand::thread_rng().gen_range(0..=10);

    if (rand_number - user_guess_int) == 1 {
        println!("|    Your guess was -> {}    |   The random number was -> {} |", user_guess, rand_number)
    }else {
        
    }
    if user_guess_int < rand_number && (rand_number - user_guess_int) == 1 {println!("|    Your guess was -> {}    |   The random number was -> {} |", user_guess, rand_number)}
    if user_guess == "help" { score::help() }



}

fn input() -> String {
    let mut user_input: String = String::new();

    io::stdin().read_line(&mut user_input).expect("Failed to read line");

    //println!("the input was -> {:?}", user_input);

    user_input.replace("\r", "").trim().to_string()
}

mod score {
    use std::collections::HashMap;
    
    pub fn help(){
        println!("
        +-------------------+
        |   Points Chart    |
        +-------------------+-----------+-----------+
        | Exact same number             |   20pts   |
        | Off by one number             |   10pts   |
        | Off by 2 -5 numbers           |   8pts    |
        | Off by more than 5 numbers    |   0pts    |
        +-------------------------------+-----------+ 
        ");    
    }
    
    pub fn add(key:i32, value:&str){
        let mut SCORES_MAP: HashMap<i32, &str> = HashMap::new();
        unsafe { SCORES_MAP.insert(key, value) };
        
    }

    pub fn show() {
        println!("
        +-------------------+
        |   ScoreBoards     |
        +-------------------+
        
        +-------------------+
        
        ", );
    }
}

fn finish(){
    let exit_code: i32 = 202;
    std::process::exit(exit_code);
}