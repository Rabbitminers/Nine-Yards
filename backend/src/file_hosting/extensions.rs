
use std::path::{Component, Path};
use std::time::SystemTime;

use mime_guess::{mime, Mime};

use crate::server::PathType;

pub trait MimeExt {
    fn is_compressed_format(&self) -> bool;
    fn guess_charset(&self) -> Option<mime::Name>;
}

impl MimeExt for Mime {
    fn is_compressed_format(&self) -> bool {
        let subtype = self.subtype();
        #[allow(clippy::match_like_matches_macro)]
        match (self.type_(), subtype, subtype.as_str()) {
            (mime::VIDEO | mime::AUDIO, _, _) => true,
            (_, mime::GIF | mime::JPEG | mime::PNG | mime::BMP, _) => true,
            (_, _, "avif" | "webp" | "tiff") => true,
            _ => false,
        }
    }

    fn guess_charset(&self) -> Option<mime::Name> {
        match (self.type_(), self.subtype(), self.suffix()) {
            (mime::TEXT, _, _)
            | (_, mime::XML | mime::JAVASCRIPT | mime::JSON, _)
            | (_, _, Some(mime::XML | mime::JSON)) => Some(mime::UTF_8),
            _ => None,
        } 
    } 
}
