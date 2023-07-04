use brainfuck::brainfuck;

// This example compiles to the following rust instructions:
//
// let mut ptr = 0;
// let mut tape: Vec<u8> = vec![0;1];
// tape[ptr] += 8;
// print!("{}", tape[ptr] as u8 as char);
fn main() {
    brainfuck!("++++++++++++++++++++++++++++++++++++++++++++++++.");
}
