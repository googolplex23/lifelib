
pub mod compile;
mod lowlevel; //maybe this doesn't actually need to be public...

fn main() {
    
	let library = compile::compile_rules([Some("b3s23"), None, None, None, None, None, None, None]);
	
	
	unsafe {
		let lifetree = lowlevel::create_lifetree(&library, 10000, 17).expect("epic fail!");
		
		
		//let mut andtest1 = lowlevel::create_pattern_from_file_contents(&library, lifetree, "x = 10, y = 5, rule = B3/S \n10o$10o$10o$10o$10o!").unwrap();
		let mut andtest2 = lowlevel::create_pattern_from_rle(&library, lifetree, "5o$5o$5o$5o$5o$5o$5o$5o$5o$5o!","b3s23").unwrap();
		
		//let mut hashsouptest = lowlevel::hashsoup(&library, lifetree, "b3s23", "C1", "1020304325").unwrap();
		let mut testadvance = lowlevel::advance_pattern(&library,andtest2, 10,0).unwrap();
		lowlevel::save_pattern_rle(&library, testadvance, "hashsouptest.rle", "", "");
		
		
	}
}

