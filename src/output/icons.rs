use ansi_term::Style;
use phf::{phf_map, Map};

use crate::fs::File;

#[non_exhaustive]
struct Icons;

impl Icons {
    const AUDIO: char           = '\u{f001}';  // 
    const BINARY: char          = '\u{eae8}';  // 
    const BOOK: char            = '\u{e28b}';  // 
    const CALENDAR: char        = '\u{eab0}';  // 
    const COMPRESSED: char      = '\u{f410}';  // 
    const CONFIG: char          = '\u{e615}';  // 
    const CONFIG_FOLDER: char   = '\u{e5fc}';  // 
    const CSS3: char            = '\u{e749}';  // 
    const DATABASE: char        = '\u{f1c0}';  // 
    const DIFF: char            = '\u{f440}';  // 
    const DISK_IMAGE: char      = '\u{e271}';  // 
    const DOCKER: char          = '\u{e650}';  // 
    const DOCUMENT: char        = '\u{f1c2}';  // 
    const EMACS: char           = '\u{e632}';  // 
    const FONT: char            = '\u{f031}';  // 
    const GIST_SECRET: char     = '\u{eafa}';  // 
    const GIT: char             = '\u{f1d3}';  // 
    const GRADLE: char          = '\u{e660}';  // 
    const GRUNT: char           = '\u{e611}';  // 
    const GULP: char            = '\u{e610}';  // 
    const HEADER: char          = '\u{f0fd}';  // 
    const HTML5: char           = '\u{f13b}';  // 
    const IMAGE: char           = '\u{f1c5}';  // 
    const INTELLIJ: char        = '\u{e7b5}';  // 
    const KEY: char             = '\u{eb11}';  // 
    const KEYPASS: char         = '\u{f23e}';  // 
    const JSON: char            = '\u{e60b}';  // 
    const LANG_ASSEMBLY: char   = '\u{e637}';  // 
    const LANG_C: char          = '\u{e61e}';  // 
    const LANG_CPP: char        = '\u{e61d}';  // 
    const LANG_CSHARP: char     = '\u{f031b}'; // 󰌛
    const LANG_ELIXIR: char     = '\u{e62d}';  // 
    const LANG_FSHARP: char     = '\u{e7a7}';  // 
    const LANG_GO: char         = '\u{e724}';  // 
    const LANG_HASKELL: char    = '\u{e777}';  // 
    const LANG_JAVA: char       = '\u{e256}';  // 
    const LANG_JAVASCRIPT: char = '\u{e74e}';  // 
    const LANG_OCAML: char      = '\u{e67a}';  // 
    const LANG_PERL: char       = '\u{e769}';  // 
    const LANG_PHP: char        = '\u{e73d}';  // 
    const LANG_PYTHON: char     = '\u{e606}';  // 
    const LANG_R: char          = '\u{f25d}';  // 
    const LANG_RUBY: char       = '\u{e21e}';  // 
    const LANG_RUBYRAILS: char  = '\u{e73b}';  // 
    const LANG_RUST: char       = '\u{e7a8}';  // 
    const LANG_STYLUS: char     = '\u{e600}';  // 
    const LANG_TEX: char        = '\u{e69b}';  // 
    const LANG_TYPESCRIPT: char = '\u{e628}';  // 
    const LOCK: char            = '\u{f023}';  // 
    const LICENSE: char         = '\u{f02d}';  // 
    const MARKDOWN: char        = '\u{f48a}';  // 
    const MUSTACHE: char        = '\u{e60f}';  // 
    const NODEJS: char          = '\u{e718}';  // 
    const NPM: char             = '\u{e71e}';  // 
    const OS_APPLE: char        = '\u{f179}';  // 
    const OS_ANDROID: char      = '\u{e70e}';  // 
    const OS_LINUX: char        = '\u{f17c}';  // 
    const OS_WINDOWS: char      = '\u{f17a}';  // 
    const OS_WINDOWS_CMD: char  = '\u{ebc4}';  // 
    const POWERSHELL: char      = '\u{ebc7}';  // 
    const REACT: char           = '\u{e7ba}';  // 
    const RAZOR: char           = '\u{f1fa}';  // 
    const SHEET: char           = '\u{f1c3}';  // 
    const SHELL: char           = '\u{f489}';  // 
    const SLIDE: char           = '\u{f1c4}';  // 
    const TEXT: char            = '\u{f15c}';  // 
    const UNITY: char           = '\u{e721}';  // 
    const VIDEO: char           = '\u{f03d}';  // 
    const VIM: char             = '\u{e7c5}';  // 
    const XML: char             = '\u{f05c0}'; // 󰗀
    const YAML: char            = '\u{f481}';  //  - Find better icon
}

