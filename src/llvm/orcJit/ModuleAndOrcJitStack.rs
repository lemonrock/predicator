// This file is part of predicator. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/predicator/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of predicator. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/predicator/master/COPYRIGHT.


pub struct ModuleAndOrcJitStack<'a, 'b, 'c>
	where 'a: 'c, 'b: 'c
{
	reference: LLVMOrcModuleHandle,
	parent: &'c OrcJitStack<'a>,
	#[allow(dead_code)] parent2: &'c Module<'b>,
}

impl<'a, 'b, 'c> Drop for ModuleAndOrcJitStack<'a, 'b, 'c>
	where 'a: 'c, 'b: 'c
{
	#[inline(always)]
	fn drop(&mut self)
	{
		unsafe { LLVMRemoveModule(self.parent.reference, self.reference) }
	}
}
