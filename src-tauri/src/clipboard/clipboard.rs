// Copyright 2019 The Druid Authors.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

//! Interacting with the system pasteboard/clipboard. 
//! Copy from https://github.com/linebender/druid/blob/master/druid-shell/src/clipboard.rs
pub use crate::clipboard::backend::clipboard as backend;

#[derive(Debug, Clone)]
pub struct Clipboard(pub(crate) backend::Clipboard);

impl Clipboard {
    /// Put a string onto the system clipboard.
    pub fn put_string(&mut self, s: impl AsRef<str>) {
        self.0.put_string(s);
    }

    /// Put multi-format data on the system clipboard.
    pub fn put_formats(&mut self, formats: &[ClipboardFormat]) {
        self.0.put_formats(formats)
    }

    /// Get a string from the system clipboard, if one is available.
    pub fn get_string(&self) -> Option<String> {
        self.0.get_string()
    }

    /// Given a list of supported clipboard types, returns the supported type which has
    /// highest priority on the system clipboard, or `None` if no types are supported.
    pub fn preferred_format(&self, formats: &[FormatId]) -> Option<FormatId> {
        self.0.preferred_format(formats)
    }

    /// Return data in a given format, if available.
    ///
    /// It is recommended that the [`FormatId`] argument be a format returned by
    /// [`Clipboard::preferred_format`].
    ///
    /// [`Clipboard::preferred_format`]: struct.Clipboard.html#method.preferred_format
    /// [`FormatId`]: type.FormatId.html
    pub fn get_format(&self, format: FormatId) -> Option<Vec<u8>> {
        self.0.get_format(format)
    }

    /// For debugging: print the resolved identifiers for each type currently
    /// on the clipboard.
    #[doc(hidden)]
    pub fn available_type_names(&self) -> Vec<String> {
        self.0.available_type_names()
    }
}

/// A type identifier for the system clipboard.
///
/// These should be [`UTI` strings] on macOS, and (by convention?) [MIME types] elsewhere.
///
/// [`UTI` strings]: https://escapetech.eu/manuals/qdrop/uti.html
/// [MIME types]: https://developer.mozilla.org/en-US/docs/Web/HTTP/Basics_of_HTTP/MIME_types
pub type FormatId = &'static str;

/// Data coupled with a type identifier.
#[derive(Debug, Clone)]
#[cfg_attr(feature = "wayland", allow(dead_code))]
#[cfg_attr(target_arch = "wasm32", allow(dead_code))]
pub struct ClipboardFormat {
    pub(crate) identifier: FormatId,
    pub(crate) data: Vec<u8>,
}

impl ClipboardFormat {
    /// Create a new `ClipboardFormat` with the given `FormatId` and bytes.
    ///
    /// You are responsible for ensuring that this data can be interpreted
    /// as the provided format.
    pub fn new(identifier: FormatId, data: impl Into<Vec<u8>>) -> Self {
        let data = data.into();
        ClipboardFormat { identifier, data }
    }
}

impl From<String> for ClipboardFormat {
    fn from(src: String) -> ClipboardFormat {
        let data = src.into_bytes();
        ClipboardFormat::new(ClipboardFormat::TEXT, data)
    }
}

impl From<&str> for ClipboardFormat {
    fn from(src: &str) -> ClipboardFormat {
        src.to_string().into()
    }
}

impl From<backend::Clipboard> for Clipboard {
    fn from(src: backend::Clipboard) -> Clipboard {
        Clipboard(src)
    }
}

cfg_if::cfg_if! {
    if #[cfg(target_os = "macos")] {
        impl ClipboardFormat {
            pub const PDF: &'static str = "com.adobe.pdf";
            pub const TEXT: &'static str = "public.utf8-plain-text";
            pub const SVG: &'static str = "public.svg-image";
        }
    } else {
        impl ClipboardFormat {
            cfg_if::cfg_if! {
                if #[cfg(any(target_os = "freebsd", target_os = "linux", target_os = "openbsd"))] {
                    // trial and error; this is the most supported string type for gtk?
                    pub const TEXT: &'static str = "UTF8_STRING";
                } else {
                    pub const TEXT: &'static str = "text/plain";
                }
            }
            pub const PDF: &'static str = "application/pdf";
            pub const SVG: &'static str = "image/svg+xml";
        }
    }
}