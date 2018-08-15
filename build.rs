// Builds the duktape C library
extern crate gcc;
extern crate bindgen;

use std::env;
use std::path::PathBuf;
use bindgen::callbacks::*;

fn main() {
    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());

    // The bindgen::Builder is the main entry point
    // to bindgen, and lets you build up options for
    // the resulting bindings.
    bindgen::Builder::default()
        // The input header we would like to generate
        // bindings for.
        .header("c_src/duktape.h")
        .parse_callbacks(Box::new(Foo))
        // Finish the builder and generate the bindings.
        .generate()
        // Unwrap the Result and panic on failure.
        .expect("Unable to generate bindings")
        // Write the bindings to the $OUT_DIR/bindings.rs file.
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");

    gcc::Build::new()
        .file("c_src/duktape.c")
        .include("c_src")
        .compile("libduktape.a");
}

#[derive(Debug)]
struct Foo;
impl ParseCallbacks for Foo {
    fn int_macro(&self, _name: &str, _value: i64) -> Option<IntKind> { None }
    fn enum_variant_behavior(&self, _enum_name: Option<&str>, _original_variant_name: &str, _variant_value: EnumVariantValue) -> Option<EnumVariantCustomBehavior> { None }
    fn enum_variant_name(&self, _enum_name: Option<&str>, _original_variant_name: &str, _variant_value: EnumVariantValue) -> Option<String> { None }
    fn will_parse_macro(&self, name: &str) -> MacroParsingBehavior {
        match name {
            "FP_NAN"       => MacroParsingBehavior::Ignore,
            "FP_INFINITE"  => MacroParsingBehavior::Ignore,
            "FP_ZERO"      => MacroParsingBehavior::Ignore,
            "FP_SUBNORMAL" => MacroParsingBehavior::Ignore,
            "FP_NORMAL"    => MacroParsingBehavior::Ignore,
            _              => MacroParsingBehavior::Default,
        }
    }
}
