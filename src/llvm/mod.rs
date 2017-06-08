// This file is part of predicator. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/predicator/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of predicator. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/predicator/master/COPYRIGHT.


include!("handle_boolean_and_error_message.rs");
include!("panic_on_false.rs");


use ::libc::c_char;
use ::libc::c_void;
use ::llvm_sys::analysis::LLVMVerifyModule;
use ::llvm_sys::analysis::LLVMVerifierFailureAction;
use ::llvm_sys::bit_reader::*;
use ::llvm_sys::execution_engine::LLVMCreateMCJITCompilerForModule;
use ::llvm_sys::execution_engine::LLVMDisposeExecutionEngine;
use ::llvm_sys::execution_engine::LLVMExecutionEngineRef;
use ::llvm_sys::execution_engine::LLVMGetFunctionAddress;
use ::llvm_sys::execution_engine::LLVMGetGlobalValueAddress;
use ::llvm_sys::execution_engine::LLVMInitializeMCJITCompilerOptions;
use ::llvm_sys::execution_engine::LLVMLinkInMCJIT;
use ::llvm_sys::execution_engine::LLVMMCJITCompilerOptions;
//use ::llvm_sys::execution_engine::LLVMRemoveModule as executionEngineRemoveModule;
use ::llvm_sys::core::*;
use ::llvm_sys::ir_reader::LLVMParseIRInContext;
use ::llvm_sys::orc::*;
use ::llvm_sys::prelude::*;
use ::llvm_sys::target::*;
use ::llvm_sys::target_machine::*;
use ::rust_extra::unlikely;
use ::std::ffi::CString;
use ::std::ffi::CStr;
use ::std::mem::uninitialized;
use ::std::mem::size_of;
use ::std::mem::transmute;
use ::std::mem::zeroed;
use ::std::ptr::null_mut;


include!("ExecutionEngine.rs");
include!("MemoryBuffer.rs");
include!("Module.rs");
include!("PerThreadContext.rs");


























pub struct Target
{
	reference: LLVMTargetRef,
	triple: *const c_char,
}

impl Target
{
	#[inline(always)]
	fn defaultTargetTriple() -> *const c_char
	{
		unsafe { LLVMGetDefaultTargetTriple() }
	}
	
	#[inline(always)]
	pub fn createForDefault() -> Result<Self, String>
	{
		let targetTriple = Self::defaultTargetTriple();
		
		let mut targetReference = unsafe { uninitialized() };
		
		let mut errorMessage = null_mut();
		let boolean = unsafe { LLVMGetTargetFromTriple(targetTriple, &mut targetReference, &mut errorMessage) };
		handle_boolean_and_error_message!(boolean, errorMessage, LLVMGetTargetFromTriple);
		
		Ok
		(
			Self
			{
				reference: targetReference,
				triple: targetTriple,
			}
		)
	}
	
	#[inline(always)]
	pub fn name<'a>(&'a self) -> &'a CStr
	{
		unsafe { CStr::from_ptr(LLVMGetTargetName(self.reference)) }
	}
	
	#[inline(always)]
	pub fn description<'a>(&'a self) -> &'a CStr
	{
		unsafe { CStr::from_ptr(LLVMGetTargetDescription(self.reference)) }
	}
	
	#[inline(always)]
	pub fn hasJit(&self) -> bool
	{
		(unsafe { LLVMTargetHasJIT(self.reference) }) != 0
	}
	
	#[inline(always)]
	pub fn hasAssemblerBackend(&self) -> bool
	{
		(unsafe { LLVMTargetHasAsmBackend(self.reference) }) != 0
	}
	
	#[inline(always)]
	pub fn hasTargetMachine(&self) -> bool
	{
		(unsafe { LLVMTargetHasTargetMachine(self.reference) }) != 0
	}
	
	#[inline(always)]
	pub fn createTargetMachine<'a>(&'a self, cpu: *const c_char, features: *const c_char, level: LLVMCodeGenOptLevel, relocationMode: LLVMRelocMode, codeModel: LLVMCodeModel) -> TargetMachine<'a>
	{
		// LLVMDisposeTargetMachine
		TargetMachine
		{
			reference: unsafe { LLVMCreateTargetMachine(self.reference, self.triple, cpu, features, level, relocationMode, codeModel) },
			parent: self,
		}
	}
}

pub struct TargetMachine<'a>
{
	reference: LLVMTargetMachineRef,
	parent: &'a Target,
}

impl<'a> Drop for TargetMachine<'a>
{
	#[inline(always)]
	fn drop(&mut self)
	{
		if !self.reference.is_null()
		{
			unsafe { LLVMDisposeTargetMachine(self.reference) }
		}
	}
}

impl<'a> TargetMachine<'a>
{
	#[inline(always)]
	pub fn toOrcJitStack(mut self) -> OrcJitStack<'a>
	{
		// orcJitStackReference takes internal ownership of self.reference
		let orcJitStackReference = unsafe { LLVMOrcCreateInstance(self.reference) };
		self.reference = null_mut();
		
		OrcJitStack
		{
			reference: orcJitStackReference,
			parent: self.parent,
		}
	}
}

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
