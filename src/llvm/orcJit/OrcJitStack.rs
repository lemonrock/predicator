// This file is part of predicator. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/predicator/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of predicator. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/predicator/master/COPYRIGHT.


#[derive(Clone)]
pub struct OrcJitStack
{
	reference: LLVMOrcJITStackRef,
	dropWrapper: Rc<OrcJitStackDropWrapper>,
}

impl OrcJitStack
{
	#[inline(always)]
	pub fn eagerlyAddModule(&self, module: &Module, symbolResolver: LLVMOrcSymbolResolverFn, symbolResolverContext: *mut c_void) -> ModuleInOrcJitStack
	{
		ModuleInOrcJitStack
		{
			reference: unsafe { LLVMOrcAddEagerlyCompiledIR(self.reference, module.reference, symbolResolver, symbolResolverContext) },
			orcJitStack: self.clone(),
		}
	}
	
	/// NOTE: The API for this doesn't appear in some versions of the documentation
	#[inline(always)]
	pub fn addObjectFile(&self, objectFile: &ObjectFile, symbolResolver: LLVMOrcSymbolResolverFn, symbolResolverContext: *mut c_void) -> ModuleInOrcJitStack
	{
		ModuleInOrcJitStack
		{
			reference: unsafe { LLVMOrcAddObjectFile(self.reference, objectFile.reference, symbolResolver, symbolResolverContext) },
			orcJitStack: self.clone(),
		}
	}
}
