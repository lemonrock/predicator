// This file is part of predicator. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/predicator/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of predicator. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/predicator/master/COPYRIGHT.


#[derive(Clone)]
pub struct Context
{
	reference: LLVMContextRef,
	dropWrapper: Rc<ContextDropWrapper>,
	typeRefCache: RefCell<LLVMTypeRefCache>,
	functionAttributeCache: RefCell<HashMap<FunctionAttribute, LLVMAttributeRef>>,
	parameterAttributeCache: RefCell<HashMap<ParameterAttribute, LLVMAttributeRef>>,
	enumAttributeIdentifierCache: EnumAttributeIdentifierCache,
}

impl Context
{
	#[inline(always)]
	pub fn new(enumAttributeIdentifierCache: EnumAttributeIdentifierCache) -> Result<Self, String>
	{
		let reference = unsafe { LLVMContextCreate() };
		if reference.is_null()
		{
			Err("Could not create context".to_owned())
		}
		else
		{
			Ok
			(
				Self
				{
					reference: reference,
					dropWrapper: Rc::new(ContextDropWrapper(reference)),
					typeRefCache: RefCell::new(LLVMTypeRefCache::new()),
					functionAttributeCache: RefCell::new(HashMap::with_capacity(8)),
					parameterAttributeCache: RefCell::new(HashMap::with_capacity(8)),
					enumAttributeIdentifierCache: enumAttributeIdentifierCache,
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
	
	#[inline(always)]
	pub fn builder(&self) -> Builder
	{
		let reference = unsafe { LLVMCreateBuilderInContext(self.reference) };
		
		Builder
		{
			reference: reference,
			parentDropWrapper: self.dropWrapper.clone(),
		}
	}
	
	#[inline(always)]
	pub fn typeRef(&self, llvmType: &LlvmType) -> LLVMTypeRef
	{
		llvmType.to_LLVMTypeRef(self.reference, &mut self.typeRefCache.borrow_mut())
	}
	
	#[inline(always)]
	pub fn functionAttributeRef(&self, attribute: FunctionAttribute) -> LLVMAttributeRef
	{
		self.attributeRef(attribute, &mut self.functionAttributeCache.borrow_mut())
	}
	
	#[inline(always)]
	pub fn parameterAttributeRef(&self, attribute: ParameterAttribute) -> LLVMAttributeRef
	{
		self.attributeRef(attribute, &mut self.parameterAttributeCache.borrow_mut())
	}
	
	#[inline(always)]
	fn attributeRef<A: Attribute>(&self, attribute: A, cache: &mut HashMap<A, LLVMAttributeRef>) -> LLVMAttributeRef
	{
		if let Some(attribute) = cache.get(&attribute)
		{
			return *attribute;
		}
		
		let (enumAttributeName, value) = attribute.to_value();
		let identifier = self.enumAttributeIdentifierCache.identifier(enumAttributeName);
		let attributeRef = unsafe { LLVMCreateEnumAttribute(self.reference, identifier.0, value) };
		
		cache.insert(attribute, attributeRef);
		
		attributeRef
	}
}
