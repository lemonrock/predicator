// This file is part of predicator. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/predicator/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of predicator. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/predicator/master/COPYRIGHT.


#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(C)]
pub enum UsefulLLVMLinkage
{
	LLVMExternalLinkage = 0,
	LLVMAvailableExternallyLinkage = 1,
	LLVMLinkOnceAnyLinkage = 2,
	LLVMLinkOnceODRLinkage = 3,
	LLVMLinkOnceODRAutoHideLinkage = 4,
	LLVMWeakAnyLinkage = 5,
	LLVMWeakODRLinkage = 6,
	LLVMAppendingLinkage = 7,
	LLVMInternalLinkage = 8,
	LLVMPrivateLinkage = 9,
	LLVMDLLImportLinkage = 10,
	LLVMDLLExportLinkage = 11,
	LLVMExternalWeakLinkage = 12,
	LLVMGhostLinkage = 13,
	LLVMCommonLinkage = 14,
	LLVMLinkerPrivateLinkage = 15,
	LLVMLinkerPrivateWeakLinkage = 16,
}

impl UsefulLLVMLinkage
{
	#[inline(always)]
	pub fn to_LLVMLinkage(&self) -> LLVMLinkage
	{
		unsafe { transmute(*self) }
	}
}
