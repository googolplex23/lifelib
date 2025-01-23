/*lowlevel.rs, low level c++ functions ported into rust with libloading for dynamic loading features.

These features are less safe than the python equivalents so these may or may not be public.

All of these functions are unsafe because there's nothing stopping you from putting in incorrect c_voids. 
The higher level functions should prevent this pretty handily but using these functions on their own is not advised.

Other reasons that these are unsafe and can cause memory leaks is that it us up to the user to call delete functions or the memory remains in use (!!)
Destructor Functions for higher level functions should avoid this.
*/

use libloading::Library;
use libloading::Symbol;
use core::ffi::c_int;
use core::ffi::c_uint;
use core::ffi::c_void;
use core::ffi::c_longlong;
use core::ffi::c_ulonglong;
use std::ffi::CString;
use std::os::raw::c_char;

fn to_const_char(input: &str) -> CString { return CString::new(input).expect("Unable to perform CString::new");}

pub unsafe fn create_lifetree(lib: &Library, maxmem: u16, nlayers: i8) -> Result<*mut c_void, &'static str> {// tested successfully.
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

pub unsafe fn save_pattern_rle(lib: &Library, pointer: *mut c_void, filename: &str, header: &str, footer: &str) {  //tested successfully
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

pub unsafe fn boolean_pattern_immutable(lib: &Library, pointer1: *mut c_void, pointer2: *mut c_void, op: u8) -> Result<*mut c_void, &'static str> { //tested successfully
	//the difference between the mutable and immutable boolean operations is that immutable creates a new pattern while mutable memorizes an existing one.
	if op > 7 {return Err("operation code must be between 0 and 7")};
	unsafe {
		let unsafe_boolean_pattern_immutable:Symbol<unsafe extern fn(*mut c_void, *mut c_void, c_int) -> *mut c_void> =
			lib.get(b"BooleanPatternImmutable\0").unwrap();
		let result = unsafe_boolean_pattern_immutable(pointer1,pointer2,op.into());
		return Ok(result)
	}
}

pub unsafe fn boolean_pattern_mutable(lib: &Library, pointer1: *mut c_void, pointer2: *mut c_void, op: u8) { //tested successfully
	if op > 7 {panic!("operation code must be between 0 and 7")};
	unsafe {
		let unsafe_boolean_pattern_mutable:Symbol<unsafe extern fn(*mut c_void, *mut c_void, c_int) -> *mut c_void> =
			lib.get(b"BooleanPatternMutable\0").unwrap();
		unsafe_boolean_pattern_mutable(pointer1,pointer2,op.into());
	}
}

pub unsafe fn create_pattern_from_file(lib: &Library, lifetree: *mut c_void, filename: &str) -> Result<*mut c_void, &'static str>{
	unsafe {
		let unsafe_create_pattern_from_file:Symbol<unsafe extern fn(*mut c_void, *const c_char) -> *mut c_void> = 
			lib.get(b"CreatePatternFromFile\0").unwrap();
		let result = unsafe_create_pattern_from_file(lifetree, to_const_char(filename).as_ptr());
		return Ok(result)
	}
}

pub unsafe fn create_pattern_from_file_contents(lib: &Library, lifetree: *mut c_void, contents: &str) -> Result<*mut c_void, &'static str>{ //tested successfully
	unsafe {
		let unsafe_create_pattern_from_file_contents:Symbol<unsafe extern fn(*mut c_void, *const c_char) -> *mut c_void> = 
			lib.get(b"CreatePatternFromFileContents\0").unwrap();
		let result = unsafe_create_pattern_from_file_contents(lifetree, to_const_char(contents).as_ptr());
		return Ok(result)
	}
}

pub unsafe fn create_rectangle(lib: &Library, lifetree: *mut c_void, x: i64, y: i64, width: u64, height: u64, rule: &str) -> Result<*mut c_void, &'static str>{
	//not sure if longlong is the correct format for this. the internal c++ lifelib function (see lifetree_abstract.h) uses int64_t and uint64_t,
	//but the c function in lifelib.cpp uses plain old int.
	//further testing needed.
	unsafe {
		let unsafe_create_rectangle:Symbol<unsafe extern fn(*mut c_void, c_longlong, c_longlong, c_ulonglong, c_ulonglong, *const c_char) -> *mut c_void> =
			lib.get(b"CreateRectangle\0").unwrap();
		let result = unsafe_create_rectangle(lifetree, x.into(), y.into(), width.into(), height.into(), to_const_char(rule).as_ptr());
		return Ok(result)
	}
}

