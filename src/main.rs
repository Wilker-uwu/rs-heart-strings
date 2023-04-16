use std::env::args;
use std::process::exit;
use rand::seq::SliceRandom;
use rand::thread_rng;

fn main() {
    let mut rng = thread_rng();
    let emojis = ["❤️","💘","💝","💖","💗","💓","💞","💕","❣️"];
    
    let lengths: Vec<String> = args().skip(1).collect();
    if lengths.is_empty() {
        eprintln!("Please numbers of hearts you want ❤️");
        exit(exitcode::USAGE);
    };

    for line in lengths {
        let length = line.parse::<i32>().unwrap_or_else(|err| {
            eprintln!("Failed to convert this argument 💔: {}", err);
            exit(exitcode::DATAERR);
        })
            .abs();

        println!("{}", (0..length)
            .map(|_| emojis.choose(&mut rng).unwrap())
            .cloned()
            .collect::<String>())
    }
}
