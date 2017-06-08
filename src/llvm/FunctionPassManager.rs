// This file is part of predicator. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/predicator/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of predicator. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/predicator/master/COPYRIGHT.


pub struct FunctionPassManager<'a, 'b>
	where 'a: 'b
{
	reference: LLVMPassManagerRef,
	parent: &'b Module<'a>,
}

impl<'a, 'b> Drop for FunctionPassManager<'a, 'b>
	where 'a: 'b
{
	#[inline(always)]
	fn drop(&mut self)
	{
		unsafe { LLVMDisposePassManager(self.reference) }
	}
}

impl<'a, 'b> FunctionPassManager<'a, 'b>
	where 'a: 'b
{
	pub fn runPassesOnModule(self) -> Result<(), FunctionPassManagerError>
	{
		self.addFunctionPasses();
		
		self.initialize()?;
		
		let mut functionReference = unsafe { LLVMGetFirstFunction(self.parent.reference) };
		
		while !functionReference.is_null()
		{
			self.verifyFunctionBeforeRunningPasses(functionReference)?;
			self.runPassesOnFunction(functionReference)?;
			self.verifyFunctionAfterRunningPasses(functionReference)?;
			
			functionReference = unsafe { LLVMGetNextFunction(functionReference) }
		}
		self.finalize()
	}
	
	#[inline(always)]
	fn addFunctionPasses(&self)
	{
		let reference = self.reference;
		
		unsafe
		{
			LLVMAddBasicAliasAnalysisPass(reference);
			
			LLVMAddInstructionCombiningPass(reference);
			
			LLVMAddReassociatePass(reference);
			
			LLVMAddGVNPass(reference);
			
			LLVMAddCFGSimplificationPass(reference);
			
			/*
			
			LLVMAddAggressiveDCEPass(reference);
			
			LLVMAddAlignmentFromAssumptionsPass(reference);
			
			LLVMAddDeadStoreEliminationPass(reference);
			
			LLVMAddScalarizerPass(reference);
			
			LLVMAddMergedLoadStoreMotionPass(reference);
			
			LLVMAddIndVarSimplifyPass(reference);
			
			LLVMAddJumpThreadingPass(reference);
			
			LLVMAddLICMPass(reference);
			
			LLVMAddLoopIdiomPass(reference);
			
			LLVMAddLoopRotatePass(reference);
			
			LLVMAddLoopRerollPass(reference);
			
			LLVMAddLoopUnrollPass(reference);
			
			LLVMAddLoopUnswitchPass(reference);
			
			LLVMAddMemCpyOptPass(reference);
			
			LLVMAddPartiallyInlineLibCallsPass(reference);
			
			LLVMAddPromoteMemoryToRegisterPass(reference);
			
			LLVMAddSCCPPass(reference);
			
			LLVMAddScalarReplAggregatesPass(reference);
			
			LLVMAddScalarReplAggregatesPassSSA(reference);
			
			LLVMAddScalarReplAggregatesPassWithThreshold(reference, threshold);
			
			LLVMAddScalarReplAggregatesPass(reference);
			
			LLVMAddScalarReplAggregatesPass(reference);
			
			LLVMAddScalarReplAggregatesPass(reference);
			
			LLVMAddSimplifyLibCallsPass(reference);
			
			LLVMAddTailCallEliminationPass(reference);
			
			LLVMAddConstantPropagationPass(reference);
			
			LLVMAddDemoteMemoryToRegisterPass(reference);
			
			LLVMAddDemoteMemoryToRegisterPass(reference);
			
			LLVMAddDemoteMemoryToRegisterPass(reference);
			
			LLVMAddVerifierPass(reference);
			
			LLVMAddCorrelatedValuePropagationPass(reference);
			
			LLVMAddEarlyCSEPass(reference);
			
			LLVMAddLowerExpectIntrinsicPass(reference);
			
			LLVMAddTypeBasedAliasAnalysisPass(reference);
			
			LLVMAddScopedNoAliasAAPass(reference);
			
			*/
		}
	}
	
	fn initialize(&self) -> Result<(), FunctionPassManagerError>
	{
		if unlikely(unsafe { LLVMInitializeFunctionPassManager(self.reference) } != 0)
		{
			Err(FunctionPassManagerError::CouldNotInitialize)
		}
		else
		{
			Ok(())
		}
	}
	
	#[inline(always)]
	fn verifyFunctionBeforeRunningPasses(&self, functionReference: LLVMValueRef) -> Result<(), FunctionPassManagerError>
	{
		if unlikely(unsafe { LLVMVerifyFunction(functionReference, LLVMVerifierFailureAction::LLVMReturnStatusAction) } != 0)
		{
			Err(FunctionPassManagerError::FunctionInvalidBeforeRunningPasses(Self::functionName(functionReference)))
		}
		else
		{
			Ok(())
		}
	}
	
	#[inline(always)]
	fn runPassesOnFunction(&self, functionReference: LLVMValueRef) -> Result<(), FunctionPassManagerError>
	{
		if unlikely(unsafe { LLVMRunFunctionPassManager(self.reference, functionReference) } != 0)
		{
			Err(FunctionPassManagerError::CouldNotRunPassesOnFunction(Self::functionName(functionReference)))
		}
		else
		{
			Ok(())
		}
	}
	
	#[inline(always)]
	fn verifyFunctionAfterRunningPasses(&self, functionReference: LLVMValueRef) -> Result<(), FunctionPassManagerError>
	{
		if unlikely(unsafe { LLVMVerifyFunction(functionReference, LLVMVerifierFailureAction::LLVMReturnStatusAction) } != 0)
		{
			Err(FunctionPassManagerError::FunctionInvalidAfterRunningPasses(Self::functionName(functionReference)))
		}
		else
		{
			Ok(())
		}
	}
	
	#[inline(always)]
	fn finalize(self) -> Result<(), FunctionPassManagerError>
	{
		let boolean = unsafe { LLVMFinalizeFunctionPassManager(self.reference) };
		if unlikely(boolean != 0)
		{
			Err(FunctionPassManagerError::CouldNotFinalize)
		}
		else
		{
			Ok(())
		}
	}
	
	#[inline(always)]
	fn functionName(functionReference: LLVMValueRef) -> CString
	{
		let functionNamePointer = unsafe { LLVMGetValueName(functionReference) };
		if functionNamePointer.is_null()
		{
			CString::new("(unknown)").unwrap()
		}
		else
		{
			let wrapped = unsafe { CStr::from_ptr(functionNamePointer) };
			wrapped.to_owned()
		}
	}
}
