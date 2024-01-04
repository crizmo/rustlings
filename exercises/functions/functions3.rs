// functions3.rs
//
// Execute `rustlings hint functions3` or use the `hint` watch subcommand for a
// hint.

// I AM DONE

fn main() {
    call_me(5);
}

fn call_me(num: u32) { // u32 is an unsigned 32-bit integer
    for i in 0..num {
        println!("Ring! Call number {}", i + 1);
    }
}
