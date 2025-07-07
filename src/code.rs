use mime_guess::MimeGuess;
use std::path::Path;

pub fn is_code_file<P: AsRef<Path>>(path: P) -> bool {
    let path = path.as_ref();

    let extension = match path.extension() {
        Some(ext) => ext.to_str().unwrap_or(""),
        None => return false,
    };

    let guess = MimeGuess::from_path(path);
    let mime_type = guess.first();

    if let Some(mime) = mime_type {
        let mime_str = mime.to_string();

        if mime_str.starts_with("text/") {
            return is_code_extension(extension);
        }

        match mime_str.as_str() {
            "application/javascript"
            | "application/json"
            | "application/xml"
            | "application/x-httpd-php"
            | "application/x-python-code"
            | "application/x-sh"
            | "application/x-shellscript"
            | "application/x-perl"
            | "application/x-ruby"
            | "application/x-tcl"
            | "application/x-csh"
            | "application/x-javascript" => return true,
            _ => {}
        }
    }

    is_code_extension(extension)
}

pub fn is_code_extension(extension: &str) -> bool {
    let code_extensions = [
        "html",
        "htm",
        "css",
        "js",
        "ts",
        "jsx",
        "tsx",
        "vue",
        "svelte",
        "php",
        "asp",
        "aspx",
        "jsp",
        "erb",
        "ejs",
        "hbs",
        "mustache",
        "rs",
        "py",
        "java",
        "c",
        "cpp",
        "cc",
        "cxx",
        "h",
        "hpp",
        "hxx",
        "cs",
        "vb",
        "go",
        "rb",
        "pl",
        "pm",
        "lua",
        "swift",
        "kt",
        "scala",
        "clj",
        "cljs",
        "hs",
        "ml",
        "fs",
        "fsx",
        "elm",
        "dart",
        "r",
        "sh",
        "bash",
        "zsh",
        "fish",
        "csh",
        "tcsh",
        "ps1",
        "bat",
        "cmd",
        "json",
        "xml",
        "yaml",
        "yml",
        "toml",
        "ini",
        "cfg",
        "conf",
        "properties",
        "plist",
        "gradle",
        "maven",
        "sbt",
        "sql",
        "mysql",
        "pgsql",
        "sqlite",
        "makefile",
        "cmake",
        "dockerfile",
        "containerfile",
        "compose",
        "package",
        "lock",
        "requirements",
        "pipfile",
        "gemfile",
        "md",
        "rst",
        "tex",
        "latex",
        "vim",
        "emacs",
        "gitignore",
        "editorconfig",
        "eslintrc",
        "prettierrc",
        "babelrc",
        "webpack",
        "rollup",
        "vite",
        "tsconfig",
        "jsconfig",
        "tslint",
        "stylelint",
        "m",
        "mm",
        "xib",
        "storyboard",
        "pch",
        "pbxproj",
        "xcconfig",
        "gradle",
        "proguard",
        "manifest",
        "lisp",
        "scm",
        "ss",
        "rkt",
        "el",
        "cl",
        "lsp",
        "ipynb",
        "rmd",
        "qmd",
        "jl",
        "m",
        "wl",
        "nb",
        "cs",
        "js",
        "lua",
        "gd",
        "shader",
        "hlsl",
        "glsl",
        "asm",
        "s",
        "nasm",
        "masm",
        "gas",
        "ld",
        "lds",
    ];
    let ext_lower = extension.to_lowercase();
    code_extensions.contains(&ext_lower.as_str())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_code_file() {
        assert!(is_code_file("main.rs"));
        assert!(is_code_file("script.py"));
        assert!(is_code_file("app.js"));
        assert!(is_code_file("style.css"));
        assert!(is_code_file("index.html"));
        assert!(is_code_file("config.json"));
        assert!(is_code_file("README.md"));

        assert!(!is_code_file("image.jpg"));
        assert!(!is_code_file("document.pdf"));
        assert!(!is_code_file("music.mp3"));
        assert!(!is_code_file("video.mp4"));
        assert!(!is_code_file("archive.zip"));

        assert!(!is_code_file("README"));
        assert!(!is_code_file("file_without_extension"));

        assert!(is_code_file("src/main.rs"));
        assert!(is_code_file("/home/user/project/app.py"));
        assert!(is_code_file("C:\\Users\\project\\index.html"));
    }
}
