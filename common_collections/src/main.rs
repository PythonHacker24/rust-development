use std::collections::HashMap;

enum login_info {
    username(String),
    age(i32),
}

fn main() {
    let mut test_vector: Vec<i32> = Vec::new();
    for i in 1..10 {
        test_vector.push(i);
    }
    let captain = test_vector.get(8);
    match captain {
        Some(&captain) => println!("Exists, {}", captain),
        None => println!("Nope, doesn't exist!"),
    }
    
    for i in &mut test_vector {
        println!("{},", *i);
    }

    let mut login_vector = vec![
        login_info::username(String::from("roboto")),
        login_info::age(19),
    ];

    let mut s1 = String::from("hello");
    let s2 = ", world";
    s1.push_str(s2);        // s2 is still valid as the push_str does not take the ownership of s2 
    println!("{}", s1);

    let mut shop = HashMap::new();
    shop.insert(String::from("nuts"), 10);
    shop.insert(String::from("bolts"), 20);
    shop.insert(String::from("wires"), 50);
    shop.insert(String::from("drivers"), 80);
    
    shop.entry(String::from("nuts")).or_insert(20);
    shop.entry(String::from("screws")).or_insert(100);

    println!("{:#?}", shop);
    
    //code to count number of words and occurances of them in a string
    let text = "When a man tries to be a man trying to try the man";
    let mut map = HashMap::new();
    
    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }

    println!("No. of words in the text: {:#?}", map);
}
