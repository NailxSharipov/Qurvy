use i_triangle::i_overlay::i_float::float::point::FloatPoint;
use i_triangle::stroke::butt::ButtStrokeBuilder;
use i_triangle::stroke::style::StrokeStyle;
use i_triangle::triangulation::float::TriangulationBuilder;
use iced::{Color, Rectangle, Transformation};
use iced::advanced::graphics::color::pack;
use iced::advanced::graphics::Mesh;
use iced::advanced::graphics::mesh::{Indexed, SolidVertex2D};
use qurvy::int::math::point::IntPoint;
use crate::compat::convert::Convert;
use crate::geom::camera::Camera;

pub(crate) struct PathBuilder {
    camera: Camera,
    offset: FloatPoint<f32>,
    builder: TriangulationBuilder<FloatPoint<f32>>,
}

impl PathBuilder {

    #[inline]
    pub(crate) fn new(camera: Camera, offset: FloatPoint<f32>) -> Self {
        Self { camera, offset, builder: TriangulationBuilder::new() }
    }

    #[inline]
    pub(crate) fn add_segment(&mut self, a: FloatPoint<f32>, b: FloatPoint<f32>, width: f32) {
        let stroke_builder = ButtStrokeBuilder::new(StrokeStyle::with_width(width));
        let sub_triangulation = stroke_builder.build_open_path_mesh(&[a, b]);
        self.builder.append(sub_triangulation);
    }

    #[inline]
    pub(crate) fn add_paths(&mut self, points: &[IntPoint], closed: bool, width: f32) {
        let float_points: Vec<_> = points.iter()
            .map(|p| {
                let screen = self.camera.world_to_screen(self.offset.convert(), p.convert());
                screen.convert()
            }).collect();

        let stroke_builder = ButtStrokeBuilder::new(StrokeStyle::with_width(width));
        let sub_triangulation = if closed {
            stroke_builder.build_closed_path_mesh(&float_points)
        } else {
            stroke_builder.build_open_path_mesh(&float_points)
        };
        self.builder.append(sub_triangulation);
    }


    pub(crate) fn into_mesh(self, color: Color) -> Option<Mesh> {
        let triangulation = self.builder.build();
        if triangulation.indices.is_empty() {
            return None;
        }
        let color_pack = pack(color);
        let vertices = triangulation.points.iter().map(|&p| {
            let dp = p - self.offset;
            SolidVertex2D { position: [dp.x, dp.y], color: color_pack }
        }).collect();

        let indices = triangulation.indices.iter().map(|&i| i as u32).collect();

        Some(Mesh::Solid {
            buffers: Indexed { vertices, indices },
            transformation: Transformation::translate(self.offset.x, self.offset.y),
            clip_bounds: Rectangle::INFINITE,
        })
    }
}