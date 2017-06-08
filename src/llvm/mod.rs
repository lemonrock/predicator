// This file is part of predicator. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/predicator/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of predicator. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/predicator/master/COPYRIGHT.


include!("handle_boolean_and_error_message.rs");
include!("panic_on_false.rs");


use self::machineCodeJit::ExecutionEngine;
use ::libc::c_char;
use ::llvm_sys::analysis::LLVMVerifyModule;
use ::llvm_sys::analysis::LLVMVerifierFailureAction;
use ::llvm_sys::bit_reader::*;
use ::llvm_sys::execution_engine::LLVMCreateMCJITCompilerForModule;
use ::llvm_sys::execution_engine::LLVMInitializeMCJITCompilerOptions;
use ::llvm_sys::execution_engine::LLVMLinkInMCJIT;
use ::llvm_sys::execution_engine::LLVMMCJITCompilerOptions;
use ::llvm_sys::object::*;
//use ::llvm_sys::execution_engine::LLVMRemoveModule as executionEngineRemoveModule;
use ::llvm_sys::core::*;
use ::llvm_sys::ir_reader::LLVMParseIRInContext;
use ::llvm_sys::orc::*;
use ::llvm_sys::prelude::*;
use ::llvm_sys::target::*;
use ::llvm_sys::target_machine::*;
use ::std::ffi::CString;
use ::std::mem::uninitialized;
use ::std::mem::size_of;
use ::std::mem::zeroed;
use ::std::ptr::null_mut;


pub mod machineCodeJit;
pub mod orcJit;


include!("MemoryBuffer.rs");
include!("Module.rs");
include!("PerThreadContext.rs");
