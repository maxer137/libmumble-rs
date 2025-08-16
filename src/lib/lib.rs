pub mod domain;

pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

#[unsafe(no_mangle)]
pub extern "C" fn say_hello() {
    println!("Hello, Rusty world!");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