/// Mapping from full filenames to file type. This mapping should also contain all the "dot"
/// files/directories that have a custom icon.
const FILENAME_ICONS: Map<&'static str, char> = phf_map! {
    ".atom"               => '\u{e764}',            // 
    ".bashprofile"        => Icons::CONFIG,         // 
    ".bashrc"             => Icons::SHELL,          // 
    ".emacs"              => Icons::EMACS,          // 
    ".git"                => Icons::GIT,            // 
    ".gitattributes"      => Icons::GIT,            // 
    ".gitconfig"          => Icons::GIT,            // 
    ".github"             => '\u{f408}',            // 
    ".gitignore"          => Icons::GIT,            // 
    ".gitignore_global"   => Icons::GIT,            // 
    ".gitmodules"         => Icons::GIT,            // 
    ".idea"               => Icons::INTELLIJ,       // 
    ".rvm"                => Icons::LANG_RUBY,      // 
    ".Trash"              => '\u{f1f8}',            // 
    ".vimrc"              => Icons::VIM,            // 
    ".vscode"             => '\u{f0a1e}',           // 󰨞
    ".zshrc"              => Icons::SHELL,          // 
    "bin"                 => Icons::CONFIG_FOLDER,  // 
    "Cargo.lock"          => Icons::LANG_RUST,      // 
    "config"              => Icons::CONFIG_FOLDER,  // 
    "docker-compose.yml"  => Icons::DOCKER,         // 
    "Dockerfile"          => Icons::DOCKER,         // 
    "ds_store"            => Icons::OS_APPLE,       // 
    "Earthfile"           => '\u{f0ac}',            // 
    "gitignore_global"    => Icons::GIT,            // 
    "gitlab-ci.yml"       => '\u{f296}',            // 
    "go.mod"              => Icons::LANG_GO,        // 
    "go.sum"              => Icons::LANG_GO,        // 
    "gradle"              => Icons::GRADLE,         // 
    "gruntfile.coffee"    => Icons::GRUNT,          // 
    "gruntfile.js"        => Icons::GRUNT,          // 
    "gruntfile.ls"        => Icons::GRUNT,          // 
    "gulpfile.coffee"     => Icons::GULP,           // 
    "gulpfile.js"         => Icons::GULP,           // 
    "gulpfile.ls"         => Icons::GULP,           // 
    "hidden"              => Icons::LOCK,           // 
    "include"             => Icons::CONFIG_FOLDER,  // 
    "lib"                 => '\u{f121}',            // 
    "LICENSE"             => Icons::LICENSE,        // 
    "localized"           => Icons::OS_APPLE,       // 
    "Makefile"            => Icons::SHELL,          // 
    "node_modules"        => Icons::NODEJS,         // 
    "npmignore"           => Icons::NPM,            // 
    "PKGBUILD"            => '\u{f303}',            // 
    "rubydoc"             => Icons::LANG_RUBYRAILS, // 
    "Vagrantfile"         => '\u{2371}',            // ⍱
    "yarn.lock"           => '\u{e6a7}',            // 
};

