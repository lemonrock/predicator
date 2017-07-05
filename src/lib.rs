// This file is part of predicator. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/predicator/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of predicator. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/predicator/master/COPYRIGHT.


#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![feature(associated_consts)]


#[macro_use] extern crate cpp;
extern crate libc;
pub extern crate llvm_sys;
#[macro_use] extern crate maplit;
extern crate rand;
pub extern crate rust_extra;


use ::libc::c_char;
use ::libc::c_void;
use ::libc::free;
use ::std::ffi::CStr;
use ::std::ffi::CString;


pub mod compiler;
pub mod llvm;


cpp!
{
	{
		#include "llvm/Support/Host.h"
		#include "llvm/ADT/Triple.h"
		
		#include <cstdio>
		#include <cstring>
		#include <sstream>
	}
}

pub(crate) fn llvmHostCpuName() -> Result<CString, String>
{
	unsafe
	{
		let result = cpp!([] -> *mut c_char as "char *"
		{
			return strdup(llvm::sys::getHostCPUName().data());
		});
		
		if result.is_null()
		{
			Err("Could not obtain host CPU features".to_owned())
		}
		else
		{
			// We do not pass back the LLVM pointer 'result' as we have no g'tee that Rust's memory allocator is the same as the one used in C
			let features = CStr::from_ptr(result as *const c_char).to_owned();
			free(result as *mut c_void);
			Ok(features)
		}
	}
}

pub(crate) fn llvmHostCpuFeatures() -> Result<CString, String>
{
	unsafe
	{
		let result = cpp!([] -> *mut c_char as "char *"
		{
			llvm::StringMap<bool> features;
			std::ostringstream buf;
		
			if (llvm::sys::getHostCPUFeatures(features))
			{
				for (auto &F : features)
				{
					if (buf.tellp())
					{
						buf << ',';
					}
					buf << ((F.second ? "+" : "-") + F.first()).str();
				}
				return strdup(buf.str().c_str());
			}
			return NULL;
		});
		
		if result.is_null()
		{
			Err("Could not obtain host CPU features".to_owned())
		}
		else
		{
			// We do not pass back the LLVM pointer 'result' as we have no g'tee that Rust's memory allocator is the same as the one used in C
			let features = CStr::from_ptr(result as *const c_char).to_owned();
			free(result as *mut c_void);
			Ok(features)
		}
	}
}
