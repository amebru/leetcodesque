fn main() {
    for n in 1..=100 {
        match (n % 3 == 0, n % 5 == 0) {
            (true, true) => println!("CracklePop"),
            (true, false) => println!("Crackle"),
            (false, true) => println!("Pop"),
            (false, false) => println!("{}", n),        
        }
    }
}
