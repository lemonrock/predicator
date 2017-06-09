// This file is part of predicator. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/predicator/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of predicator. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/predicator/master/COPYRIGHT.


#[derive(Clone)]
pub struct Target
{
	reference: LLVMTargetRef,
	triple: CString,
}

impl Target
{
	// llvmlite exposes these, but not llvm-sys: https://github.com/numba/llvmlite/blob/646fb3f396fa3c5bd853025466ac6d78e7e4ed94/ffi/targets.cpp
	// getHostCPUFeatures
	pub fn createHostOrcJitStack<'a>(cpu: *const c_char, features: *const c_char) -> Result<OrcJitStack, String>
	{
		let hostTarget = Target::obtainTargetForHost()?;
		let hostTargetMachine = hostTarget.createTargetMachine(cpu, features, LLVMCodeGenOptLevel::LLVMCodeGenLevelAggressive, LLVMRelocMode::LLVMRelocStatic, LLVMCodeModel::LLVMCodeModelJITDefault)?;
		let orcJitStack = hostTargetMachine.toOrcJitStack()?;
		Ok(orcJitStack)
	}
	
	#[inline(always)]
	fn defaultTargetTriple() -> CString
	{
		let result = unsafe { LLVMGetDefaultTargetTriple() };
		
		let value = (unsafe { CStr::from_ptr(result) }).to_owned();
		
		unsafe { LLVMDisposeMessage(result) };
		
		value
	}
	
	#[inline(always)]
	pub fn obtainTargetForHost() -> Result<Self, String>
	{
		let targetTriple = Self::defaultTargetTriple();
		
		let mut targetReference = unsafe { uninitialized() };
		
		let mut errorMessage = null_mut();
		let boolean = unsafe { LLVMGetTargetFromTriple(targetTriple.as_ptr(), &mut targetReference, &mut errorMessage) };
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
	pub fn createTargetMachine(&self, cpu: *const c_char, features: *const c_char, level: LLVMCodeGenOptLevel, relocationMode: LLVMRelocMode, codeModel: LLVMCodeModel) -> Result<TargetMachine, String>
	{
		let reference = unsafe { LLVMCreateTargetMachine(self.reference, self.triple.as_ptr(), cpu, features, level, relocationMode, codeModel) };
		if reference.is_null()
		{
			Err("Could not create target machine".to_owned())
		}
		else
		{
			Ok
			(
				TargetMachine
				{
					reference: reference,
				}
			)
		}
	}
}

