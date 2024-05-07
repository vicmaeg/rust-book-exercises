mod constants;
mod mutability;
mod shadowing;

fn main() {
    mutability::mutability();
    constants::constants();
    shadowing::shadowing();
}
