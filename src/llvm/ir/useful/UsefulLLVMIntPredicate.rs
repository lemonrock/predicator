// This file is part of predicator. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/predicator/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of predicator. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/predicator/master/COPYRIGHT.


#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(C)]
pub enum UsefulLLVMIntPredicate
{
	LLVMIntEQ = 32,
	LLVMIntNE = 33,
	LLVMIntUGT = 34,
	LLVMIntUGE = 35,
	LLVMIntULT = 36,
	LLVMIntULE = 37,
	LLVMIntSGT = 38,
	LLVMIntSGE = 39,
	LLVMIntSLT = 40,
	LLVMIntSLE = 41,
}

impl UsefulLLVMIntPredicate
{
	#[inline(always)]
	pub fn to_LLVMIntPredicate(&self) -> LLVMIntPredicate
	{
		unsafe { transmute(*self) }
	}
}
