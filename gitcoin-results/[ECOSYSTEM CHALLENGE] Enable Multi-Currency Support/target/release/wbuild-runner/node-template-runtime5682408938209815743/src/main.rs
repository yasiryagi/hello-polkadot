
				//! This is automatically generated code by `substrate-wasm-builder`.

				use substrate_wasm_builder::build_project_with_default_rustflags;

				fn main() {
					build_project_with_default_rustflags(
						"/substrate-node-template/target/release/build/node-template-runtime-1f2e2337c867a425/out/wasm_binary.rs",
						"/substrate-node-template/runtime/Cargo.toml",
						"-Clink-arg=--export=__heap_base -C link-arg=--import-memory ",
					)
				}
			