// This file is part of predicator. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/predicator/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of predicator. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/predicator/master/COPYRIGHT.


pub struct JitContext<SR: SymbolResolver>
{
	reference: LLVMOrcJITStackRef,
	dropWrapper: Rc<OrcJitStackDropWrapper>,
	symbolResolver: SR,
}

impl<SR: SymbolResolver> JitContext<SR>
{
	pub fn new(symbolResolver: SR) -> Result<Self, String>
	{
		let reference = Target::createHostOrcJitStack()?;
		
		Ok
		(
			Self
			{
				reference: reference,
				dropWrapper: Rc::new(OrcJitStackDropWrapper(reference)),
				symbolResolver: symbolResolver,
			}
		)
	}
	
	pub fn loadPlugins(&self, moduleSourceCodeType: ModuleSourceCodeType, memoryBufferCreator: &MemoryBufferCreator, context: &Context) -> Result<ModuleInOrcJitStack, String>
	{
		let module = moduleSourceCodeType.createVerifiedModule(context, memoryBufferCreator)?;
		
		Ok(self.loadPluginFromModule(&module))
	}
	
	#[inline]
	pub fn loadPluginFromModule(&self, module: &Module) -> ModuleInOrcJitStack
	{
		let reference = unsafe { LLVMOrcAddEagerlyCompiledIR(self.reference, module.reference, Self::resolveSymbol, self.symbolResolver()) };
		ModuleInOrcJitStack
		{
			reference: reference,
			orcJitStackReference: self.reference,
			orcJitStackReferenceDropWrapper: self.dropWrapper.clone(),
		}
	}
	
	/// NOTE: The API for this doesn't appear in some versions of the documentation
	#[inline]
	pub fn loadPluginFromObjectFile(&self, objectFile: &ObjectFile) -> ModuleInOrcJitStack
	{
		let reference = unsafe { LLVMOrcAddObjectFile(self.reference, objectFile.reference, Self::resolveSymbol, self.symbolResolver()) };
		ModuleInOrcJitStack
		{
			reference: reference,
			orcJitStackReference: self.reference,
			orcJitStackReferenceDropWrapper: self.dropWrapper.clone(),
		}
	}
	
	extern "C" fn resolveSymbol(symbolName: *const c_char, lookupContext: *mut c_void) -> u64
	{
		unsafe { &*(lookupContext as *mut SR) }.resolveSymbolAddress(unsafe { CStr::from_ptr(symbolName) })
	}
	
	fn symbolResolver(&self) -> *mut c_void
	{
		&self.symbolResolver as *const _ as *mut _
	}
}

pub struct NaiveSymbolResolver(pub u64);

impl SymbolResolver for NaiveSymbolResolver
{
	fn resolveSymbolAddress<'a>(&self, symbolName: &'a CStr) -> u64
	{
		println!("Trying to resolve symbol: '{:?}", symbolName);
		
		SymbolNotFound
	}
}
