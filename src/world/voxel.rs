#[derive(Copy, Clone, Debug)]
pub(crate) struct Voxel {
    pub(crate) active: bool
}

impl Voxel {
    pub(crate) const fn active() -> Self {
        Self { active: true }
    }
    
    pub(crate) const fn inactive() -> Self {
        Self { active: false }
    }
}