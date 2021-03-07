#[derive(Clone, Copy)]
pub enum ColourType {
    GreyScale,
    RGB,
}

#[derive(Clone)]
pub struct ImageData {
    pub colour_type: ColourType,
    pub size: (u32, u32),
    pub bytes: Vec<u8>,
    pub offset: Option<(u32, u32)>,
}
