use chumsky::Parser;

fn main() {
    let text = "((\\x.\\y.x, a), b)";
    println!("text to read: {}", text);

    let parser = lambda::parser::parser();
    let ast = parser.parse(&text).unwrap();
    println!("{:?}", ast)
}
