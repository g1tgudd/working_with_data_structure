fn find_prime(input: &i32) -> bool {

    if *input <= 1 {
        return false;
    }
    for n in 2..*input {
        if *input % n == 0 {
            return false;
        }
    }
    true
}

fn convert_bool(input: &[i32]) -> Vec<bool> {

    return input.iter().map(find_prime).collect(); 
}

fn main() {

    let input: Vec<i32> = vec![1, 2, 3, 4, 5];
    let result = convert_bool(&input);

    for (bool, value) in input.iter().enumerate() {
        println!("{} = {:?}", value, result.get(bool).unwrap());
    }
}