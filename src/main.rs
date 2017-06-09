// This file is part of predicator. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/predicator/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of predicator. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/predicator/master/COPYRIGHT.


extern crate predicator;


use ::predicator::llvm::*;


fn main()
{
	initialiseOnceOnMainThread();
	
	// There needs to be at least one context per thread
	let jit_context = JitContext::new(NaiveSymbolResolver(0)).expect("Could not create a new JIT context");
	
	// Can also be created from a slice, and from intermediate representation (.ll files)
	let plugins = jit_context.loadPlugins(ModuleSourceCodeType::BitCode, &MemoryBufferCreator::File("/path/to/bitcode/file.bc")).expect("Could not parse bit code into module");
	
	// Note that there is no way to know the correct arity or arguments for the function pointer
	let simple_plugin_function_pointer = plugins.nullaryFunctionPointer::<()>("simple_plugin").expect("Missing function for simple_plugin");
	
	// Execute the function
	unsafe { simple_plugin_function_pointer() };
	
	// Note that once `plugins` is dropped the function pointer is no longer valid
}
