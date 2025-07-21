use mime_guess::MimeGuess;
use std::path::Path;

pub fn is_code_file<P: AsRef<Path>>(path: P) -> bool {
    let path = path.as_ref();

    let extension = match path.extension() {
        Some(ext) => ext.to_str(),
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

fn is_code_extension(extension: Option<&str>) -> bool {
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
    let ext_lower = extension.map(|ext| ext.to_lowercase());
    code_extensions.contains(&ext_lower.as_deref().unwrap_or(""))
}
