/// echo prints to the standard output, with a newline.
///
/// On all platforms, the newline is the LINE FEED character (`\n`/`U+000A`) alone
/// (no additional CARRIAGE RETURN (`\r`/`U+000D`)).
///
/// This macro uses the same syntax as [`format!`], but writes to the standard output instead.
/// See [`std::fmt`] for more information.
///
/// The `println!` macro will lock the standard output on each call. If you call
/// `println!` within a hot loop, this behavior may be the bottleneck of the loop.
/// To avoid this, lock stdout with [`io::stdout().lock()`][lock]:
///
/// Use `println!` only for the primary output of your program. Use
/// [`eprintln!`] instead to print error and progress messages.
///
/// [`std::fmt`]: crate::fmt
/// [`eprintln!`]: crate::eprintln
/// [lock]: crate::io::Stdout
///
/// # Panics
///
/// Panics if writing to [`io::stdout`] fails.
///
/// Writing to non-blocking stdout can cause an error, which will lead
/// this macro to panic.
///
pub fn echo(_flags: &[String]) {
    for flag in _flags {
        print!("{}", flag);
    }
}

// test echo
#[cfg(test)]
mod echo_test {
    use super::*;

    #[test]
    fn test_echo() {
        let flags = vec!["hello".to_string(), "world".to_string()];
        echo(&flags);
    }
}
