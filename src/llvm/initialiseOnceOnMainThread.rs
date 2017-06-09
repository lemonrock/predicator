// This file is part of predicator. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/predicator/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of predicator. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/predicator/master/COPYRIGHT.


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
