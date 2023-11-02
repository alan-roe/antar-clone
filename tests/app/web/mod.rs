mod chat;
use wasm_bindgen_test::*;
use web_sys::Element;

wasm_bindgen_test_configure!(run_in_browser);

fn data_test_id(id: &str) -> Element {
    let selector = format!("[data-testid=\"{}\"]", id);
    leptos::document()
        .query_selector(&selector)
        .unwrap()
        .unwrap_or_else(|| panic!("couldn't find data-testid {id}"))
}