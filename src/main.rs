fn main() {
    #[cfg(feature = "auth")]
    println!("Hello, world!");
    #[cfg(not(feature = "auth"))]
    println!("Hey, world.");
}
