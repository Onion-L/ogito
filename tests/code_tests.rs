use ogito::code::is_code_file;

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
