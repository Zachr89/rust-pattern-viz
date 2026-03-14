use wasm_bindgen::prelude::*;
use crate::{CodeAnalyzer, AnalysisReport};

#[cfg(feature = "wasm")]
#[wasm_bindgen]
pub fn init_panic_hook() {
    console_error_panic_hook::set_once();
}

#[cfg(feature = "wasm")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
pub struct WasmAnalyzer {
    analyzer: CodeAnalyzer,
}

#[wasm_bindgen]
impl WasmAnalyzer {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Self {
        init_panic_hook();
        Self {
            analyzer: CodeAnalyzer::new(),
        }
    }

    #[wasm_bindgen]
    pub fn analyze(&self, source: &str, file_path: &str) -> Result<String, JsValue> {
        let report = self.analyzer
            .analyze(source, file_path)
            .map_err(|e| JsValue::from_str(&format!("Analysis error: {}", e)))?;
        
        serde_json::to_string(&report)
            .map_err(|e| JsValue::from_str(&format!("Serialization error: {}", e)))
    }

    #[wasm_bindgen]
    pub fn version(&self) -> String {
        env!("CARGO_PKG_VERSION").to_string()
    }
}

impl Default for WasmAnalyzer {
    fn default() -> Self {
        Self::new()
    }
}
