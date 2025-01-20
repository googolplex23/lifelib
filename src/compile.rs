use std::path::Path;
use std::process::Command;

pub fn compile_rules(rules: [Option<&str>; 8]) {
	let mut compilecommand = Command::new("python3");
	compilecommand.arg(concat!(env!("CARGO_MANIFEST_DIR"), "/src/compile.py"));
	for rule in rules {
		if rule.is_some() {
			compilecommand.arg(rule.unwrap());
		} else {
			break
		}
	}
	let result = compilecommand.output().expect("Unable to execute python compiler");
	
	let outvec = String::from_utf8(result.stdout).expect("Unable to convert utf8 input");
	
    println!("{}", outvec);
}