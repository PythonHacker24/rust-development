fn main() {
    let mut s = String::from("Hello");
    string_mod(&mut s);     // calling the string modifier function with mutable reference  
    
    let s_ref = &s;
    print_string(s_ref);       // printing the modified string
    
    {
        let mut mod_str = s.clone();
        mod_str.push_str(", mod it is!");
        print_string(&mod_str);
    }

    print_string(&mod_str);

}

fn string_mod(some_string: &mut String) {       // Takes a mutable reference
    some_string.push_str(", World!");
}

fn print_string(print_string: &String) {        // Takes a immutable reference
    println!("{}", print_string);
}
