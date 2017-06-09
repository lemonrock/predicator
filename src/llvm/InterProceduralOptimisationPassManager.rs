// This file is part of predicator. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/predicator/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of predicator. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/predicator/master/COPYRIGHT.


pub struct InterProceduralOptimisationPassManager
{
	reference: LLVMPassManagerRef,
}

impl Drop for InterProceduralOptimisationPassManager
{
	#[inline(always)]
	fn drop(&mut self)
	{
		unsafe { LLVMDisposePassManager(self.reference) }
	}
}

impl InterProceduralOptimisationPassManager
{
	#[inline(always)]
	pub fn create() -> Result<Self, InterProceduralOptimisationPassManagerError>
	{
		let reference = unsafe { LLVMCreatePassManager() };
		if unlikely(reference.is_null())
		{
			Err(InterProceduralOptimisationPassManagerError::CouldNotCreate)
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
	
	#[inline(always)]
	pub fn runPassesOnModule(self, module: Module) -> Result<Module, InterProceduralOptimisationPassManagerError>
	{
		self.addInterProceduralOptimisationPasses();
		
		let module = module.verify().map_err(|error| InterProceduralOptimisationPassManagerError::ModuleInvalidBeforeRunningPasses(error))?;
		
		if unlikely(unsafe { LLVMRunPassManager(self.reference, module.reference) } != 0)
		{
			return Err(InterProceduralOptimisationPassManagerError::CouldNotRunPassesOnModule);
		}
		
		module.verify().map_err(|error| InterProceduralOptimisationPassManagerError::ModuleInvalidAfterRunningPasses(error))
	}
	
	#[inline(always)]
	fn addInterProceduralOptimisationPasses(&self)
	{
		let reference = self.reference;

		unsafe
		{
			LLVMAddAlwaysInlinerPass(reference);

			LLVMAddArgumentPromotionPass(reference);

			LLVMAddConstantMergePass(reference);

			LLVMAddDeadArgEliminationPass(reference);

			LLVMAddFunctionAttrsPass(reference);

			LLVMAddFunctionInliningPass(reference);

			LLVMAddFunctionAttrsPass(reference);

			LLVMAddGlobalDCEPass(reference);

			LLVMAddGlobalOptimizerPass(reference);

			LLVMAddIPConstantPropagationPass(reference);

			LLVMAddIPSCCPPass(reference);

			const AllButMain: u32 = 0;
			LLVMAddInternalizePass(reference, AllButMain);

			LLVMAddPruneEHPass(reference);

			LLVMAddStripDeadPrototypesPass(reference);

			LLVMAddStripSymbolsPass(reference);
		}
	}
}
