// This file is part of predicator. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/predicator/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of predicator. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/predicator/master/COPYRIGHT.


//noinspection SpellCheckingInspection
#[repr(u32)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum UsefulLLVMCallConv
{
	LLVMCCallConv = 0,
	LLVMFastCallConv = 8,
	LLVMColdCallConv = 9,
	LLVMWebKitJSCallConv = 12,
	LLVMAnyRegCallConv = 13,
	LLVMX86StdcallCallConv = 64,
	LLVMX86FastcallCallConv = 65,
}

impl UsefulLLVMCallConv
{
	#[inline(always)]
	pub fn to_LLVMCallConv(&self) -> LLVMCallConv
	{
		unsafe { transmute(*self) }
	}
}
