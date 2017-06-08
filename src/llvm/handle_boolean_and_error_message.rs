// This file is part of predicator. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/predicator/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of predicator. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/predicator/master/COPYRIGHT.


macro_rules! handle_boolean_and_error_message
{
	($boolean: ident, $errorMessage: ident, $functionName: ident) =>
	{
		{
			if $crate::rust_extra::unlikely(!$errorMessage.is_null())
			{
				if $crate::rust_extra::unlikely($boolean != 0)
				{
					let message = format!("{}:{:?}", stringify!($functionName), unsafe { ::std::ffi::CStr::from_ptr($errorMessage) });
					unsafe { $crate::llvm_sys::core::LLVMDisposeMessage($errorMessage) };
					return Err(message)
				}
				unsafe { $crate::llvm_sys::core::LLVMDisposeMessage($errorMessage) };
			}
			if $crate::rust_extra::unlikely($boolean != 0)
			{
				return Err(format!("{}:(unknown)", stringify!($functionName)))
			}
		}
	}
}
