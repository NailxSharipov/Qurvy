use iced::{Color, Theme};

#[derive(Debug, Clone, Copy)]
pub(crate) struct BezierEditorColorSchema {
    pub(crate) main: Color,
    pub(crate) drag: Color,
    pub(crate) hover: Color,
    pub(crate) line: Color,
}

impl BezierEditorColorSchema {

    pub(crate) fn with_theme(theme: Theme) -> Self {
        let palette = theme.extended_palette();

        if palette.is_dark {
            Self {
                main: Color::WHITE,
                drag: palette.primary.base.color,
                hover: palette.primary.weak.color,
                line: Color::new(0.8,0.8, 1.0, 1.0),
            }
        } else {
            Self {
                main: Color::BLACK,
                drag: palette.primary.base.color,
                hover: palette.primary.weak.color,
                line: Color::new(0.2,0.2, 5.0, 1.0),
            }
        }
    }
}