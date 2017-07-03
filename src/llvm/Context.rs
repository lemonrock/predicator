// This file is part of predicator. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/predicator/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of predicator. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/predicator/master/COPYRIGHT.


#[derive(Clone)]
pub struct Context
{
	reference: LLVMContextRef,
	dropWrapper: Rc<ContextDropWrapper>,
	typeRefCache: RefCell<HashMap<LlvmType, LLVMTypeRefWrapper>>,
	constantCache: RefCell<HashMap<Constant, ConstantValue>>,
	functionAttributeCache: RefCell<HashMap<FunctionAttribute, LLVMAttributeRef>>,
	parameterAttributeCache: RefCell<HashMap<ParameterAttribute, LLVMAttributeRef>>,
	enumAttributeIdentifierCache: EnumAttributeIdentifierCache,
	metaDataStringsCache: RefCell<HashMap<String, MetadataStringValue>>,
	typeBasedAliasAnalysisNodeCache: RefCell<HashMap<TypeBasedAliasAnalysisNode, TypeBasedAliasAnalysisNodeValue>>,
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
			let this = Self
			{
				reference: reference,
				dropWrapper: Rc::new(ContextDropWrapper(reference)),
				typeRefCache: RefCell::new(HashMap::new()),
				constantCache: RefCell::new(HashMap::new()),
				functionAttributeCache: RefCell::new(HashMap::with_capacity(8)),
				parameterAttributeCache: RefCell::new(HashMap::with_capacity(8)),
				enumAttributeIdentifierCache: enumAttributeIdentifierCache,
				metaDataStringsCache: RefCell::new(HashMap::new()),
				typeBasedAliasAnalysisNodeCache: RefCell::new(HashMap::new()),
			};
			
