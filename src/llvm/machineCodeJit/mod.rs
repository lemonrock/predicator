// This file is part of predicator. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/predicator/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of predicator. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/predicator/master/COPYRIGHT.


use super::*;
use ::llvm_sys::execution_engine::LLVMDisposeExecutionEngine;
use ::llvm_sys::execution_engine::LLVMExecutionEngineRef;
use ::llvm_sys::execution_engine::LLVMGetFunctionAddress;
use ::llvm_sys::execution_engine::LLVMGetGlobalValueAddress;
use ::rust_extra::unlikely;
use ::std::ffi::CString;
use ::std::mem::transmute;
use ::std::ptr::null_mut;


include!("ExecutionEngine.rs");
