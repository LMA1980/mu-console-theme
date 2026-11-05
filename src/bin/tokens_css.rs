// SPDX-License-Identifier: LGPL-3.0-only WITH LGPL-3.0-linking-exception
//! Print the generated `tokens.css`.
//!
//! ```text
//! cargo run -q --bin mu-console-tokens-css > assets/tokens.css
//! ```

fn main() {
    print!("{}", mu_console_theme::css::tokens_css());
}
