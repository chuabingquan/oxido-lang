fn main() {
    let reference_to_nothing = dangle();
}

fn dangle() -> &String {
    let s = StringFrom("hello");
    &s
}