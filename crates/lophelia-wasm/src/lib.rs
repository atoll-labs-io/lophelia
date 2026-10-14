use wasm_bindgen::prelude::wasm_bindgen;

#[wasm_bindgen]
pub fn engine_version() -> String {
    lophelia_core::VERSION.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn engine_version_is_the_core_version() {
        assert_eq!(engine_version(), lophelia_core::VERSION);
    }
}
