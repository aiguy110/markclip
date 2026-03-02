use std::{io::Read, thread, time::Duration};

use pulldown_cmark::{html as cmark_html, Options as CmarkOptions, Parser};
use wl_clipboard_rs::{
    copy::{MimeSource, MimeType as CopyMimeType, Options as CopyOptions, Source},
    paste::{get_contents, ClipboardType, MimeType as PasteMimeType, Seat},
};

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

trait ClipboardBackend {
    fn has_html(&self) -> bool;
    fn plain_text(&self) -> Option<String>;
    fn set(&self, plain: &str, html: &str) -> Result<()>;
}

struct WaylandBackend;

impl ClipboardBackend for WaylandBackend {
    fn has_html(&self) -> bool {
        get_contents(
            ClipboardType::Regular,
            Seat::Unspecified,
            PasteMimeType::Specific("text/html"),
        )
        .is_ok()
    }

    fn plain_text(&self) -> Option<String> {
        let (mut reader, _mime) = get_contents(
            ClipboardType::Regular,
            Seat::Unspecified,
            PasteMimeType::Text,
        )
        .ok()?;
        let mut text = String::new();
        reader.read_to_string(&mut text).ok()?;
        if text.is_empty() { None } else { Some(text) }
    }

    fn set(&self, plain: &str, html: &str) -> Result<()> {
        CopyOptions::new().copy_multi(vec![
            MimeSource {
                source: Source::Bytes(plain.as_bytes().to_vec().into_boxed_slice()),
                mime_type: CopyMimeType::Text,
            },
            MimeSource {
                source: Source::Bytes(html.as_bytes().to_vec().into_boxed_slice()),
                mime_type: CopyMimeType::Specific("text/html".to_owned()),
            },
        ])?;
        Ok(())
    }
}

fn is_markdown(text: &str) -> bool {
    for line in text.lines() {
        let trimmed = line.trim_start();

        // ATX headers: # followed by space or another #
        if let Some(rest) = trimmed.strip_prefix('#') {
            if rest.starts_with(' ') || rest.starts_with('#') {
                return true;
            }
        }

        // Fenced code blocks
        if trimmed.starts_with("```") || trimmed.starts_with("~~~") {
            return true;
        }

        // Blockquotes
        if trimmed.starts_with("> ") {
            return true;
        }

        // Inline links or images
        if line.contains("](") {
            return true;
        }

        // Setext underlines / thematic breaks: 3+ identical =, -, or *
        let chars: Vec<char> = trimmed.chars().collect();
        if chars.len() >= 3 {
            let first = chars[0];
            if matches!(first, '=' | '-' | '*') && chars.iter().all(|&c| c == first) {
                return true;
            }
        }
    }

    // Balanced inline markers
    for marker in ["**", "__", "~~", "`"] {
        if text.matches(marker).count() >= 2 {
            return true;
        }
    }

    false
}

fn markdown_to_html(text: &str) -> String {
    let options = CmarkOptions::ENABLE_TABLES
        | CmarkOptions::ENABLE_STRIKETHROUGH
        | CmarkOptions::ENABLE_TASKLISTS
        | CmarkOptions::ENABLE_FOOTNOTES;
    let parser = Parser::new_ext(text, options);
    let mut output = String::new();
    cmark_html::push_html(&mut output, parser);
    output
}

fn main() {
    if std::env::var("WAYLAND_DISPLAY").is_err() {
        eprintln!("error: WAYLAND_DISPLAY is not set; markclip requires a Wayland session");
        std::process::exit(1);
    }

    let backend = WaylandBackend;
    loop {
        thread::sleep(Duration::from_millis(500));

        if backend.has_html() {
            continue;
        }

        let Some(plain) = backend.plain_text() else {
            continue;
        };

        if !is_markdown(&plain) {
            continue;
        }

        let html = markdown_to_html(&plain);
        if let Err(e) = backend.set(&plain, &html) {
            eprintln!("error setting clipboard: {e}");
        }
    }
}
