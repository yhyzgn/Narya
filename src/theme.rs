use gpui::{rgb, Rgba};

pub struct Theme {
    pub bg: Rgba,
    pub surface: Rgba,
    pub border: Rgba,
    pub primary: Rgba,
    pub success: Rgba,
    pub text_primary: Rgba,
    pub text_secondary: Rgba,
}

impl Default for Theme {
    fn default() -> Self {
        Self {
            bg: rgb(0xf9fafc),
            surface: Rgba { r: 1.0, g: 1.0, b: 1.0, a: 0.8 },
            border: Rgba { r: 0.88, g: 0.90, b: 0.92, a: 0.5 },
            primary: rgb(0x4f46e5),
            success: rgb(0x10b981),
            text_primary: rgb(0x1e293b),
            text_secondary: rgb(0x64748b),
        }
    }
}
