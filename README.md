 # ErrorCode Macro

 The `ErrorCode` macro simplifies adding numeric error codes to your Rust enums. It automatically generates an `error_code` method that returns the error code associated with each enum variant. This macro is designed to be easy to use while offering flexibility for more anced needs.

 ## Table of Contents
 - [Basic Usage](#basic-usage)
 - [Advanced Usage](#advanced-usage)
 - [Compatibility with `thiserror`](#compatibility-with-thiserror)

 ## Basic Usage

 The simplest way to use the `ErrorCode` macro requires just a few steps:

 1. Add the `error_code` crate to your `Cargo.toml`.
 2. Apply `#[derive(ErrorCode)]` to your enum.
 3. Assign an error code to each variant using `#[error_code(value)]`.

 At its most basic, every variant must have an explicitly defined error code, and the error code type defaults to `u32`.

 ### Example: Basic Usage

 Here’s a minimal example of defining an error enum with error codes:

 ```rust
 use error_code::ErrorCode;

 #[derive(ErrorCode, Debug)]
 enum MyError {
     #[error_code(100)]
     InvalidInput,
     #[error_code(200)]
     NotFound,
 }
 ```

 In this example:
 - `MyError::InvalidInput` has an error code of `100`.
 - `MyError::NotFound` has an error code of `200`.

 You can then use the `error_code` method to retrieve these codes:

 ```rust
 let error = MyError::InvalidInput;
 println!("Error code: {}", error.error_code()); // Outputs: "100"
 ```

 ## Advanced Usage

 For more control, the `ErrorCode` macro supports customizing the error code type and providing a default error code for variants that don’t specify one.

 ### Customizing the Error Code Type

 By default, error codes are `u32`. You can change this to another unsigned integer type (e.g., `u8`, `u16`, `u64`) using the `type` attribute at the enum level.

 - **Syntax**: `#[error_code(type = "u16")]`
 - **Default**: If `type` is not specified, it’s `u32`.

 ### Setting a Default Error Code

 You can define a default error code for variants that lack an explicit `#[error_code(value)]` using the `default` attribute.

 - **Syntax**: `#[error_code(default = 300)]`

 ### Example: Advanced Usage

 Here’s an example that uses both `type` and `default`:

 ```rust
 use error_code::ErrorCode;

 #[derive(ErrorCode, Debug)]
 #[error_code(type = "u16", default = 300)]
 enum MyError {
     #[error_code(100)]
     InvalidInput,
     NotFound, // Uses the default error code
 }
 ```

 In this example:
 - The error code type is `u16` instead of the default `u32`.
 - `MyError::InvalidInput` has an explicit error code of `100`.
 - `MyError::NotFound` uses the default error code of `300` since it lacks an explicit value.

 Using the `error_code` method:

 ```rust
 let error1 = MyError::InvalidInput;
 println!("Error code: {}", error1.error_code()); // Outputs: "100"

 let error2 = MyError::NotFound;
 println!("Error code: {}", error2.error_code()); // Outputs: "300"
 ```

 **Key Points**:
 - If you don’t set `type`, it defaults to `u32`.

 ## Compatibility with `thiserror`

 The `ErrorCode` macro works seamlessly with `thiserror`, allowing you to combine rich error messages with numeric error codes. This is especially useful when you want both human-readable error details and machine-readable codes.

 ### Example: Using `ErrorCode` with `thiserror`

 Here’s an example that integrates both macros:

 ```rust
 use error_code::ErrorCode;
 use thiserror::Error;
 use std::io::Error as IoError;

 #[derive(Error, ErrorCode, Debug)]
 #[error_code(type = "u16", default = 300)]
 enum MyError {
     #[error("Invalid input: {0}")]
     #[error_code(100)]
     InvalidInput(String),

     #[error("Resource not found: {name}")]
     #[error_code(200)]
     NotFound { name: String },

     #[error(transparent)]
     IoError(#[from] IoError), // Uses default error code
 }
 ```

 In this example:
 - `thiserror` provides descriptive error messages.
 - `ErrorCode` adds numeric codes.
 - `IoError` uses the default code (`300`) since it doesn’t have an explicit `#[error_code(value)]`.

 Using both features:

 ```rust
 let error = MyError::InvalidInput("test".to_string());
 println!("Error: {}", error);          // Outputs: "Invalid input: test"
 println!("Error code: {}", error.error_code()); // Outputs: "100"

 let io_error = IoError::new(std::io::ErrorKind::Other, "io failure");
 let error = MyError::IoError(io_error);
 println!("Error: {}", error);          // Outputs: "io failure"
 println!("Error code: {}", error.error_code()); // Outputs: "300"
 ```

 **Why It Works Well**:
 - `ErrorCode` and `thiserror` complement each other without overlap.
 - You can use `ErrorCode` alone for codes, `thiserror` alone for messages, or both together, giving you full flexibility.
