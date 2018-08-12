
#[no_mangle]
pub extern fn rust_hello_world() -> i32 {
    println!("Hello, I'm in Rust code! I'm about to return 10.");
    10
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
