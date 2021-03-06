/// Smoke tests for Notion, that will be run in CI.
///
/// To run these locally:
/// (CAUTION: this will destroy the Notion installation on the system where this is run)
///
/// ```
/// NOTION_DEV=1 cargo test --test smoke --features smoke-tests -- --test-threads 1
/// ```
///
/// Also note that each test uses a different version of node and yarn. This is to prevent
/// false positives if the tests are not cleaned up correctly. Any new tests should use
/// different versions of node and yarn.

cfg_if::cfg_if! {
    if #[cfg(all(unix, feature = "smoke-tests"))] {
        mod support;
        mod notion_fetch;
        mod notion_install;
        mod autodownload;
    }
}
