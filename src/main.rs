
pub mod compile;
mod lowlevel; //maybe this doesn't actually need to be public...

fn main() {
    
	let library = compile::compile_rules([Some("b3s"), None, None, None, None, None, None, None]);
	unsafe {
		let lifetree = lowlevel::create_lifetree(&library, 1000, 17).expect("epic fail!");
		
		let mut andtest1 = lowlevel::create_pattern_from_file_contents(&library, lifetree, "x = 10, y = 5, rule = B3/S \n10o$10o$10o$10o$10o!").unwrap();
		let mut andtest2 = lowlevel::create_pattern_from_file_contents(&library, lifetree, "x = 10, y = 5, rule = B3/S \n5o$5o$5o$5o$5o$5o$5o$5o$5o$5o!").unwrap();
		
		let mut andtestcombined = lowlevel::boolean_pattern_immutable(&library, andtest1, andtest2, 0).unwrap();
		lowlevel::save_pattern_rle(&library, andtestcombined, "andtestresult.rle", "", "");
	}
}

