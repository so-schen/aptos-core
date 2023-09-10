 cargo test --package language-e2e-testsuite --lib --all-features -- tests::execution_strategies::test_execution_strategies --exact --nocapture 

  *  Executing task: cargo test --package language-e2e-testsuite --lib --all-features -- tests::execution_strategies::test_execution_strategies --exact --nocapture 

   Compiling rustix v0.36.11
   Compiling is-terminal v0.4.5
   Compiling anstream v0.3.2
   Compiling clap_builder v4.3.21
   Compiling clap v4.3.21
   Compiling move-compiler v0.0.1 (/home/gftea/repo/aptos-core/third_party/move/move-compiler)
   Compiling move-coverage v0.1.0 (/home/gftea/repo/aptos-core/third_party/move/tools/move-coverage)
   Compiling aptos-sdk-builder v0.2.0 (/home/gftea/repo/aptos-core/aptos-move/aptos-sdk-builder)
   Compiling aptos-block-partitioner v0.1.0 (/home/gftea/repo/aptos-core/execution/block-partitioner)
   Compiling inferno v0.11.15
   Compiling move-ir-compiler v0.1.0 (/home/gftea/repo/aptos-core/third_party/move/move-ir-compiler)
   Compiling move-disassembler v0.1.0 (/home/gftea/repo/aptos-core/third_party/move/tools/move-disassembler)
   Compiling move-model v0.1.0 (/home/gftea/repo/aptos-core/third_party/move/move-model)
   Compiling move-stackless-bytecode v0.1.0 (/home/gftea/repo/aptos-core/third_party/move/move-model/bytecode)
   Compiling move-docgen v0.1.0 (/home/gftea/repo/aptos-core/third_party/move/move-prover/move-docgen)
   Compiling move-abigen v0.1.0 (/home/gftea/repo/aptos-core/third_party/move/move-prover/move-abigen)
   Compiling move-prover-bytecode-pipeline v0.1.0 (/home/gftea/repo/aptos-core/third_party/move/move-prover/bytecode-pipeline)
   Compiling move-errmapgen v0.1.0 (/home/gftea/repo/aptos-core/third_party/move/move-prover/move-errmapgen)
   Compiling move-compiler-v2 v0.1.0 (/home/gftea/repo/aptos-core/third_party/move/move-compiler-v2)
   Compiling move-prover-boogie-backend v0.1.0 (/home/gftea/repo/aptos-core/third_party/move/move-prover/boogie-backend)
   Compiling move-package v0.1.0 (/home/gftea/repo/aptos-core/third_party/move/tools/move-package)
   Compiling move-prover v0.1.0 (/home/gftea/repo/aptos-core/third_party/move/move-prover)
   Compiling aptos-move-stdlib v0.1.1 (/home/gftea/repo/aptos-core/aptos-move/framework/move-stdlib)
   Compiling aptos-framework v0.1.0 (/home/gftea/repo/aptos-core/aptos-move/framework)
   Compiling aptos-vm v0.1.0 (/home/gftea/repo/aptos-core/aptos-move/aptos-vm)
   Compiling aptos-package-builder v0.1.0 (/home/gftea/repo/aptos-core/aptos-move/package-builder)
   Compiling aptos-gas-profiling v0.1.0 (/home/gftea/repo/aptos-core/aptos-move/aptos-gas-profiling)
warning: unused variable: `error_type`
  --> aptos-move/aptos-vm/src/testing.rs:11:42
   |
11 | ...jected_error(error_type: InjectedError) ->...
   |                 ^^^^^^^^^^ help: if this is intentional, prefix it with an underscore: `_error_type`
   |
   = note: `#[warn(unused_variables)]` on by default

   Compiling aptos-cached-packages v0.1.0 (/home/gftea/repo/aptos-core/aptos-move/framework/cached-packages)
warning: `aptos-vm` (lib) generated 1 warning (run `cargo fix --lib -p aptos-vm` to apply 1 suggestion)
   Compiling aptos-vm-genesis v0.1.0 (/home/gftea/repo/aptos-core/aptos-move/vm-genesis)
   Compiling aptos-writeset-generator v0.1.0 (/home/gftea/repo/aptos-core/aptos-move/writeset-transaction-generator)
   Compiling aptos-language-e2e-tests v0.1.0 (/home/gftea/repo/aptos-core/aptos-move/e2e-tests)
   Compiling language-e2e-testsuite v0.1.0 (/home/gftea/repo/aptos-core/aptos-move/e2e-testsuite)
    Finished test [unoptimized + debuginfo] target(s) in 1m 38s
warning: the following packages contain code that will be rejected by a future version of Rust: fs_extra v1.2.0, nom v5.1.2
note: to see what the problems were, use the option `--future-incompat-report`, or run `cargo report future-incompatibilities --id 5`
     Running unittests src/lib.rs (target/debug/deps/language_e2e_testsuite-18e5b05744408563)

