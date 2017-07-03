# predicator

[predicator] is a rust crate that lets you write plug ins that are can be loaded and unloaded and optimized at runtime, in Rust and any other language that LLVM JIT supports. It's ideal for filters, security checks and short-lived snippets of code that live for 10s to 100s of seconds in long-lived processes.

It is thread safe, but plugins are limited to `#[no_std]` crates at this time. A longer term plan is to add support for hosting cargo, and experimenting with symbol resolvers.

Statically-linked programs will work, but if they use a third-party library (even the libc), then they'll need to be specially compiled to pull in those definitions.


## Impatient to get getting?

Look at `src/main.rs`. This is all you need to create and load a plug in. Creation can be in a separate process, or even on a separate machine.

Demonstrate it works on your machine with `cargo run`. You'll need Rust nightly. Tested on Mac OS X as of 9th June 2017.


## Writing a Plug In


### Setting up `Cargo.toml`

The values in `Cargo.toml` need to be carefully set in order to generate a suitable plug-in.


#### Firstly, make sure the `[profile]` section contains these two entries exactly:-

```toml
[profile.dev]
opt-level = 3
debug = true
rpath = false
lto = true
debug-assertions = true
codegen-units = 1
panic = 'abort'

[profile.release]
opt-level = 3
debug = false
rpath = false
lto = true
debug-assertions = false
codegen-units = 1
panic = 'abort'
```

The settings of `lto = true` is particularly important. The setting of `panic = abort` isn't strictly necessary, but, as there is no way to pass panics in a LLVM JIT plugin back to Rust, makes development a little easier, as panics will die safely during development and debug testing.


#### Secondly, add or replace the `[lib]` section so it matches this exactly:-

```toml
crate-type = ["bin"]
```


### Writing the code for a LLVM JIT plugin

A LLVM JIT plugin needs some boiler plate code in order to work. Put this code at the *top* of your `lib.rs`:-

```rust
#![no_std]
#![feature(lang_items)]
#![feature(libc)]
#![no_main]


extern crate libc;


#[lang = "panic_fmt"]
fn panic_fmt() -> !
{
    loop
	{
	}
}

#[lang = "eh_personality"]
extern fn eh_personality()
{
}

#[allow(unused_variables)]
#[cfg(not(debug_assertions))]
#[no_mangle]
pub extern fn main(argc: isize, argv: *const *const u8) -> isize
{
    0
}

#[cfg(debug_assertions)]
#[no_mangle]
pub extern fn main(argc: isize, argv: *const *const u8) -> isize
{
    test(argc, argv)
}

/// Change this code to run unit tests when compiled in `debug` mode
/// Return a non-zero value to mark a test failure
fn test(argc: isize, argv: *const *const u8) -> isize
{
	0
}
```

This code does the following:-

* Tells the Rust compiler to use just the `core` crate (also known as `libcore`)
	* This means anything requiring (heap) memory allocation isn't available
		* No collections (eg `Vec`, `HashSet`, etc)
		* No `Box`, `Rc` or `Arc`
		* No strings (`String`, `str`, `CString`, `CStr`, `OsString` and `OsStr`)
		* No paths (`Path`, `PathBuf`)
		* And other similar restrictions
	* Also, that code that panics is not particularly useful
* Tells the Rust compiler that we will supply language item functions missing from `core`
* Tells the Rust compiler that we will use its toolchain's version of the `libc` crate (not strictly necessary in advanced scenarios)
* Tells the Rust compiler not to generate the usual logic to support Rust's `main()` function, as we don't need them for a plugin
* Uses the `libc` crate; without it, the plugin won't compile as it relies on the libc start files like `crt1.o`
* Adds a definition of `panic_fmt()` which endlessly loops; this isn't ideal (a better solution is probablty to write to stderr or syslog)
* Adds a piece of code to support exeption handling which does nothing
* Defines a `main()` method suitable for calling from libc, which does nothing when used in release mode. This is to make it possible to compile the code.
* Defines a `main()` method that handles bridging to C and then forwards to a `test()` method
* Defines a stub `test()` function, which you can replace with logic to test your code

Now you're reading to go. Just write normal functions and code. To make a function usable, it will need to be `pub` and `#[no_mangle]`.

For example, a really simple function might be:-

```rust
#[no_mangle]
pub fn simple_plugin()
{
}
```

This could then be used as a LLVM JIT plugin by looking up the function `simple_plugin` as:-

```rust
extern crate predicator;


use ::predicator::llvm::*;


fn main()
{
	// Create a super context
	let super_context = SuperContext::default();
	
	// There needs to be at least one context per thread
	let jit_context = super_context.newJitContext(NaiveSymbolResolver(0)).expect("Could not create a new JIT context");
	
	// Can also be created from a slice, and from intermediate representation (.ll files)
	let plugins = jit_context.loadPlugins(ModuleSourceCodeType::BitCode, &MemoryBufferCreator::File("/path/to/bitcode/file.bc")).expect("Could not parse bit code into module");
	
	// Note that there is no way to know the correct arity or arguments for the function pointer
	let simple_plugin_function_pointer = plugins.nullaryFunctionPointer::<()>("simple_plugin").expect("Missing function for simple_plugin");
	
	// Execute the function
	unsafe { simple_plugin_function_pointer() };
	
	// Note that once `plugins` is dropped the function pointer is no longer valid
}
```

