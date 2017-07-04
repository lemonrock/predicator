// This file is part of predicator. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/predicator/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of predicator. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/predicator/master/COPYRIGHT.


#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(C)]
pub enum UsefulLLVMRealPredicate
{
	LLVMRealPredicateFalse = 0,
	LLVMRealOEQ = 1,
	LLVMRealOGT = 2,
	LLVMRealOGE = 3,
	LLVMRealOLT = 4,
	LLVMRealOLE = 5,
	LLVMRealONE = 6,
	LLVMRealORD = 7,
	LLVMRealUNO = 8,
	LLVMRealUEQ = 9,
	LLVMRealUGT = 10,
	LLVMRealUGE = 11,
	LLVMRealULT = 12,
	LLVMRealULE = 13,
	LLVMRealUNE = 14,
	LLVMRealPredicateTrue = 15,
}

impl UsefulLLVMRealPredicate
{
	#[inline(always)]
	pub fn to_LLVMRealPredicate(&self) -> LLVMRealPredicate
	{
		unsafe { transmute(*self) }
	}
}
