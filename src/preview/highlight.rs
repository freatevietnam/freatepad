use syntect::easy::HighlightLines;
use syntect::highlighting::ThemeSet;
use syntect::parsing::SyntaxSet;

#[allow(dead_code)]
pub struct SyntaxHighlighter {
    syntax_set: SyntaxSet,
    theme_set: ThemeSet,
}

#[allow(dead_code)]
impl SyntaxHighlighter {
    pub fn new() -> Self {
        Self {
            syntax_set: SyntaxSet::load_defaults_newlines(),
            theme_set: ThemeSet::load_defaults(),
        }
    }

    pub fn highlight(&self, code: &str, language: &str) -> String {
        let syntax = self
            .syntax_set
            .find_syntax_by_token(language)
            .unwrap_or_else(|| self.syntax_set.find_syntax_plain_text());

        let theme = &self.theme_set.themes["base16-ocean.dark"];
        let mut h = HighlightLines::new(syntax, theme);

        let mut html = String::from("<pre><code>");
        for line in code.lines() {
            if let Ok(highlights) = h.highlight_line(line, &self.syntax_set) {
                for (style, text) in highlights {
                    let color = style.foreground;
                    html.push_str(&format!(
                        "<span style=\"color:#{:02x}{:02x}{:02x}\">{}</span>",
                        color.r, color.g, color.b, escape_html(text)
                    ));
                }
                html.push('\n');
            }
        }
        html.push_str("</code></pre>");

        html
    }

    pub fn get_supported_languages(&self) -> Vec<&str> {
        vec![
            "rust", "python", "javascript", "typescript", "c", "cpp", "go", "java",
            "shell", "bash", "markdown", "json", "yaml", "toml",
        ]
    }
}

#[allow(dead_code)]
fn escape_html(s: &str) -> String {
    s.replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('"', "&quot;")
        .replace('\'', "&#39;")
}
