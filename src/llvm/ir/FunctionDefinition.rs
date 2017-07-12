// This file is part of predicator. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/predicator/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of predicator. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/predicator/master/COPYRIGHT.


pub struct FunctionDefinition
{
	name: CString,
	returns: FunctionParameter,
	parameters: Vec<(String, FunctionParameter)>,
	hasVarArgs: bool,
	functionAttributes: HashSet<FunctionAttribute>,
	targetDependentFunctionAttributes: HashSet<TargetDependentFunctionAttribute>,
	callingConvention: UsefulLLVMCallConv,
	garbageCollectorStrategy: Option<CString>,
	
	linkage: UsefulLLVMLinkage,
	visibility: UsefulLLVMVisibility,
	section: Option<String>,
	dllStorageClass: Option<UsefulLLVMDLLStorageClass>,
	hasUnnamedAddress: bool,
	alignment: Option<PowerOfTwoThirtyTwoBit>,
}

impl FunctionDefinition
{
	#[inline(always)]
	pub fn public(name: &str, returns: FunctionParameter, parameters: Vec<(String, FunctionParameter)>) -> Self
	{
		use self::FunctionAttribute::*;
		
		let functionAttributes = hashset!
		{
			norecurse,
			
			nounwind,
			uwtable,
			
			sspstrong,
		};
		
		use self::TargetFeature::*;
		use self::ToggledTargetFeature::*;
		let targetDependentFunctionAttributes = hashset!
		{
			TargetDependentFunctionAttribute::stack_protector_buffer_size(PowerOfTwoThirtyTwoBit::_8),
			
			TargetDependentFunctionAttribute::disable_tail_calls(false),
			TargetDependentFunctionAttribute::no_frame_pointer_elim(true),
			TargetDependentFunctionAttribute::no_frame_pointer_elim_non_leaf,
			TargetDependentFunctionAttribute::no_jump_tables(false),
			
			TargetDependentFunctionAttribute::correctly_rounded_divide_sqrt_fp_math(false),
			TargetDependentFunctionAttribute::less_precise_fpmad(false),
			TargetDependentFunctionAttribute::no_infs_fp_math(false),
			TargetDependentFunctionAttribute::no_nans_fp_math(false),
			TargetDependentFunctionAttribute::no_signed_zeros_fp_math(false),
			TargetDependentFunctionAttribute::no_trapping_math(false),
			TargetDependentFunctionAttribute::unsafe_fp_math(false),
			TargetDependentFunctionAttribute::use_soft_float(false),
			
			//TargetDependentFunctionAttribute::target_cpu(b"core2"),
			TargetDependentFunctionAttribute::target_features(vec![On(cx16), On(fxsr), On(cx16), On(sse), On(sse2), On(ssse3), On(x87)]),
		};
		
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
			
			linkage: UsefulLLVMLinkage::LLVMExternalLinkage,
			visibility: UsefulLLVMVisibility::LLVMDefaultVisibility,
			section: None,
			dllStorageClass: None,
			hasUnnamedAddress: false,
			alignment: None,
		}
	}
	
	#[inline(always)]
	pub fn private(name: &str, returns: FunctionParameter, parameters: Vec<(String, FunctionParameter)>, functionAttributes: HashSet<FunctionAttribute>, targetDependentFunctionAttributes: HashSet<TargetDependentFunctionAttribute>) -> Self
	{
		Self
		{
			name: CString::new(name).unwrap(),
			returns: returns,
			parameters: parameters,
			hasVarArgs: false,
			functionAttributes: functionAttributes,
			targetDependentFunctionAttributes: targetDependentFunctionAttributes,
			callingConvention: UsefulLLVMCallConv::LLVMFastCallConv,
			garbageCollectorStrategy: None,
			
			linkage: UsefulLLVMLinkage::LLVMLinkerPrivateLinkage,
			visibility: UsefulLLVMVisibility::LLVMDefaultVisibility,
			section: None,
			dllStorageClass: None,
			hasUnnamedAddress: false,
			alignment: None,
		}
	}
	
	pub(crate) fn create(&self, context: &Context, module: &Module) -> FunctionValue
	{
		let functionType = context.typeRef(&LlvmType::Function { returns: Box::new(self.returns.llvmType.clone()), parameters: self.parameters.iter().map(|ref functionParameter| functionParameter.1.llvmType.clone() ).collect(), hasVarArgs: self.hasVarArgs }).asLLVMTypeRef();
		let functionReference = unsafe { LLVMAddFunction(module.reference, self.name.as_ptr(), functionType) };
		
		let functionValue = FunctionValue::fromLLVMValueRef(functionReference);
		
		for attribute in self.returns.attributes.iter()
		{
			functionValue.setFunctionReturnsAttribute(context, attribute);
		}
		
		let mut parameterIndex = 1u32;
		for &(ref name, ref parameter) in self.parameters.iter()
		{
			let parameterValue = functionValue.parameterAt(parameterIndex as usize - 1).unwrap();
			
			parameterValue.setName(&name[..]);
			
			if let Some(alignment) = parameter.alignment
			{
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
		
		functionValue.setUnnamedAddress(self.hasUnnamedAddress);
		
		functionValue.setSection(&self.section);
		
		//functionValue.setPersonalityFunctionReference(self.personalityFunctionReference);
		
		functionValue
	}
}
