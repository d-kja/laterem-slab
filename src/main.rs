mod utils;

use utils::parser;

fn main() {
    let mut parser = parser::parse();

    parser.setup();
}
