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

    for i in login_vector {
        match i {
            age => println!("i32"),
        }
    }

    let mut s1 = String::from("hello");
    let s2 = ", world";
    s1.push_str(s2);        // s2 is still valid as the push_str does not take the ownership of s2 
    println!("{}", s1);
}
