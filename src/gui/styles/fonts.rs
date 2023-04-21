use iced::Font;
use once_cell::sync::OnceCell;
use plotters::style::RGBColor;
use std::collections::HashMap;

use crate::translations::types::language::Language;

pub type FontsMap = HashMap<&'static str, Font>;

pub static FONTS: OnceCell<FontsMap> = OnceCell::new();

struct FontMetadata {
    pub name: &'static str,
    pub file: &'static str,
}

const FONT_METADATA: [FontMetadata; 9] = [
    FontMetadata { name: "noto-sans-symbols", file: "NotoSansSymbols2-Regular.ttf" },
    FontMetadata { name: "noto-sans-regular", file: "NotoSans-Regular.ttf" },
    FontMetadata { name: "noto-sans-bold", file: "NotoSans-Bold.ttf" },
    FontMetadata { name: "noto-naskh-arabic-regular", file: "NotoNaskhArabic-Regular.ttf" },
    FontMetadata { name: "noto-naskh-arabic-bold", file: "NotoNaskhArabic-Bold.ttf" },
    FontMetadata { name: "noto-sans-korean-regular", file: "NotoSansKR-Regular.otf" },
    FontMetadata { name: "noto-sans-korean-bold", file: "NotoSansKR-Bold.otf" },
    FontMetadata { name: "noto-sans-chinese-regular", file: "NotoSansSC-Regular.otf" },
    FontMetadata { name: "noto-sans-chinese-bold", file: "NotoSansSC-Bold.otf" },
];

async fn download_font(base_url: &str, metadata: &FontMetadata) -> Result<Font, reqwest::Error> {
    let url = format!("{}/{}", base_url, metadata.file);
    let client = reqwest::Client::builder().redirect(reqwest::redirect::Policy::none()).build()?;
    let response = client.get(url).send().await?;
    let content_type = response.headers().get(reqwest::header::CONTENT_TYPE).map(|ct| ct.to_str().unwrap_or("")).unwrap_or("");

    if response.status() != reqwest::StatusCode::OK
    || content_type != "application/octet-stream" {
        return Ok(Font::Default);
    }

    let bytes = response.bytes().await?;
    let font_data = Box::new(bytes);
    let font = Font::External {
        name: metadata.name,
        bytes: &*Box::leak(font_data)
    };
    Ok(font)
}

#[tokio::main]
pub async fn load_fonts() {
    let base_url = String::from("https://raw.githubusercontent.com/GyulyVGC/sniffnet/main/resources/fonts/noto");

    let mut download_futures = Vec::new();

    for metadata in &FONT_METADATA {
        download_futures.push(download_font(&base_url, metadata));
    }

    let mut fonts_map = HashMap::new();
    for (i, font_result) in download_futures.into_iter().enumerate() {
        if let Ok(font) = font_result.await {
            fonts_map.insert(FONT_METADATA[i].name, font);
        }
    }

    FONTS.set(fonts_map).ok();
}

pub fn get_language_font(color: RGBColor, language: Language) -> Font {
    match color {
        // if white non-bold
        RGBColor(255, 255, 255) => match language {
            Language::FA => FONTS.get().and_then(|f| f.get("noto-naskh-arabic-regular")).unwrap_or(&Font::Default).clone(),
            Language::KO => FONTS.get().and_then(|f| f.get("noto-sans-korean-regular")).unwrap_or(&Font::Default).clone(),
            Language::ZH => FONTS.get().and_then(|f| f.get("noto-sans-chinese-regular")).unwrap_or(&Font::Default).clone(),
            _ => FONTS.get().and_then(|f| f.get("noto-sans-regular")).unwrap_or(&Font::Default).clone(),
        },
        _ => match language {
            Language::FA => FONTS.get().and_then(|f| f.get("noto-naskh-arabic-bold")).unwrap_or(&Font::Default).clone(),
            Language::KO => FONTS.get().and_then(|f| f.get("noto-sans-korean-bold")).unwrap_or(&Font::Default).clone(),
            Language::ZH => FONTS.get().and_then(|f| f.get("noto-sans-chinese-bold")).unwrap_or(&Font::Default).clone(),
            _ => FONTS.get().and_then(|f| f.get("noto-sans-bold")).unwrap_or(&Font::Default).clone(),
        },
    }
}

pub fn get_symbols_font() -> Font {
    FONTS.get().and_then(|f| f.get("noto-sans-symbols")).unwrap_or(&Font::Default).clone()
}