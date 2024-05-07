pub fn constants() {
    // Rust’s naming convention for constants is to use all uppercase with underscores between words.
    // The compiler is able to evaluate a limited set of operations at compile time,
    // which lets us choose to write out this value in a way that’s easier to understand and verify,
    // rather than setting this constant to the value 10,800
    const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;
    println!("Three hours in seconds is {THREE_HOURS_IN_SECONDS}");
}
