// This file is part of predicator. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/predicator/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of predicator. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/predicator/master/COPYRIGHT.


pub struct ExecutionEngine
{
	pub(crate) reference: LLVMExecutionEngineRef,
	#[allow(dead_code)] pub(crate) parentDropWrapper: Rc<ModuleDropWrapper>,
}

impl Drop for ExecutionEngine
{
	#[inline(always)]
	fn drop(&mut self)
	{
		//use ::llvm_sys::execution_engine::LLVMRemoveModule as executionEngineRemoveModule;
		//		fn removeModule(executionEngineReference: LLVMExecutionEngineRef, moduleReference: LLVMModuleRef) -> Result<(), String>
		//		{
		//			let mut outReference = null_mut();
		//			let mut errorMessage = null_mut();
		//			let boolean = unsafe { executionEngineRemoveModule(executionEngineReference, moduleReference, &mut outReference, &mut errorMessage) };
		//			handle_boolean_and_error_message!(boolean, errorMessage, executionEngineRemoveModule);
		//			Ok(())
		//		}
		//		removeModule(self.reference, self.parent.reference);
		
		unsafe { LLVMDisposeExecutionEngine(self.reference) }
	}
}

impl ExecutionEngine
{
	#[inline(always)]
	pub fn globalValuePointer<T: Sized>(&self, staticName: &str) -> *mut T
	{
		let staticNameCString = CString::new(staticName).expect("Contains embedded ASCII NULs");
		let address = unsafe { LLVMGetGlobalValueAddress(self.reference, staticNameCString.as_ptr()) };
		if unlikely(address == 0)
		{
			null_mut()
		}
		else
		{
			unsafe { transmute(address) }
		}
	}
	
	#[inline(always)]
	fn voidFunctionAddress(&self, functionName: &str) -> u64
	{
		let functionNameCString = CString::new(functionName).expect("Contains embedded ASCII NULs");
		unsafe { LLVMGetFunctionAddress(self.reference, functionNameCString.as_ptr()) }
	}
	
	#[inline(always)]
	pub fn voidFunctionPointer(&self, functionName: &str) -> Option<extern "C" fn()>
	{
		let address = self.voidFunctionAddress(functionName);
		if unlikely(address == 0)
		{
			None
		}
		else
		{
			let functionPointer: extern "C" fn() = unsafe { transmute(address) };
			
			Some(functionPointer)
		}
	}
	
	#[inline(always)]
	pub fn executeVoidFunctionPointer(&self, functionName: &str) -> Option<()>
	{
		let address = self.voidFunctionAddress(functionName);
		if unlikely(address == 0)
		{
			None
		}
		else
		{
			let functionPointer: extern "C" fn() = unsafe { transmute(address) };
			
			Some(functionPointer())
		}
	}
	
	// llvm_sys::execution_engine::LLVMRunFunction - takes a number of arguments; call LLVMFindFunction() first to get a function LLVMValueRef
}
