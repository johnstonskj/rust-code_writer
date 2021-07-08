/*!
Standard `Error`, `ErrorKind`, and `Result` types.
 */

#![allow(missing_docs)]

// ------------------------------------------------------------------------------------------------
// Public Types
// ------------------------------------------------------------------------------------------------

error_chain! {
    errors {
        #[doc = "Invalid identifier value."]
        InvalidIdentifierValue(identifier: String) {
            description("Invalid identifier value.")
            display("Invalid identifier value: '{}'.", identifier)
        }
        #[doc = "Unsupported model element kind."]
        UnsupportedElementKind(kind: String) {
            description("Unsupported model element kind.")
            display("Unsupported model element kind: '{}'.", kind)
        }
     }

    foreign_links {
        Fmt(::std::fmt::Error);
        Io(::std::io::Error);
    }
}
