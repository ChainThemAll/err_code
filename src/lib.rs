//! # ErrorCode Macro
//!
//! The `ErrorCode` macro simplifies adding numeric error codes to your Rust enums. It automatically generates an `error_code` method that returns the error code associated with each enum variant. This macro is designed to be easy to use while offering flexibility for more advanced needs.
//!
//! ## Table of Contents
//! - [Basic Usage](#basic-usage)
//! - [Advanced Usage](#advanced-usage)
//! - [Compatibility with `thiserror`](#compatibility-with-thiserror)
//!
//!  ## Github
//! - [https://github.com/ChainThemAll/error_code](https://github.com/ChainThemAll/error_code)
//!
//! ## Basic Usage
//!
//! The simplest way to use the `ErrorCode` macro requires just a few steps:
//!
//! 1. Add the `error_code` crate to your `Cargo.toml`.
//! 2. Apply `#[derive(ErrorCode)]` to your enum.
//! 3. Assign an error code to each variant using `#[error_code(value)]`.
//!
//! At its most basic, every variant must have an explicitly defined error code, and the error code type defaults to `u32`.
//!
//! ### Example: Basic Usage
//!
//! Here’s a minimal example of defining an error enum with error codes:
//!
//! ```rust
//! use error_code::ErrorCode;
//!
//! #[derive(ErrorCode, Debug)]
//! enum MyError {
//!     #[error_code(100)]
//!     InvalidInput,
//!     #[error_code(200)]
//!     NotFound,
//! }
//! ```
//!
//! In this example:
//! - `MyError::InvalidInput` has an error code of `100`.
//! - `MyError::NotFound` has an error code of `200`.
//!
//! You can then use the `error_code` method to retrieve these codes:
//!
//! ```rust
//! let error = MyError::InvalidInput;
//! println!("Error code: {}", error.error_code()); // Outputs: "100"
//! ```
//!
//! ## Advanced Usage
//!
//! For more control, the `ErrorCode` macro supports customizing the error code type and providing a default error code for variants that don’t specify one.
//!
//! ### Customizing the Error Code Type
//!
//! By default, error codes are `u32`. You can change this to another unsigned integer type (e.g., `u8`, `u16`, `u64`) using the `type` attribute at the enum level.
//!
//! - **Syntax**: `#[error_code(type = "u16")]`
//! - **Default**: If `type` is not specified, it’s `u32`.
//!
//! ### Setting a Default Error Code
//!
//! You can define a default error code for variants that lack an explicit `#[error_code(value)]` using the `default` attribute.
//!
//! - **Syntax**: `#[error_code(default = 300)]`
//!
//! ### Example: Advanced Usage
//!
//! Here’s an example that uses both `type` and `default`:
//!
//! ```rust
//! use error_code::ErrorCode;
//!
//! #[derive(ErrorCode, Debug)]
//! #[error_code(type = "u16", default = 300)]
//! enum MyError {
//!     #[error_code(100)]
//!     InvalidInput,
//!     NotFound, // Uses the default error code
//! }
//! ```
//!
//! In this example:
//! - The error code type is `u16` instead of the default `u32`.
//! - `MyError::InvalidInput` has an explicit error code of `100`.
//! - `MyError::NotFound` uses the default error code of `300` since it lacks an explicit value.
//!
//! Using the `error_code` method:
//!
//! ```rust
//! let error1 = MyError::InvalidInput;
//! println!("Error code: {}", error1.error_code()); // Outputs: "100"
//!
//! let error2 = MyError::NotFound;
//! println!("Error code: {}", error2.error_code()); // Outputs: "300"
//! ```
//!
//! **Key Points**:
//! - If you don’t set `type`, it defaults to `u32`.
//!
//! ## Compatibility with `thiserror`
//!
//! The `ErrorCode` macro works seamlessly with `thiserror`, allowing you to combine rich error messages with numeric error codes. This is especially useful when you want both human-readable error details and machine-readable codes.
//!
//! ### Example: Using `ErrorCode` with `thiserror`
//!
//! Here’s an example that integrates both macros:
//!
//! ```rust
//! use error_code::ErrorCode;
//! use thiserror::Error;
//! use std::io::Error as IoError;
//!
//! #[derive(Error, ErrorCode, Debug)]
//! #[error_code(type = "u16", default = 300)]
//! enum MyError {
//!     #[error("Invalid input: {0}")]
//!     #[error_code(100)]
//!     InvalidInput(String),
//!
//!     #[error("Resource not found: {name}")]
//!     #[error_code(200)]
//!     NotFound { name: String },
//!
//!     #[error(transparent)]
//!     IoError(#[from] IoError), // Uses default error code
//! }
//! ```
//!
//! In this example:
//! - `thiserror` provides descriptive error messages.
//! - `ErrorCode` adds numeric codes.
//! - `IoError` uses the default code (`300`) since it doesn’t have an explicit `#[error_code(value)]`.
//!
//! Using both features:
//!
//! ```rust
//! let error = MyError::InvalidInput("test".to_string());
//! println!("Error: {}", error);          // Outputs: "Invalid input: test"
//! println!("Error code: {}", error.error_code()); // Outputs: "100"
//!
//! let io_error = IoError::new(std::io::ErrorKind::Other, "io failure");
//! let error = MyError::IoError(io_error);
//! println!("Error: {}", error);          // Outputs: "io failure"
//! println!("Error code: {}", error.error_code()); // Outputs: "300"
//! ```
//!
//! **Why It Works Well**:
//! - `ErrorCode` and `thiserror` complement each other without overlap.
//! - You can use `ErrorCode` alone for codes, `thiserror` alone for messages, or both together, giving you full flexibility.
//!
use proc_macro::{Delimiter, TokenStream, TokenTree};

