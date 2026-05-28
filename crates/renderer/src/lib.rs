pub mod error;

pub use error::*;

use graphitepdf_render::{RenderCommand, RenderDocument};

pub trait RenderBackend {
    fn render_command(&mut self, command: &RenderCommand) -> Result<()>;
}

#[derive(Debug, Default)]
pub struct NoopRenderBackend;

impl RenderBackend for NoopRenderBackend {
    fn render_command(&mut self, _command: &RenderCommand) -> Result<()> {
        Ok(())
    }
}

pub struct Renderer<B: RenderBackend> {
    backend: B,
}

impl<B: RenderBackend> Renderer<B> {
    pub fn new(backend: B) -> Self {
        Self { backend }
    }

    pub fn backend(&self) -> &B {
        &self.backend
    }

    pub fn backend_mut(&mut self) -> &mut B {
        &mut self.backend
    }

    pub fn render(&mut self, document: &RenderDocument) -> Result<()> {
        for page in &document.pages {
            for command in &page.commands {
                self.backend.render_command(command)?;
            }
        }

        Ok(())
    }
}
