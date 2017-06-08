// This file is part of predicator. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/predicator/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of predicator. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/predicator/master/COPYRIGHT.



use ::libc::c_char;
use ::libc::c_void;
use ::llvm_sys::analysis::LLVMVerifyModule;
use ::llvm_sys::analysis::LLVMVerifierFailureAction;
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


macro_rules! handle_boolean_and_error_message
{
	($boolean: ident, $errorMessage: ident, $functionName: ident) =>
	{
		{
			if $crate::rust_extra::unlikely(!$errorMessage.is_null())
			{
				if $crate::rust_extra::unlikely($boolean != 0)
				{
					let message = format!("{}:{:?}", stringify!($functionName), unsafe { ::std::ffi::CStr::from_ptr($errorMessage) });
					unsafe { $crate::llvm_sys::core::LLVMDisposeMessage($errorMessage) };
					return Err(message)
				}
				unsafe { $crate::llvm_sys::core::LLVMDisposeMessage($errorMessage) };
			}
			if $crate::rust_extra::unlikely($boolean != 0)
			{
				return Err(format!("{}:(unknown)", stringify!($functionName)))
			}
		}
	}
}


macro_rules! panic_on_false
{
	($boolean: ident, $functionName: ident) =>
	{
		{
			if $crate::rust_extra::unlikely($boolean != 0)
			{
				panic!("{}:(unknown)", stringify!($functionName));
			}
		}
	}
}

pub struct PerThreadContext
{
	reference: LLVMContextRef
}

impl Drop for PerThreadContext
{
	#[inline(always)]
	fn drop(&mut self)
	{
		unsafe { LLVMContextDispose(self.reference) }
	}
}

impl PerThreadContext
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
	
	#[inline(always)]
	pub fn new() -> Self
	{
		Self
		{
			reference: unsafe { LLVMContextCreate() }
		}
	}
	
	// See also LLVMGetBitcodeModuleInContext2 and  LLVMParseBitcodeInContext2
	// in http://www.llvm.org/docs/doxygen/html/group__LLVMCBitReader.html
	// Also: llvm_sys::ir_reader::LLVMParseIRInContext  http://rustdoc.taricorp.net/llvm-sys/llvm_sys/ir_reader/fn.LLVMParseIRInContext.html
	// - reads IR from a memory buffer
	
	#[inline(always)]
	pub fn createModule<'a>(&'a self, name: String) -> Module<'a>
	{
		let name = CString::new(name).expect("name contains embedded NULs");
		
		Module
		{
			reference: unsafe { LLVMModuleCreateWithNameInContext(name.as_ptr(), self.reference) },
			parent: self,
		}
	}
	
	#[inline(always)]
	pub fn parseModule<'a, 'b>(&'a self, memoryBuffer: &MemoryBuffer<'b>) -> Result<Module<'a>, String>
	{
		let mut moduleReference = unsafe { uninitialized() };
		
		let mut errorMessage = null_mut();
		let boolean = unsafe { LLVMParseIRInContext(self.reference, memoryBuffer.reference, &mut moduleReference, &mut errorMessage) };
		handle_boolean_and_error_message!(boolean, errorMessage, LLVMParseIRInContext);
		
		Ok
		(
			Module
			{
				reference: moduleReference,
				parent: self,
			}
		)
	}
}

pub struct MemoryBuffer<'a>
{
	reference: LLVMMemoryBufferRef,
	#[allow(dead_code)] slice: Option<&'a [u8]>,
}

impl<'a> Drop for MemoryBuffer<'a>
{
	#[inline(always)]
	fn drop(&mut self)
	{
		unsafe { LLVMDisposeMemoryBuffer(self.reference) }
	}
}

impl<'a> MemoryBuffer<'a>
{
	#[inline(always)]
	pub fn fromSlice(slice: &'a [u8]) -> Self
	{
		// "a\0"
		static BufferName: [i8; 2] = [65, 0];
		
		Self
		{
			reference: unsafe { LLVMCreateMemoryBufferWithMemoryRange(slice.as_ptr() as *const c_char, slice.len(), BufferName.as_ptr(), 1) },
			slice: Some(slice),
		}
	}
	
	#[inline(always)]
	pub fn fromFile(filePath: &str) -> Result<Self, String>
	{
		let filePath = CString::new(filePath).expect("File path contains embedded NULs");
		
		let mut reference = unsafe { uninitialized() };
		let mut errorMessage = null_mut();
		let boolean = unsafe { LLVMCreateMemoryBufferWithContentsOfFile(filePath.as_ptr(), &mut reference, &mut errorMessage) };
		handle_boolean_and_error_message!(boolean, errorMessage, LLVMParseIRInContext);
		
		Ok
		(
			Self
			{
				reference: reference,
				slice: None,
			}
		)
	}
}

pub struct Module<'a>
{
	reference: LLVMModuleRef,
	parent: &'a PerThreadContext,
}

impl<'a> Drop for Module<'a>
{
	#[inline(always)]
	fn drop(&mut self)
	{
		unsafe { LLVMDisposeModule(self.reference) }
	}
}

impl<'a> Clone for Module<'a>
{
	#[inline(always)]
	fn clone(&self) -> Self
	{
		Module
		{
			reference: unsafe { LLVMCloneModule(self.reference) },
			parent: self.parent,
		}
	}
}

impl<'a> Module<'a>
{
	// ParseIRFile
	
