/// Ownership Exercise

fn main() {
    let s = String::from("book");
    let plural_s = pluralize(s.clone());
    println!("I have one {}, you have two {:?}", s, plural_s);
}

fn pluralize(z: String) -> String {
    z + "s"
}
