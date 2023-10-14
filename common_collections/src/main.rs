fn main() {
    let mut test_vector: Vec<i32> = Vec::new();
    for i in 1..10 {
        test_vector.push(i);
    }
    for num in test_vector {
        print!("{}", num);
    }
}
