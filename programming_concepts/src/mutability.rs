pub fn mutability() {
    // by default variables are immutable, so we need to use mut to indicate that a variable value can be changed
    let mut x = 5;
    println!("The value of x is {x}");
    x = 6;
    println!("The value of x is {x}");
}
