use std::process::Command;
use libloading::Library;

pub fn compile_rules(rules: [Option<&str>; 8]) -> Library { //this will compile and link lifelib for the specified rules.
	let mut compilecommand = Command::new("python3"); //call compile.py, which is a very minimal wrapper which compiles and sanitizes rules.
	compilecommand.arg(concat!(env!("CARGO_MANIFEST_DIR"), "/src/compile.py")); //looks in /src for compile.py. This might need to be moved when this is used as a package.
	for rule in rules {
		if rule.is_some() {
			compilecommand.arg(rule.unwrap()); //add rules to args
		} else {
			break // this means that the rules list must start with Some(rule) or all rules will not be loaded.
		}
	}
	println!("Starting lifelib compilation...");
	
	let result = compilecommand.output().expect("Unable to execute python compiler");  //actually execute the command.
	
	println!("Compilation complete!");
	
	let outvec = String::from_utf8(result.stdout).expect("Unable to convert utf8 input"); //convert stdout to a string
	let soname = outvec.lines().last().unwrap();
	
    println!("Compiled lifelib library located at {}", soname);
	
	let library = unsafe {
		Library::new(soname).expect("Unable to link lifelib library") //link the library using libloading. TODO: maybe move this to a different function? also potentially use stabby for safer code.
	};
	
	println!("Succesfully linked library!");
	
	return library
	
}