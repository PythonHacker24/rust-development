const THREE_HOURS_IN_SECONDS: i32 = 3 * 60 * 60;

fn main() {
    
    let mut x = 10;
    println!("The number first declared is: {}", x);
    
    let y = 10;
    println!("The y when declared was: {y}");

    let y = y * 10;
    println!("The shadowed y is: {y}");
    println!("Three Hours in seconds: {THREE_HOURS_IN_SECONDS}");

    let spaces = "    ";
    let spaces = spaces.len();
    println!("No. of spaces: {spaces}");

    x = 20;
    println!("The updated number is: {}", x);

    println!("----------------- tuple ----------------");

    let tup = (1, 2, 3, 4, 5);
    let (a, b, c, d, e) = tup;

    println!("The value of d is {d}");

}
