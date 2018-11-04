fn main() {
    let mut x = String::from("hello, ");
    let word = &x[0..2];
    x.clear();
    println!("{}", x);
}
