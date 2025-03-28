use iced::{event, mouse, Color, Element, Event, Length, Point, Rectangle, Renderer, Size, Theme};
use iced::advanced::{layout, renderer, Clipboard, Layout, Shell, Widget};
use iced::advanced::graphics::Mesh;
use iced::advanced::widget::{tree, Tree};
use iced::advanced::widget::tree::State;
use qurvy::int::bezier::anchor::IntBezierAnchor;
use qurvy::int::bezier::path::IntBezierPath;
use crate::bezier_editor::color::BezierEditorColorSchema;
use crate::bezier_editor::state::{ActiveAnchor, AnchorPart, BezierEditorState, MeshCache, SelectState};
use crate::compat::convert::Convert;
use crate::geom::camera::Camera;
use crate::mesh::path_builder::PathBuilder;

pub(crate) struct BezierEditorWidget<'a, Message> {
    pub(super) id: usize,
    pub(super) path: &'a IntBezierPath,
    pub(super) camera: Camera,
    pub(super) schema: BezierEditorColorSchema,
    pub(super) mesh_radius: f32,
    pub(super) hover_radius: f32,
    on_update: Box<dyn Fn(BezierEditorUpdateEvent) -> Message + 'a>,
}

#[derive(Debug, Clone)]
pub(crate) struct BezierEditorUpdateEvent {
    pub(crate) curve_index: usize,
    pub(crate) anchor_index: usize,
    pub(crate) anchor: IntBezierAnchor,
}

impl<'a, Message> BezierEditorWidget<'a, Message> {
    pub(crate) fn new(id: usize, path: &'a IntBezierPath, camera: Camera, on_update: impl Fn(BezierEditorUpdateEvent) -> Message + 'a) -> Self {
        Self {
            id,
            path,
            camera,
            mesh_radius: 6.0,
            hover_radius: 12.0,
            schema: BezierEditorColorSchema::with_theme(Theme::default()),
            on_update: Box::new(on_update),
        }
    }

    pub(crate) fn set_schema(mut self, schema: BezierEditorColorSchema) -> Self {
        self.schema = schema;
        self
    }
}

