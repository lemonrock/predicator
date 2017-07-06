// This file is part of mqtt. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/mqtt/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of mqtt. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/mqtt/master/COPYRIGHT.


use super::*;
use ::std::fmt;
use ::std::fmt::Debug;
use ::std::fmt::Formatter;


include!("CallValue.rs");
include!("ComparisonResultValue.rs");
include!("ConstantValue.rs");
include!("FunctionValue.rs");
include!("FunctionParameterValue.rs");
include!("GlobalValue.rs");
include!("LLVMValueRefWrapper.rs");
include!("MetadataNodeValue.rs");
include!("MetadataStringValue.rs");
include!("PhiInstructionValue.rs");
include!("PointerValue.rs");
include!("TerminatorValue.rs");
include!("TypeBasedAliasAnalysisNodeValue.rs");
include!("Value.rs");
