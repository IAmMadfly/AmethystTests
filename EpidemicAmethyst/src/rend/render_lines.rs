use amethyst::{
    core::ecs::{World},
    renderer::{
        Backend,
        Factory,
        bundle::{
            RenderPlugin, RenderPlan, RenderOrder, Target
        }
    },
    error::Error
};

use crate::rend::draw_lines;

/// A [RenderPlugin] for drawing debug lines.
/// Use with [debug_drawing::DebugLines] resource 
/// or [debug_drawing::DebugLinesComponent].
#[derive(Default, Debug)]
pub struct RenderLines {
    target: Target,
}

impl RenderLines {
    /// Set target to which debug lines will be rendered.
    pub fn with_target(mut self, target: Target) -> Self {
        self.target = target;
        self
    }
}

impl<B: Backend> RenderPlugin<B> for RenderLines {
    fn on_plan(
        &mut self,
        plan: &mut RenderPlan<B>,
        _factory: &mut Factory<B>,
        _world: &World,
    ) -> Result<(), Error> {
        plan.extend_target(self.target, |ctx| {
            ctx.add(
                RenderOrder::BeforeTransparent,
                draw_lines::DrawDebugLinesDesc::new().builder(),
            )?;
            Ok(())
        });
        Ok(())
    }
}