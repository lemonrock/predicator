// This file is part of predicator. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/predicator/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of predicator. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/predicator/master/COPYRIGHT.


pub struct FunctionDeclaration
{
	name: CString,
	returns: FunctionParameter,
	parameters: Vec<FunctionParameter>,
	hasVarArgs: bool,
	functionAttributes: HashSet<FunctionAttribute>,
	targetDependentFunctionAttributes: HashSet<TargetDependentFunctionAttribute>,
	callingConvention: UsefulLLVMCallConv,
	garbageCollectorStrategy: Option<CString>,
	
	linkage: UsefulLLVMLinkage,
	visibility: UsefulLLVMVisibility,
	dllStorageClass: Option<UsefulLLVMDLLStorageClass>,
	unnamedAddress: Option<UnnamedAddressAttribute>,
	alignment: Option<PowerOfTwoThirtyTwoBit>,
}

impl FunctionDeclaration
{
	#[inline(always)]
	pub fn intrinsic_llvm_memcpy_p0i8_p0i8_i64() -> Self
	{
		Self
		{
			name: CString::new("llvm.memcpy.p0i8.p0i8.i64").unwrap(),
			returns: FunctionParameter::void(),
			parameters: vec!
			[
				// i8* nocapture writeonly, i8* nocapture readonly, i64, i32, i1
				FunctionParameter::pointer(&LlvmType::Int8, hashset!
				{
					ParameterAttribute::nocapture,
					ParameterAttribute::writeonly,
				}),
				FunctionParameter::pointer(&LlvmType::Int8, hashset!
				{
					ParameterAttribute::nocapture,
					ParameterAttribute::readonly,
				}),
				FunctionParameter::simple(LlvmType::Int64),
				FunctionParameter::simple(LlvmType::Int32),
				FunctionParameter::boolean(),
			],
			hasVarArgs: false,
			functionAttributes: hashset!
			{
				FunctionAttribute::argmemonly,
				FunctionAttribute::nounwind
			},
			targetDependentFunctionAttributes: hashset!{},
			callingConvention: UsefulLLVMCallConv::LLVMCCallConv,
			garbageCollectorStrategy: None,
			
			linkage: UsefulLLVMLinkage::LLVMExternalLinkage,
			visibility: UsefulLLVMVisibility::LLVMDefaultVisibility,
			dllStorageClass: None,
			unnamedAddress: None,
			alignment: None,
		}
	}
	
	#[inline(always)]
	pub fn intrinsic(name: &str, returns: FunctionParameter, parameters: Vec<FunctionParameter>, functionAttributes: HashSet<FunctionAttribute>, targetDependentFunctionAttributes: HashSet<TargetDependentFunctionAttribute>) -> Self
	{
		Self
		{
			name: CString::new(name).unwrap(),
			returns: returns,
			parameters: parameters,
			hasVarArgs: false,
			functionAttributes: functionAttributes,
			targetDependentFunctionAttributes: targetDependentFunctionAttributes,
			callingConvention: UsefulLLVMCallConv::LLVMCCallConv,
			garbageCollectorStrategy: None,
			// prefix: ?,
			// prologue: ?,
			
			linkage: UsefulLLVMLinkage::LLVMExternalLinkage,
			visibility: UsefulLLVMVisibility::LLVMDefaultVisibility,
			dllStorageClass: None,
			unnamedAddress: None,
			alignment: None,
		}
	}
	
	pub(crate) fn create(&self, context: &Context, module: &Module) -> FunctionValue
	{
		let functionType = context.typeRef(&LlvmType::Function { returns: Box::new(self.returns.llvmType.clone()), parameters: self.parameters.iter().map(|ref functionParameter| functionParameter.llvmType.clone() ).collect(), hasVarArgs: self.hasVarArgs }).asLLVMTypeRef();
		let functionValue = FunctionValue::fromLLVMValueRef(unsafe { LLVMAddFunction(module.reference, self.name.as_ptr(), functionType) });
		
		for attribute in self.returns.attributes.iter()
		{
			functionValue.setFunctionReturnsAttribute(context, attribute);
		}
		
		let mut parameterIndex = 1u32;
		for parameter in self.parameters.iter()
		{
			if let Some(alignment) = parameter.alignment
			{
				let parameterValue = functionValue.parameterAt(parameterIndex as usize - 1).unwrap();
				parameterValue.setAlignment(alignment);
			}
			
			for attribute in parameter.attributes.iter()
			{
				functionValue.setFunctionParameterAttribute(context, parameterIndex, attribute);
			}
			
			parameterIndex += 1;
		}
		
		for attribute in self.functionAttributes.iter()
		{
			functionValue.setFunctionAttribute(context, attribute);
		}
		
		for attribute in self.targetDependentFunctionAttributes.iter()
		{
			attribute.addToFunction(context, functionValue);
		}
		
		functionValue.setCallingConvention(self.callingConvention);
		functionValue.setGarbageCollectorStrategy(&self.garbageCollectorStrategy);
		functionValue.setLinkage(self.linkage);
		functionValue.setVisibility(self.visibility);
		functionValue.setDllStorageClass(self.dllStorageClass);
		functionValue.setAlignment(self.alignment);
		
		if let Some(unnamedAddress) = self.unnamedAddress
		{
			use self::UnnamedAddressAttribute::*;
			
			match unnamedAddress
			{
				UnnamedAddress => functionValue.setUnnamedAddress(true),
				ModuleWideLocalUnnamedAddress => panic!("local unnamed address setting is not supported at this time for function declarations (we do not know which API call to use)"),
			}
		}
		else
		{
			functionValue.setUnnamedAddress(false)
		}
		
		functionValue
	}
}
