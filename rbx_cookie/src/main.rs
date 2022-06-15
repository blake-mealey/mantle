fn main() {
    match rbx_cookie::get() {
        Ok(cookie) => println!("{}", cookie),
        Err(e) => println!("{}", e),
    };
}
