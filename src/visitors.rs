//! Defines several basic visitors (Json and Text) and
//! various visitors-combinarots
mod ignore_scoped;
mod json;
mod text;

pub use ignore_scoped::IgnoreScoped;
pub use json::JsonVisitor;
pub use text::TextVisitor;
