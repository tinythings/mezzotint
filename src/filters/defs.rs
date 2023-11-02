/*
Definitions, constants
*/

/// Stub doc files
pub const DOC_STUB_FILES: &[&str] =
    &["AUTHORS", "COPYING", "LICENSE", "DEBUG", "DISTRIB", "DOC", "HISTORY", "README", "TERMS", "TODO"];

/// Docfiles
pub const DOC_F_EXT: &[&str] = &[".txt", ".doc", ".rtf", ".md", ".rtx", ".tex", ".xml", ".htm", ".html", ".log"];

/// Docfiles portable
pub const DOC_FP_EXT: &[&str] = &[".eps", ".pdf", ".ps"];

/// Typically, docs
pub const DOC_LOCATIONS: &[&str] = &["/usr/share/doc"];

/// Headers
pub const H_SRC_F_EXT: &[&str] = &[".h", ".hpp"];

/// Archives
pub const ARC_F_EXT: &[&str] = &[".gz", ".bz2", ".xz", ".zip", ".tar"];

/// Graphic files
pub const IMG_F_EXT: &[&str] = &[
    ".ani", ".bmp", ".dib", ".pcx", ".jpg", ".jpeg", ".jpx", ".jxr", ".png", ".gif", ".xpm", ".xbm", ".tif", ".tiff", ".iff",
    ".lbm", ".pbm", ".pgm", ".pict", ".svg", ".ico", ".ai",
];

/// Manpages
pub const D_MANPAGES: &str = "/usr/share/man";

/// Localisation
pub const D_L10N: &str = "/usr/share/locale";

/// Intetrnaetiomns... i18n
pub const D_I18N: &str = "/usr/share/i18n";
