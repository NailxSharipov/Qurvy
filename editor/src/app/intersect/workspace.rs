use crate::draw::shape::ShapeWidget;
use crate::geom::camera::Camera;
use crate::sheet::widget::SheetWidget;
use crate::point_editor::point::EditorPoint;
use crate::point_editor::widget::{PointEditUpdate, PointsEditorWidget};
use crate::app::intersect::content::IntersectMessage;
use crate::app::design::{style_sheet_background, Design};
use crate::app::main::{EditorApp, AppMessage};
use i_triangle::i_overlay::i_shape::int::count::IntShapes;
use i_triangle::i_overlay::i_shape::int::path::IntPaths;
use i_triangle::i_overlay::vector::edge::VectorEdge;
use iced::widget::Stack;
use iced::widget::Container;
use iced::{Length, Padding, Size, Vector};
use crate::app::intersect::control::ModeOption;
use crate::draw::vectors::VectorsWidget;

pub(crate) struct WorkspaceState {
    pub(crate) camera: Camera,
    pub(crate) subj: IntPaths,
    pub(crate) clip: IntPaths,
    pub(crate) solution: IntShapes,
    pub(crate) points: Vec<EditorPoint>,
    pub(crate) vectors: Vec<VectorEdge>,
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
            if self.state.intersect.workspace.camera.is_not_empty() {
                match self.state.intersect.mode {
                    ModeOption::Edit => {
                        stack = stack.push(
                            Container::new(ShapeWidget::with_paths(
                                &self.state.intersect.workspace.subj,
                                self.state.intersect.workspace.camera,
                                Some(self.state.intersect.fill.fill_rule()),
                                Some(Design::subject_color().scale_alpha(0.2)),
                                Some(Design::subject_color()),
                                4.0,
                            ))
                                .width(Length::Fill)
                                .height(Length::Fill)
                        ).push(
                            Container::new(ShapeWidget::with_paths(
                                &self.state.intersect.workspace.clip,
                                self.state.intersect.workspace.camera,
                                Some(self.state.intersect.fill.fill_rule()),
                                Some(Design::clip_color().scale_alpha(0.2)),
                                Some(Design::clip_color()),
                                4.0,
                            ))
                                .width(Length::Fill)
                                .height(Length::Fill)
                        )
                    }
                    ModeOption::Debug => {
                        stack = stack.push(
                            Container::new(VectorsWidget::with_vectors(
                                &self.state.intersect.workspace.vectors,
                                self.state.intersect.workspace.camera,
                                Design::subject_color(),
                                Design::clip_color(),
                                Design::both_color(),
                                2.0,
                            ))
                                .width(Length::Fill)
                                .height(Length::Fill)
                        )
                    }
                    _ => {
                        stack = stack.push(
                            Container::new(ShapeWidget::with_paths(
                                &self.state.intersect.workspace.subj,
                                self.state.intersect.workspace.camera,
                                Some(self.state.intersect.fill.fill_rule()),
                                None,
                                Some(Design::subject_color()),
                                1.0,
                            ))
                                .width(Length::Fill)
                                .height(Length::Fill)
                        ).push(
                            Container::new(ShapeWidget::with_paths(
                                &self.state.intersect.workspace.clip,
                                self.state.intersect.workspace.camera,
                                Some(self.state.intersect.fill.fill_rule()),
                                None,
                                Some(Design::clip_color()),
                                1.0,
                            ))
                                .width(Length::Fill)
                                .height(Length::Fill)
                        ).push(
                                Container::new(ShapeWidget::with_shapes(
                                    &self.state.intersect.workspace.solution,
                                    self.state.intersect.workspace.camera,
                                    None,
                                    Some(Design::solution_color().scale_alpha(0.2)),
                                    Some(Design::solution_color()),
                                    4.0,
                                ))
                                    .width(Length::Fill)
                                    .height(Length::Fill)
                            )
                    }
                }
                stack = stack.push(
                    Container::new(PointsEditorWidget::new(
                        &self.state.intersect.workspace.points,
                        self.state.intersect.workspace.camera,
                        on_update_point)
                        .set_drag_color(Design::accent_color())
                        .set_hover_color(Design::negative_color())
                    )
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

    pub(super) fn intersect_update_point(&mut self, update: PointEditUpdate) {
        self.state.intersect.intersect_update_point(update);
    }

    pub(super) fn intersect_update_zoom(&mut self, camera: Camera) {
        self.state.intersect.workspace.camera = camera;
    }

    pub(super) fn intersect_update_drag(&mut self, new_pos: Vector<f32>) {
        self.state.intersect.workspace.camera.pos = new_pos;
    }
}

fn on_update_point(event: PointEditUpdate) -> AppMessage {
    AppMessage::Intersect(IntersectMessage::PointEdited(event))
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
        WorkspaceState { camera: Camera::empty(), subj: vec![], clip: vec![], solution: vec![], points: vec![], vectors: vec![] }
    }
}