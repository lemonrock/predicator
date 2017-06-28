// This file is part of predicator. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/predicator/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of predicator. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/predicator/master/COPYRIGHT.


use super::*;
use ::llvm_sys::*;
use ::std::cmp::Eq;
use ::std::hash::Hash;
use ::std::collections::HashMap;
use ::std::collections::HashSet;
use ::std::cell::RefCell;


include!("Attribute.rs");
include!("BasicBlockBuilder.rs");
include!("Builder.rs");
include!("EnumAttributeIdentifier.rs");
include!("EnumAttributeIdentifierCache.rs");
include!("EnumAttributeName.rs");
include!("FunctionAttribute.rs");
include!("FunctionBuilder.rs");
include!("FunctionDeclaration.rs");
include!("FunctionParameter.rs");
include!("LlvmType.rs");
include!("LLVMTypeRefCache.rs");
include!("ParameterAttribute.rs");
include!("UsefulCallingConvention.rs");
