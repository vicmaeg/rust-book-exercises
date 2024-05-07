pub fn shadowing() {
    // Shadowing you can change the value of an immutable variable by using let again
    let y = 5;

    let y = y + 1;

    {
        let y = y * 2;
        println!("The value of y in the inner scope is: {y}");
    }

    println!("The value of y is: {y}");

    // Shadowing can also be used to change the type of a variable
    let spaces = "    ";
    let spaces = spaces.len();
    println!("The number of spaces are {spaces}");
}
