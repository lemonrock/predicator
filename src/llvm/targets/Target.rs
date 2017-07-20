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
	pub fn createHostOrcJitStack(optimisationLevel: LLVMCodeGenOptLevel) -> Result<LLVMOrcJITStackRef, String>
	{
		let hostTargetMachine = Self::createHostTargetMachine(optimisationLevel)?;
		hostTargetMachine.toOrcJitStack()
	}
	
	pub fn createHostTargetMachine(optimisationLevel: LLVMCodeGenOptLevel) -> Result<TargetMachine, String>
	{
		let hostTarget = Target::obtainTargetForHost()?;
		let hostCpuName = ::llvmHostCpuName()?;
		let hostCpuFeatures = ::llvmHostCpuFeatures()?;
		
		hostTarget.createTargetMachine(hostCpuName.as_ptr(), hostCpuFeatures.as_ptr(), optimisationLevel, LLVMRelocMode::LLVMRelocStatic, LLVMCodeModel::LLVMCodeModelJITDefault)
	}
	
	#[inline(always)]
	pub fn defaultTargetTriple() -> CString
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

