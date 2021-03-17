#[allow(unused_macros)]
macro_rules! assert_eq_err {
  ($actual:expr, $expected:expr, $max_err:expr) => {{
    assert_eq_err!($actual, $expected, $max_err, "");
  }};
  ($actual:expr, $expected:expr, $max_err:expr, $note:expr) => {{
    let (actual, expected): (&f64, &f64) = (&$actual, &$expected);
    let max_err = $max_err;
    let note = $note;
    // Add a negligible fraction to make sure we
    // don't divide by zero.
    let error = (
      (*actual - *expected) /
      if *expected == 0.0 { *expected + 0.000001 } else { *expected }
    ).abs();
    assert!(
        error <= max_err,
        "assertion failed: `(left !== right)` \
         (left: `{:?}`, right: `{:?}`, expected error: `{:?}`, real error: `{:?}`){}{}",
        *actual,
        *expected,
        max_err,
        error,
        if note.len() > 0 { " - " } else { "" },
        note
    );
  }};
}
