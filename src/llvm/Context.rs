// This file is part of predicator. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/predicator/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of predicator. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/predicator/master/COPYRIGHT.


#[derive(Clone)]
pub struct Context
{
	reference: LLVMContextRef,
	dropWrapper: Rc<ContextDropWrapper>,
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
	
	/// TODO: Implement From for MemoryBuffer
	#[inline(always)]
	pub fn createModuleFromIntermediateRepresentationBuffer<'a>(self, buffer: &'a [u8]) -> Result<Module, String>
	{
		let memoryBuffer = MemoryBuffer::fromSlice(buffer);
		let module = self.parseTextualIntermediateRepresentationIntoModule(&memoryBuffer)?;
		module.verify()
	}
	
	/// TODO: Implement From for MemoryBuffer
	#[inline(always)]
	pub fn createModuleFromIntermediateRepresentationFile(self, path: &str) -> Result<Module, String>
	{
		let memoryBuffer = MemoryBuffer::fromFile(path)?;
		let module = self.parseTextualIntermediateRepresentationIntoModule(&memoryBuffer)?;
		module.verify()
	}
	
	/// TODO: Implement From for MemoryBuffer
	#[inline(always)]
	pub fn createModuleFromBitCodeBuffer<'a>(self, buffer: &'a [u8]) -> Result<Module, String>
	{
		let memoryBuffer = MemoryBuffer::fromSlice(buffer);
		let module = self.parseBitCodeIntoModule(&memoryBuffer)?;
		module.verify()
	}
	
	/// TODO: Implement From for MemoryBuffer
	#[inline(always)]
	pub fn createModuleFromBitCodeFile(self, path: &str) -> Result<Module, String>
	{
		let memoryBuffer = MemoryBuffer::fromFile(path)?;
		let module = self.parseBitCodeIntoModule(&memoryBuffer)?;
		module.verify()
	}
	
	#[inline(always)]
	pub fn createModule(self, name: String) -> Result<Module, String>
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
					parentDropWrapper: self.dropWrapper,
				}
			)
		}
	}
	
	#[inline(always)]
	pub fn loadBitCodeIntoModule<'a>(self, memoryBuffer: &MemoryBuffer<'a>) -> Result<Module, String>
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
					parentDropWrapper: self.dropWrapper,
				}
			)
		}
	}
	
	#[inline(always)]
	pub fn parseBitCodeIntoModule<'a>(self, memoryBuffer: &MemoryBuffer<'a>) -> Result<Module, String>
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
					parentDropWrapper: self.dropWrapper,
				}
			)
		}
	}
	
	#[inline(always)]
	pub fn parseTextualIntermediateRepresentationIntoModule<'a>(self, memoryBuffer: &MemoryBuffer<'a>) -> Result<Module, String>
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
				parentDropWrapper: self.dropWrapper,
			}
		)
	}
}
