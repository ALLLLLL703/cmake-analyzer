// Contains AI-generated documentation edits.

//! Demonstrate borrowed string selection and a small printing entrypoint.
//! Only the entrypoint writes stdout; no files or child processes are used.

/// Select the input string with more UTF-8 bytes; ties select the second input.
/// Returns a borrowed slice without allocation, mutation, or external side effects.
/// The printing demo in [`main`] is the local caller; the second declared lifetime is unused.
///
/// # Arguments
/// - `str1` `[in]`: first borrowed string, unchanged.
/// - `str2` `[in]`: second borrowed string with the same return-compatible lifetime, unchanged.
///
/// # Returns
/// `str1` if its byte length is strictly greater; otherwise `str2`, including equal lengths.
/// The returned slice borrows an input for lifetime 'a; it is not an owned copy.
///
/// # Examples
/// Local playground target context (the function is not exported from the analyzer library):
/// ```text
/// assert_eq!(longer("abc", "de"), "abc");
/// assert_eq!(longer("114", "514"), "514");
/// ```
pub fn longer<'a, 'b>(str1: &'a str, str2: &'a str) -> &'a str {
    if str1.len() > str2.len() { str1 } else { str2 }
}

/// Print the result of selecting between 114 and 514, demonstrating a borrow used after an inner scope.
/// Both string literals have static storage; equal byte lengths select 514. Writes stdout only.
///
/// # Arguments
/// None.
///
/// # Returns
/// `()` after printing 514 followed by a newline.
///
/// # Panics
/// If stdout printing fails.
///
/// # Examples
/// Local playground target context:
/// ```text
/// main(); // prints 514
/// ```
fn main() {
    let a = "114";
    let result;
    {
        let b = "514";
        result = longer(a, b);
    }
    println!("{result}");
}
