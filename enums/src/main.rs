#[derive(Debug)]
struct Hero {
    value_1: i32,
    value_2: i32,
}

impl Hero {
    fn add(&self) -> i32 {
        return self.value_1 + self.value_2; 
    }
}

enum Place {
    Earth,
    Asguard,
}

fn main() {
    let nums = Hero {
        value_1: 22,
        value_2: 22,
    };
    
    let sum = nums.add();
    println!("sum: {}", sum);
    
    let cap = Place::Earth;
    let thor = Place::Asguard;

    match thor {
        Place::Earth => println!("It's from the home!"),
        Place::Asguard => println!("Is it a god?"),
    }

    match cap {
        Place::Earth => println!("Hey Captain Rogers!"),
        Place::Asguard => println!("Not from the Asguard!"),
    }

    if let Place::Earth = thor{
        println!("Earth?");
    }
    if let Place::Asguard = thor{
        println!("Asguard!");
    }
}