// Helper function to parse error code type and default value from enum attributes
fn parse_error_code_attrs(attrs: &[TokenTree]) -> (String, Option<u64>) {
    let mut error_code_type = "u32".to_string(); // Default type is u32
    let mut default_code = None; // Default code is not set initially

    for attr in attrs {
        if let TokenTree::Group(g) = attr {
            if g.delimiter() == Delimiter::Bracket {
                let mut attr_iter = g.stream().into_iter();
                if let Some(TokenTree::Ident(id)) = attr_iter.next() {
                    if id.to_string() == "error_code" {
                        if let Some(TokenTree::Group(inner_group)) = attr_iter.next() {
                            let inner_tokens: Vec<_> = inner_group.stream().into_iter().collect();
                            let mut i = 0;
                            while i < inner_tokens.len() {
                                if let Some(TokenTree::Ident(key)) = inner_tokens.get(i) {
                                    if i + 2 < inner_tokens.len()
                                        && matches!(inner_tokens[i + 1], TokenTree::Punct(ref p) if p.as_char() == '=')
                                    {
                                        if let TokenTree::Literal(value) = &inner_tokens[i + 2] {
                                            if key.to_string() == "type" {
                                                error_code_type =
                                                    value.to_string().replace("\"", "");
                                            } else if key.to_string() == "default" {
                                                default_code =
                                                    Some(value.to_string().parse::<u64>().unwrap());
                                            }
                                        }
                                        i += 3; // Skip key, '=', value
                                    } else {
                                        i += 1;
                                    }
                                } else {
                                    i += 1;
                                }
                            }
                        }
                    }
                }
            }
        }
    }

    (error_code_type, default_code)
}

