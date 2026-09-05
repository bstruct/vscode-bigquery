use website_base::{
    base_result::BaseResult,
    document::{HtmlNode, create_element_with_text},
};

use crate::{InnerTableBuilder, TableBuilder};

/// Macro to include and minify CSS at compile time (minification happens at first call)
macro_rules! include_minified_css {
    ($path:expr) => {{
        use std::sync::OnceLock;
        static MINIFIED: OnceLock<String> = OnceLock::new();
        MINIFIED
            .get_or_init(|| minify_css(include_str!($path)))
            .as_str()
    }};
}

impl TableBuilder {
    pub(crate) fn get_style_elements(&self) -> BaseResult<Vec<HtmlNode>> {
        let element = create_element_with_text("style", Some(self.get_style().as_str()))?;

        Ok(vec![element])
    }

    fn get_style(&self) -> String {
        let column_widths = self.get_column_width_css_variables();
        let common_css = include_minified_css!("../static/common.css");
        let added_styles = self.get_added_styles();

        let css = if self.dynamic_table_render {
            include_minified_css!("../static/dynamic_render.css")
        } else {
            ""
        };

        format!(
            ":host {{{}}} {} {} {}",
            column_widths, common_css, css, added_styles
        )
    }

    fn get_column_width_css_variables(&self) -> String {
        self.columns
            .iter()
            .flat_map(|col| col.get_widths())
            .enumerate()
            .map(|(index, width)| format!("--c{}:{}px;", index, width))
            .collect::<Vec<String>>()
            .join(" ")
    }

    fn get_added_styles(&self) -> String {
        let mut combined = String::new();
        for entry in &self.style.css_entries {
            combined.push_str(entry);
            combined.push(' ');
        }
        minify_css(&combined)
    }
}

impl InnerTableBuilder {
    pub(crate) fn get_style_element(&self) -> BaseResult<HtmlNode> {
        let element = create_element_with_text("style", Some(self.get_style().as_str()))?;

        Ok(element)
    }

    /// Column widths are not emitted here: every inner cell carries an inline
    /// `min-width`/`max-width` computed from the parent's `--c<N>` variables
    /// (see `inner_table_builder_render.rs`), which keeps cells aligned even when
    /// a cell spans several columns.
    fn get_style(&self) -> String {
        let common_css = include_minified_css!("../static/common.css");
        let added_styles = self.get_added_styles();

        format!("{} {}", common_css, added_styles)
    }

    fn get_added_styles(&self) -> String {
        let mut combined = String::new();
        for entry in &self.style.css_entries {
            combined.push_str(entry);
            combined.push(' ');
        }
        minify_css(&combined)
    }
}

fn minify_css(css: &str) -> String {
    // Remove CSS comments /* ... */
    let mut result = String::new();
    let mut chars = css.chars().peekable();

    while let Some(c) = chars.next() {
        if c == '/' && chars.peek() == Some(&'*') {
            // Skip comment
            chars.next(); // consume '*'
            while let Some(c) = chars.next() {
                if c == '*' && chars.peek() == Some(&'/') {
                    chars.next(); // consume '/'
                    break;
                }
            }
        } else {
            result.push(c);
        }
    }

    // Trim lines and join with spaces
    result
        .lines()
        .map(|line| line.trim())
        .filter(|line| !line.is_empty())
        .collect::<Vec<&str>>()
        .join(" ")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_minify_css_removes_comments() {
        let css = r#"
/* This is a comment */
.class {
    color: red; /* inline comment */
}
        "#;
        let result = minify_css(css);
        assert_eq!(result, ".class { color: red; }");
    }

    #[test]
    fn test_minify_css_trims_whitespace() {
        let css = r#"
        .class1 {
            color: blue;
        }
        
        .class2 {
            margin: 10px;
        }
        "#;
        let result = minify_css(css);
        assert_eq!(result, ".class1 { color: blue; } .class2 { margin: 10px; }");
    }

    #[test]
    fn test_minify_css_removes_empty_lines() {
        let css = r#"
.class1 { color: red; }

.class2 { color: blue; }


.class3 { color: green; }
        "#;
        let result = minify_css(css);
        assert_eq!(
            result,
            ".class1 { color: red; } .class2 { color: blue; } .class3 { color: green; }"
        );
    }

    #[test]
    fn test_minify_css_handles_multiline_comments() {
        let css = r#"
/* This is a
   multiline
   comment */
.class {
    color: red;
}
        "#;
        let result = minify_css(css);
        assert_eq!(result, ".class { color: red; }");
    }

    #[test]
    fn test_minify_css_preserves_css_rules() {
        let css = r#"
/* Header styles */
.header {
    background-color: #333;
    color: white;
    padding: 10px;
}

/* Body styles */
.body {
    font-size: 14px;
}
        "#;
        let result = minify_css(css);
        assert_eq!(
            result,
            ".header { background-color: #333; color: white; padding: 10px; } .body { font-size: 14px; }"
        );
    }

    #[test]
    fn test_minify_css_handles_adjacent_comments() {
        let css = r#"
/* Comment 1 *//* Comment 2 */
.class { color: red; }
        "#;
        let result = minify_css(css);
        assert_eq!(result, ".class { color: red; }");
    }

    #[test]
    fn test_minify_css_empty_input() {
        let css = "";
        let result = minify_css(css);
        assert_eq!(result, "");
    }

    #[test]
    fn test_minify_css_only_comments() {
        let css = "/* Just a comment */";
        let result = minify_css(css);
        assert_eq!(result, "");
    }
}
