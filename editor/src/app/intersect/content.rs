use std::collections::HashMap;
use i_triangle::i_overlay::i_float::int::point::IntPoint;
use i_triangle::i_overlay::i_float::int::rect::IntRect;
use iced::widget::scrollable;
use iced::{Alignment, Length, Padding, Size, Vector};
use iced::widget::{Button, Column, Container, Row, Space, Text};
use qurvy::convert::to_int::ToInt;
use qurvy::int::bezier::path::IntBezierPath;
use crate::app::design;
use crate::app::intersect::control::ModeOption;
use crate::app::intersect::workspace::WorkspaceState;
use crate::app::main::{EditorApp, AppMessage};
use crate::bezier_editor::widget::BezierEditorUpdateEvent;
use crate::compat::convert::Convert;
use crate::data::intersect::IntersectResource;
use crate::geom::camera::Camera;

pub(crate) struct IntersectState {
    pub(crate) test: usize,
    pub(crate) mode: ModeOption,
    pub(crate) workspace: WorkspaceState,
    pub(crate) size: Size,
    pub(crate) cameras: HashMap<usize, Camera>,
}

#[derive(Debug, Clone)]
pub(crate) enum IntersectMessage {
    TestSelected(usize),
    ModeSelected(ModeOption),
    BezierEdited(BezierEditorUpdateEvent),
    WorkspaceSized(Size),
    WorkspaceZoomed(Camera),
    WorkspaceDragged(Vector<f32>),
}

impl EditorApp {
    fn intersect_sidebar(&self) -> Column<AppMessage> {
        let count = self.app_resource.intersect.count;
        let mut column = Column::new().push(Space::new(Length::Fill, Length::Fixed(2.0)));
        for index in 0..count {
            let is_selected = self.state.intersect.test == index;

            column = column.push(
                Container::new(
                    Button::new(
                        Text::new(format!("test_{}", index))
                            .style(if is_selected { design::style_sidebar_text_selected } else { design::style_sidebar_text })
                            .size(14)
                    )
                        .width(Length::Fill)
                        .on_press(AppMessage::Intersect(IntersectMessage::TestSelected(index)))
                        .style(if is_selected { design::style_sidebar_button_selected } else { design::style_sidebar_button })
                ).padding(self.design.action_padding())
            );
        }

        column
    }

    pub(crate) fn intersect_content(&self) -> Row<AppMessage> {
        Row::new()
            .push(
                scrollable(
                    Container::new(self.intersect_sidebar())
                        .width(Length::Fixed(160.0))
                        .height(Length::Shrink)
                        .align_x(Alignment::Start)
                        .padding(Padding::new(0.0).right(8))
                        .style(design::style_sidebar_background)
                ).direction(scrollable::Direction::Vertical(
                    scrollable::Scrollbar::new()
                        .width(4)
                        .margin(0)
                        .scroller_width(4)
                        .anchor(scrollable::Anchor::Start),
                ))
            )
            .push(self.intersect_workspace())
    }

    pub(crate) fn intersect_update(&mut self, message: IntersectMessage) {
        match message {
            IntersectMessage::TestSelected(index) => self.intersect_set_test(index),
            IntersectMessage::ModeSelected(mode) => self.intersect_update_mode(mode),
            IntersectMessage::BezierEdited(update) => self.intersect_update_anchor(update),
            IntersectMessage::WorkspaceSized(size) => self.intersect_update_size(size),
            IntersectMessage::WorkspaceZoomed(zoom) => self.intersect_update_zoom(zoom),
            IntersectMessage::WorkspaceDragged(drag) => self.intersect_update_drag(drag),
        }
    }

    fn intersect_set_test(&mut self, index: usize) {
        self.state.intersect.load_test(index, &mut self.app_resource.intersect);
        self.state.intersect.update_solution();
    }

    pub(crate) fn intersect_init(&mut self) {
        self.intersect_set_test(self.state.intersect.test);
    }

    pub(crate) fn intersect_next_test(&mut self) {
        let next_test = self.state.intersect.test + 1;
        if next_test < self.app_resource.intersect.count {
            self.intersect_set_test(next_test);
        }
    }

    pub(crate) fn intersect_prev_test(&mut self) {
        let test = self.state.intersect.test;
        if test >= 1 {
            self.intersect_set_test(test - 1);
        }
    }

    fn intersect_update_size(&mut self, size: Size) {
        self.state.intersect.size = size;
        let curves = &self.state.intersect.workspace.curves;
        if self.state.intersect.workspace.camera.is_empty() && !curves.is_empty() {
            let camera = Camera::with_size_and_curves(size, curves);
            self.state.intersect.workspace.camera = camera;
        } else {
            self.state.intersect.workspace.camera.size = size;
        }
    }

    fn intersect_update_mode(&mut self, mode: ModeOption) {
        self.state.intersect.mode = mode;
        self.state.intersect.update_solution();
    }
}

impl IntersectState {
    pub(crate) fn new(resource: &mut IntersectResource) -> Self {
        let mut state = IntersectState {
            test: usize::MAX,
            mode: ModeOption::Edit,
            workspace: Default::default(),
            cameras: HashMap::with_capacity(resource.count),
            size: Size::ZERO,
        };

        state.load_test(0, resource);
        state.update_solution();
        state
    }

    fn load_test(&mut self, index: usize, resource: &mut IntersectResource) {
        if let Some(test) = resource.load(index) {
            self.workspace.curves.clear();
            self.workspace.curves = test.curves.iter().map(|c|c.to_int(2.0)).collect();

            self.cameras.insert(self.test, self.workspace.camera);
            let mut camera = *self.cameras.get(&index).unwrap_or(&Camera::empty());
            if camera.is_empty() && self.size.width > 0.001 {
                camera = Camera::with_size_and_curves(self.size, &self.workspace.curves);
            }

            self.workspace.camera = camera;

            self.test = index;
        }
    }

    fn update_solution(&mut self) {
        // let subj = &self.workspace.subj;
        // let clip = &self.workspace.clip;
        match self.mode {
            ModeOption::Edit => {}
            ModeOption::Debug => {
                // self.workspace.vectors = Overlay::with_contours(subj, clip).into_separate_vectors(FillRule::NonZero, Default::default());
            }
        }
    }

    pub(super) fn intersect_update_point(&mut self, update: BezierEditorUpdateEvent) {
        self.workspace.curves[update.curve_index].anchors[update.anchor_index] = update.anchor;
        self.update_solution();
    }
}

impl Camera {
    fn with_size_and_curves(size: Size, curves: &Vec<IntBezierPath>) -> Self {
        let rect = if curves.is_empty() {
            IntRect::new(-10_000, 10_000, -10_000, 10_000)
        } else {
            let mut rect = IntRect::new(i32::MAX, i32::MIN, i32::MAX, i32::MIN);
            for curve in curves {
                for anchor in curve.anchors.iter() {
                    let point: IntPoint = anchor.point.convert();
                    rect.add_point(&point);
                    if let Some(point) = anchor.handle_in_point() {
                        rect.add_point(&point.convert());
                    }
                    if let Some(point) = anchor.handle_out_point() {
                        rect.add_point(&point.convert());
                    }
                }
            }
            rect
        };

        Self::new(rect, size)
    }
}