/// # ErrorCode 宏
///
/// `ErrorCode` 宏用于为枚举变体定义错误码，并生成一个 `error_code` 方法。
///
/// ## 使用方法
/// - 在枚举上添加 `#[derive(ErrorCode)]`。
/// - 使用 `#[error_code(type = "u16", default = 300)]` 指定错误码类型和默认值。
/// - 在每个变体上使用 `#[error_code(value)]` 设置具体的错误码。
///
/// ### 示例
/// ```rust
/// #[derive(ErrorCode)]
/// #[error_code(type = "u16", default = 300)]
/// enum MyError {
///     #[error_code(100)]
///     InvalidInput,
///     #[error_code(200)]
///     NotFound,
/// }
/// ```
#[proc_macro_derive(ErrorCode, attributes(error_code))]
pub fn derive_error_code(input: TokenStream) -> TokenStream {
    let mut tokens = input.into_iter();
    let enum_name;
    let mut variants = Vec::new();
    let mut enum_attrs = Vec::new();

    // 1. Parse enum attributes and name
    for token in tokens.by_ref() {
        if let TokenTree::Group(g) = &token {
            if g.delimiter() == Delimiter::Bracket {
                enum_attrs.push(token.clone()); // Collect attributes before enum keyword
            }
        } else if let TokenTree::Ident(ident) = token {
            if ident.to_string() == "enum" {
                break;
            }
        }
    }
    if let Some(TokenTree::Ident(ident)) = tokens.next() {
        enum_name = Some(ident.to_string());
    } else {
        panic!("Expected enum name");
    }

    // Parse error code type and default value from enum attributes
    let (error_code_type, default_code) = parse_error_code_attrs(&enum_attrs);

    // 2. Parse enum body
    if let Some(TokenTree::Group(group)) = tokens.next() {
        if group.delimiter() == Delimiter::Brace {
            let mut variant_tokens = group.stream().into_iter().peekable();

            while variant_tokens.peek().is_some() {
                // Collect attributes
                let mut attributes = Vec::new();
                while let Some(TokenTree::Group(g)) = variant_tokens.peek() {
                    if g.delimiter() == Delimiter::Bracket {
                        attributes.push(variant_tokens.next().unwrap());
                    } else {
                        break;
                    }
                }

                // Parse variant name
                if let Some(TokenTree::Ident(ident)) = variant_tokens.next() {
                    let variant_name = ident.to_string();
                    let mut error_code = None;

                    // Find error_code attribute
                    for attr in &attributes {
                        if let TokenTree::Group(g) = attr {
                            let mut attr_iter = g.stream().into_iter();
                            if let Some(TokenTree::Ident(id)) = attr_iter.next() {
                                if id.to_string() == "error_code" {
                                    if let Some(TokenTree::Group(code_group)) = attr_iter.next() {
                                        if code_group.delimiter() == Delimiter::Parenthesis {
                                            if let Some(TokenTree::Literal(lit)) =
                                                code_group.stream().into_iter().next()
                                            {
                                                let lit_str = lit.to_string();
                                                match error_code_type.as_str() {
                                                    "u8" => match lit_str.parse::<u8>() {
                                                        Ok(code) => error_code = Some(code.to_string()),
                                                        Err(e) => panic!(
                                                            "Variant {}'s #[error_code] must be a u8 value, but '{}' is invalid: {}",
                                                            variant_name, lit_str, e
                                                        ),
                                                    },
                                                    "u16" => match lit_str.parse::<u16>() {
                                                        Ok(code) => error_code = Some(code.to_string()),
                                                        Err(e) => panic!(
                                                            "Variant {}'s #[error_code] must be a u16 value, but '{}' is invalid: {}",
                                                            variant_name, lit_str, e
                                                        ),
                                                    },
                                                    "u32" => match lit_str.parse::<u32>() {
                                                        Ok(code) => error_code = Some(code.to_string()),
                                                        Err(e) => panic!(
                                                            "Variant {}'s #[error_code] must be a u32 value, but '{}' is invalid: {}",
                                                            variant_name, lit_str, e
                                                        ),
                                                    },
                                                    "u64" => match lit_str.parse::<u64>() {
                                                        Ok(code) => error_code = Some(code.to_string()),
                                                        Err(e) => panic!(
                                                            "Variant {}'s #[error_code] must be a u64 value, but '{}' is invalid: {}",
                                                            variant_name, lit_str, e
                                                        ),
                                                    },
                                                    _ => panic!("Unsupported error code type: {}", error_code_type),
                                                }
                                            } else {
                                                panic!(
                                                    "#[error_code] attribute requires a parameter"
                                                );
                                            }
                                        } else {
                                            panic!("#[error_code] attribute parameter must use parentheses");
                                        }
                                    }
                                }
                            }
                        } else {
                            panic!("Attribute should be a Group type");
                        }
                    }

                    // Use default code if no error_code attribute is provided
                    let code = error_code.unwrap_or_else(|| default_code.unwrap_or(0).to_string());

                    // Skip field description if exists
                    if let Some(TokenTree::Group(_)) = variant_tokens.peek() {
                        variant_tokens.next(); // Consume the field group
                    }
                    variants.push((variant_name, code));
                }

                // Handle comma
                if let Some(TokenTree::Punct(p)) = variant_tokens.peek() {
                    if p.as_char() == ',' {
                        variant_tokens.next(); // Consume comma
                    }
                }
            }
        }
    }

    let enum_name = enum_name.expect("Unable to parse enum name");

    // 3. Generate match arms
    let match_arms: Vec<String> = variants
        .iter()
        .map(|(variant_name, code)| {
            format!("{}::{} {{ .. }} => {},", enum_name, variant_name, code)
        })
        .collect();
    let match_arms_str = match_arms.join("\n        ");

    // 4. Generate impl block with the specified error code type
    let impl_code = format!(
        "impl {} {{
            pub fn error_code(&self) -> {} {{
                match self {{
                    {}
                }}
            }}
        }}",
        enum_name, error_code_type, match_arms_str
    );

    impl_code.parse().unwrap()
}
