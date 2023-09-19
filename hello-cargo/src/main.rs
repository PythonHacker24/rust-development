fn num_print() {
    let num: i32 = 44;

    println!("This is being printed from the function");
    println!("{}", num);
}

fn main() {
    let tuple = (12, 24);
   
    num_print();

    println!("\nPrinting the test tuple: ");
    println!("{}", tuple.0);

    println!("Hello, world!");
}
