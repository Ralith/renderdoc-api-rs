#![allow(non_camel_case_types, non_upper_case_globals, non_snake_case)]
// Generate with: bindgen --constified-enum '.*' --no-prepend-enum-name --whitelist-type 'p?RENDERDOC.*' renderdoc_app.h -o bindings.rs
include!("bindings.rs");