running 1 test
===========================================================================
TESTING BASIC STRATEGY
===========================================================================
===========================================================================
TESTING RANDOM STRATEGY
===========================================================================
===========================================================================
TESTING GUIDED STRATEGY
===========================================================================
===========================================================================
TESTING COMPOSED STRATEGY 1
===========================================================================
===========================================================================
TESTING COMPOSED STRATEGY 2
===========================================================================
test tests::execution_strategies::test_execution_strategies ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 71 filtered out; finished in 6.42s

 *  Terminal will be reused by tasks, press any key to close it. 

 # Non 3rd-party packages
 - [ ] what inside vm-genesis?
 - [ ] diff genesis block vs. vm-genesis?
 - [ ] how gas-profiling tool work?
 
 ```
   Compiling aptos-sdk-builder v0.2.0 (/home/gftea/repo/aptos-core/aptos-move/aptos-sdk-builder)
   Compiling aptos-block-partitioner v0.1.0 (/home/gftea/repo/aptos-core/execution/block-partitioner)
   Compiling aptos-move-stdlib v0.1.1 (/home/gftea/repo/aptos-core/aptos-move/framework/move-stdlib)
   Compiling aptos-framework v0.1.0 (/home/gftea/repo/aptos-core/aptos-move/framework)
   Compiling aptos-vm v0.1.0 (/home/gftea/repo/aptos-core/aptos-move/aptos-vm)
   Compiling aptos-package-builder v0.1.0 (/home/gftea/repo/aptos-core/aptos-move/package-builder)
   Compiling aptos-gas-profiling v0.1.0 (/home/gftea/repo/aptos-core/aptos-move/aptos-gas-profiling)
   Compiling aptos-cached-packages v0.1.0 (/home/gftea/repo/aptos-core/aptos-move/framework/cached-packages)
  Compiling aptos-vm-genesis v0.1.0 (/home/gftea/repo/aptos-core/aptos-move/vm-genesis)
   Compiling aptos-writeset-generator v0.1.0 (/home/gftea/repo/aptos-core/aptos-move/writeset-transaction-generator)
   Compiling aptos-language-e2e-tests v0.1.0 (/home/gftea/repo/aptos-core/aptos-move/e2e-tests)
   Compiling language-e2e-testsuite v0.1.0 (/home/gftea/repo/aptos-core/aptos-move/e2e-testsuite)
```


