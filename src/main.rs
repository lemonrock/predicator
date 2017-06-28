// This file is part of predicator. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/predicator/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of predicator. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/predicator/master/COPYRIGHT.


extern crate predicator;


use ::predicator::compiler::*;
use ::predicator::llvm::*;
use ::predicator::llvm::ir::*;


fn main()
{
	// Compile plugin
	let mut rust_plugin_compiler = RustPluginCompiler::new(TemporaryFolderPath::TempDir);
	let plugin_bit_code_file_path = rust_plugin_compiler.example().expect("Did not compile plugin");
	
	// Initialise LLVM functionality
	initialiseOnceOnMainThread();
	
	// There needs to be at least one context per thread
	let jit_context = JitContext::new(NaiveSymbolResolver(0), EnumAttributeIdentifierCache::default()).expect("Could not create a new JIT context");
	
	// Can also be created from a slice, and from intermediate representation (.ll files)
	let plugins = jit_context.loadPlugins(ModuleSourceCodeType::BitCode, &MemoryBufferCreator::File(&plugin_bit_code_file_path)).expect("Could not load plugin");
	
	// Note that there is no way to know the correct arity or arguments for the function pointer
	let sample_plugin_function_pointer = plugins.nullaryFunctionPointer::<*const i8>("sample_plugin").expect("Missing function for sample_plugin");
	
	// Execute the function
	let result = unsafe { sample_plugin_function_pointer() };
	
	// Prove the plugin was run
	assert!(::std::ffi::CString::new("Hello, world from sample_plugin!").unwrap() == unsafe { ::std::ffi::CStr::from_ptr(result) }.to_owned());
	
	// Note that once `plugins` is dropped the function pointer is no longer valid
}
