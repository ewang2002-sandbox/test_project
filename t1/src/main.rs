fn main() {
    #[cfg(feature = "auth")]
    {
        println!("Hello, world! this is test 1");
    }
    #[cfg(not(feature = "auth"))]
    {
        use webweg::wrapper::WebRegWrapper;
        let _wrapper = WebRegWrapper::builder();
    }
}
