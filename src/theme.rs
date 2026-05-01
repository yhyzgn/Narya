use gpui::{rgb, Rgba};

#[allow(dead_code)]
pub struct Theme {
    pub bg: Rgba,
    pub bg_gradient_start: Rgba,
    pub bg_gradient_end: Rgba,
    pub surface: Rgba,
    pub border: Rgba,
    pub primary: Rgba,
    pub primary_light: Rgba,
    pub success: Rgba,
    pub warning: Rgba,
    pub danger: Rgba,
    pub text_primary: Rgba,
    pub text_secondary: Rgba,
    pub text_muted: Rgba,
}

impl Default for Theme {
    fn default() -> Self {
        Self {
            bg: rgb(0xf9fafc),
            bg_gradient_start: rgb(0xecf2fd),
            bg_gradient_end: rgb(0xb1a1f5), // Approximate bottom_left from splash spec
            surface: Rgba {
                r: 1.0,
                g: 1.0,
                b: 1.0,
                a: 0.7,
            },
            border: Rgba {
                r: 0.49, // 126/255
                g: 0.56, // 143/255
                b: 0.67, // 170/255
                a: 0.2,
            },
            primary: rgb(0x4f46e5),
            primary_light: rgb(0x8b9ce8),
            success: rgb(0x10b981),
            warning: rgb(0xf59e0b),
            danger: rgb(0xef4444),
            text_primary: rgb(0x1e293b),
            text_secondary: rgb(0x475569),
            text_muted: rgb(0x94a3b8),
        }
    }
}
