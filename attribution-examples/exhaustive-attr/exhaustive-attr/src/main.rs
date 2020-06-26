use exhaustive_attr_macros::exhaustive;

#[exhaustive(flag = true, simple_flag, string = "foo", integer = 1, float = 4.0, array_of_integers = [1, 2, 3])]
fn main() {
    println!("End of main!");
}