cargo test --package language-e2e-testsuite --lib --all-features -- tests::genesis::execute_genesis_write_set --exact --nocapture 

     Running `CARGO=/home/gftea/.rustup/toolchains/1.71.1-aarch64-unknown-linux-gnu/bin/cargo CARGO_CRATE_NAME=language_e2e_testsuite CARGO_MANIFEST_DIR=/home/gftea/repo/aptos-core/aptos-move/e2e-testsuite CARGO_PKG_AUTHORS='Aptos Labs <opensource@aptoslabs.com>' CARGO_PKG_DESCRIPTION='Move language e2e tests' CARGO_PKG_HOMEPAGE='https://aptoslabs.com' CARGO_PKG_LICENSE=Apache-2.0 CARGO_PKG_LICENSE_FILE='' CARGO_PKG_NAME=language-e2e-testsuite CARGO_PKG_README='' CARGO_PKG_REPOSITORY='https://github.com/aptos-labs/aptos-core' CARGO_PKG_RUST_VERSION=1.71.1 CARGO_PKG_VERSION=0.1.0 CARGO_PKG_VERSION_MAJOR=0 CARGO_PKG_VERSION_MINOR=1 CARGO_PKG_VERSION_PATCH=0 CARGO_PKG_VERSION_PRE='' CARGO_PRIMARY_PACKAGE=1 LD_LIBRARY_PATH='/home/gftea/repo/aptos-core/target/debug/deps:/home/gftea/.rustup/toolchains/1.71.1-aarch64-unknown-linux-gnu/lib:/home/gftea/.rustup/toolchains/1.71.1-aarch64-unknown-linux-gnu/lib' /home/gftea/.rustup/toolchains/1.71.1-aarch64-unknown-linux-gnu/bin/rustc --crate-name language_e2e_testsuite --edition=2021 aptos-move/e2e-testsuite/src/lib.rs --error-format=json --json=diagnostic-rendered-ansi,artifacts,future-incompat --diagnostic-width=204 --emit=dep-info,link -C embed-bitcode=no -C debuginfo=2 --test --cfg 'feature="default"' -C metadata=18e5b05744408563 -C extra-filename=-18e5b05744408563 --out-dir /home/gftea/repo/aptos-core/target/debug/deps -C incremental=/home/gftea/repo/aptos-core/target/debug/incremental -L dependency=/home/gftea/repo/aptos-core/target/debug/deps --extern aptos_block_executor=/home/gftea/repo/aptos-core/target/debug/deps/libaptos_block_executor-c4631ca264c84019.rlib --extern aptos_cached_packages=/home/gftea/repo/aptos-core/target/debug/deps/libaptos_cached_packages-585704b40c782acf.rlib --extern aptos_crypto=/home/gftea/repo/aptos-core/target/debug/deps/libaptos_crypto-c8cdceb615b28c7b.rlib --extern aptos_framework=/home/gftea/repo/aptos-core/target/debug/deps/libaptos_framework-f780f6760efadb3e.rlib --extern aptos_gas_algebra=/home/gftea/repo/aptos-core/target/debug/deps/libaptos_gas_algebra-7563d0d48bc596ba.rlib --extern aptos_gas_meter=/home/gftea/repo/aptos-core/target/debug/deps/libaptos_gas_meter-d113a3b65a280045.rlib --extern aptos_gas_schedule=/home/gftea/repo/aptos-core/target/debug/deps/libaptos_gas_schedule-53d4424ec10ec8fb.rlib --extern aptos_keygen=/home/gftea/repo/aptos-core/target/debug/deps/libaptos_keygen-e49443cf307723e5.rlib --extern aptos_language_e2e_tests=/home/gftea/repo/aptos-core/target/debug/deps/libaptos_language_e2e_tests-8c8218ad519a5f3f.rlib --extern aptos_logger=/home/gftea/repo/aptos-core/target/debug/deps/libaptos_logger-6a592b778bf44a5c.rlib --extern aptos_memory_usage_tracker=/home/gftea/repo/aptos-core/target/debug/deps/libaptos_memory_usage_tracker-057b0a9452bde986.rlib --extern aptos_state_view=/home/gftea/repo/aptos-core/target/debug/deps/libaptos_state_view-30a6e5770d16fd86.rlib --extern aptos_types=/home/gftea/repo/aptos-core/target/debug/deps/libaptos_types-e76eda65813714a1.rlib --extern aptos_vm=/home/gftea/repo/aptos-core/target/debug/deps/libaptos_vm-7ea774dc9bc33821.rlib --extern aptos_vm_genesis=/home/gftea/repo/aptos-core/target/debug/deps/libaptos_vm_genesis-a3a69e5e784c3e08.rlib --extern aptos_vm_logging=/home/gftea/repo/aptos-core/target/debug/deps/libaptos_vm_logging-2fc2ae7fb51e3384.rlib --extern aptos_vm_types=/home/gftea/repo/aptos-core/target/debug/deps/libaptos_vm_types-b68d81a7561e9ebf.rlib --extern aptos_writeset_generator=/home/gftea/repo/aptos-core/target/debug/deps/libaptos_writeset_generator-a13df1bafe8dcd39.rlib --extern bcs=/home/gftea/repo/aptos-core/target/debug/deps/libbcs-d7ac2f637d994b9d.rlib --extern fail=/home/gftea/repo/aptos-core/target/debug/deps/libfail-cdc1d5c85a2eabb3.rlib --extern itertools=/home/gftea/repo/aptos-core/target/debug/deps/libitertools-5256bcee77716f6c.rlib --extern move_binary_format=/home/gftea/repo/aptos-core/target/debug/deps/libmove_binary_format-85961aff90fe5b4a.rlib --extern move_bytecode_verifier=/home/gftea/repo/aptos-core/target/debug/deps/libmove_bytecode_verifier-db5381110f7dbb97.rlib --extern move_core_types=/home/gftea/repo/aptos-core/target/debug/deps/libmove_core_types-da59be05025b4b2b.rlib --extern move_ir_compiler=/home/gftea/repo/aptos-core/target/debug/deps/libmove_ir_compiler-d5aebc1708ce84b7.rlib --extern proptest=/home/gftea/repo/aptos-core/target/debug/deps/libproptest-c1a2ae976cd911e2.rlib --extern serde=/home/gftea/repo/aptos-core/target/debug/deps/libserde-b4c57aefa97faa98.rlib --cfg tokio_unstable -C force-frame-pointers=yes -C force-unwind-tables=yes -L native=/home/gftea/repo/aptos-core/target/debug/build/blst-1c7b8b222b27864f/out -L native=/home/gftea/repo/aptos-core/target/debug/build/clear_on_drop-d8fa5528dcfe7b82/out -L native=/home/gftea/repo/aptos-core/target/debug/build/ring-b8aea15431227a80/out -L native=/home/gftea/repo/aptos-core/target/debug/build/jemalloc-sys-6024fbc8b0d7bffe/out/build/lib`