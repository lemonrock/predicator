// This file is part of predicator. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/predicator/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of predicator. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/predicator/master/COPYRIGHT.


pub struct Module<'a>
{
	reference: LLVMModuleRef,
	parent: &'a PerThreadContext,
}

impl<'a> Drop for Module<'a>
{
	#[inline(always)]
	fn drop(&mut self)
	{
		unsafe { LLVMDisposeModule(self.reference) }
	}
}

impl<'a> Clone for Module<'a>
{
	#[inline(always)]
	fn clone(&self) -> Self
	{
		Module
		{
			reference: unsafe { LLVMCloneModule(self.reference) },
			parent: self.parent,
		}
	}
}

impl<'a> Module<'a>
{
	// ParseIRFile
	
	#[inline(always)]
	fn verify(&self) -> Result<(), String>
	{
		let mut errorMessage = null_mut();
		let boolean = unsafe { LLVMVerifyModule(self.reference, LLVMVerifierFailureAction::LLVMReturnStatusAction, &mut errorMessage) };
		handle_boolean_and_error_message!(boolean, errorMessage, LLVMVerifyModule);
		Ok(())
	}
	
	#[inline(always)]
	pub fn createFunctionPassManager<'b>(&'b self) -> Result<FunctionPassManager<'a, 'b>, FunctionPassManagerError>
	{
		let reference = unsafe { LLVMCreateFunctionPassManagerForModule(self.reference) };
		if reference.is_null()
		{
			Err(FunctionPassManagerError::CouldNotCreate)
		}
		else
		{
			Ok
			(
				FunctionPassManager
				{
					reference: reference,
					parent: self,
				}
			)
		}
	}
	
	#[inline(always)]
	pub fn executionEngineMachineCodeJit<'b>(&'b self) -> Result<ExecutionEngine<'a, 'b>, String>
	{
		self.verify()?;
		
		let sizeOfOptions = size_of::<LLVMMCJITCompilerOptions>();
		
		let mut options = unsafe { zeroed() };
		unsafe { LLVMInitializeMCJITCompilerOptions(&mut options, sizeOfOptions) };
		options.OptLevel = 3;
		options.CodeModel = LLVMCodeModel::LLVMCodeModelJITDefault;
		options.NoFramePointerElim = 0;
		options.EnableFastISel = 1;
		//options.MCJMM = ??? LLVMMCJITMemoryManagerRef
		
		let mut executionEngine = unsafe { uninitialized() };
		
		let mut errorMessage = null_mut();
		let boolean = unsafe { LLVMCreateMCJITCompilerForModule(&mut executionEngine, self.reference, &mut options, sizeOfOptions, &mut errorMessage) };
		handle_boolean_and_error_message!(boolean, errorMessage, LLVMCreateMCJITCompilerForModule);
		
		Ok
		(
			ExecutionEngine
			{
				reference: executionEngine,
				parent: self
			}
		)
	}
}
