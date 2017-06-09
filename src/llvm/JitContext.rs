// This file is part of predicator. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/predicator/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of predicator. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/predicator/master/COPYRIGHT.


pub struct JitContext<SR: SymbolResolver>
{
	context: Context,
	orcJitStack: OrcJitStack,
	symbolResolver: SR,
}

impl<SR: SymbolResolver> JitContext<SR>
{
	#[inline(always)]
	pub fn initialiseOnceOnMainThread()
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
	
	pub fn new(symbolResolver: SR, cpu: *const c_char, features: *const c_char) -> Result<Self, String>
	{
		let context = Context::new()?;
		let orcJitStack = Target::createHostOrcJitStack(cpu, features)?;
		
		Ok
		(
			Self
			{
				context: context,
				orcJitStack: orcJitStack,
				symbolResolver: symbolResolver,
			}
		)
	}
	
	pub fn loadPlugins(&self, moduleSourceCodeType: ModuleSourceCodeType, memoryBufferCreator: &MemoryBufferCreator) -> Result<ModuleInOrcJitStack, String>
	{
		let moduleContainingPlugIn = moduleSourceCodeType.createVerifiedModule(&self.context, memoryBufferCreator)?;
		
		Ok(self.orcJitStack.eagerlyAddModule(&moduleContainingPlugIn, Self::resolveSymbol, &self.symbolResolver as *const _ as *mut _))
	}
	
	extern "C" fn resolveSymbol(symbolName: *const c_char, lookupContext: *mut c_void) -> u64
	{
		unsafe { &*(lookupContext as *mut SR) }.resolveSymbolAddress(unsafe { CStr::from_ptr(symbolName) })
	}
}

pub struct NaiveSymbolResolver(u64);

impl SymbolResolver for NaiveSymbolResolver
{
	fn resolveSymbolAddress<'a>(&self, symbolName: &'a CStr) -> u64
	{
		println!("Trying to resolve symbol: '{:?}", symbolName);
		
		SymbolNotFound
	}
}
