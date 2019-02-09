//! Grep Lite

fn main() {
    // PARAMS
    let context_lines = 2;
    let needle = "oo";
    let haystack = "Every face, every shop,
    bedroom window, public-house, and
    dark square is a picture
    fervishly turned--in search of what?
    It is the same with books.
    What do we seek
    through millions of pages?";

    // INIT
    // usize - The size of this primitive is how many bytes it takes
    // to reference any location in memory. For example, on a 32 bit
    // target, this is 4 bytes and on a 64 bit target, this is 8 bytes.
    let mut tags: Vec<usize> = Vec::new(); // <- Holds line nums where matches occur
    // Vec of Vecs - whee T is of type (usize, String) -> a tuple used to store
    // line numbers along with teh text that's near matches
    let mut ctx: Vec<Vec<(usize, String)>> = Vec::new(); // <- a vec per match to hold that match's context lines

    // PASS 1
    for (i, line) in haystack.lines().enumerate() {
        if line.contains(needle) {
            tags.push(i); // line #s where matches occur

            // Reserves space for n items
            let v = Vec::with_capacity(2 * context_lines + 1);
            ctx.push(v);
        }
    }

    if tags.len() == 0 {
        return;
    }

    // Pass 2
    for (i, line) in haystack.lines().enumerate() {
        for (j, tag) in tags.iter().enumerate() {
            // usize.saturating_sub() is subtraction that returns 0 on
            // integer underflow rather than crashing the program
            // (CPUs don’t like attempting to send usize below zero)
            let lower_bound = tag.saturating_sub(context_lines);
            let upper_bound = tag + context_lines;

            if (i >= lower_bound) && (i <= upper_bound) {
                let line_as_string = String::from(line);
                let local_ctx = (i, line_as_string);
                ctx[j].push(local_ctx)
            }
        }
    }

    // OUTPUT
    for local_ctx in ctx.iter() {
        // Borrow this value rather than move it.
        for &(i, ref line) in local_ctx.iter() {
            let line_num = i + 1;
            println!("{}: {}", line_num, line);
        }
    }



    // let mut line_num: usize = 1; // mutable (used in first example)

    // Objects in Rust have methods; strings provide an iterator of lines 
    // for line in quote.lines() {
    //     if line.contains(search_term) {
    //         println!("{}: {}", line_num, line);
    //     }
    //
    //     line_num += 1;
    // }

    // Another approach is using the enumerate() method
     // for (idx, line) in quote.lines().enumerate() {
     //    if line.contains(search_term) {
     //        let line_num = idx + 1;
     //        println!("{}: {}", line_num, line);
     //    }
     // }
}
