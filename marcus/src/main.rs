use rand::Rng;
use colored::*;

fn main() {
    let quotes = vec![
        "Waste no more time arguing about what a good man should be. Be one.",
        "You have power over your mind - not outside events. Realize this, and you will find strength.",
        "The best revenge is to be unlike him who performed the injury.",
        "The soul becomes dyed with the color of its thoughts.",
        "Very little is needed to make a happy life; it is all within yourself, in your way of thinking.",
        "The only wealth which you will keep forever is the wealth you have given away.",
        "When you arise in the morning, think of what a precious privilege it is to be alive - to breathe, to think, to enjoy, to love.",
        "The happiness of your life depends upon the quality of your thoughts.",
        "Accept the things to which fate binds you, and love the people with whom fate brings you together, but do so with all your heart.",
        "Our life is what our thoughts make it.",
        "If it is not right, do not do it; if it is not true, do not say it.",
        "The universe is change; our life is what our thoughts make it.",
        "Everything we hear is an opinion, not a fact. Everything we see is a perspective, not the truth.",
        "The object of life is not to be on the side of the majority, but to escape finding oneself in the ranks of the insane.",
        "The best revenge is not to be like your enemy.",
        "The only thing that is constant is change.",
        "How much time he gains who does not look to see what his neighbor says or does or thinks, but only at what he does himself, to make it just and holy.",
        "Loss is nothing else but change, and change is Nature's delight.",
        "Do every act of your life as if it were your last.",
        "The happiness of your life depends upon the quality of your thoughts."
    ];

    let mut rng = rand::thread_rng();
    let quote_num: usize = quotes.len() - 1;
    let random_number = rng.gen_range(1..=quote_num);
    
    let print_string = format!(r#"{} - Marcus Aurelius"#, quotes[random_number]).bold().cyan();

    println!("\n{}\n", print_string);
}
