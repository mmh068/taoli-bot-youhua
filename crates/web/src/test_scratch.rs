//! 测试临时目录。只在 `cargo test` 时编译。
use std::cell::RefCell;
use std::path::PathBuf;

thread_local! {
    static DIRS: RefCell<Vec<tempfile::TempDir>> = const { RefCell::new(Vec::new()) };
}

/// 每个测试线程一份独立临时目录，线程结束时删除。
pub(crate) fn scratch_dir() -> PathBuf {
    let dir = tempfile::tempdir().expect("创建测试临时目录");
    let path = dir.path().to_path_buf();
    DIRS.with(|dirs| dirs.borrow_mut().push(dir));
    path
}