			Ok(this)
		}
	}
	
	#[inline(always)]
	pub fn typeRef(&self, llvmType: &LlvmType) -> LLVMTypeRefWrapper
	{
		let mut typeRefCache = &mut self.typeRefCache.borrow_mut();
		if let Some(extant) = typeRefCache.get(llvmType)
		{
			return *extant;
		}
		
		let value = llvmType.toLLVMTypeRefWrapper(self);
		
		typeRefCache.insert(llvmType.clone(), value);
		
		value
	}
	
	#[inline(always)]
	pub fn constant(&self, constant: &Constant) -> ConstantValue
	{
		let mut constantCache = &mut self.constantCache.borrow_mut();
		if let Some(extant) = constantCache.get(constant)
		{
			return *extant;
		}
		
		let value = constant.toConstantValue(self);
		
		constantCache.insert(constant.clone(), value);
		
		value
	}
	
	#[inline(always)]
	pub fn enumAttribute(&self, enumAttributeName: EnumAttributeName, value: u64) -> LLVMAttributeRef
	{
		let identifier = self.enumAttributeIdentifierCache.identifier(enumAttributeName);
		let reference = unsafe { LLVMCreateEnumAttribute(self.reference, identifier.0, value) };
		debug_assert!(!reference.is_null(), "Enum Attribute Name '{:?}' is null", enumAttributeName);
		reference
	}
	
	#[inline(always)]
	pub fn stringAttribute(&self, name: &[u8], value: Option<&[u8]>) -> LLVMAttributeRef
	{
		let reference = if let Some(value) = value
		{
			unsafe { LLVMCreateStringAttribute(self.reference, name.as_ptr() as *const _, name.len() as u32, value.as_ptr() as *const _, value.len() as u32) }
		}
		else
		{
			unsafe { LLVMCreateStringAttribute(self.reference, name.as_ptr() as *const _, name.len() as u32, null(), 0) }
		};
		debug_assert!(!reference.is_null(), "String Attribute is null");
		reference
	}
	
	pub fn metadataString(&self, string: &str) -> MetadataStringValue
	{
		let mut cache = self.metaDataStringsCache.borrow_mut();
		if let Some(value) = cache.get(string)
		{
			return *value;
		}
		
		let owned = string.to_owned();
		
		let value =
		{
			let bytes = owned.as_bytes();
			MetadataStringValue::fromLLVMValueRef(unsafe { LLVMMDStringInContext(self.reference, bytes.as_ptr() as *const _, bytes.len() as u32) })
		};
		
		cache.insert(owned, value);
		
		value
	}
	
	pub fn typeBasedAliasAnalysisNode(&self, typeBasedAliasAnalysisNode: &TypeBasedAliasAnalysisNode) -> TypeBasedAliasAnalysisNodeValue
	{
		let mut cache = self.typeBasedAliasAnalysisNodeCache.borrow_mut();
		if let Some(value) = cache.get(typeBasedAliasAnalysisNode)
		{
			return *value;
		}
		
		let value = typeBasedAliasAnalysisNode.toTypeBasedAliasAnalysisNodeValue(self);
		
		cache.insert(typeBasedAliasAnalysisNode.clone(), value);
		
		value
	}
	
	pub fn functionAttributeRef(&self, attribute: FunctionAttribute) -> LLVMAttributeRef
	{
		let cache = &mut self.functionAttributeCache.borrow_mut();
		
		if let Some(attribute) = cache.get(&attribute)
		{
			return *attribute;
		}
		
		let attributeRef = attribute.to_attributeRef(self);
		
		cache.insert(attribute, attributeRef);
		
		attributeRef
	}
	
	pub fn parameterAttributeRef(&self, attribute: ParameterAttribute) -> LLVMAttributeRef
	{
		let cache = &mut self.parameterAttributeCache.borrow_mut();
		
		if let Some(attribute) = cache.get(&attribute)
		{
			return *attribute;
		}
		
		let attributeRef = attribute.to_attributeRef(self);
		
		cache.insert(attribute, attributeRef);
		
		attributeRef
	}
	
	#[inline(always)]
	pub fn metadataKind_tbaa(&self) -> u32
	{
		self.metadataKind(b"tbaa")
	}
	
	#[inline(always)]
	pub fn metadataKind_tbaa_struct(&self) -> u32
	{
		self.metadataKind(b"tbaa.struct")
	}
	
	#[inline(always)]
	pub fn metadataKind(&self, name: &[u8]) -> u32
	{
		unsafe { LLVMGetMDKindIDInContext(self.reference, name.as_ptr() as *const _, name.len() as u32) }
	}
	
	#[inline(always)]
	pub fn createModule(&self, name: &str, dataLayout: &str, targetTriple: &str, inlineAssembler: Option<&str>) -> Result<Module, String>
	{
		let cName = CString::new(name).expect("name contains embedded NULs");
		let reference = unsafe { LLVMModuleCreateWithNameInContext(cName.as_ptr(), self.reference) };
		if unlikely(reference.is_null())
		{
			Err(format!("Could not create a new module with name '{:?}'", cName))
		}
		else
		{
			// It is believed that the identifier is the same as the name
			//let identifierBytes = identifier.as_bytes();
			//unsafe { LLVMSetModuleIdentifier(reference, identifierBytes.as_ptr() as *const _, identifierBytes.len() as usize) };
			
			let cDataLayout = CString::new(dataLayout).expect("dataLayout contains embedded NULs");
			unsafe { LLVMSetDataLayout(reference, cDataLayout.as_ptr()) };
			
			let cTargetTriple = CString::new(targetTriple).expect("targetTriple contains embedded NULs");
			unsafe { LLVMSetTarget(reference, cTargetTriple.as_ptr()) };
			
			if let Some(inlineAssembler) = inlineAssembler
			{
				let cInlineAssembler = CString::new(inlineAssembler).expect("inlineAssembler contains embedded NULs");
				unsafe { LLVMSetModuleInlineAsm(reference, cInlineAssembler.as_ptr()) };
			}
			
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
	pub fn builder<'a>(&'a self) -> Builder<'a>
	{
		let reference = unsafe { LLVMCreateBuilderInContext(self.reference) };
		
		Builder
		{
			reference: reference,
			context: self,
		}
	}
}
