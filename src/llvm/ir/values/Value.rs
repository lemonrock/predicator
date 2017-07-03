// This file is part of mqtt. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/mqtt/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of mqtt. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/mqtt/master/COPYRIGHT.


pub trait Value: Sized
{
	#[inline(always)]
	fn fromLLVMValueRef(value: LLVMValueRef) -> Self;
	
	#[inline(always)]
	fn asLLVMValueRef(&self) -> LLVMValueRef;
	
	#[inline(always)]
	fn fromLLVMValueRefWrapper(wrapper: LLVMValueRefWrapper) -> Self
	{
		Self::fromLLVMValueRef(wrapper.asLLVMValueRef())
	}
	
	#[inline(always)]
	fn typeOf(&self) -> LLVMTypeRef
	{
		unsafe { LLVMTypeOf(self.asLLVMValueRef()) }
	}
	
	#[inline(always)]
	fn valueKind(&self) -> LLVMValueKind
	{
		unsafe { LLVMGetValueKind(self.asLLVMValueRef()) }
	}
	
	#[inline(always)]
	fn getNameMightBeNull(&self) -> *const c_char
	{
		unsafe { LLVMGetValueName(self.asLLVMValueRef()) }
	}
	
	#[inline(always)]
	fn setNameMightBeNull(&self, name: *const c_char)
	{
		unsafe { LLVMSetValueName(self.asLLVMValueRef(), name) }
	}
	
	#[inline(always)]
	fn dumpToStandardError(&self)
	{
		unsafe { LLVMDumpValue(self.asLLVMValueRef()) }
	}
	
	#[inline(always)]
	fn toString(&self) -> CString
	{
		let pointer = unsafe { LLVMPrintValueToString(self.asLLVMValueRef()) };
		let cString = (unsafe { CStr::from_ptr(pointer) }).to_owned();
		unsafe { LLVMDisposeMessage(pointer) };
		cString
	}
	
	#[inline(always)]
	fn isConstant(&self) -> bool
	{
		if unsafe { LLVMIsConstant(self.asLLVMValueRef()) } == 0
		{
			false
		}
		else
		{
			true
		}
	}
	
	#[inline(always)]
	fn isUndefined(&self) -> bool
	{
		if unsafe { LLVMIsUndef(self.asLLVMValueRef()) } == 0
		{
			false
		}
		else
		{
			true
		}
	}
}
