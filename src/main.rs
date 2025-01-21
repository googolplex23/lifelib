
pub mod compile;
pub mod lowlevel; //maybe this doesn't actually need to be public...

fn main() {
    
	let library = compile::compile_rules([Some("b3s"), None, None, None, None, None, None, None]);
	
	let lifetree = lowlevel::create_lifetree(&library, 1000, 17).expect("epic fail!");
	
	lowlevel::delete_lifetree(&library, lifetree, 17);
	}

