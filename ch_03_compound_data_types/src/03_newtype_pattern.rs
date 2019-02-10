#[derive(PartialEq)] //  PartialEq enables types to be compared for equality.
struct Hostname(String); // Newtype

fn main() {
    let ordinary_string = String::from("localhost");
    let host = Hostname ( ordinary_string.clone() );

    if host == ordinary_string { // Will not compile b/c Hostname and String are distinct
        println!("huh?");
    };
}
