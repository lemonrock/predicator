// This file is part of mqtt. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/mqtt/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of mqtt. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/mqtt/master/COPYRIGHT.


#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct TargetMachineDataLayout
{
	reference: LLVMTargetDataRef,
}

impl Drop for TargetMachineDataLayout
{
	#[inline(always)]
	fn drop(&mut self)
	{
		unsafe { LLVMDisposeTargetData(self.reference) }
	}
}

impl TargetMachineDataLayout
{
	#[inline(always)]
	pub fn setOnModule(&self, moduleReference: LLVMModuleRef)
	{
		unsafe { LLVMSetModuleDataLayout(moduleReference, self.reference) }
	}
}
