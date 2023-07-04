extern crate proc_macro;
use proc_macro::TokenStream;

#[proc_macro]
pub fn brainfuck(item: TokenStream) -> TokenStream {
    let mut output = "".to_string();
    let mut buffer_size = 0;

    for t in item {
        let s = format!("{}", t);
        let mut s = s.chars();
        s.next();
        s.next_back();
        let s = s.as_str();
        buffer_size = s.matches(">").count();

        for char in s.chars() {
            match format!("{}", char).chars().nth(0).unwrap() {
                '+' => output.push_str("tape[ptr] += 1;\n"),
                '-' => output.push_str("tape[ptr] -= 1;\n"),
                '>' => output.push_str("ptr += 1;\n"),
                '<' => output.push_str("ptr -= 1;\n"),
                '.' => output.push_str("print!(\"{}\", tape[ptr] as u8 as char);\n"),
                ',' => output.push_str("unimplemented!();\n"),
                '[' => output.push_str("while tape[ptr] != 0 {\n"),
                ']' => output.push_str("}\n"),
                _ => (),
            }
        }
    }

    let output = format!(
        "let mut ptr: usize= 0;
let mut tape: Vec<u8> = vec![0;{}];
{}",
        buffer_size,
        output.as_str()
    )
    .to_string();

    output.parse().unwrap()
}
