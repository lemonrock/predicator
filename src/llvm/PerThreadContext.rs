// This file is part of predicator. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/predicator/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of predicator. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/predicator/master/COPYRIGHT.


pub struct PerThreadContext
{
	reference: LLVMContextRef
}

impl Drop for PerThreadContext
{
	#[inline(always)]
	fn drop(&mut self)
	{
		unsafe { LLVMContextDispose(self.reference) }
	}
}

impl PerThreadContext
{
	#[inline(always)]
	pub fn initialiseOnceOnMainThread()
	{
		unsafe { LLVMLinkInMCJIT() };
		
		let boolean = unsafe { LLVM_InitializeNativeTarget() };
		panic_on_false!(boolean, LLVM_InitializeNativeTarget);
		
		unsafe { LLVM_InitializeAllTargetMCs() };
		
		let boolean = unsafe { LLVM_InitializeNativeAsmPrinter() };
		panic_on_false!(boolean, LLVM_InitializeNativeAsmPrinter);
		
		let boolean = unsafe { LLVM_InitializeNativeAsmParser() };
		panic_on_false!(boolean, LLVM_InitializeNativeAsmParser);
	}
	
	#[inline(always)]
	pub fn new() -> Result<Self, ()>
	{
		let reference = unsafe { LLVMContextCreate() };
		if reference.is_null()
		{
			Err(())
		}
		else
		{
			Ok
			(
				Self
				{
					reference: reference
				}
			)
		}
	}
	
	// See also LLVMGetBitcodeModuleInContext2 and  LLVMParseBitcodeInContext2
	// in http://www.llvm.org/docs/doxygen/html/group__LLVMCBitReader.html
	
	#[inline(always)]
	pub fn createModule<'a>(&'a self, name: String) -> Module<'a>
	{
		let name = CString::new(name).expect("name contains embedded NULs");
		
		Module
		{
			reference: unsafe { LLVMModuleCreateWithNameInContext(name.as_ptr(), self.reference) },
			parent: self,
		}
	}
	
	#[inline(always)]
	pub fn loadBitCodeIntoModule<'a, 'b>(&'a self, memoryBuffer: &MemoryBuffer<'b>) -> Result<Module<'a>, String>
	{
		let mut moduleReference = unsafe { uninitialized() };
		
		let boolean = unsafe { LLVMGetBitcodeModuleInContext2(self.reference, memoryBuffer.reference, &mut moduleReference) };
		panic_on_false!(boolean, LLVMGetBitcodeModuleInContext2);
		
		Ok
		(
			Module
			{
				reference: moduleReference,
				parent: self,
			}
		)
	}
	
	#[inline(always)]
	pub fn parseBitCodeIntoModule<'a, 'b>(&'a self, memoryBuffer: &MemoryBuffer<'b>) -> Result<Module<'a>, String>
	{
		let mut moduleReference = unsafe { uninitialized() };
		
		let boolean = unsafe { LLVMParseBitcodeInContext2(self.reference, memoryBuffer.reference, &mut moduleReference) };
		panic_on_false!(boolean, LLVMParseBitcodeInContext2);
		
		Ok
		(
			Module
			{
				reference: moduleReference,
				parent: self,
			}
		)
	}
	
	#[inline(always)]
	pub fn parseTextualIntermediateRepresentationIntoModule<'a, 'b>(&'a self, memoryBuffer: &MemoryBuffer<'b>) -> Result<Module<'a>, String>
	{
		let mut moduleReference = unsafe { uninitialized() };
		
		let mut errorMessage = null_mut();
		let boolean = unsafe { LLVMParseIRInContext(self.reference, memoryBuffer.reference, &mut moduleReference, &mut errorMessage) };
		handle_boolean_and_error_message!(boolean, errorMessage, LLVMParseIRInContext);
		
		Ok
		(
			Module
			{
				reference: moduleReference,
				parent: self,
			}
		)
	}
}
