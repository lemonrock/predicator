// This file is part of predicator. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/predicator/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of predicator. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/predicator/master/COPYRIGHT.


#[derive(Clone)]
pub struct Context
{
	reference: LLVMContextRef,
	dropWrapper: Rc<ContextDropWrapper>,
	enumAttributeIdentifierCache: EnumAttributeIdentifierCache,
	typeRefCache: ContextCache<LlvmType, LLVMTypeRefWrapper>,
	constantCache: ContextCache<Constant, ConstantValue>,
	callParameterAttributeCache: ContextCache<CallParameterAttribute, LLVMAttributeRef>,
	functionAttributeCache: ContextCache<FunctionAttribute, LLVMAttributeRef>,
	parameterAttributeCache: ContextCache<ParameterAttribute, LLVMAttributeRef>,
	metadataStringCache: RefCell<HashMap<String, MetadataStringValue>>,
	metadataNodeCache: ContextCache<MetadataNode, MetadataNodeValue>,
}

#[derive(Clone)]
struct ContextCache<Key: ToReference<Value> + Clone, Value: Copy>
{
	cache: RefCell<HashMap<Key, Value>>
}

impl<Key: ToReference<Value> + Clone, Value: Copy> Default for ContextCache<Key, Value>
{
	fn default() -> Self
	{
		Self
		{
			cache: RefCell::new(HashMap::with_capacity(16)),
		}
	}
}

impl<Key: ToReference<Value> + Clone, Value: Copy> ContextCache<Key, Value>
{
	#[inline(always)]
	pub fn getOrAdd(&self, key: &Key, context: &Context) -> Value
	{
		{
			let cache = &mut self.cache.borrow();
			if let Some(value) = cache.get(key)
			{
				return *value;
			}
		}
		
		let value = key.toReference(context);
		
		let cache = &mut self.cache.borrow_mut();
		
		cache.insert(key.clone(), value);
		
		value
	}
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
				enumAttributeIdentifierCache: enumAttributeIdentifierCache,
				typeRefCache: ContextCache::default(),
				constantCache: ContextCache::default(),
				callParameterAttributeCache: ContextCache::default(),
				functionAttributeCache: ContextCache::default(),
				parameterAttributeCache: ContextCache::default(),
				metadataStringCache: RefCell::new(HashMap::new()),
				metadataNodeCache: ContextCache::default(),
			};
			
			Ok(this)
		}
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
	
	#[inline(always)]
	pub fn enumAttribute(&self, enumAttributeName: EnumAttributeName, value: u64) -> LLVMAttributeRef
	{
		let identifier = self.enumAttributeIdentifierCache.identifier(enumAttributeName);
		let reference = unsafe { LLVMCreateEnumAttribute(self.reference, identifier.0, value) };
		debug_assert!(!reference.is_null(), "Enum Attribute Name '{:?}' is null", enumAttributeName);
		reference
	}
	
	#[inline(always)]
	pub fn typeRef(&self, llvmType: &LlvmType) -> LLVMTypeRefWrapper
	{
		self.typeRefCache.getOrAdd(llvmType, self)
	}
	
	#[inline(always)]
	pub fn constant(&self, constant: &Constant) -> ConstantValue
	{
		self.constantCache.getOrAdd(constant, self)
	}
	
	pub fn metadataString(&self, string: &str) -> MetadataStringValue
	{
		{
			let cache = self.metadataStringCache.borrow();
			if let Some(value) = cache.get(string)
			{
				return *value;
			}
		}
		
		let owned = string.to_owned();
		
		let value =
		{
			let bytes = owned.as_bytes();
			MetadataStringValue::fromLLVMValueRef(unsafe { LLVMMDStringInContext(self.reference, bytes.as_ptr() as *const _, bytes.len() as u32) })
		};
		
		let mut cache = self.metadataStringCache.borrow_mut();
		cache.insert(owned, value);
		
		value
	}
	
	#[inline(always)]
	pub fn typeBasedAliasAnalysisNode(&self, typeBasedAliasAnalysisNode: &TypeBasedAliasAnalysisNode) -> TypeBasedAliasAnalysisNodeValue
	{
		self.metadataNode(&typeBasedAliasAnalysisNode.toMetadataNode()).toTypeBasedAliasAnalysisNodeValue()
	}
	
	#[inline(always)]
	pub fn metadataNode(&self, metadataNode: &MetadataNode) -> MetadataNodeValue
	{
		self.metadataNodeCache.getOrAdd(metadataNode, self)
	}
	
	#[inline(always)]
	pub fn callParameterAttributeRef(&self, attribute: &CallParameterAttribute) -> LLVMAttributeRef
	{
		self.callParameterAttributeCache.getOrAdd(attribute, self)
	}
	
	#[inline(always)]
	pub fn functionAttributeRef(&self, attribute: &FunctionAttribute) -> LLVMAttributeRef
	{
		self.functionAttributeCache.getOrAdd(attribute, self)
	}
	
	// LLVMAddTargetDependentFunctionAttr is deprecated
	#[inline(always)]
	pub(crate) fn LLVMAddTargetDependentFunctionAttr(&self, functionValue: FunctionValue, name: &[u8], value: Option<&[u8]>)
	{
		let attributeRef = self.stringAttribute(name, value);
		functionValue.setAttribute(LLVMAttributeFunctionIndex, attributeRef)
	}
	
	#[inline(always)]
	pub fn parameterAttributeRef(&self, attribute: &ParameterAttribute) -> LLVMAttributeRef
	{
		self.parameterAttributeCache.getOrAdd(attribute, self)
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
	
	pub fn createModule(&self, name: &str, identifier: &str, targetTriple: &CStr, targetMachineDataLayout: &TargetMachineDataLayout, inlineAssembler: Option<&str>) -> Result<Module, String>
	{
		let cName = CString::new(name).expect("name contains embedded NULs");
		let reference = unsafe { LLVMModuleCreateWithNameInContext(cName.as_ptr(), self.reference) };
		if unlikely(reference.is_null())
		{
			Err(format!("Could not create a new module with name '{:?}'", cName))
		}
		else
		{
			let identifierBytes = identifier.as_bytes();
			unsafe { LLVMSetModuleIdentifier(reference, identifierBytes.as_ptr() as *const _, identifierBytes.len() as usize) };
			
			unsafe { LLVMSetTarget(reference, targetTriple.as_ptr()) };
			
			targetMachineDataLayout.setOnModule(reference);
			
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
	pub(crate) fn builder(&self) -> LLVMBuilderRef
	{
		unsafe { LLVMCreateBuilderInContext(self.reference) }
	}
}
