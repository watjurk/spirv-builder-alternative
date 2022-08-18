use rust_cargo_build_nested::dylib_path;

fn main() {
    // Possibly somewhere in spirv-builder
    let p = dylib_path();
    println!("{:?}", p);
}
