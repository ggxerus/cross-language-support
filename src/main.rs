extern "C" {
    fn is_prime(n: u64) -> bool;
}

fn main() {
    println!("Enter a number:");
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    let num: u64 = input.trim().parse().unwrap();

    unsafe {
        if is_prime(num) {
            println!("{} is prime!", num);
        } else {
            println!("{} is NOT prime!", num);
        }
    }
}
