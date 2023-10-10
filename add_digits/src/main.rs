struct Solution; 

impl Solution {
    fn num_length(num_len: &i32) -> i32 {
        let mut counter = 0;
        let mut quotient: i32 = 1;
        let mut divider: i32 = 1;
        while quotient != 0 {
            quotient = num_len / divider;
            divider = divider * 10;
            counter += 1;
        }
        return counter - 1;
    }

    fn split_num(number: &i32, count: &i32) -> Vec<i32> {
        let mut ret_vector: Vec<i32> = Vec::new();
        let mut n = 1;
        let mut counter = *count;
        while counter != 0 {
            let element = ((number % (n * 10)) - (number % n)) / n;
            ret_vector.push(element);
            n = n * 10;
            counter -= 1;
        }
        return ret_vector;
    }
}


fn main() {
    println!("executed!");
    let num = 15548;
    let length = Solution::num_length(&num);
    let num_vector = Solution::split_num(&num, &length);
    
    println!("length: {}", length);
    for element in num_vector {
        println!("number: {}", element);
    }
}

