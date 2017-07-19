// This file is part of predicator. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/predicator/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of predicator. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/predicator/master/COPYRIGHT.


#[derive(Clone)]
pub struct Module
{
	reference: LLVMModuleRef,
	dropWrapper: Rc<ModuleDropWrapper>,
	parentDropWrapper: Rc<ContextDropWrapper>,
}

impl Module
{
	#[inline(always)]
	pub fn useAsTemplateForNewModule(&self) -> Result<Self, String>
	{
		let reference = unsafe { LLVMCloneModule(self.reference) };
		if unlikely(reference.is_null())
		{
			Err("Could not clone".to_owned())
		}
		else
		{
			Ok
			(
				Self
				{
					reference: reference,
					dropWrapper: Rc::new(ModuleDropWrapper(reference)),
					parentDropWrapper: self.parentDropWrapper.clone(),
				}
			)
		}
	}
	
	#[inline(always)]
	pub fn verify(self) -> Result<Self, String>
	{
		self.verifyReference().map(|_| self)
	}
	
	#[inline(always)]
	pub fn verifyReference(&self) -> Result<(), String>
	{
		let mut errorMessage = null_mut();
		let boolean = unsafe { LLVMVerifyModule(self.reference, LLVMVerifierFailureAction::LLVMReturnStatusAction, &mut errorMessage) };
		handle_boolean_and_error_message!(boolean, errorMessage, LLVMVerifyModule);
		Ok(())
	}
	
	#[inline(always)]
	pub fn executionEngineMachineCodeJit(&self) -> Result<ExecutionEngine, String>
	{
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
				parentDropWrapper: self.dropWrapper.clone(),
			}
		)
	}
	
	#[inline(always)]
	pub fn addFunctionDefinition(&self, context: &Context, functionDefinition: &FunctionDefinition) -> FunctionValue
	{
		functionDefinition.create(context, self)
	}
	
	#[inline(always)]
	pub fn addFunctionDeclaration(&self, context: &Context, functionDeclaration: &FunctionDeclaration) -> FunctionValue
	{
		functionDeclaration.create(context, self)
	}
	
	#[inline(always)]
	pub fn addGlobalField(&self, context: &Context, globalFieldDefinition: &GlobalFieldDefinition) -> GlobalValue
	{
		globalFieldDefinition.create(context, self)
	}
	
	#[inline(always)]
	pub fn addMetadata(&self, context: &Context, key: &str, metadata: &MetadataNode)
	{
		let key = CString::new(key).unwrap();
		unsafe { LLVMAddNamedMetadataOperand(self.reference, key.as_ptr(), context.metadataNode(metadata).asLLVMValueRef()) }
	}
	
	#[inline(always)]
	pub fn dumpToStandardError(&self)
	{
		unsafe { LLVMDumpModule(self.reference) }
	}
	
	#[inline(always)]
	pub fn writeBitCodeToFile(&self, path: &CStr) -> Result<(), String>
	{
		let result = unsafe { LLVMWriteBitcodeToFile(self.reference, path.as_ptr()) };
		if result == 0
		{
			Ok(())
		}
		else
		{
			Err(format!("Unknown failure '{:?}' writing bit code to file path {:?}", result, path))
		}
	}
	
	#[inline(always)]
	pub fn writeBitCodeToMemoryBuffer<'a>(&self) -> MemoryBuffer<'a>
	{
		let reference = unsafe { LLVMWriteBitcodeToMemoryBuffer(self.reference) };
		MemoryBuffer::fromReference(reference)
	}
	
	#[inline(always)]
	pub fn addNamelessGlobal(&self, constantType: LLVMTypeRef) -> GlobalValue
	{
		GlobalValue::fromLLVMValueRef(unsafe { LLVMAddGlobal(self.reference, constantType, emptyName!()) })
	}
}
