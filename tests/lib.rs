extern crate first;

use first::workflow_url;

#[test]
fn test_name() {
    let result = workflow_url("foo", "bar");
    assert_eq!("http://styx.spotify.net/api/v0/workflows/foo/bar", result);
}

#[test]
fn other_test() {
    let result = workflow_url("baz", "qux");
    assert_eq!("http://styx.spotify.net/api/v0/workflows/baz/qux", result);
}
