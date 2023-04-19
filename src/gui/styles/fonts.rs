use iced::Font;

pub const NOTO_SANS_SYMBOLS: Font = Font::External {
    name: "noto-sans-symbols",
    bytes: include_bytes!("../../../resources/fonts/noto-sans-symbols.ttf"),
};

pub const NOTO_SANS_REGULAR: Font = Font::External {
    name: "noto-sans-regular",
    bytes: include_bytes!("../../../resources/fonts/noto-sans-regular.ttf"),
};

pub const NOTO_SANS_BOLD: Font = Font::External {
    name: "noto-sans-bold",
    bytes: include_bytes!("../../../resources/fonts/noto-sans-bold.ttf"),
};

pub const NOTO_NASKH_ARABIC_REGULAR: Font = Font::External {
    name: "noto-naskh-arabic-regular",
    bytes: include_bytes!("../../../resources/fonts/noto-naskh-arabic-regular.ttf"),
};

pub const NOTO_NASKH_ARABIC_BOLD: Font = Font::External {
    name: "noto-naskh-arabic-bold",
    bytes: include_bytes!("../../../resources/fonts/noto-naskh-arabic-bold.ttf"),
};

pub const NOTO_SANS_KOREAN_REGULAR: Font = Font::External {
    name: "noto-sans-korean-regular",
    bytes: include_bytes!("../../../resources/fonts/noto-sans-korean-regular.otf"),
};

pub const NOTO_SANS_KOREAN_BOLD: Font = Font::External {
    name: "noto-sans-korean-bold",
    bytes: include_bytes!("../../../resources/fonts/noto-sans-korean-bold.otf"),
};

pub const NOTO_SANS_CHINESE_REGULAR: Font = Font::External {
    name: "noto-sans-chinese-regular",
    bytes: include_bytes!("../../../resources/fonts/noto-sans-chinese-regular.otf"),
};

pub const NOTO_SANS_CHINESE_BOLD: Font = Font::External {
    name: "noto-sans-chinese-bold",
    bytes: include_bytes!("../../../resources/fonts/noto-sans-chinese-bold.otf"),
};

#[macro_export]
macro_rules! font_selector {
    ($color:expr, $language:expr) => {
        match $color {
            // if white non-bold
            RGBColor(255, 255, 255) => match $language {
                Language::FA => fonts::NOTO_NASKH_ARABIC_REGULAR,
                Language::KO => fonts::NOTO_SANS_KOREAN_REGULAR,
                Language::ZH => fonts::NOTO_SANS_CHINESE_REGULAR,
                _ => fonts::NOTO_SANS_REGULAR,
            },
            _ => match $language {
                Language::FA => fonts::NOTO_NASKH_ARABIC_BOLD,
                Language::KO => fonts::NOTO_SANS_KOREAN_BOLD,
                Language::ZH => fonts::NOTO_SANS_CHINESE_BOLD,
                _ => fonts::NOTO_SANS_BOLD,
            }
        }
    };
}