impl<Message> Widget<Message, Theme, Renderer> for BezierEditorWidget<'_, Message> {

    fn tag(&self) -> tree::Tag {
        tree::Tag::of::<BezierEditorState>()
    }

    fn state(&self) -> State {
        State::new(BezierEditorState::default())
    }

    fn size(&self) -> Size<Length> {
        Size {
            width: Length::Fill,
            height: Length::Fill,
        }
    }

    fn layout(
        &self,
        tree: &mut Tree,
        _renderer: &Renderer,
        limits: &layout::Limits,
    ) -> layout::Node {
        if let State::Some(state_box) = &mut tree.state {
            state_box.downcast_mut::<BezierEditorState>().unwrap()
                .update_mesh(
                    self.mesh_radius,
                    self.schema
                )
        };

        layout::Node::new(limits.max())
    }

    fn on_event(
        &mut self,
        tree: &mut Tree,
        event: Event,
        layout: Layout<'_>,
        cursor: mouse::Cursor,
        _renderer: &Renderer,
        _clipboard: &mut dyn Clipboard,
        shell: &mut Shell<'_, Message>,
        _viewport: &Rectangle,
    ) -> event::Status {
        let state = tree.state.downcast_mut::<BezierEditorState>();


        let bounds = layout.bounds();
        if let Event::Mouse(mouse_event) = event {
            match mouse_event {
                mouse::Event::CursorMoved { position } => {
                    if bounds.contains(position) {
                        let view_cursor = position - bounds.position();
                        if let Some(updated_point) = state.mouse_move(
                            &*self,
                            view_cursor,
                        ) {
                            shell.publish((self.on_update)(updated_point));
                            return event::Status::Captured;
                        }
                    }
                }
                mouse::Event::ButtonPressed(mouse::Button::Left) => {
                    let position = cursor.position().unwrap_or(Point::ORIGIN);
                    if bounds.contains(position) {
                        let view_cursor = position - bounds.position();
                        if state.mouse_press(
                            &*self,
                            view_cursor,
                        ) {
                            return event::Status::Captured;
                        }
                    }
                }
                mouse::Event::ButtonReleased(mouse::Button::Left) => {
                    let position = cursor.position().unwrap_or(Point::ORIGIN);
                    let view_cursor = position - bounds.position();
                    if state.mouse_release(
                        &*self,
                        view_cursor,
                    ) {
                        return event::Status::Captured;
                    }
                }
                _ => {}
            }
        }

        event::Status::Ignored
    }

    fn draw(
        &self,
        tree: &Tree,
        renderer: &mut Renderer,
        _theme: &Theme,
        _style: &renderer::Style,
        layout: Layout<'_>,
        _cursor: mouse::Cursor,
        _viewport: &Rectangle,
    ) {
        let state = tree.state.downcast_ref::<BezierEditorState>();

        let mesh_cache = if let Some(mesh) = &state.mesh_cache { mesh } else { return; };

        use iced::advanced::graphics::mesh::Renderer as _;
        use iced::advanced::Renderer as _;

        let offset = layout.position() - Point::new(self.mesh_radius, self.mesh_radius);

        for (index, anchor) in self.path.anchors.iter().enumerate() {
            let main = self.camera.world_to_screen(offset, anchor.point.convert());

            let mut path_builder = PathBuilder::new(self.camera, offset.convert());
            if let Some(handle) = anchor.handle_in_point() {
                path_builder.add_segment(main.convert(), handle.convert(), 1.0);
            }

            if let Some(handle) = anchor.handle_out_point() {
                path_builder.add_segment(main.convert(), handle.convert(), 1.0);
            }

            if let Some(mesh) = path_builder.into_mesh(Color::new(0.8, 0.8, 1.0, 1.0)) {
                renderer.with_translation(main, |renderer| renderer.draw_mesh(mesh));
            }
            {
                let mesh = mesh_cache.point_mesh(index, state.active_anchor);
                renderer.with_translation(main, |renderer| renderer.draw_mesh(mesh));
            }

            if let Some(point) = anchor.handle_in_point() {
                let mesh = mesh_cache.handle_in_mesh(index, state.active_anchor);
                renderer.with_translation(point.convert(), |renderer| renderer.draw_mesh(mesh));
            }

            if let Some(point) = anchor.handle_out_point() {
                let mesh = mesh_cache.handle_out_mesh(index, state.active_anchor);
                renderer.with_translation(point.convert(), |renderer| renderer.draw_mesh(mesh));
            }
        }
    }
}

impl MeshCache {

    #[inline]
    fn point_mesh(&self, index: usize, active_anchor: Option<ActiveAnchor>) -> Mesh {
        let active = if let Some(active) = active_anchor {
            active
        } else {
            return self.point.clone();
        };
        if active.index != index || active.part != AnchorPart::Point {
            return self.point.clone();
        }
        match active.select_state {
            SelectState::Hover => self.hover.clone(),
            SelectState::Drag(_) => self.drag.clone()
        }
    }

    #[inline]
    fn handle_in_mesh(&self, index: usize, active_anchor: Option<ActiveAnchor>) -> Mesh {
        let active = if let Some(active) = active_anchor {
            active
        } else {
            return self.point.clone();
        };
        if active.index != index || active.part != AnchorPart::HandleIn {
            return self.point.clone();
        }
        match active.select_state {
            SelectState::Hover => self.hover.clone(),
            SelectState::Drag(_) => self.drag.clone()
        }
    }

    #[inline]
    fn handle_out_mesh(&self, index: usize, active_anchor: Option<ActiveAnchor>) -> Mesh {
        let active = if let Some(active) = active_anchor {
            active
        } else {
            return self.point.clone();
        };
        if active.index != index || active.part != AnchorPart::HandleOut {
            return self.point.clone();
        }
        match active.select_state {
            SelectState::Hover => self.hover.clone(),
            SelectState::Drag(_) => self.drag.clone()
        }
    }
}


impl<'a, Message: 'a> From<BezierEditorWidget<'a, Message>> for Element<'a, Message> {
    fn from(editor: BezierEditorWidget<'a, Message>) -> Self {
        Self::new(editor)
    }
}