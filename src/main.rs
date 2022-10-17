fn test() -> bool {
   true
}

fn main() {
    println!("Hello, world!");

    test();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert!(test());
    }
}
