use std::collections::HashMap;
use i_triangle::i_overlay::core::overlay::Overlay;
use i_triangle::i_overlay::i_shape::int::count::PointsCount;
use i_triangle::i_overlay::i_float::int::rect::IntRect;
use iced::widget::scrollable;
use iced::{Alignment, Length, Padding, Size, Vector};
use iced::widget::{Button, Column, Container, Row, Space, Text};
use crate::app::design;
use crate::app::intersect::control::ModeOption;
use crate::app::intersect::workspace::WorkspaceState;
use crate::app::fill_option::FillOption;
use crate::app::main::{EditorApp, AppMessage};
use crate::app::solver_option::SolverOption;
use crate::data::intersect::IntersectResource;
use crate::geom::camera::Camera;
use crate::point_editor::point::PathsToEditorPoints;
use crate::point_editor::widget::PointEditUpdate;

pub(crate) struct IntersectState {
    pub(crate) test: usize,
    pub(crate) fill: FillOption,
    pub(crate) mode: ModeOption,
    pub(crate) solver: SolverOption,
    pub(crate) workspace: WorkspaceState,
    pub(crate) size: Size,
    pub(crate) cameras: HashMap<usize, Camera>,
}

#[derive(Debug, Clone)]
pub(crate) enum IntersectMessage {
    TestSelected(usize),
    FillSelected(FillOption),
    ModeSelected(ModeOption),
    SolverSelected(SolverOption),
    PointEdited(PointEditUpdate),
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
            IntersectMessage::SolverSelected(solver) => self.intersect_update_solver(solver),
            IntersectMessage::FillSelected(fill) => self.intersect_update_fill(fill),
            IntersectMessage::ModeSelected(mode) => self.intersect_update_mode(mode),
            IntersectMessage::PointEdited(update) => self.intersect_update_point(update),
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
        let points = &self.state.intersect.workspace.points;
        if self.state.intersect.workspace.camera.is_empty() && !points.is_empty() {
            let rect = IntRect::with_iter(points.iter().map(|p| &p.pos))
                .unwrap_or(IntRect::new(-10_000, 10_000, -10_000, 10_000));
            let camera = Camera::new(rect, size);
            self.state.intersect.workspace.camera = camera;
        } else {
            self.state.intersect.workspace.camera.size = size;
        }
    }

    fn intersect_update_solver(&mut self, solver: SolverOption) {
        self.state.intersect.solver = solver;
        self.state.intersect.update_solution();
    }

    fn intersect_update_fill(&mut self, fill: FillOption) {
        self.state.intersect.fill = fill;
        self.state.intersect.update_solution();
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
            fill: FillOption::NonZero,
            mode: ModeOption::Xor,
            solver: SolverOption::Auto,
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
            let editor_points = &mut self.workspace.points;

            if editor_points.is_empty() {
                editor_points.reserve(test.clip_paths.points_count() + test.subj_paths.points_count())
            } else {
                editor_points.clear();
            }

            self.workspace.subj = test.subj_paths.clone();
            self.workspace.clip = test.clip_paths.clone();

            self.workspace.subj.feed_edit_points(0, editor_points);
            self.workspace.clip.feed_edit_points(1, editor_points);

            self.cameras.insert(self.test, self.workspace.camera);
            let mut camera = *self.cameras.get(&index).unwrap_or(&Camera::empty());
            if camera.is_empty() && self.size.width > 0.001 {
                let rect = IntRect::with_iter(editor_points.iter().map(|p| &p.pos))
                    .unwrap_or(IntRect::new(-10_000, 10_000, -10_000, 10_000));
                camera = Camera::new(rect, self.size);
            }

            self.workspace.camera = camera;

            self.test = index;
        }
    }

    fn update_solution(&mut self) {
        let subj = &self.workspace.subj;
        let clip = &self.workspace.clip;
        let fill_rule = self.fill.fill_rule();
        match self.mode {
            ModeOption::Edit => {},
            ModeOption::Debug => {
                self.workspace.vectors = Overlay::with_contours(subj, clip).into_separate_vectors(fill_rule, Default::default());
            },
            _ => {
                let overlay_rule = self.mode.overlay_rule().unwrap();
                let solution = Overlay::with_contours(subj, clip)
                    .into_graph(fill_rule)
                    .extract_shapes_min_area(overlay_rule, 0);
                self.workspace.solution = solution;
            }
        }
    }

    pub(super) fn intersect_update_point(&mut self, update: PointEditUpdate) {
        self.workspace.points[update.index] = update.point.clone();
        let m_index = update.point.index;
        if m_index.group_index == 0 {
            self.workspace.subj[m_index.path_index][m_index.point_index] = update.point.pos;
        } else {
            self.workspace.clip[m_index.path_index][m_index.point_index] = update.point.pos;
        }
        self.update_solution();
    }
}

