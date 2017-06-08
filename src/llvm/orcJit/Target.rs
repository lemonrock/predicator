// This file is part of predicator. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/predicator/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of predicator. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/predicator/master/COPYRIGHT.


pub struct Target
{
	reference: LLVMTargetRef,
	triple: *const c_char,
}

impl Target
{
	#[inline(always)]
	fn defaultTargetTriple() -> *const c_char
	{
		unsafe { LLVMGetDefaultTargetTriple() }
	}
	
	#[inline(always)]
	pub fn createForDefault() -> Result<Self, String>
	{
		let targetTriple = Self::defaultTargetTriple();
		
		let mut targetReference = unsafe { uninitialized() };
		
		let mut errorMessage = null_mut();
		let boolean = unsafe { LLVMGetTargetFromTriple(targetTriple, &mut targetReference, &mut errorMessage) };
		handle_boolean_and_error_message!(boolean, errorMessage, LLVMGetTargetFromTriple);
		
		Ok
		(
			Self
			{
				reference: targetReference,
				triple: targetTriple,
			}
		)
	}
	
	#[inline(always)]
	pub fn name<'a>(&'a self) -> &'a CStr
	{
		unsafe { CStr::from_ptr(LLVMGetTargetName(self.reference)) }
	}
	
	#[inline(always)]
	pub fn description<'a>(&'a self) -> &'a CStr
	{
		unsafe { CStr::from_ptr(LLVMGetTargetDescription(self.reference)) }
	}
	
	#[inline(always)]
	pub fn hasJit(&self) -> bool
	{
		(unsafe { LLVMTargetHasJIT(self.reference) }) != 0
	}
	
	#[inline(always)]
	pub fn hasAssemblerBackend(&self) -> bool
	{
		(unsafe { LLVMTargetHasAsmBackend(self.reference) }) != 0
	}
	
	#[inline(always)]
	pub fn hasTargetMachine(&self) -> bool
	{
		(unsafe { LLVMTargetHasTargetMachine(self.reference) }) != 0
	}
	
	#[inline(always)]
	pub fn createTargetMachine<'a>(&'a self, cpu: *const c_char, features: *const c_char, level: LLVMCodeGenOptLevel, relocationMode: LLVMRelocMode, codeModel: LLVMCodeModel) -> TargetMachine<'a>
	{
		// LLVMDisposeTargetMachine
		TargetMachine
		{
			reference: unsafe { LLVMCreateTargetMachine(self.reference, self.triple, cpu, features, level, relocationMode, codeModel) },
			parent: self,
		}
	}
}

