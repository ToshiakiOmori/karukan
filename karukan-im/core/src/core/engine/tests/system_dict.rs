use super::*;

/// Write a one-entry system dictionary in the binary format and return the file.
fn system_dict_file() -> tempfile::NamedTempFile {
    let dict =
        dict_from_json(r#"[{"reading":"かるかん","candidates":[{"surface":"軽羹","score":1.0}]}]"#);
    let tmp = tempfile::NamedTempFile::new().unwrap();
    dict.save(tmp.path()).unwrap();
    tmp
}

#[test]
fn test_system_dictionary_shared_across_engines() {
    // One engine per input context: the second must reuse the first's copy.
    let file = system_dict_file();
    let path = file.path().to_str().unwrap();

    let mut first = InputMethodEngine::new();
    first.init_system_dictionary(Some(path));
    let mut second = InputMethodEngine::new();
    second.init_system_dictionary(Some(path));

    let a = first
        .dicts
        .system
        .as_ref()
        .expect("first engine loads the dictionary");
    let b = second
        .dicts
        .system
        .as_ref()
        .expect("second engine gets the dictionary");
    assert!(std::sync::Arc::ptr_eq(a, b));
}

#[test]
fn test_system_dictionary_missing_file_is_not_cached() {
    // A path that fails to load leaves no cache entry, so a later engine
    // retries once the file appears.
    let dir = tempfile::tempdir().unwrap();
    let path = dir.path().join("dict.bin");
    let path_str = path.to_str().unwrap();

    let mut before = InputMethodEngine::new();
    before.init_system_dictionary(Some(path_str));
    assert!(before.dicts.system.is_none());

    let file = system_dict_file();
    std::fs::copy(file.path(), &path).unwrap();
    let mut after = InputMethodEngine::new();
    after.init_system_dictionary(Some(path_str));
    assert!(after.dicts.system.is_some());
}
