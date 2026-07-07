#[allow(dead_code)]
pub fn render_latex_to_svg(latex: &str) -> Option<Vec<u8>> {
    // For now, return a simple SVG with the LaTeX text
    // TODO: Implement proper Typst rendering
    let svg = format!(
        r#"<svg xmlns="http://www.w3.org/2000/svg" width="200" height="50">
            <text x="10" y="30" font-family="serif" font-size="16">{}</text>
        </svg>"#,
        escape_xml(latex)
    );

    Some(svg.into_bytes())
}

#[allow(dead_code)]
fn escape_xml(s: &str) -> String {
    s.replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('"', "&quot;")
        .replace('\'', "&apos;")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_render_math() {
        let result = render_latex_to_svg("x^2");
        assert!(result.is_some());
        let svg = String::from_utf8(result.unwrap()).unwrap();
        assert!(svg.contains("x^2"));
    }
}
