fn reverse_string(inptstr: &str) -> String {
    
    inptstr.trim().chars().rev().collect::<String>()
}

fn main() {
    
    let inptstr = "Test";
    let result = reverse_string(&inptstr);
    
    println!("Reversed string = {:?}", result);
}