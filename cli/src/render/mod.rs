pub mod blender;
pub mod file;
pub mod geom;
pub mod html_report;
pub mod ifc;
pub mod scene_roll;
pub mod table;
pub mod topology;
pub mod ui;
pub mod viewer_3d;

/// Resolve a (possibly relative) path to an absolute string for a render
/// primitive's output contract: joins the current dir without resolving
/// symlinks or requiring the file to exist. Falls back to the input on the
/// rare `absolute()` error so a downstream node always gets a usable path.
///
/// `render::file` and `render::blender` each previously carried a byte-
/// identical copy (`blender`'s doc comment even flagged the duplication).
/// Extracted here so a future "resolve symlinks" or "reject relative"
/// decision lands in one place.
pub(super) fn abs_path(path: &str) -> String {
    std::path::absolute(path)
        .map(|p| p.to_string_lossy().into_owned())
        .unwrap_or_else(|_| path.to_string())
}
