fn fib(number: u32, memory: &mut Vec<u32>) -> u32 {
    if number <= 1 {
        return 1;
    } else {
        if memory[number as usize] == 0 { 
            memory[number as usize] = fib(number - 1, memory) + fib(number - 2, memory); 
        }  
    }
    return memory[number as usize]; // This too is getting used in recursion in the if
                                    // memory[number as usize] == 0 part 
}

fn main() {
    let mut memory: Vec<u32> = Vec::new();
    for _ in 1..100 { memory.push(0) }
    // println!("{}", fib(10, &mut memory));
    // println!("{}", fib(25, &mut memory));
    // println!("{}", fib(50, &mut memory));
    // println!("{}", fib(80, &mut memory));
    println!("{}", fib(90, &mut memory));
    for num in memory {
        println!("{}", num);
    }
}
