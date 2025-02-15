use anyhow::Result;
use ratatui::{backend::Backend, layout::Rect, Frame};

pub trait RenderAbleComponent {
	fn render<B: Backend>(&self, f: &mut Frame<B>, rect: Rect, focused: bool) -> Result<()>;
}