// This file is part of mqtt. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/mqtt/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of mqtt. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/mqtt/master/COPYRIGHT.


#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct LLVMTypeRefWrapper(LLVMTypeRef);

impl Debug for LLVMTypeRefWrapper
{
	fn fmt(&self, f: &mut Formatter) -> Result<(), fmt::Error>
	{
		write!(f, "LLVMTypeRefWrapper({:?}={:?})", self.0, self.toString())
	}
}

impl LLVMTypeRefWrapper
{
	#[inline(always)]
	pub fn fromLLVMTypeRef(value: LLVMTypeRef) -> Self
	{
		debug_assert!(!value.is_null(), "value is null");
		
		LLVMTypeRefWrapper(value)
	}
	
	#[inline(always)]
	pub fn asLLVMTypeRef(&self) -> LLVMTypeRef
	{
		self.0
	}
	
	#[inline(always)]
	pub fn typeKind(&self) -> LLVMTypeKind
	{
		unsafe { LLVMGetTypeKind(self.0) }
	}
	
	#[inline(always)]
	pub fn dumpToStandardError(&self)
	{
		unsafe { LLVMDumpType(self.0) }
	}
	
	#[inline(always)]
	pub fn toString(&self) -> CString
	{
		let pointer = unsafe { LLVMPrintTypeToString(self.0) };
		let cString = (unsafe { CStr::from_ptr(pointer) }).to_owned();
		unsafe { LLVMDisposeMessage(pointer) };
		cString
	}
	
	#[inline(always)]
	pub fn isSized(&self) -> bool
	{
		if unsafe { LLVMTypeIsSized(self.0) } == 0
		{
			false
		}
		else
		{
			true
		}
	}
	
	#[inline(always)]
	pub fn context(&self) -> LLVMContextRef
	{
		unsafe { LLVMGetTypeContext(self.0) }
	}
}
