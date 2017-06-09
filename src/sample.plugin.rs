// This file is part of predicator. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/predicator/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of predicator. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/predicator/master/COPYRIGHT.


// START BOILERPLATE

#![no_std]
#![feature(lang_items)]
#![feature(libc)]
#![no_main]


extern crate libc;


#[lang = "panic_fmt"]
fn panic_fmt() -> !
{
	loop
	{
	}
}

#[lang = "eh_personality"]
extern fn eh_personality()
{
}

#[allow(unused_variables)]
#[cfg(not(debug_assertions))]
#[no_mangle]
pub extern fn main(argc: isize, argv: *const *const u8) -> isize
{
	0
}

#[cfg(debug_assertions)]
#[no_mangle]
pub extern fn main(argc: isize, argv: *const *const u8) -> isize
{
	test(argc, argv)
}

// END BOILERPLATE


use ::libc::c_char;


#[cfg(debug_assertions)]
pub fn test(argc: isize, argv: *const *const u8) -> isize
{
	const HELLO: &'static [u8] = b"Hello World!\x00";
	
	use ::libc::puts;
	
	unsafe
	{
		puts(HELLO.as_ptr() as *const i8);
	}
	
	0
}

//#[derive(Default, Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
//pub struct MyStruct
//{
//	item: u32,
//}

#[no_mangle]
pub extern fn sample_plugin() -> *const c_char
{
	"Hello, world from sample_plugin!\0".as_ptr() as *const c_char
}
