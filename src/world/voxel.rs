// todo: different voxel types/themes, make as compact as possible, maybe a u8 where if no bits are
//  set the voxel is inactive, and otherwise it indicates the theme ID
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