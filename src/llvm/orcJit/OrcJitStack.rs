// This file is part of predicator. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/predicator/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of predicator. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/predicator/master/COPYRIGHT.


pub struct OrcJitStack<'a>
{
	reference: LLVMOrcJITStackRef,
	#[allow(dead_code)] parent: &'a Target,
}

impl<'a> Drop for OrcJitStack<'a>
{
	#[inline(always)]
	fn drop(&mut self)
	{
		unsafe { LLVMOrcDisposeInstance(self.reference) }
	}
}

impl<'a> OrcJitStack<'a>
{
	#[inline(always)]
	pub fn eagerlyAddIrCode<'b>(&'b self, module: &Module, symbolResolver: LLVMOrcSymbolResolverFn, symbolResolverContext: *mut c_void) -> ModuleInOrcJitStack<'a, 'b>
	{
		ModuleInOrcJitStack
		{
			reference: unsafe { LLVMOrcAddEagerlyCompiledIR(self.reference, module.reference, symbolResolver, symbolResolverContext) },
			parent: self,
		}
	}
	
	/// NOTE: The API for this doesn't appear in some versions of the documentation
	#[inline(always)]
	pub fn addObjectFile<'b>(&'b self, objectFile: &ObjectFile, symbolResolver: LLVMOrcSymbolResolverFn, symbolResolverContext: *mut c_void) -> ModuleInOrcJitStack<'a, 'b>
	{
		ModuleInOrcJitStack
		{
			reference: unsafe { LLVMOrcAddObjectFile(self.reference, objectFile.reference, symbolResolver, symbolResolverContext) },
			parent: self,
		}
	}
	
	#[inline(always)]
	pub fn globalValuePointer<T: Sized>(&self, staticName: &str) -> *mut T
	{
		let address = self.getSymbolAddress(staticName);
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
	pub fn voidFunctionPointer(&self, functionName: &str) -> Option<extern "C" fn()>
	{
		let address = self.getSymbolAddress(functionName);
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
	
	/// 0 is not found
	#[inline(always)]
	fn getSymbolAddress(&self, symbolName: &str) -> LLVMOrcTargetAddress
	{
		let symbolNameCString = CString::new(symbolName).expect("Contains embedded NULs");
		
		unsafe { LLVMOrcGetSymbolAddress(self.reference, symbolNameCString.as_ptr()) }
	}
}