	#[inline(always)]
	fn verify(&self) -> Result<(), String>
	{
		let mut errorMessage = null_mut();
		let boolean = unsafe { LLVMVerifyModule(self.reference, LLVMVerifierFailureAction::LLVMReturnStatusAction, &mut errorMessage) };
		handle_boolean_and_error_message!(boolean, errorMessage, LLVMVerifyModule);
		Ok(())
	}

	#[inline(always)]
	pub fn executionEngineMachineCodeJit<'b>(&'b self) -> Result<ExecutionEngine<'a, 'b>, String>
	{
		self.verify()?;
		
		let sizeOfOptions = size_of::<LLVMMCJITCompilerOptions>();
		
		let mut options = unsafe { zeroed() };
		unsafe { LLVMInitializeMCJITCompilerOptions(&mut options, sizeOfOptions) };
		options.OptLevel = 3;
		options.CodeModel = LLVMCodeModel::LLVMCodeModelJITDefault;
		options.NoFramePointerElim = 0;
		options.EnableFastISel = 1;
		//options.MCJMM = ??? LLVMMCJITMemoryManagerRef
		
		let mut executionEngine = unsafe { uninitialized() };
		
		let mut errorMessage = null_mut();
		let boolean = unsafe { LLVMCreateMCJITCompilerForModule(&mut executionEngine, self.reference, &mut options, sizeOfOptions, &mut errorMessage) };
		handle_boolean_and_error_message!(boolean, errorMessage, LLVMCreateMCJITCompilerForModule);
		
		Ok
		(
			ExecutionEngine
			{
				reference: executionEngine,
				parent: self
			}
		)
	}
}

pub struct ExecutionEngine<'a, 'b>
where 'a: 'b
{
	reference: LLVMExecutionEngineRef,
	#[allow(dead_code)] parent: &'b Module<'a>,
}

impl<'a, 'b> Drop for ExecutionEngine<'a, 'b>
where 'a: 'b
{
	#[inline(always)]
	fn drop(&mut self)
	{
//		fn removeModule(executionEngineReference: LLVMExecutionEngineRef, moduleReference: LLVMModuleRef) -> Result<(), String>
//		{
//			let mut outReference = null_mut();
//			let mut errorMessage = null_mut();
//			let boolean = unsafe { executionEngineRemoveModule(executionEngineReference, moduleReference, &mut outReference, &mut errorMessage) };
//			handle_boolean_and_error_message!(boolean, errorMessage, executionEngineRemoveModule);
//			Ok(())
//		}
//		removeModule(self.reference, self.parent.reference);
		
		unsafe { LLVMDisposeExecutionEngine(self.reference) }
	}
}

impl<'a, 'b> ExecutionEngine<'a, 'b>
where 'a: 'b
{
	#[inline(always)]
	pub fn globalValuePointer<T: Sized>(&self, staticName: &str) -> *mut T
	{
		let staticNameCString = CString::new(staticName).expect("Contains embedded ASCII NULs");
		let address = unsafe { LLVMGetGlobalValueAddress(self.reference, staticNameCString.as_ptr()) };
		if unlikely(address == 0)
		{
			null_mut()
		}
		else
		{
			unsafe { transmute(address) }
		}
	}
	
	#[inline(always)]
	fn voidFunctionAddress(&self, functionName: &str) -> u64
	{
		let functionNameCString = CString::new(functionName).expect("Contains embedded ASCII NULs");
		unsafe { LLVMGetFunctionAddress(self.reference, functionNameCString.as_ptr()) }
	}
	
	#[inline(always)]
	pub fn voidFunctionPointer(&self, functionName: &str) -> Option<extern "C" fn()>
	{
		let address = self.voidFunctionAddress(functionName);
		if unlikely(address == 0)
		{
			None
		}
		else
		{
			let functionPointer: extern "C" fn() = unsafe { transmute(address) };
			
			Some(functionPointer)
		}
	}
	
	#[inline(always)]
	pub fn executeVoidFunctionPointer(&self, functionName: &str) -> Option<()>
	{
		let address = self.voidFunctionAddress(functionName);
		if unlikely(address == 0)
		{
			None
		}
		else
		{
			let functionPointer: extern "C" fn() = unsafe { transmute(address) };
			
			Some(functionPointer())
		}
	}
	
	// llvm_sys::execution_engine::LLVMRunFunction - takes a number of arguments; call LLVMFindFunction() first to get a function LLVMValueRef
}

























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
