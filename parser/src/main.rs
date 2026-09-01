use parser::block_parser;

fn main() {
    print!("{}", block_parser("```\nlet x = 1;\n```"));
}