/// Mapping from lowercase file extension to icons.  If an image, video, or audio extension is add
/// also update the extension filetype map.
const EXTENSION_ICONS: Map<&'static str, char> = phf_map! {
    "7z"             => Icons::COMPRESSED,       // 
    "a"              => Icons::OS_LINUX,         // 
    "acc"            => Icons::AUDIO,            // 
    "acf"            => '\u{f1b6}',              // 
    "ai"             => '\u{e7b4}',              // 
    "alac"           => Icons::AUDIO,            // 
    "android"        => Icons::OS_ANDROID,       // 
    "ape"            => Icons::AUDIO,            // 
    "apk"            => Icons::OS_ANDROID,       // 
    "apple"          => Icons::OS_APPLE,         // 
    "ar"             => Icons::COMPRESSED,       // 
    "arw"            => Icons::IMAGE,            // 
    "asm"            => Icons::LANG_ASSEMBLY,    // 
    "avi"            => Icons::VIDEO,            // 
    "avif"           => Icons::IMAGE,            // 
    "avro"           => Icons::JSON,             // 
    "awk"            => Icons::SHELL,            // 
    "bash"           => Icons::SHELL,            // 
    "bashrc"         => Icons::SHELL,            // 
    "bash_history"   => Icons::SHELL,            // 
    "bash_profile"   => Icons::SHELL,            // 
    "bat"            => Icons::OS_WINDOWS_CMD,   // 
    "bats"           => Icons::SHELL,            // 
    "bib"            => Icons::LANG_TEX,         // 
    "bin"            => Icons::BINARY,           // 
    "bmp"            => Icons::IMAGE,            // 
    "bst"            => Icons::LANG_TEX,         // 
    "bz"             => Icons::COMPRESSED,       // 
    "bz2"            => Icons::COMPRESSED,       // 
    "c"              => Icons::LANG_C,           // 
    "c++"            => Icons::LANG_CPP,         // 
    "cab"            => Icons::OS_WINDOWS,       // 
    "cbr"            => Icons::IMAGE,            // 
    "cbz"            => Icons::IMAGE,            // 
    "cc"             => Icons::LANG_CPP,         // 
    "cert"           => Icons::GIST_SECRET,      // 
    "cfg"            => Icons::CONFIG,           // 
    "cjs"            => Icons::LANG_JAVASCRIPT,  // 
    "class"          => Icons::LANG_JAVA,        // 
    "clj"            => '\u{e768}',              // 
    "cljs"           => '\u{e76a}',              // 
    "cls"            => Icons::LANG_TEX,         // 
    "cmd"            => Icons::OS_WINDOWS,       // 
    "coffee"         => '\u{f0f4}',              // 
    "conf"           => Icons::CONFIG,           // 
    "config"         => Icons::CONFIG,           // 
    "cp"             => Icons::LANG_CPP,         // 
    "cpio"           => Icons::COMPRESSED,       // 
    "cpp"            => Icons::LANG_CPP,         // 
    "cr2"            => Icons::IMAGE,            // 
    "crt"            => Icons::GIST_SECRET,      // 
    "cs"             => Icons::LANG_CSHARP,      // 󰌛
    "csh"            => Icons::SHELL,            // 
    "cshtml"         => Icons::RAZOR,            // 
    "csproj"         => Icons::LANG_CSHARP,      // 󰌛
    "css"            => Icons::CSS3,             // 
    "csv"            => Icons::SHEET,            // 
    "csx"            => Icons::LANG_CSHARP,      // 󰌛
    "cts"            => Icons::LANG_TYPESCRIPT,  // 
    "cu"             => '\u{e64b}',              // 
    "cxx"            => Icons::LANG_CPP,         // 
    "d"              => '\u{e7af}',              // 
    "dart"           => '\u{e798}',              // 
    "db"             => Icons::DATABASE,         // 
    "deb"            => '\u{e77d}',              // 
    "desktop"        => '\u{ebd1}',              // 
    "diff"           => Icons::DIFF,             // 
    "djv"            => Icons::DOCUMENT,         // 
    "djvu"           => Icons::DOCUMENT,         // 
    "dll"            => Icons::OS_WINDOWS,       // 
    "dmg"            => Icons::DISK_IMAGE,       // 
    "doc"            => Icons::DOCUMENT,         // 
    "docx"           => Icons::DOCUMENT,         // 
    "drawio"         => '\u{ebba}',              // 
    "ds_store"       => Icons::OS_APPLE,         // 
    "dump"           => Icons::DATABASE,         // 
    "dvi"            => Icons::IMAGE,            // 
    "ebook"          => Icons::BOOK,             // 
    "ebuild"         => '\u{f30d}',              // 
    "editorconfig"   => Icons::CONFIG,           // 
    "ejs"            => '\u{e618}',              // 
    "el"             => Icons::EMACS,            // 
    "elm"            => '\u{e62c}',              // 
    "eml"            => '\u{f003}',              // 
    "env"            => '\u{f462}',              // 
    "eot"            => Icons::FONT,             // 
    "eps"            => Icons::IMAGE,            // 
    "epub"           => Icons::BOOK,             // 
    "erb"            => Icons::LANG_RUBYRAILS,   // 
    "erl"            => '\u{e7b1}',              // 
    "ex"             => Icons::LANG_ELIXIR,      // 
    "exe"            => Icons::OS_WINDOWS,       // 
    "exs"            => Icons::LANG_ELIXIR,      // 
    "fish"           => Icons::SHELL,            // 
    "flac"           => Icons::AUDIO,            // 
    "flv"            => Icons::VIDEO,            // 
    "font"           => Icons::FONT,             // 
    "fs"             => Icons::LANG_FSHARP,      // 
    "fsi"            => Icons::LANG_FSHARP,      // 
    "fsx"            => Icons::LANG_FSHARP,      // 
    "gdoc"           => Icons::DOCUMENT,         // 
    "gem"            => Icons::LANG_RUBY,        // 
    "gemfile"        => Icons::LANG_RUBY,        // 
    "gemspec"        => Icons::LANG_RUBY,        // 
    "gform"          => '\u{f298}',              // 
    "gif"            => Icons::IMAGE,            // 
    "git"            => Icons::GIT,              // 
    "gitattributes"  => Icons::GIT,              // 
    "gitignore"      => Icons::GIT,              // 
    "gitmodules"     => Icons::GIT,              // 
    "go"             => Icons::LANG_GO,          // 
    "gpg"            => '\u{e60a}',              // 
    "gradle"         => Icons::GRADLE,           // 
    "groovy"         => '\u{e775}',              // 
    "gsheet"         => Icons::SHEET,            // 
    "gslides"        => Icons::SLIDE,            // 
    "guardfile"      => Icons::LANG_RUBY,        // 
    "gz"             => Icons::COMPRESSED,       // 
    "h"              => Icons::HEADER,           // 
    "hbs"            => Icons::MUSTACHE,         // 
    "heic"           => Icons::VIDEO,            // 
    "heif"           => Icons::IMAGE,            // 
    "hpp"            => Icons::HEADER,           // 
    "hs"             => Icons::LANG_HASKELL,     // 
    "htm"            => Icons::HTML5,            // 
    "html"           => Icons::HTML5,            // 
    "hxx"            => Icons::HEADER,           // 
    "ical"           => Icons::CALENDAR,         // 
    "icalendar"      => Icons::CALENDAR,         // 
    "ico"            => Icons::IMAGE,            // 
    "ics"            => Icons::CALENDAR,         // 
    "ifb"            => Icons::CALENDAR,         // 
    "image"          => Icons::IMAGE,            // 
    "img"            => Icons::DISK_IMAGE,       // 
    "iml"            => Icons::INTELLIJ,         // 
    "ini"            => Icons::OS_WINDOWS,       // 
    "ipynb"          => '\u{e678}',              // 
    "iso"            => Icons::DISK_IMAGE,       // 
    "j2c"            => Icons::IMAGE,            // 
    "j2k"            => Icons::IMAGE,            // 
    "jad"            => Icons::LANG_JAVA,        // 
    "jar"            => Icons::LANG_JAVA,        // 
    "java"           => Icons::LANG_JAVA,        // 
    "jfi"            => Icons::IMAGE,            // 
    "jfif"           => Icons::IMAGE,            // 
    "jif"            => Icons::IMAGE,            // 
    "jl"             => '\u{e624}',              // 
    "jmd"            => Icons::MARKDOWN,         // 
    "jp2"            => Icons::IMAGE,            // 
    "jpe"            => Icons::IMAGE,            // 
    "jpeg"           => Icons::IMAGE,            // 
    "jpf"            => Icons::IMAGE,            // 
    "jpg"            => Icons::IMAGE,            // 
    "jpx"            => Icons::IMAGE,            // 
    "js"             => Icons::LANG_JAVASCRIPT,  // 
    "json"           => Icons::JSON,             // 
    "jsx"            => Icons::REACT,            // 
    "jxl"            => Icons::IMAGE,            // 
    "kdb"            => Icons::KEYPASS,          // 
    "kdbx"           => Icons::KEYPASS,          // 
    "key"            => Icons::KEY,              // 
    "ko"             => Icons::OS_LINUX,         // 
    "ksh"            => Icons::SHELL,            // 
    "latex"          => Icons::LANG_TEX,         // 
    "less"           => '\u{e758}',              // 
    "lhs"            => Icons::LANG_HASKELL,     // 
    "license"        => Icons::LICENSE,          // 
    "localized"      => Icons::OS_APPLE,         // 
    "lock"           => Icons::LOCK,             // 
    "log"            => '\u{f18d}',              // 
    "lua"            => '\u{e620}',              // 
    "lz"             => Icons::COMPRESSED,       // 
    "lz4"            => Icons::COMPRESSED,       // 
    "lzh"            => Icons::COMPRESSED,       // 
    "lzma"           => Icons::COMPRESSED,       // 
    "lzo"            => Icons::COMPRESSED,       // 
    "m"              => Icons::LANG_C,           // 
    "m2ts"           => Icons::VIDEO,            // 
    "m2v"            => Icons::VIDEO,            // 
    "m4a"            => Icons::AUDIO,            // 
    "m4v"            => Icons::VIDEO,            // 
    "magnet"         => '\u{f076}',              // 
    "markdown"       => Icons::MARKDOWN,         // 
    "md"             => Icons::MARKDOWN,         // 
    "mjs"            => Icons::LANG_JAVASCRIPT,  // 
    "mk"             => Icons::SHELL,            // 
    "mka"            => Icons::AUDIO,            // 
    "mkd"            => Icons::MARKDOWN,         // 
    "mkv"            => Icons::VIDEO,            // 
    "ml"             => Icons::LANG_OCAML,       // 
    "mli"            => Icons::LANG_OCAML,       // 
    "mll"            => Icons::LANG_OCAML,       // 
    "mly"            => Icons::LANG_OCAML,       // 
    "mm"             => Icons::LANG_CPP,         // 
    "mobi"           => Icons::BOOK,             // 
    "mov"            => Icons::VIDEO,            // 
    "mp2"            => Icons::AUDIO,            // 
    "mp3"            => Icons::AUDIO,            // 
    "mp4"            => Icons::VIDEO,            // 
    "mpeg"           => Icons::VIDEO,            // 
    "mpg"            => Icons::VIDEO,            // 
    "msi"            => Icons::OS_WINDOWS,       // 
    "mts"            => Icons::LANG_TYPESCRIPT,  // 
    "mustache"       => Icons::MUSTACHE,         // 
    "nef"            => Icons::IMAGE,            // 
    "ninja"          => '\u{f0774}',             // 󰝴
    "nix"            => '\u{f313}',              // 
    "node"           => '\u{f0399}',             // 󰎙
    "npmignore"      => Icons::NPM,              // 
    "o"              => Icons::BINARY,           // 
    "odp"            => Icons::SLIDE,            // 
    "ods"            => Icons::SHEET,            // 
    "odt"            => Icons::DOCUMENT,         // 
    "ogg"            => Icons::AUDIO,            // 
    "ogm"            => Icons::VIDEO,            // 
    "ogv"            => Icons::VIDEO,            // 
    "opus"           => Icons::AUDIO,            // 
    "orf"            => Icons::IMAGE,            // 
    "org"            => '\u{e633}',              // 
    "otf"            => Icons::FONT,             // 
    "out"            => '\u{eb2c}',              // 
    "par"            => Icons::COMPRESSED,       // 
    "part"           => '\u{f43a}',              // 
    "patch"          => Icons::DIFF,             // 
    "pbm"            => Icons::IMAGE,            // 
    "pdf"            => '\u{f1c1}',              // 
    "pem"            => Icons::KEY,              // 
    "pgm"            => Icons::IMAGE,            // 
    "php"            => Icons::LANG_PHP,         // 
    "pl"             => Icons::LANG_PERL,        // 
    "plx"            => Icons::LANG_PERL,        // 
    "pm"             => Icons::LANG_PERL,        // 
    "png"            => Icons::IMAGE,            // 
    "pnm"            => Icons::IMAGE,            // 
    "pod"            => Icons::LANG_PERL,        // 
    "ppm"            => Icons::IMAGE,            // 
    "ppt"            => Icons::SLIDE,            // 
    "pptx"           => Icons::SLIDE,            // 
    "procfile"       => Icons::LANG_RUBY,        //   - Can not find a reference to procfile being Ruby
    "properties"     => Icons::JSON,             // 
    "ps"             => Icons::IMAGE,            // 
    "ps1"            => Icons::POWERSHELL,       // 
    "psd"            => '\u{e7b8}',              // 
    "psd1"           => Icons::POWERSHELL,       // 
    "psm1"           => Icons::POWERSHELL,       // 
    "pxm"            => Icons::IMAGE,            // 
    "py"             => Icons::LANG_PYTHON,      // 
    "pyc"            => Icons::LANG_PYTHON,      // 
    "qcow2"          => Icons::DISK_IMAGE,       // 
    "r"              => Icons::LANG_R,           // 
    "rakefile"       => Icons::LANG_RUBY,        // 
    "rar"            => Icons::COMPRESSED,       // 
    "raw"            => Icons::IMAGE,            // 
    "razor"          => Icons::RAZOR,            // 
    "rb"             => Icons::LANG_RUBY,        // 
    "rdata"          => Icons::LANG_R,           // 
    "rdb"            => '\u{e76d}',              // 
    "rdoc"           => Icons::MARKDOWN,         // 
    "rds"            => Icons::LANG_R,           // 
    "readme"         => Icons::MARKDOWN,         // 
    "rlib"           => Icons::LANG_RUST,        // 
    "rmd"            => Icons::MARKDOWN,         // 
    "rmeta"          => Icons::LANG_RUST,        // 
    "rpm"            => '\u{e7bb}',              // 
    "rs"             => Icons::LANG_RUST,        // 
    "rspec"          => Icons::LANG_RUBY,        // 
    "rspec_parallel" => Icons::LANG_RUBY,        // 
    "rspec_status"   => Icons::LANG_RUBY,        // 
    "rss"            => '\u{f09e}',              // 
    "rst"            => Icons::TEXT,             // 
    "rtf"            => Icons::TEXT,             // 
    "ru"             => Icons::LANG_RUBY,        // 
    "rubydoc"        => Icons::LANG_RUBYRAILS,   // 
    "s"              => Icons::LANG_ASSEMBLY,    // 
    "sass"           => '\u{e603}',              // 
    "scala"          => '\u{e737}',              // 
    "scss"           => Icons::CSS3,             // 
    "service"        => '\u{eba2}',              // 
    "sh"             => Icons::SHELL,            // 
    "shell"          => Icons::SHELL,            // 
    "slim"           => Icons::LANG_RUBYRAILS,   // 
    "sln"            => '\u{e70c}',              // 
    "so"             => Icons::OS_LINUX,         // 
    "sql"            => Icons::DATABASE,         // 
    "sqlite3"        => '\u{e7c4}',              // 
    "stl"            => Icons::IMAGE,            // 
    "sty"            => Icons::LANG_TEX,         // 
    "styl"           => Icons::LANG_STYLUS,      // 
    "stylus"         => Icons::LANG_STYLUS,      // 
    "svelte"         => '\u{e697}',              // 
    "svg"            => Icons::IMAGE,            // 
    "swift"          => '\u{e755}',              // 
    "t"              => Icons::LANG_PERL,        // 
    "tar"            => Icons::COMPRESSED,       // 
    "taz"            => Icons::COMPRESSED,       // 
    "tbz"            => Icons::COMPRESSED,       // 
    "tbz2"           => Icons::COMPRESSED,       // 
    "tc"             => Icons::COMPRESSED,       // 
    "tex"            => Icons::LANG_TEX,         // 
    "tgz"            => Icons::COMPRESSED,       // 
    "tif"            => Icons::IMAGE,            // 
    "tiff"           => Icons::IMAGE,            // 
    "tlz"            => Icons::COMPRESSED,       // 
    "toml"           => Icons::CONFIG,           // 
    "torrent"        => '\u{e275}',              // 
    "ts"             => Icons::LANG_TYPESCRIPT,  // 
    "tsv"            => Icons::SHEET,            // 
    "tsx"            => Icons::REACT,            // 
    "ttf"            => Icons::FONT,             // 
    "twig"           => '\u{e61c}',              // 
    "txt"            => Icons::TEXT,             // 
    "txz"            => Icons::COMPRESSED,       // 
    "tz"             => Icons::COMPRESSED,       // 
    "tzo"            => Icons::COMPRESSED,       // 
    "unity"          => Icons::UNITY,            // 
    "unity3d"        => Icons::UNITY,            // 
    "vdi"            => Icons::DISK_IMAGE,       // 
    "vhd"            => Icons::DISK_IMAGE,       // 
    "video"          => Icons::VIDEO,            // 
    "vim"            => Icons::VIM,              // 
    "vmdk"           => Icons::DISK_IMAGE,       // 
    "vob"            => Icons::VIDEO,            // 
    "vue"            => '\u{f0844}',             // 󰡄
    "war"            => Icons::LANG_JAVA,        // 
    "wav"            => Icons::AUDIO,            // 
    "webm"           => Icons::VIDEO,            // 
    "webp"           => Icons::IMAGE,            // 
    "windows"        => Icons::OS_WINDOWS,       // 
    "wma"            => Icons::AUDIO,            // 
    "wmv"            => Icons::VIDEO,            // 
    "woff"           => Icons::FONT,             // 
    "woff2"          => Icons::FONT,             // 
    "xhtml"          => Icons::HTML5,            // 
    "xls"            => Icons::SHEET,            // 
    "xlsm"           => Icons::SHEET,            // 
    "xlsx"           => Icons::SHEET,            // 
    "xml"            => Icons::XML,              // 󰗀
    "xpm"            => Icons::IMAGE,            // 
    "xul"            => Icons::XML,              // 󰗀
    "xz"             => Icons::COMPRESSED,       // 
    "yaml"           => Icons::YAML,             // 
    "yml"            => Icons::YAML,             // 
    "z"              => Icons::COMPRESSED,       // 
    "zig"            => '\u{21af}',              // ↯
    "zip"            => Icons::COMPRESSED,       // 
    "zsh"            => Icons::SHELL,            // 
    "zsh-theme"      => Icons::SHELL,            // 
    "zshrc"          => Icons::SHELL,            // 
    "zst"            => Icons::COMPRESSED,       // 
};

/// Converts the style used to paint a file name into the style that should be
/// used to paint an icon.
///
/// - The background colour should be preferred to the foreground colour, as
///   if one is set, it’s the more “obvious” colour choice.
/// - If neither is set, just use the default style.
/// - Attributes such as bold or underline should not be used to paint the
///   icon, as they can make it look weird.
pub fn iconify_style(style: Style) -> Style {
    style.background.or(style.foreground)
         .map(Style::from)
         .unwrap_or_default()
}

/// Lookup the icon for a file based on the file's name, if the entry is a
/// directory, or by the lowercase file extension.
pub fn icon_for_file(file: &File<'_>) -> char {
    if let Some(icon) = FILENAME_ICONS.get(file.name.as_str()) {
        *icon
    } else if file.points_to_directory() {
        if file.is_empty_dir() {
            '\u{f115}' // 
        } else {
            '\u{f07b}' // 
        }
    } else if let Some(ext) = file.ext.as_ref() {
        *EXTENSION_ICONS.get(ext.as_str()).unwrap_or(&'\u{f15b}') // 
    } else {
        '\u{f016}' // 
    }
}
