// This file is part of predicator. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/predicator/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of predicator. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/predicator/master/COPYRIGHT.


include!("handle_boolean_and_error_message.rs");
include!("panic_on_false.rs");


use self::ir::*;
use self::ir::attributes::*;
use self::ir::attributes::enums::*;
use self::ir::constants::*;
use self::ir::globalFields::*;
use self::ir::metadata::*;
use self::ir::typeBasedAliasAnalysis::*;
use self::ir::types::*;
use self::ir::values::*;
use self::machineCodeJit::*;
use self::orcJit::*;
use self::targets::*;
use ::libc::c_char;
use ::libc::c_uint;
use ::libc::c_void;
use ::llvm_sys::*;
use ::llvm_sys::analysis::*;
use ::llvm_sys::bit_reader::*;
use ::llvm_sys::bit_writer::*;
use ::llvm_sys::core::*;
use ::llvm_sys::execution_engine::LLVMCreateMCJITCompilerForModule;
use ::llvm_sys::execution_engine::LLVMInitializeMCJITCompilerOptions;
use ::llvm_sys::execution_engine::LLVMLinkInMCJIT;
use ::llvm_sys::execution_engine::LLVMMCJITCompilerOptions;
use ::llvm_sys::ir_reader::LLVMParseIRInContext;
use ::llvm_sys::object::*;
use ::llvm_sys::orc::*;
use ::llvm_sys::prelude::*;
use ::llvm_sys::target::*;
use ::llvm_sys::target_machine::*;
use ::rust_extra::unlikely;
use ::std::cell::RefCell;
use ::std::collections::HashMap;
use ::std::ffi::CStr;
use ::std::ffi::CString;
use ::std::io;
use ::std::io::Write;
use ::std::mem::uninitialized;
use ::std::mem::size_of;
use ::std::mem::zeroed;
use ::std::ptr::null;
use ::std::ptr::null_mut;
use ::std::rc::Rc;
use ::std::slice::from_raw_parts;


#[macro_use] pub mod ir;
pub mod machineCodeJit;
pub mod orcJit;
pub mod targets;


include!("Context.rs");
include!("ContextDropWrapper.rs");
include!("JitContext.rs");
include!("MemoryBuffer.rs");
include!("MemoryBufferCreator.rs");
include!("Module.rs");
include!("ModuleDropWrapper.rs");
include!("ModuleSourceCodeType.rs");
include!("SuperContext.rs");
include!("SymbolResolver.rs");
