use crate::TableStyle;

impl Default for TableStyle {
    fn default() -> Self {
        Self::default_dark()
    }
}

impl TableStyle {
    pub fn default_dark() -> Self {
        TableStyle {
            margin_px: 1,
            padding_px: 8,
            css_entries: vec![
                "th div.text {
                    padding: 8px;
                }",
                "th, td.index {
                  background-color: #444d56;
                  color: #f6f8fa;
                  border: 1px solid #24292e;
                  text-align: left;
                  font-weight: 600;
                }",
                "td {
                  background-color: #2f363d;
                  color: #f6f8fa;
                  border: 1px solid #24292e;
                  padding: 8px;
                }",
                "td.null {
                  font-style: italic;
                  color: #888888;
                }",
                "td.boolean {
                  color: #79c0ff;
                  font-weight: 500;
                }",
                "td.number {
                  text-align: right;
                  color: #a5d6ff;
                }",
                "td.string {
                  color: #a5d6ff;
                }",
                "td.array {
                  padding: 0;
                }",
                "td.array div div.ias {
                  padding: 8px;
                  background-color: #444d56;
                  font-style: italic;
                  color: #8b949e;
                  text-align: center;
                  font-size: 0.9em;
                }",
                "tr:hover td {
                  background-color: #3d444d;
                }",
                "td.array div {
                    max-height: 200px;
                    overflow-y: auto;
                    overflow-x: clip;
                }",
            ],
        }
    }

    pub fn default_light() -> Self {
        TableStyle {
            margin_px: 1,
            padding_px: 8,
            css_entries: vec![
                "th div.text {
                    padding: 8px;
                }",
                "th, td.index {
                  background-color: #f6f8fa;
                  color: #24292e;
                  border: 1px solid #d0d7de;
                  text-align: left;
                  font-weight: 600;
                }",
                "td {
                  background-color: #ffffff;
                  color: #24292e;
                  border: 1px solid #d0d7de;
                  padding: 8px;
                }",
                "td.null {
                  font-style: italic;
                  color: #888888;
                }",
                "td.boolean {
                  color: #0969da;
                  font-weight: 500;
                }",
                "td.number {
                  text-align: right;
                  color: #0550ae;
                }",
                "td.string {
                  color: #0a3069;
                }",
                "td.array {
                  padding: 0;
                }",
                "td.array div div.ias {
                  padding: 8px;
                  background-color: #f6f8fa;
                  font-style: italic;
                  color: #57606a;
                  text-align: center;
                  font-size: 0.9em;
                }",
                "tr:hover td {
                  background-color: #f6f8fa;
                }",
                "td.array div {
                    max-height: 200px;
                    overflow-y: auto;
                    overflow-x: clip;
                }",
            ],
        }
    }

    pub fn monokai() -> Self {
        TableStyle {
            margin_px: 1,
            padding_px: 8,
            css_entries: vec![
                "th div.text {
                    padding: 8px;
                }",
                "th, td.index {
                  background-color: #3e3d32;
                  color: #f8f8f2;
                  border: 1px solid #272822;
                  text-align: left;
                  font-weight: 600;
                }",
                "td {
                  background-color: #272822;
                  color: #f8f8f2;
                  border: 1px solid #3e3d32;
                  padding: 8px;
                }",
                "td.null {
                  font-style: italic;
                  color: #75715e;
                }",
                "td.boolean {
                  color: #ae81ff;
                  font-weight: 500;
                }",
                "td.number {
                  text-align: right;
                  color: #ae81ff;
                }",
                "td.string {
                  color: #e6db74;
                }",
                "td.array {
                  padding: 0;
                }",
                "td.array div div.ias {
                  padding: 8px;
                  background-color: #3e3d32;
                  font-style: italic;
                  color: #75715e;
                  text-align: center;
                  font-size: 0.9em;
                }",
                "tr:hover td {
                  background-color: #3e3d32;
                }",
                "td.array div {
                    max-height: 200px;
                    overflow-y: auto;
                    overflow-x: clip;
                }",
            ],
        }
    }

    pub fn solarized_dark() -> Self {
        TableStyle {
            margin_px: 1,
            padding_px: 8,
            css_entries: vec![
                "th div.text {                    
                    padding: 8px;
                }",
                "th, td.index {
                  background-color: #073642;
                  color: #839496;
                  border: 1px solid #002b36;
                  text-align: left;
                  font-weight: 600;
                }",
                "td {
                  background-color: #002b36;
                  color: #839496;
                  border: 1px solid #073642;
                  padding: 8px;
                }",
                "td.null {
                  font-style: italic;
                  color: #586e75;
                }",
                "td.boolean {
                  color: #268bd2;
                  font-weight: 500;
                }",
                "td.number {
                  text-align: right;
                  color: #2aa198;
                }",
                "td.string {
                  color: #b58900;
                }",
                "td.array {
                  padding: 0;
                }",
                "td.array div div.ias {
                  padding: 8px;
                  background-color: #073642;
                  font-style: italic;
                  color: #586e75;
                  text-align: center;
                  font-size: 0.9em;
                }",
                "tr:hover td {
                  background-color: #073642;
                }",
                "td.array div {
                    max-height: 200px;
                    overflow-y: auto;
                    overflow-x: clip;
                }",
            ],
        }
    }

    pub fn solarized_light() -> Self {
        TableStyle {
            margin_px: 1,
            padding_px: 8,
            css_entries: vec![
                "th div.text {
                    padding: 8px;
                }",
                "th, td.index {
                  background-color: #eee8d5;
                  color: #657b83;
                  border: 1px solid #93a1a1;
                  text-align: left;
                  font-weight: 600;
                }",
                "td {
                  background-color: #fdf6e3;
                  color: #657b83;
                  border: 1px solid #eee8d5;
                  padding: 8px;
                }",
                "td.null {
                  font-style: italic;
                  color: #93a1a1;
                }",
                "td.boolean {
                  color: #268bd2;
                  font-weight: 500;
                }",
                "td.number {
                  text-align: right;
                  color: #2aa198;
                }",
                "td.string {
                  color: #b58900;
                }",
                "td.array {
                  padding: 0;
                }",
                "td.array div div.ias {
                  padding: 8px;
                  background-color: #eee8d5;
                  font-style: italic;
                  color: #93a1a1;
                  text-align: center;
                  font-size: 0.9em;
                }",
                "tr:hover td {
                  background-color: #eee8d5;
                }",
                "td.array div {
                    max-height: 200px;
                    overflow-y: auto;
                    overflow-x: clip;
                }",
            ],
        }
    }

    pub fn dracula() -> Self {
        TableStyle {
            margin_px: 1,
            padding_px: 8,
            css_entries: vec![
                "th div.text {
                    padding: 8px;
                }",
                "th, td.index {
                  background-color: #44475a;
                  color: #f8f8f2;
                  border: 1px solid #282a36;
                  text-align: left;
                  font-weight: 600;
                }",
                "td {
                  background-color: #282a36;
                  color: #f8f8f2;
                  border: 1px solid #44475a;
                  padding: 8px;
                }",
                "td.null {
                  font-style: italic;
                  color: #6272a4;
                }",
                "td.boolean {
                  color: #bd93f9;
                  font-weight: 500;
                }",
                "td.number {
                  text-align: right;
                  color: #bd93f9;
                }",
                "td.string {
                  color: #f1fa8c;
                }",
                "td.array {
                  padding: 0;
                }",
                "td.array div div.ias {
                  padding: 8px;
                  background-color: #44475a;
                  font-style: italic;
                  color: #6272a4;
                  text-align: center;
                  font-size: 0.9em;
                }",
                "tr:hover td {
                  background-color: #44475a;
                }",
                "td.array div {
                    max-height: 200px;
                    overflow-y: auto;
                    overflow-x: clip;
                }",
            ],
        }
    }

    pub fn nord() -> Self {
        TableStyle {
            margin_px: 1,
            padding_px: 8,
            css_entries: vec![
                "th div.text {
                    padding: 8px;
                }",
                "th, td.index {
                  background-color: #3b4252;
                  color: #eceff4;
                  border: 1px solid #2e3440;
                  text-align: left;
                  font-weight: 600;
                }",
                "td {
                  background-color: #2e3440;
                  color: #eceff4;
                  border: 1px solid #3b4252;
                  padding: 8px;
                }",
                "td.null {
                  font-style: italic;
                  color: #4c566a;
                }",
                "td.boolean {
                  color: #88c0d0;
                  font-weight: 500;
                }",
                "td.number {
                  text-align: right;
                  color: #b48ead;
                }",
                "td.string {
                  color: #a3be8c;
                }",
                "td.array {
                  padding: 0;
                }",
                "td.array div div.ias {
                  padding: 8px;
                  background-color: #3b4252;
                  font-style: italic;
                  color: #4c566a;
                  text-align: center;
                  font-size: 0.9em;
                }",
                "tr:hover td {
                  background-color: #3b4252;
                }",
                "td.array div {
                    max-height: 200px;
                    overflow-y: auto;
                    overflow-x: clip;
                }",
            ],
        }
    }

    pub fn gruvbox_dark() -> Self {
        TableStyle {
            margin_px: 1,
            padding_px: 8,
            css_entries: vec![
                "th div.text {
                    padding: 8px;
                }",
                "th, td.index {
                  background-color: #3c3836;
                  color: #ebdbb2;
                  border: 1px solid #282828;
                  text-align: left;
                  font-weight: 600;
                }",
                "td {
                  background-color: #282828;
                  color: #ebdbb2;
                  border: 1px solid #3c3836;
                  padding: 8px;
                }",
                "td.null {
                  font-style: italic;
                  color: #928374;
                }",
                "td.boolean {
                  color: #d3869b;
                  font-weight: 500;
                }",
                "td.number {
                  text-align: right;
                  color: #d3869b;
                }",
                "td.string {
                  color: #b8bb26;
                }",
                "td.array {
                  padding: 0;
                }",
                "td.array div div.ias {
                  padding: 8px;
                  background-color: #3c3836;
                  font-style: italic;
                  color: #928374;
                  text-align: center;
                  font-size: 0.9em;
                }",
                "tr:hover td {
                  background-color: #3c3836;
                }",
                "td.array div {
                    max-height: 200px;
                    overflow-y: auto;
                    overflow-x: clip;
                }",
            ],
        }
    }

    pub fn one_dark() -> Self {
        TableStyle {
            margin_px: 1,
            padding_px: 8,
            css_entries: vec![
                "th div.text {
                    padding: 8px;
                }",
                "th, td.index {
                  background-color: #2c323c;
                  color: #abb2bf;
                  border: 1px solid #21252b;
                  text-align: left;
                  font-weight: 600;
                }",
                "td {
                  background-color: #282c34;
                  color: #abb2bf;
                  border: 1px solid #2c323c;
                  padding: 8px;
                }",
                "td.null {
                  font-style: italic;
                  color: #5c6370;
                }",
                "td.boolean {
                  color: #c678dd;
                  font-weight: 500;
                }",
                "td.number {
                  text-align: right;
                  color: #d19a66;
                }",
                "td.string {
                  color: #98c379;
                }",
                "td.array {
                  padding: 0;
                }",
                "td.array div div.ias {
                  padding: 8px;
                  background-color: #2c323c;
                  font-style: italic;
                  color: #5c6370;
                  text-align: center;
                  font-size: 0.9em;
                }",
                "tr:hover td {
                  background-color: #2c323c;
                }",
                "td.array div {
                    max-height: 200px;
                    overflow-y: auto;
                    overflow-x: clip;
                }",
            ],
        }
    }

    pub fn material() -> Self {
        TableStyle {
            margin_px: 1,
            padding_px: 8,
            css_entries: vec![
                "th div.text {
                    padding: 8px;
                }",
                "th, td.index {
                  background-color: #37474f;
                  color: #eeffff;
                  border: 1px solid #263238;
                  text-align: left;
                  font-weight: 600;
                }",
                "td {
                  background-color: #263238;
                  color: #eeffff;
                  border: 1px solid #37474f;
                  padding: 8px;
                }",
                "td.null {
                  font-style: italic;
                  color: #546e7a;
                }",
                "td.boolean {
                  color: #c792ea;
                  font-weight: 500;
                }",
                "td.number {
                  text-align: right;
                  color: #f78c6c;
                }",
                "td.string {
                  color: #c3e88d;
                }",
                "td.array {
                  padding: 0;
                }",
                "td.array div div.ias {
                  padding: 8px;
                  background-color: #37474f;
                  font-style: italic;
                  color: #546e7a;
                  text-align: center;
                  font-size: 0.9em;
                }",
                "tr:hover td {
                  background-color: #37474f;
                }",
                "td.array div {
                    max-height: 200px;
                    overflow-y: auto;
                    overflow-x: clip;
                }",
            ],
        }
    }
}
