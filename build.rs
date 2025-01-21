//build.rs
//This should ensure that lifelib is installed on the machine of the user.
//TODO: ensure that the only requirement is python3 being installed. (eg. install pip)
//TODO: install python-lifelib in a virtual environment and not globally.

use std::process::Command;

fn main() {
	Command::new("pip").args(&["install","python-lifelib"]).status().expect("Unable to install python-lifelib"); //attempts to install lifelib via pip
}