Functions can also make use of anything defined in the `core` crate. Be wary of using anything in the `libc` crate, as LLVM JIT plugins can be used with statically-linked code and so libc functions and global statics may not have been linked in to the program using your plugin. This caveat does not apply when running the plug in directly to test it.

Function can take arguments and return results, eg this plugin:-

```rust
pub fn binary_plugin(size: usize, some_array: [u8; 2]) -> u32
{
	17
}
```

Can then be used like this:-

```rust
	let binary_plugin_function_pointer = orcJitStack.binaryFunctionPointer::<u32, usize, [u8; 64]>("binary_plugin").expect("Missing function for binary_plugin");
	
	// Execute the function
	let size = 20;
	let some_array = [3, 9];
	let result = unsafe { binary_plugin_function_pointer(size, some_array) };
	assert!(result == 17, "result wasn't 17");
```

Be aware that you are crossing the equivalent of a 'C' boundary. The predicator framework can not prevent you from not catching panic!, passing `Box` values, mismatching types, etc. It is recommended that you stick to very simple structures and primitives. More complexity will work, but it is extremely hard to debug when something breaks. There is nothing to stop you using types from other `no_std` crates, but you should avoid importing global (or thread local) statics from them. Defining global statics and thread local statics in the plugin code should be fine, however.


### Building

To build the LLVM JIT plugin, use:-

```bash
cargo rustc --release --target x86_64-unknown-linux-musl -- --emit=llvm-bc
```

_NOTE: We need to check if `-C relocation-model=static` is needed_

_NOTE: The switch `-C lto` should not actually be needed if your `Cargo.toml` is correctly set up._

You can find the output inside wherever `cargo` has defined the target folder. If you've forked this crate, you'll find it at `.cargo/target/x86_64-unknown-linux-musl/release/deps/NAME_OF_CRATE-HEX_RANDOM_VALUE.bc` where `NAME_OF_CRATE` is the name of your crate and `RANDOM_VALUE` is the value of `-C metadata=RANDOM_VALUE -C extra-filename=-RANDOM_VALUE cargo passes as part of the build. An example path might be `.cargo/target/x86_64-unknown-linux-musl/release/deps/experiment_with_ffi-805d16cbb3e10aad.bc`. When cargo builds it produces build output that contains a hint to this path:-

```bash
   Compiling experiment-with-ffi v0.0.0 (file:///Volumes/Source/GitHub/lemonrock/experiment-with-ffi)
     Running `rustc --crate-name experiment_with_ffi src/lib.rs --crate-type bin --emit=dep-info,link -C opt-level=3 -C panic=abort -C lto --emit=llvm-bc -C metadata=24221fe0742db2e8 -C extra-filename=-24221fe0742db2e8 --out-dir /Volumes/Source/GitHub/lemonrock/experiment-with-ffi/.cargo/target/x86_64-unknown-linux-musl/release/deps --target x86_64-unknown-linux-musl -C ar=x86_64-linux-musl-ar -C linker=x86_64-linux-musl-cc -L dependency=/Volumes/Source/GitHub/lemonrock/experiment-with-ffi/.cargo/target/x86_64-unknown-linux-musl/release/deps -L dependency=/Volumes/Source/GitHub/lemonrock/experiment-with-ffi/.cargo/target/release/deps`
    Finished release [optimized] target(s) in 0.25 secs
```

In this scenario, `NAME_OF_CRATE` is `experiment-with-ffi` and `RANDOM_VALUE` is `24221fe0742db2e8`, seen above as `-C metadata=24221fe0742db2e8 -C extra-filename=-24221fe0742db2e8`

#### Alternative build approaches


##### Debug mode

To build in debug mode use:-

```bash
cargo rustc --target x86_64-unknown-linux-musl -- --emit=llvm-bc
```


##### To check the LLVM IR code the plugin is created from

To check the generated LLVM IR, use:-

```bash
cargo rustc --release --target x86_64-unknown-linux-musl -- --emit=llvm-ir
```

This code is *not* built in debug mode, as it becomes very difficult to read. If you want to build it in debug mode, omit the `--release` switch above.


##### Stripping Release Mode Code

By default, the LLVM IR and bit code generated by Rust's compiler contains some additional debug information even when generated with `--release`. This can be removed using the `opt` program.

To removeit from IR code (`.ll` files), do the following:-

```bash
opt -strip-debug -S -o .cargo/target/x86_64-unknown-linux-musl/release/deps/NAME_OF_CRATE-RANDOM_VALUE.stripped.ll .cargo/target/x86_64-unknown-linux-musl/release/deps/NAME_OF_CRATE-RANDOM_VALUE.ll
```

and from bit code (`.bc` files):-

```bash
opt -strip-debug -o .cargo/target/x86_64-unknown-linux-musl/release/deps/NAME_OF_CRATE-RANDOM_VALUE.stripped.bc .cargo/target/x86_64-unknown-linux-musl/release/deps/NAME_OF_CRATE-RANDOM_VALUE.bc
```

Note the absence of the `-S` switch.

*Interestingly, the generated `.ll` code can sometimes be smaller than the `.bc` code.* I've observed as much as 24%.


## Thanks

With thanks to the tutorial at <https://github.com/jauhien/iron-kaleidoscope#chapter-3-optimizer-and-jit-support>


## Licensing

The license for this project is AFGPL-3.0.

[predicator]: https://github.com/lemonrock/predicator "predicator GitHub page"
