use crate::data::intersect::IntersectResource;

pub struct AppResource {
    pub(crate) intersect: IntersectResource,
}

impl AppResource {
    #[cfg(not(target_arch = "wasm32"))]
    pub fn with_paths(intersect: &str) -> Self {
        Self {
            intersect: IntersectResource::with_path(intersect),
        }
    }

    #[cfg(target_arch = "wasm32")]
    pub fn with_content(boolean: String, string: String, stroke: String, outline: String) -> Self {
        Self {
            intersect: BooleanResource::with_content(boolean),
            string: StringResource::with_content(string),
            stroke: StrokeResource::with_content(stroke),
            outline: OutlineResource::with_content(outline),
        }
    }

}