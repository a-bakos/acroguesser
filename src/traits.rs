pub trait Log {
    fn print_info(&self) {
        println!("Hello from trait");
    }
}
