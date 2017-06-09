// This file is part of predicator. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/predicator/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of predicator. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/predicator/master/COPYRIGHT.


#[derive(Clone)]
pub struct Context
{
	reference: LLVMContextRef,
	dropWrapper: Rc<ContextDropWrapper>,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum MemoryBufferCreator<'a>
{
	Buffer(&'a [u8]),
	
	/// We could use Path, but it is such a pain to get a *const c_char null terminated string from...
	File(&'a str),
}

impl<'a> MemoryBufferCreator<'a>
{
	// Potentially could be replaced by an implementation of the From trait
	#[inline(always)]
	pub fn createMemoryBuffer(&self) -> Result<MemoryBuffer<'a>, String>
	{
		use self::MemoryBufferCreator::*;
		
		match *self
		{
			Buffer(buffer) => Ok(MemoryBuffer::fromSlice(buffer)),
			File(filePath) => MemoryBuffer::fromFile(filePath),
		}
	}
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum ModuleSource
{
	IntermediateRepresentation,
	BitCode,
}

impl ModuleSource
{
	#[inline(always)]
	pub fn createVerifiedModule<'a>(&self, context: &Context, memoryBufferCreator: &MemoryBufferCreator<'a>) -> Result<Module, String>
	{
		use self::ModuleSource::*;
		
		let memoryBuffer = memoryBufferCreator.createMemoryBuffer()?;
		
		let module = match *self
		{
			IntermediateRepresentation => context.parseTextualIntermediateRepresentationIntoModule(&memoryBuffer),
			BitCode => context.parseBitCodeIntoModule(&memoryBuffer),
		}?;
		
		module.verify()
	}
}

impl Context
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
	pub fn new() -> Result<Self, ()>
	{
		let reference = unsafe { LLVMContextCreate() };
		if reference.is_null()
		{
			Err(())
		}
		else
		{
			Ok
			(
				Self
				{
					reference: reference,
					dropWrapper: Rc::new(ContextDropWrapper(reference))
				}
			)
		}
	}
	
	#[inline(always)]
	pub fn createModule(&self, name: String) -> Result<Module, String>
	{
		let cName = CString::new(name).expect("name contains embedded NULs");
		let reference = unsafe { LLVMModuleCreateWithNameInContext(cName.as_ptr(), self.reference) };
		if unlikely(reference.is_null())
		{
			Err(format!("Could not create a new module with name '{:?}'", cName))
		}
		else
		{
			Ok
			(
				Module
				{
					reference: reference,
					dropWrapper: Rc::new(ModuleDropWrapper(reference)),
					parentDropWrapper: self.dropWrapper.clone(),
				}
			)
		}
	}
	
	#[inline(always)]
	pub fn loadBitCodeIntoModule<'a>(&self, memoryBuffer: &MemoryBuffer<'a>) -> Result<Module, String>
	{
		let mut reference = unsafe { uninitialized() };
		
		let boolean = unsafe { LLVMGetBitcodeModuleInContext2(self.reference, memoryBuffer.reference, &mut reference) };
		if unlikely(boolean != 0)
		{
			Err("Could not load bit code into module".to_owned())
		}
		else
		{
			Ok
			(
				Module
				{
					reference: reference,
					dropWrapper: Rc::new(ModuleDropWrapper(reference)),
					parentDropWrapper: self.dropWrapper.clone(),
				}
			)
		}
	}
	
	#[inline(always)]
	pub fn parseBitCodeIntoModule<'a>(&self, memoryBuffer: &MemoryBuffer<'a>) -> Result<Module, String>
	{
		let mut reference = unsafe { uninitialized() };
		
		let boolean = unsafe { LLVMParseBitcodeInContext2(self.reference, memoryBuffer.reference, &mut reference) };
		if unlikely(boolean != 0)
		{
			Err("Could not parse bit code into module".to_owned())
		}
		else
		{
			Ok
			(
				Module
				{
					reference: reference,
					dropWrapper: Rc::new(ModuleDropWrapper(reference)),
					parentDropWrapper: self.dropWrapper.clone(),
				}
			)
		}
	}
	
	#[inline(always)]
	pub fn parseTextualIntermediateRepresentationIntoModule<'a>(&self, memoryBuffer: &MemoryBuffer<'a>) -> Result<Module, String>
	{
		let mut reference = unsafe { uninitialized() };
		
		let mut errorMessage = null_mut();
		let boolean = unsafe { LLVMParseIRInContext(self.reference, memoryBuffer.reference, &mut reference, &mut errorMessage) };
		handle_boolean_and_error_message!(boolean, errorMessage, LLVMParseIRInContext);
		
		Ok
		(
			Module
			{
				reference: reference,
				dropWrapper: Rc::new(ModuleDropWrapper(reference)),
				parentDropWrapper: self.dropWrapper.clone(),
			}
		)
	}
}
