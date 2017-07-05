// This file is part of predicator. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/predicator/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of predicator. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/predicator/master/COPYRIGHT.


use super::*;
use self::useful::*;
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


pub mod attributes;
pub mod constants;
pub mod globalFields;
pub mod metadata;
pub mod typeBasedAliasAnalysis;
pub mod types;
pub mod useful;
pub mod values;


include!("BasicBlockBuilder.rs");
include!("Builder.rs");
include!("BuilderSwitchInstruction.rs");
include!("BuilderTailCall.rs");
include!("CallParameter.rs");
include!("FunctionDeclaration.rs");
include!("FunctionDefinition.rs");
include!("FunctionParameter.rs");
include!("LlvmType.rs");
include!("ModuleDefinition.rs");
include!("ToLLVMValueRefWrapper.rs");
include!("ToReference.rs");
include!("UnnamedAddressAttribute.rs");
