// This file is part of predicator. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/predicator/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of predicator. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/predicator/master/COPYRIGHT.


use super::*;
use ::llvm_sys::*;
use ::rust_extra::powersOfTwo::AsU32;
use ::rust_extra::powersOfTwo::PowerOfTwoThirtyTwoBit;
use ::std::cmp::Eq;
use ::std::hash::Hash;
use ::std::mem::transmute;
use ::std::ptr::null;
use ::std::collections::BTreeMap;
use ::std::collections::HashMap;
use ::std::collections::HashSet;
use ::std::cell::RefCell;


include!("AnyConstant.rs");
include!("Attribute.rs");
include!("BasicBlockBuilder.rs");
include!("Builder.rs");
include!("Constant.rs");
include!("EnumAttributeIdentifier.rs");
include!("EnumAttributeIdentifierCache.rs");
include!("EnumAttributeName.rs");
include!("FloatConstant.rs");
include!("FunctionAttribute.rs");
include!("FunctionBuilder.rs");
include!("FunctionDeclaration.rs");
include!("FunctionParameter.rs");
include!("Instruction.rs");
include!("IntegerConstant.rs");
include!("LlvmType.rs");
include!("LLVMTypeRefCache.rs");
include!("ParameterAttribute.rs");
include!("StructBody.rs");
include!("StructConstant.rs");
include!("SwitchInstruction.rs");
include!("TargetDependentFunctionAttribute.rs");
include!("TargetFeature.rs");
include!("ToggledTargetFeature.rs");
include!("UsefulCallingConvention.rs");
