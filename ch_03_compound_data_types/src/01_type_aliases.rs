#![allow(unused_variables)]
type File = String; // Creates a type alias

fn open(f: &mut File) -> bool {
    true
}

fn close(f: &mut File) -> bool {
    true
}

#[allow(dead_code)]
fn read(f: &mut File, save_to: &mut Vec<u8>) -> ! { // ! indicates that this fn never returns
    unimplemented!() // Crashes the program if encountered
}

fn main() {
     // b/c of type declaration on ln 2, File inherits all String methods
    let mut f1 = File::from("f1.txt");
    open(&mut f1);
    // read(f1, vec![]); // No point at the moment
    close(&mut f1);
}
