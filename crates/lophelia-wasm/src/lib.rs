use lophelia_core::contract::{Graph, LayoutInput, Settings};
use lophelia_core::positions::{self, DEFAULT_MAX_BYTES};
use wasm_bindgen::prelude::{JsError, wasm_bindgen};

#[wasm_bindgen]
pub fn engine_version() -> String {
    lophelia_core::VERSION.to_string()
}

/// Center, repulsion, link strength, link distance.
#[wasm_bindgen]
pub fn default_forces() -> Vec<f64> {
    Settings::default().forces().to_vec()
}

fn js_error<E: std::fmt::Display>(code: &str, e: E) -> JsError {
    JsError::new(&format!("{code}: {e}"))
}

#[wasm_bindgen]
pub struct Engine {
    graph: Graph,
}

#[wasm_bindgen]
impl Engine {
    /// Node `i` of every later call is `ids[i]`.
    #[wasm_bindgen(constructor)]
    pub fn new(ids: Vec<String>) -> Result<Engine, JsError> {
        Graph::new(ids)
            .map(|graph| Engine { graph })
            .map_err(|e| js_error(e.code(), &e))
    }

    pub fn node_count(&self) -> u32 {
        // Graph::new refuses more than u32::MAX nodes.
        self.graph.node_count() as u32
    }

    pub fn layout(
        &self,
        edges: &[u32],
        known_nodes: &[u32],
        known_xyz: &[f64],
        changed: &[u32],
        seed: u32,
        forces: &[f64],
    ) -> Result<Vec<f64>, JsError> {
        let settings = Settings::from_parts(seed, forces).map_err(|e| js_error(e.code(), &e))?;
        self.graph
            .layout(&LayoutInput {
                edges,
                known_nodes,
                known_xyz,
                changed,
                settings,
            })
            .map_err(|e| js_error(e.code(), &e))
    }

    pub fn read_positions(&self, file: &str) -> Result<KnownPositions, JsError> {
        positions::read(file, &self.graph, DEFAULT_MAX_BYTES)
            .map(|known| KnownPositions {
                nodes: known.nodes,
                xyz: known.xyz,
            })
            .map_err(|e| js_error(e.code(), &e))
    }

    pub fn write_positions(&self, xyz: &[f64]) -> Result<String, JsError> {
        positions::write(&self.graph, xyz).map_err(|e| js_error(e.code(), &e))
    }
}

/// `nodes` and `xyz` go to `Engine.layout` as `known_nodes` and `known_xyz`.
#[wasm_bindgen]
pub struct KnownPositions {
    nodes: Vec<u32>,
    xyz: Vec<f64>,
}

#[wasm_bindgen]
impl KnownPositions {
    #[wasm_bindgen(getter)]
    pub fn nodes(&self) -> Vec<u32> {
        self.nodes.clone()
    }

    #[wasm_bindgen(getter)]
    pub fn xyz(&self) -> Vec<f64> {
        self.xyz.clone()
    }
}

// `JsError::new` calls into JS and panics natively, so only `Ok` paths are tested here.
#[cfg(test)]
mod tests {
    use super::*;

    fn ids(names: &[&str]) -> Vec<String> {
        names.iter().map(|s| s.to_string()).collect()
    }

    fn engine(names: &[&str]) -> Engine {
        match Engine::new(ids(names)) {
            Ok(engine) => engine,
            Err(_) => panic!("engine refused"),
        }
    }

    #[test]
    fn engine_version_is_the_core_version() {
        assert_eq!(engine_version(), lophelia_core::VERSION);
    }

    #[test]
    fn engine_layout_is_the_core_layout() {
        let names = ["a.md", "b.md", "c.md"];
        let e = engine(&names);
        let forces = default_forces();
        let Ok(out) = e.layout(&[0, 1], &[1], &[1.0, 2.0, 3.0], &[2], 7, &forces) else {
            panic!("layout refused");
        };

        let graph = Graph::new(ids(&names)).expect("graph");
        let settings = Settings::from_parts(7, &forces).expect("settings");
        let core = graph
            .layout(&LayoutInput {
                edges: &[0, 1],
                known_nodes: &[1],
                known_xyz: &[1.0, 2.0, 3.0],
                changed: &[2],
                settings,
            })
            .expect("core layout");
        assert_eq!(e.node_count(), 3);
        assert_eq!(
            out.iter().map(|v| v.to_bits()).collect::<Vec<_>>(),
            core.iter().map(|v| v.to_bits()).collect::<Vec<_>>()
        );
    }

    #[test]
    fn written_positions_read_back_by_index() {
        let e = engine(&["b.md", "a.md"]);
        let xyz = [0.1, -0.0, 1e300, -5e-324, 2.5, 3.0];
        let Ok(text) = e.write_positions(&xyz) else {
            panic!("write refused");
        };
        let Ok(known) = e.read_positions(&text) else {
            panic!("read refused");
        };
        // Sorted by path, so node 1 (a.md) comes first.
        assert_eq!(known.nodes(), vec![1, 0]);
        let back: Vec<u64> = known.xyz().iter().map(|v| v.to_bits()).collect();
        let expected: Vec<u64> = [-5e-324, 2.5, 3.0, 0.1, -0.0, 1e300]
            .iter()
            .map(|v: &f64| v.to_bits())
            .collect();
        assert_eq!(back, expected);
    }
}
