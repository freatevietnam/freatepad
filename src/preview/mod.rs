mod math;
mod highlight;

use std::collections::HashMap;

#[allow(dead_code)]
pub struct PreviewRenderer {
    svg_cache: HashMap<String, Vec<u8>>,
}

#[allow(dead_code)]
impl PreviewRenderer {
    pub fn new() -> Self {
        Self {
            svg_cache: HashMap::new(),
        }
    }

    pub fn render_math(&mut self, latex: &str) -> Option<Vec<u8>> {
        if let Some(cached) = self.svg_cache.get(latex) {
            return Some(cached.clone());
        }

        if let Some(svg) = math::render_latex_to_svg(latex) {
            self.svg_cache.insert(latex.to_string(), svg.clone());
            Some(svg)
        } else {
            None
        }
    }

    pub fn clear_cache(&mut self) {
        self.svg_cache.clear();
    }
}
