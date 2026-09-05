use wasm_bindgen_test::wasm_bindgen_test_configure;

wasm_bindgen_test_configure!(run_in_browser);

mod simple_table_test;
#[allow(unused_imports)]
use crate::simple_table_test::*;

mod large_text_test;
#[allow(unused_imports)]
use crate::large_text_test::*;

mod color_test;
#[allow(unused_imports)]
use crate::color_test::*;

mod nested_structure_test;
#[allow(unused_imports)]
use crate::nested_structure_test::*;

mod json_data_test;
#[allow(unused_imports)]
use crate::json_data_test::*;

mod array_expandable_test;
#[allow(unused_imports)]
use crate::array_expandable_test::*;

mod complex_array_test;
#[allow(unused_imports)]
use crate::complex_array_test::*;
