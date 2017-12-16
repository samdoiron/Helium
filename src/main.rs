#[macro_use]
extern crate pest;
#[macro_use]
extern crate pest_derive;

use pest::Parser;


const _GRAMMAR: &'static str = include_str!("grammar.pest");

#[derive(Parser)]
#[grammar = "grammar.pest"]
struct HeliumParser;

fn main() {
    let source = include_str!("simple.he");
    let pairs = HeliumParser::parse_str(Rule::program, source)
        .unwrap_or_else(|e| panic!("{}", e));

    for program in pairs {
        print!("Parsed: {}", program.clone().into_span().as_str());
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn hello_world() {
        parses_to! {
            parser: HeliumParser,
            input: "print(\"hello world\");",
            rule: Rule::program,
            tokens: [
                program(0, 21, [
                    expression(0, 20, [
                        func_call(0, 20, [
                            identifier(0, 5, []),
                            expression(6, 19, [
                                string(6, 19, [])
                            ])
                        ])
                    ])
                ])
            ]
        }
    }
}
