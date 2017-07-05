// This file is part of mqtt. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/mqtt/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of mqtt. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/mqtt/master/COPYRIGHT.


pub struct SuperContext
{
	enumAttributeIdentifierCache: EnumAttributeIdentifierCache,
}

impl Default for SuperContext
{
	fn default() -> Self
	{
		Self::initialiseOnceOnMainThread();
		
		Self
		{
			enumAttributeIdentifierCache: EnumAttributeIdentifierCache::default(),
		}
	}
}

impl SuperContext
{
	#[inline(always)]
	fn initialiseOnceOnMainThread()
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
	pub fn newContext(&self) -> Result<Context, String>
	{
		Context::new(self.enumAttributeIdentifierCache.clone())
	}
	
	#[inline(always)]
	pub fn newJitContext<SR: SymbolResolver>(&self, symbolResolver: SR) -> Result<(JitContext<SR>, Context), String>
	{
		let context = self.newContext()?;
		JitContext::new(symbolResolver).map(|jitContext| (jitContext, context))
	}
}
