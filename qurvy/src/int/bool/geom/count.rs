#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ShapeCountBoolean {
    pub subj: i32,
    pub clip: i32,
}

impl ShapeCountBoolean {
    pub(crate) const SUBJ_DIRECT: ShapeCountBoolean = ShapeCountBoolean { subj: 1, clip: 0 };
    pub(crate) const SUBJ_INVERT: ShapeCountBoolean = ShapeCountBoolean { subj: -1, clip: 0 };
    pub(crate) const CLIP_DIRECT: ShapeCountBoolean = ShapeCountBoolean { subj: 0, clip: 1 };
    pub(crate) const CLIP_INVERT: ShapeCountBoolean = ShapeCountBoolean { subj: 0, clip: -1 };
}

impl ShapeCountBoolean {

    #[inline(always)]
    fn new(subj: i32, clip: i32) -> Self { Self { subj, clip } }

    #[inline(always)]
    fn add(self, count: Self) -> Self {
        let subj = self.subj + count.subj;
        let clip = self.clip + count.clip;

        Self { subj, clip }
    }

    #[inline(always)]
    fn apply(&mut self, count: Self) {
        self.subj += count.subj;
        self.clip += count.clip;
    }

    #[inline(always)]
    fn invert(self) -> Self {
        Self { subj: -self.subj, clip: -self.clip }
    }
}