pub unsafe fn create_pattern_from_rle(lib: &Library, lifetree: *mut c_void, rle: &str, rule: &str) -> Result<*mut c_void, &'static str> { //tested successfully
	unsafe {
		let unsafe_create_pattern_from_rle:Symbol<unsafe extern fn(*mut c_void, *const c_char, *const c_char) -> *mut c_void> =
			lib.get(b"CreatePatternFromRLE\0").unwrap();
		let result = unsafe_create_pattern_from_rle(lifetree, to_const_char(rle).as_ptr(), to_const_char(rule).as_ptr());
		return Ok(result)
	}
}

pub unsafe fn hashsoup(lib: &Library, pointer: *mut c_void, rule: &str, symmetry: &str, seed: &str) -> Result<*mut c_void, &'static str> { //tested successfully
	unsafe {
		let unsafe_hashsoup:Symbol<unsafe extern fn(*mut c_void, *const c_char, *const c_char, *const c_char) -> *mut c_void> =
			lib.get(b"Hashsoup\0").unwrap();
		let result = unsafe_hashsoup(pointer, to_const_char(rule).as_ptr(), to_const_char(symmetry).as_ptr(), to_const_char(seed).as_ptr());
		return Ok(result)
	}
}

pub unsafe fn advance_pattern(lib: &Library, pattern: *mut c_void, numgens: i64, exponent: u64) -> Result<*mut c_void, &'static str> { //works!! hooray!!
	unsafe {
		let unsafe_advance_pattern:Symbol<unsafe extern fn(*mut c_void, c_longlong, c_ulonglong) -> *mut c_void> =
			lib.get(b"AdvancePattern\0").unwrap();
		let result = unsafe_advance_pattern(pattern, numgens.into(), exponent.into());
		return Ok(result)
	}
}

//TODO: GetSemisolid and GetSolid. Still not entirely sure what these do.

pub unsafe fn bitshift_pattern(lib: &Library, pattern: *mut c_void, shift: i32) -> Result<*mut c_void, &'static str> {//this could have higher bitshifts. consider changing to i64 or larger.
	unsafe {
		let unsafe_bitshift_pattern:Symbol<unsafe extern fn(*mut c_void, c_int) -> *mut c_void> =
			lib.get(b"BitshiftPattern\0").unwrap();
		let result = unsafe_bitshift_pattern(pattern, shift.into());
		return Ok(result)
	}
}

pub unsafe fn shift_pattern(lib: &Library, pattern: *mut c_void, x: i64, y: i64, exponent: u32) -> Result<*mut c_void, &'static str> {//maybe change x and y types?
	unsafe {
		let unsafe_shift_pattern:Symbol<unsafe extern fn(*mut c_void, c_longlong, c_longlong, c_uint) -> *mut c_void> = 
			lib.get(b"ShiftPattern\0").unwrap();
		let result = unsafe_shift_pattern(pattern, x.into(), y.into(), exponent.into());
		return Ok(result)
	}
}

//TODO: GetPatternBox GetPatternDigest


pub unsafe fn get_pattern_digest(lib: &Library, pattern: *mut c_void) -> Result<*mut u64, &'static str> {
	unsafe {
		let unsafe_get_pattern_digest:Symbol<unsafe extern fn(*mut c_void) -> *mut c_ulonglong> =
			lib.get(b"GetPatternDigest\0").unwrap();
		let result = unsafe_get_pattern_digest(pattern);
		return Ok(result.into())
	}
}

pub unsafe fn get_pattern_octodigest(lib: &Library, pattern: *mut c_void) -> Result<*mut u64, &'static str> {
	unsafe {
		let unsafe_get_pattern_octodigest:Symbol<unsafe extern fn(*mut c_void) -> *mut c_ulonglong> =
			lib.get(b"GetPatternOctodigest\0").unwrap();
		let result = unsafe_get_pattern_octodigest(pattern);
		return Ok(result.into())
	}
}

