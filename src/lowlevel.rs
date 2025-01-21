/*lowlevel.rs, low level c++ functions ported into rust with libloading for dynamic loading features.

These features are less safe than the python equivalents so these may or may not be public.

All of these functions are unsafe because there's nothing stopping you from putting in incorrect c_voids. 
The higher level functions should prevent this pretty handily but using these functions on their own is not advised.

Other reasons that these are unsafe and can cause memory leaks is that it us up to 
*/

use libloading::Library;
use libloading::Symbol;
use core::ffi::c_int;
use core::ffi::c_void;
use std::ffi::CString;
use std::os::raw::c_char;

fn to_const_char(input: &str) -> CString { return CString::new(input).expect("Unable to perform CString::new");}

pub unsafe fn create_lifetree(lib: &Library, maxmem: u16, nlayers: i8) -> Result<*mut c_void, &'static str> {// maybe change maxmem and nlayer format in the future
	if nlayers < -1 {return Err("nlayers should be more than -2")};
	if maxmem < 1000 {println!("Warning! maxmem for a lifetree should be more than 1000!")};
	unsafe {
		let unsafe_create_lifetree: Symbol<unsafe extern fn(c_int,c_int) -> *mut c_void> = 
			lib.get(b"CreateLifetree\0").unwrap();
		let result = unsafe_create_lifetree(maxmem.into(),nlayers.into());
		return Ok(result) //I'm very sure that this is like SUPER unsafe as there's no checks other than at the start to make sure that the result we get is good. however, i'm a little sick of figuring this out so i'll come back to this later.
	}
}

pub unsafe fn delete_lifetree(lib: &Library, pointer: *mut c_void, nlayers: i8) { 
	unsafe {
		let unsafe_delete_lifetree: Symbol<unsafe extern fn(*mut c_void,c_int)> =
			lib.get(b"DeleteLifetree\0").unwrap();
		unsafe_delete_lifetree(pointer,nlayers.into());
	}
}

pub unsafe fn get_diameter_of_pattern(lib: &Library, pointer: *mut c_void) -> Result<u32, &'static str> {
	unsafe {
		let unsafe_get_diameter_of_pattern: Symbol<unsafe extern fn(*mut c_void) -> u32> =
			lib.get(b"GetDiameterOfPattern\0").unwrap();
		return Ok(unsafe_get_diameter_of_pattern(pointer))
	}
}

pub unsafe fn delete_pattern(lib: &Library, pointer: *mut c_void) {
	unsafe {
		let unsafe_delete_pattern:Symbol<unsafe extern fn(*mut c_void)> =
			lib.get(b"DeletePattern\0").unwrap();
		unsafe_delete_pattern(pointer);
	}
}

pub unsafe fn save_pattern_rle(lib: &Library, pointer: *mut c_void, filename: &str, header: &str, footer: &str) {
	unsafe {
		let unsafe_save_pattern_rle:Symbol<unsafe extern fn(*mut c_void, *const c_char, *const c_char, *const c_char)> =
			lib.get(b"SavePatternRLE\0").unwrap();
		unsafe_save_pattern_rle(pointer, to_const_char(filename).as_ptr(), to_const_char(header).as_ptr(), to_const_char(footer).as_ptr());
	}
}

pub unsafe fn save_pattern_mc(lib: &Library, pointer: *mut c_void, filename: &str, header: &str, footer: &str) {
	unsafe {
		let unsafe_save_pattern_mc:Symbol<unsafe extern fn(*mut c_void, *const c_char, *const c_char, *const c_char)> =
			lib.get(b"SavePatternMC\0").unwrap();
		unsafe_save_pattern_mc(pointer, to_const_char(filename).as_ptr(), to_const_char(header).as_ptr(), to_const_char(footer).as_ptr());
	}
}

//TODO:: LoadTimelineMC
//TODO:: SaveTimelineMC

/*Boolean operations:
and: 0
or:  1
xor: 2
sub: 3
add: 1
mul: 4
matmul: 7 
*/

pub unsafe fn boolean_pattern_immutable(lib: &Library, pointer1: *mut c_void, pointer2: *mut c_void, op: u8) -> Result<*mut c_void, &'static str> {
	if op > 7 {return Err("operation code must be between 0 and 7")};
	unsafe {
		let unsafe_boolean_pattern_immutable:Symbol<unsafe extern fn(*mut c_void, *mut c_void, c_int) -> *mut c_void> =
			lib.get(b"BooleanPatternImmutable\0").unwrap();
		let result = unsafe_boolean_pattern_immutable(pointer1,pointer2,op.into());
		return Ok(result)
	}
}

pub unsafe fn boolean_pattern_mutable(lib: &Library, pointer1: *mut c_void, pointer2: *mut c_void, op: u8) {
	if op > 7 {panic!("operation code must be between 0 and 7")};
	unsafe {
		let unsafe_boolean_pattern_mutable:Symbol<unsafe extern fn(*mut c_void, *mut c_void, c_int) -> *mut c_void> =
			lib.get(b"BooleanPatternMutable\0").unwrap();
		unsafe_boolean_pattern_mutable(pointer1,pointer2,op.into());
	}
}

pub unsafe fn create_pattern_from_file(lib: &Library, lifetreepointer: *mut c_void, filename: &str) -> Result<*mut c_void, &'static str>{
	unsafe {
		let unsafe_create_pattern_from_file:Symbol<unsafe extern fn(*mut c_void, *const c_char) -> *mut c_void> = 
			lib.get(b"CreatePatternFromFile\0").unwrap();
		let result = unsafe_create_pattern_from_file(lifetreepointer, to_const_char(filename).as_ptr());
		return Ok(result)
	}
}

pub unsafe fn create_pattern_from_file_contents(lib: &Library, lifetreepointer: *mut c_void, contents: &str) -> Result<*mut c_void, &'static str>{
	unsafe {
		let unsafe_create_pattern_from_file_contents:Symbol<unsafe extern fn(*mut c_void, *const c_char) -> *mut c_void> = 
			lib.get(b"CreatePatternFromFileContents\0").unwrap();
		let result = unsafe_create_pattern_from_file_contents(lifetreepointer, to_const_char(contents).as_ptr());
		return Ok(result)
	}
}