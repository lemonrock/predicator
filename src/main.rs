// This file is part of predicator. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/predicator/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of predicator. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/predicator/master/COPYRIGHT.


extern crate predicator;


use ::predicator::llvm::*;


fn main()
{
	let plugin_bit_code_file_path = compile_sample_plugin();
	
	initialiseOnceOnMainThread();
	
	// There needs to be at least one context per thread
	let jit_context = JitContext::new(NaiveSymbolResolver(0)).expect("Could not create a new JIT context");
	
	// Can also be created from a slice, and from intermediate representation (.ll files)
	let plugins = jit_context.loadPlugins(ModuleSourceCodeType::BitCode, &MemoryBufferCreator::File(&plugin_bit_code_file_path)).expect("Could not load plugin");
	
	// Note that there is no way to know the correct arity or arguments for the function pointer
	let sample_plugin_function_pointer = plugins.nullaryFunctionPointer::<*const i8>("sample_plugin").expect("Missing function for sample_plugin");
	
	// Execute the function
	let result = unsafe { sample_plugin_function_pointer() };
	
	eprintln!("{:?}", unsafe { ::std::ffi::CStr::from_ptr(result) });
	
	// Note that once `plugins` is dropped the function pointer is no longer valid
	
}


use ::std::path::PathBuf;
use ::std::process::Command;

fn compile_sample_plugin() -> String
{
	let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
	path.push("src");
	path.push("sample.plugin.rs");
	let plugin_source_file_path = path.into_os_string().into_string().unwrap();
	
	let crate_name = "sample_plugin";
	let random_extra = "10b7f3d0ab6b0d9f";
	
	Command::new("rustc")
		.arg("--crate-name").arg(crate_name)
		.arg(plugin_source_file_path)
		.arg("--crate-type").arg("bin")
		.arg("--emit").arg("llvm-bc")
		.arg("-C").arg("opt-level=3")
		.arg("-C").arg("panic=abort")
		.arg("-C").arg("lto")
		.arg("-C").arg("relocation-model=static")
		.arg("-C").arg(format!("metadata={}", random_extra))
		.arg("-C").arg(format!("extra-filename=-{}", random_extra))
		.arg("--out-dir").arg(env!("OUT_DIR"))
		.status().expect("Failed");
	
	let mut plugin_bit_code_file_path = PathBuf::from(env!("OUT_DIR"));
	plugin_bit_code_file_path.push(format!("{}-{}.bc", crate_name, random_extra));
	plugin_bit_code_file_path.into_os_string().into_string().unwrap()
}

// rustc --crate-name experiment_with_ffi src/lib.rs --crate-type bin --emit=llvm-bc -C opt-level=3 -C panic=abort -C lto -C relocation-model=static -C metadata=10b7f3d0ab6b0d9f -C extra-filename=-10b7f3d0ab6b0d9f --out-dir MY/out/dir --target x86_64-unknown-linux-musl -C ar=x86_64-linux-musl-ar -C linker=x86_64-linux-musl-cc
