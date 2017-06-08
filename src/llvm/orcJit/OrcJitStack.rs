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
	pub fn eagerlyAddIrCode<'b, 'c>(&'c self, module: &'c Module<'b>, symbolResolver: LLVMOrcSymbolResolverFn, symbolResolverContext: *mut c_void) -> ModuleAndOrcJitStack<'a, 'b, 'c>
	{
		ModuleAndOrcJitStack
		{
			reference: unsafe { LLVMOrcAddEagerlyCompiledIR(self.reference, module.reference, symbolResolver, symbolResolverContext) },
			parent: self,
			parent2: module,
		}
	}
}
