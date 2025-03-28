use iced::advanced::graphics::color::pack;
use iced::{Rectangle, Transformation, Vector};

use crate::bezier_editor::color::BezierEditorColorSchema;
use crate::bezier_editor::widget::{BezierEditorUpdateEvent, BezierEditorWidget};
use crate::geom::camera::Camera;
use crate::geom::vector::VectorExt;
use iced::advanced::graphics::Mesh;
use iced::advanced::graphics::mesh::{Indexed, SolidVertex2D};
use qurvy::int::bezier::anchor::IntBezierAnchor;
use qurvy::int::bezier::path::IntBezierPath;
use qurvy::int::math::offset::IntOffset;
use qurvy::int::math::point::IntPoint;
use crate::compat::convert::Convert;

#[derive(Clone)]
pub(super) struct MeshCache {
    radius: f32,
    pub(super) point: Mesh,
    pub(super) drag: Mesh,
    pub(super) hover: Mesh,
}

#[derive(Clone, Copy)]
pub(super) struct ActiveAnchor {
    pub(super) index: usize,
    pub(super) part: AnchorPart,
    pub(super) select_state: SelectState,
}

#[derive(Clone, Copy, PartialEq)]
pub(super) enum AnchorPart {
    Point,
    HandleIn,
    HandleOut,
}

#[derive(Clone, Copy)]
pub(super) struct DragData {
    start_cursor: Vector<f32>,
    start_world: IntPoint,
    part: AnchorPart,
}

#[derive(Clone, Copy)]
pub(super) enum SelectState {
    Hover,
    Drag(DragData),
}

pub(crate) struct BezierEditorState {
    pub(super) mesh_cache: Option<MeshCache>,
    pub(super) active_anchor: Option<ActiveAnchor>,
}

impl BezierEditorState {
    pub(crate) fn update_mesh(&mut self, r: f32, schema: BezierEditorColorSchema) {
        let radius = if let Some(cache) = &self.mesh_cache {
            cache.radius
        } else {
            0.0
        };

        if (radius - r).abs() < 0.1 {
            return;
        }

        let sr = 1.2 * r;

        let mut main_vertices = Vec::with_capacity(4);
        let mut hover_vertices = Vec::with_capacity(4);
        let mut drag_vertices = Vec::with_capacity(4);
        let mut indices = Vec::with_capacity(6);
        let main_pack = pack(schema.main);
        let hover_pack = pack(schema.hover);
        let drag_pack = pack(schema.drag);

        main_vertices.push(SolidVertex2D {
            position: [0.0, r],
            color: main_pack,
        });
        main_vertices.push(SolidVertex2D {
            position: [r, 2.0 * r],
            color: main_pack,
        });
        main_vertices.push(SolidVertex2D {
            position: [2.0 * r, r],
            color: main_pack,
        });
        main_vertices.push(SolidVertex2D {
            position: [r, 0.0],
            color: main_pack,
        });

        hover_vertices.push(SolidVertex2D {
            position: [0.0, sr],
            color: hover_pack,
        });
        hover_vertices.push(SolidVertex2D {
            position: [r, 2.0 * sr],
            color: hover_pack,
        });
        hover_vertices.push(SolidVertex2D {
            position: [2.0 * sr, sr],
            color: hover_pack,
        });
        hover_vertices.push(SolidVertex2D {
            position: [sr, 0.0],
            color: hover_pack,
        });

        drag_vertices.push(SolidVertex2D {
            position: [0.0, r],
            color: drag_pack,
        });
        drag_vertices.push(SolidVertex2D {
            position: [r, 2.0 * r],
            color: drag_pack,
        });
        drag_vertices.push(SolidVertex2D {
            position: [2.0 * r, r],
            color: drag_pack,
        });
        drag_vertices.push(SolidVertex2D {
            position: [r, 0.0],
            color: drag_pack,
        });

        indices.push(0);
        indices.push(1);
        indices.push(2);

        indices.push(0);
        indices.push(2);
        indices.push(3);

        self.mesh_cache = Some(MeshCache {
            radius: r,
            point: Mesh::Solid {
                buffers: Indexed {
                    vertices: main_vertices,
                    indices: indices.clone(),
                },
                transformation: Transformation::IDENTITY,
                clip_bounds: Rectangle::INFINITE,
            },
            hover: Mesh::Solid {
                buffers: Indexed {
                    vertices: hover_vertices,
                    indices: indices.clone(),
                },
                transformation: Transformation::translate(r - sr, r - sr),
                clip_bounds: Rectangle::INFINITE,
            },
            drag: Mesh::Solid {
                buffers: Indexed {
                    vertices: drag_vertices,
                    indices,
                },
                transformation: Transformation::IDENTITY,
                clip_bounds: Rectangle::INFINITE,
            },
        });
    }

    pub(super) fn mouse_press<M>(
        &mut self,
        widget: &BezierEditorWidget<M>,
        cursor: Vector<f32>,
    ) -> bool {
        let closet_point = if let Some(point) = Self::find_closest_point(widget.camera, widget.hover_radius, &widget.path, cursor) {
            point
        } else {
            return false;
        };

        let drag = DragData {
            start_cursor: cursor,
            start_world: closet_point.point,
            part: closet_point.part
        };

        self.active_anchor = Some(ActiveAnchor {
            index: closet_point.index,
            part: closet_point.part,
            select_state: SelectState::Drag(drag),
        });

        true
    }

