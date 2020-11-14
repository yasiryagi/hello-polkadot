
				//! This is automatically generated code by `substrate-wasm-builder`.

				use substrate_wasm_builder::build_project_with_default_rustflags;

				fn main() {
					build_project_with_default_rustflags(
						"/substrate-node-template/target/debug/build/node-template-runtime-5548d7d712163d17/out/wasm_binary.rs",
						"/substrate-node-template/runtime/Cargo.toml",
						"-Clink-arg=--export=__heap_base -C link-arg=--import-memory ",
					)
				}
			