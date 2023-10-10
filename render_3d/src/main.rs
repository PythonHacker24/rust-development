fn display(space_vec: Vec<Vec<char>>) {
    for vector in space_vec {
        for pixel in vector {
            print!("{}", pixel);
        }
        print!("\n");
    }
}

fn main() {

    let mut space_vec: Vec<Vec<char>> = Vec::new();
    let width: i32 = 50;
    let height: i32 = 20;
    
    let mut horizontal_vec: Vec<char> = Vec::new();

    for _ in 1..=width {
        let void_space = ' ';
        horizontal_vec.push(void_space);
    }

    for _ in 1..=height {
        space_vec.push(horizontal_vec.clone());
    }

    display(space_vec);
}