    pub(super) fn mouse_release<M>(
        &mut self,
        widget: &BezierEditorWidget<M>,
        cursor: Vector<f32>,
    ) -> bool {
        let active_anchor = if let Some(active) = self.active_anchor {
            active
        } else {
            return false;
        };

        if let SelectState::Drag(_) = &active_anchor.select_state {
            self.active_anchor = None;
            self.mouse_hover(widget.camera, widget.hover_radius, widget.path, cursor);
            true
        } else {
            false
        }
    }

    pub(super) fn mouse_move<M>(
        &mut self,
        widget: &BezierEditorWidget<M>,
        cursor: Vector<f32>,
    ) -> Option<BezierEditorUpdateEvent> {
        let active_state = &self.active_anchor?;
        if let SelectState::Drag(drag) = &active_state.select_state {
            Self::mouse_drag(
                widget.id,
                active_state.index, drag, widget.camera, widget.path, cursor)
        } else {
            self.mouse_hover(widget.camera, widget.hover_radius, widget.path, cursor);
            None
        }
    }

    fn mouse_drag(
        curve_index: usize,
        anchor_index: usize,
        drag: &DragData,
        camera: Camera,
        path: &IntBezierPath,
        cursor: Vector<f32>,
    ) -> Option<BezierEditorUpdateEvent> {
        let translate = cursor - drag.start_cursor;
        let world_dist = camera.view_distance_to_world(translate).round();
        let world_point = world_dist + drag.start_world.convert();
        let mut anchor = path.anchors[anchor_index];
        let real_point = drag.part.point(&anchor);
        if world_point != real_point.convert() {
            drag.part.update_point(world_point.convert(), &mut anchor);
            // return Some(AnchorEd { index: anchor_index, anchor });
            return Some(BezierEditorUpdateEvent {
                curve_index,
                anchor_index,
                anchor,
            });
        }

        None
    }

    fn mouse_hover(&mut self, camera: Camera, radius: f32, path: &IntBezierPath, cursor: Vector<f32>) {
        let closet_point = if let Some(close_point) = Self::find_closest_point(camera, radius, &path, cursor) {
            close_point
        } else {
            return;
        };
        self.active_anchor = Some(ActiveAnchor {
            index: closet_point.index,
            part: closet_point.part,
            select_state: SelectState::Hover,
        });
    }

    fn sqr_length(a: &Vector, b: &Vector) -> f32 {
        let dx = a.x - b.x;
        let dy = a.y - b.y;
        dx * dx + dy * dy
    }

    fn find_closest_point(camera: Camera, radius: f32, path: &IntBezierPath, cursor: Vector<f32>) -> Option<ClosestPoint> {
        let mut min_ds = radius.powi(2);
        let mut closest_point = ClosestPoint {
            index: usize::MAX,
            part: AnchorPart::Point,
            point: IntPoint { x: 0, y: 0 }
        };

        for (i, anchor) in path.anchors.iter().enumerate() {
            let view_pos = camera.int_world_to_view(anchor.point);
            let ds = Self::sqr_length(&cursor, &view_pos);
            if ds <= min_ds {
                min_ds = ds;
                closest_point.index = i;
                closest_point.part = AnchorPart::Point;
                closest_point.point = anchor.point;
            }
            if let Some(point) = anchor.handle_in_point() {
                let view_pos = camera.int_world_to_view(point);
                let ds = Self::sqr_length(&cursor, &view_pos);
                if ds <= min_ds {
                    min_ds = ds;
                    closest_point.index = i;
                    closest_point.part = AnchorPart::HandleIn;
                    closest_point.point = point;
                }
            }
            if let Some(point) = anchor.handle_out_point() {
                let view_pos = camera.int_world_to_view(point);
                let ds = Self::sqr_length(&cursor, &view_pos);
                if ds <= min_ds {
                    min_ds = ds;
                    closest_point.index = i;
                    closest_point.part = AnchorPart::HandleOut;
                    closest_point.point = point;
                }
            }
        }

        if closest_point.index == usize::MAX {
            return None;
        }

        Some(closest_point)
    }
}

impl AnchorPart {
    fn point(&self, anchor: &IntBezierAnchor) -> IntPoint {
        match self {
            AnchorPart::Point => anchor.point,
            AnchorPart::HandleIn => anchor.point + anchor.handle_in.unwrap_or(IntOffset::zero()),
            AnchorPart::HandleOut => anchor.point + anchor.handle_out.unwrap_or(IntOffset::zero()),
        }
    }

    fn update_point(&self, point: IntPoint, anchor: &mut IntBezierAnchor) {
        match self {
            AnchorPart::Point => anchor.point = point,
            AnchorPart::HandleIn => anchor.handle_in = Some((point - anchor.point).into()),
            AnchorPart::HandleOut => anchor.handle_out = Some((point - anchor.point).into()),
        }
    }
}

struct ClosestPoint {
    index: usize,
    part: AnchorPart,
    point: IntPoint,
}

impl Default for BezierEditorState {
    fn default() -> Self {
        Self {
            mesh_cache: None,
            active_anchor: Default::default()
        }
    }
}
