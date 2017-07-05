// This file is part of mqtt. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/mqtt/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of mqtt. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/mqtt/master/COPYRIGHT.


#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct GlobalValue(LLVMValueRefWrapper);

impl Value for GlobalValue
{
	#[inline(always)]
	fn fromLLVMValueRef(value: LLVMValueRef) -> Self
	{
		GlobalValue(LLVMValueRefWrapper::fromLLVMValueRef(value))
	}
	
	#[inline(always)]
	fn asLLVMValueRef(&self) -> LLVMValueRef
	{
		self.0.asLLVMValueRef()
	}
}

impl GlobalValue
{
	#[inline(always)]
	pub fn setLinkage(&self, llvmLinkage: LLVMLinkage)
	{
		unsafe { LLVMSetLinkage(self.reference(), llvmLinkage) };
	}
	
	#[inline(always)]
	pub fn setVisibility(&self, llvmVisibility: LLVMVisibility)
	{
		unsafe { LLVMSetVisibility(self.reference(), llvmVisibility) };
	}
	
	#[inline(always)]
	pub fn setSection(&self, section: &str)
	{
		let cSection = CString::new(section).expect("section contains embedded NULLs");
		unsafe { LLVMSetSection(self.reference(), cSection.as_ptr()) };
	}
	
	#[inline(always)]
	pub fn setDllStorageClass(&self, llvmDllStorageClass: LLVMDLLStorageClass)
	{
		unsafe { LLVMSetDLLStorageClass(self.reference(), llvmDllStorageClass) };
	}
	
	#[inline(always)]
	pub fn setHasUnnamedAddress(&self, hasUnnamedAddress: bool)
	{
		let HasUnnamedAddr = if hasUnnamedAddress
		{
			1
		}
		else
		{
			0
		};
		unsafe { LLVMSetUnnamedAddr(self.reference(), HasUnnamedAddr) };
	}
	
	#[inline(always)]
	pub fn setAlignment(&self, alignment: PowerOfTwoThirtyTwoBit)
	{
		unsafe { LLVMSetAlignment(self.reference(), alignment.as_u32()) };
	}
	
	#[inline(always)]
	pub fn setIsConstant(&self)
	{
		unsafe { LLVMSetGlobalConstant(self.reference(), 1) };
	}
	
	#[inline(always)]
	pub fn setIsVariable(&self, threadLocalMode: LLVMThreadLocalMode)
	{
		let reference = self.reference();
		unsafe { LLVMSetGlobalConstant(reference, 0) };
		match threadLocalMode
		{
			LLVMThreadLocalMode::LLVMNotThreadLocal => (),
			_ => unsafe { LLVMSetThreadLocal(reference, 1) }
		}
		unsafe { LLVMSetThreadLocalMode(reference, threadLocalMode) };
	}
	
	#[inline(always)]
	pub fn setIsInternallyInitialized(&self, context: &Context, constantInitializer: &Constant)
	{
		unsafe { LLVMSetExternallyInitialized(self.reference(), 0) };
		self.setInitializer(context, constantInitializer);
	}
	
	#[inline(always)]
	pub fn setIsExternallyInitialized(&self)
	{
		unsafe { LLVMSetExternallyInitialized(self.reference(), 1) };
	}
	
	#[inline(always)]
	fn setInitializer(&self, context: &Context, constantInitializer: &Constant)
	{
		unsafe { LLVMSetInitializer(self.reference(), context.constant(constantInitializer).asLLVMValueRef()) };
	}
	
	#[inline(always)]
	fn reference(&self) -> LLVMValueRef
	{
		(self.0).0
	}
}
