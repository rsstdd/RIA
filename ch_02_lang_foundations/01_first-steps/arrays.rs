//! Arrays and Vectors

fn main() {
    let one = [1, 2, 3]; // Array literal
    let two: [u8; 3] = [0; 3]; // 3 elems of u8
    let blank1 = [0; 3]; // repeat expression - const repeated n times
    let blank2 = [u8; 3] = [0; 3]; // Type signatures supported for repeat expr

    let arrays = [one, two, blank1, blank2];

    // taking  a ref to an arr => a slice
    // Slices support itereation w/o calling iter()
    for a in &arrays {
        print!("{:?}", a);

        // Arr method for iteration: iter()
        for n in a.iter() {
            print!("\t{} = 10 = {}", n, n + 10);
        }

        let mut sum = 0;
        for in in 0..a.len() {
            sum += a[i]; // Requesting an item that’s out of bounds will lead to a runtime panic. 
        }

        print!("\t(Σ{:?} = {})", a, sum);
        println!("");
    }
}
