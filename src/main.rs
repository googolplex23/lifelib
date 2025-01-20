
pub mod compile;

fn main() {
    compile::compile_rules([Some("b2s"), None, None, None, None, None, None, None])
}

