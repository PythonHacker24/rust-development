fn main() {
    let mut vector: Vec<i32> = Vec::new();
    vector.push(1);
    vector.push(2);
    vector.push(3);
    vector.push(4);
    vector.push(5);
    vector.push(6);
    println!("{:?}", vector);
    vector.insert(2, 10);
    println!("{:?}", vector);
}
