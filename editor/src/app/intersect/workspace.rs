use crate::geom::camera::Camera;
use crate::sheet::widget::SheetWidget;
use crate::app::intersect::content::IntersectMessage;
use crate::app::design::{style_sheet_background, Design};
use crate::app::main::{EditorApp, AppMessage};
use iced::widget::Stack;
use iced::widget::Container;
use iced::{Length, Padding, Size, Vector};
use qurvy::int::bezier::path::IntBezierPath;
use crate::bezier_editor::widget::{BezierEditorUpdateEvent, BezierEditorWidget};

pub(crate) struct WorkspaceState {
    pub(crate) camera: Camera,
    pub(crate) curves: Vec<IntBezierPath>
}

impl EditorApp {
    pub(crate) fn intersect_workspace(&self) -> Container<AppMessage> {
        Container::new({
            let mut stack = Stack::new();
            stack = stack.push(
                Container::new(SheetWidget::new(
                    self.state.intersect.workspace.camera,
                    Design::negative_color().scale_alpha(0.5),
                    on_update_size,
                    on_update_zoom,
                    on_update_drag,
                ))
                    .width(Length::Fill)
                    .height(Length::Fill)
            );
            for (id, curve) in self.state.intersect.workspace.curves.iter().enumerate() {
                stack = stack.push(
                    Container::new(BezierEditorWidget::new(
                        id,
                        curve,
                        self.state.intersect.workspace.camera,
                        on_update_anchor
                    ))
                        .width(Length::Fill)
                        .height(Length::Fill)
                );
            }

            stack.push(
                Container::new(self.intersect_control())
                    .width(Length::Shrink)
                    .height(Length::Shrink)
                    .padding(Padding::new(8.0))
            )
        })
            .style(style_sheet_background)
    }

    pub(super) fn intersect_update_anchor(&mut self, update: BezierEditorUpdateEvent) {
        self.state.intersect.intersect_update_point(update);
    }

    pub(super) fn intersect_update_zoom(&mut self, camera: Camera) {
        self.state.intersect.workspace.camera = camera;
    }

    pub(super) fn intersect_update_drag(&mut self, new_pos: Vector<f32>) {
        self.state.intersect.workspace.camera.pos = new_pos;
    }
}

fn on_update_anchor(event: BezierEditorUpdateEvent) -> AppMessage {
    AppMessage::Intersect(IntersectMessage::BezierEdited(event))
}

fn on_update_size(size: Size) -> AppMessage {
    AppMessage::Intersect(IntersectMessage::WorkspaceSized(size))
}

fn on_update_zoom(zoom: Camera) -> AppMessage {
    AppMessage::Intersect(IntersectMessage::WorkspaceZoomed(zoom))
}

fn on_update_drag(drag: Vector<f32>) -> AppMessage {
    AppMessage::Intersect(IntersectMessage::WorkspaceDragged(drag))
}

impl Default for WorkspaceState {
    fn default() -> Self {
        WorkspaceState { camera: Camera::empty(), curves: vec![] }
    }
}