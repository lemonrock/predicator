// This file is part of predicator. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/predicator/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of predicator. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/predicator/master/COPYRIGHT.


pub struct ModuleInOrcJitStack
{
	pub(crate) reference: LLVMOrcModuleHandle,
	pub(crate) orcJitStackReference: LLVMOrcJITStackRef,
	#[allow(dead_code)] pub(crate) orcJitStackReferenceDropWrapper: Rc<OrcJitStackDropWrapper>,
}

impl Drop for ModuleInOrcJitStack
{
	#[inline(always)]
	fn drop(&mut self)
	{
		unsafe { LLVMOrcRemoveModule(self.orcJitStackReference, self.reference) }
	}
}

macro_rules! function_pointer
{
	($this: ident, $functionName: ident, $functionType: ty) =>
	{
		{
			let address = $this.getSymbolAddress($functionName);
			if unlikely(address == 0)
			{
				None
			}
			else
			{
				let functionPointer: $functionType = unsafe { transmute(address as usize) };
				Some(functionPointer)
			}
		}
	}
}

extern "C"
{
	// Incorrectly exposed by llvm-sys crate as LLVMRemoveModule!
	pub fn LLVMOrcRemoveModule(JITStack: LLVMOrcJITStackRef, H: LLVMOrcModuleHandle);
}

impl ModuleInOrcJitStack
{
	#[inline(always)]
	pub fn globalValuePointerNullable<T: Sized>(&self, staticName: &str) -> *mut T
	{
		let address = self.getSymbolAddress(staticName);
		if unlikely(address == 0)
		{
			null_mut()
		}
		else
		{
			unsafe { transmute(address as usize) }
		}
	}
	
	#[inline(always)]
	pub fn globalValuePointerDefensive<T: Sized>(&self, staticName: &str) -> Option<&mut T>
	{
		let address = self.getSymbolAddress(staticName);
		if unlikely(address == 0)
		{
			None
		}
		else
		{
			Some(unsafe { transmute(address as usize) })
		}
	}
	
	#[inline(always)]
	pub fn nullaryFunctionPointer<R>(&self, functionName: &str) -> Option<unsafe extern "C" fn() -> R>
	{
		function_pointer!(self, functionName, unsafe extern "C" fn() -> R)
	}
	
	#[inline(always)]
	pub fn unaryFunctionPointer<R, A>(&self, functionName: &str) -> Option<unsafe extern "C" fn(A) -> R>
	{
		function_pointer!(self, functionName, unsafe extern "C" fn(a: A) -> R)
	}
	
	#[inline(always)]
	pub fn binaryFunctionPointer<R, A, B>(&self, functionName: &str) -> Option<unsafe extern "C" fn(A, B) -> R>
	{
		function_pointer!(self, functionName, unsafe extern "C" fn(a: A, b: B) -> R)
	}
	
	#[inline(always)]
	pub fn ternaryFunctionPointer<R, A, B, C>(&self, functionName: &str) -> Option<unsafe extern "C" fn(A, B, C) -> R>
	{
		function_pointer!(self, functionName, unsafe extern "C" fn(a: A, b: B, c: C) -> R)
	}
	
	#[inline(always)]
	pub fn quaternaryFunctionPointer<R, A, B, C, D>(&self, functionName: &str) -> Option<unsafe extern "C" fn(A, B, C, D) -> R>
	{
		function_pointer!(self, functionName, unsafe extern "C" fn(a: A, b: B, c: C, d: D) -> R)
	}
	
	#[inline(always)]
	pub fn quinaryFunctionPointer<R, A, B, C, D, E>(&self, functionName: &str) -> Option<unsafe extern "C" fn(A, B, C, D, E) -> R>
	{
		function_pointer!(self, functionName, unsafe extern "C" fn(a: A, b: B, c: C, d: D, e: E) -> R)
	}
	
	#[inline(always)]
	pub fn senaryFunctionPointer<R, A, B, C, D, E, F>(&self, functionName: &str) -> Option<unsafe extern "C" fn(A, B, C, D, E, F) -> R>
	{
		function_pointer!(self, functionName, unsafe extern "C" fn(a: A, b: B, c: C, d: D, e: E, f: F) -> R)
	}
	
	/// 0 is not found
	#[inline(always)]
	fn getSymbolAddress(&self, symbolName: &str) -> LLVMOrcTargetAddress
	{
		let symbolNameCString = CString::new(symbolName).expect("Contains embedded NULs");
		
		unsafe { LLVMOrcGetSymbolAddress(self.orcJitStackReference, symbolNameCString.as_ptr()) }
	}
}
