// This file is part of mqtt. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/mqtt/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of mqtt. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/mqtt/master/COPYRIGHT.


#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct LLVMValueRefWrapper(LLVMValueRef);

impl Debug for LLVMValueRefWrapper
{
	fn fmt(&self, f: &mut Formatter) -> Result<(), fmt::Error>
	{
		write!(f, "LLVMValueRefWrapper({:?}={:?})", self.0, self.toString())
	}
}

impl ToLLVMValueRefWrapper for LLVMValueRefWrapper
{
	#[inline(always)]
	fn toLLVMValueRefWrapper(&self, _: &Context) -> LLVMValueRefWrapper
	{
		*self
	}
}

impl LLVMValueRefWrapper
{
	#[inline(always)]
	pub fn fromLLVMValueRef(value: LLVMValueRef) -> Self
	{
		debug_assert!(!value.is_null(), "value is null pointer");
		
		LLVMValueRefWrapper(value)
	}
	
	#[inline(always)]
	pub fn asLLVMValueRef(&self) -> LLVMValueRef
	{
		self.0
	}
	
	#[inline(always)]
	pub fn typeOf(&self) -> LLVMTypeRef
	{
		unsafe { LLVMTypeOf(self.0) }
	}
	
	#[inline(always)]
	pub fn valueKind(&self) -> LLVMValueKind
	{
		unsafe { LLVMGetValueKind(self.0) }
	}
	
	#[inline(always)]
	pub fn getNameMightBeNull(&self) -> *const c_char
	{
		unsafe { LLVMGetValueName(self.0) }
	}
	
	#[inline(always)]
	pub fn setNameMightBeNull(&self, name: *const c_char)
	{
		unsafe { LLVMSetValueName(self.0, name) }
	}
	
	#[inline(always)]
	pub fn dumpToStandardError(&self)
	{
		unsafe { LLVMDumpValue(self.0) }
	}
	
	#[inline(always)]
	pub fn toString(&self) -> CString
	{
		let pointer = unsafe { LLVMPrintValueToString(self.0) };
		let cString = (unsafe { CStr::from_ptr(pointer) }).to_owned();
		unsafe { LLVMDisposeMessage(pointer) };
		cString
	}
	
	#[inline(always)]
	pub fn isConstant(&self) -> bool
	{
		if unsafe { LLVMIsConstant(self.0) } == 0
		{
			false
		}
		else
		{
			true
		}
	}
	
	#[inline(always)]
	pub fn isNull(&self) -> bool
	{
		if unsafe { LLVMIsNull(self.0) } == 0
		{
			false
		}
		else
		{
			true
		}
	}
}
