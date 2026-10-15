CMake 4.4.4 · [Official documentation](https://cmake.org/cmake/help/v4.4/variable/CMAKE_ERROR_DEPRECATED.html)

# CMAKE_ERROR_DEPRECATED

Deprecated since version 4.4.

Whether to issue errors for deprecated functionality.

This is a deprecated mechanism for interacting with the [`CMD_DEPRECATED`](https://cmake.org/cmake/help/v4.4/diagnostic/CMD_DEPRECATED.html#diagnostic:CMD_DEPRECATED) [`diagnostic`](https://cmake.org/cmake/help/v4.4/manual/cmake-diagnostics.7.html#manual:cmake-diagnostics(7)). If policy [`CMP0218`](https://cmake.org/cmake/help/v4.4/policy/CMP0218.html#policy:CMP0218) is set to `NEW`, this variable is ignored.

If policy [`CMP0218`](https://cmake.org/cmake/help/v4.4/policy/CMP0218.html#policy:CMP0218) is not set to `NEW`, the [`message()`](https://cmake.org/cmake/help/v4.4/command/message.html#command:message) command's `DEPRECATION` message type will use [`CMAKE_WARN_DEPRECATED`](https://cmake.org/cmake/help/v4.4/variable/CMAKE_WARN_DEPRECATED.html#variable:CMAKE_WARN_DEPRECATED) and `CMAKE_ERROR_DEPRECATED` to determine the severity of a deprecation diagnostic. The severity will be:

- `IGNORE`, if `CMAKE_WARN_DEPRECATED` is `OFF` and `CMAKE_ERROR_DEPRECATED` is unset or `OFF`.

- `WARN`, if `CMAKE_WARN_DEPRECATED` is unset or `ON` and `CMAKE_ERROR_DEPRECATED` is unset or `OFF`.

- `FATAL_ERROR` if `CMAKE_ERROR_DEPRECATED` is `ON`.

Setting `CMAKE_ERROR_DEPRECATED` in the cache will alter the default state of the [`CMD_DEPRECATED`](https://cmake.org/cmake/help/v4.4/diagnostic/CMD_DEPRECATED.html#diagnostic:CMD_DEPRECATED) [`diagnostic`](https://cmake.org/cmake/help/v4.4/manual/cmake-diagnostics.7.html#manual:cmake-diagnostics(7)); however, the [`-W[no-][error=]deprecated`](https://cmake.org/cmake/help/v4.4/manual/cmake.1.html#cmdoption-cmake-W) option, and [`warnings.deprecated`](https://cmake.org/cmake/help/v4.4/manual/cmake-presets.7.html#cmakepresets-configurepresets-warnings-deprecated) and/or [`errors.deprecated`](https://cmake.org/cmake/help/v4.4/manual/cmake-presets.7.html#cmakepresets-configurepresets-errors-deprecated) preset fields, will take precedence. CMake will also update the cached value of `CMAKE_ERROR_DEPRECATED` to reflect the actual state of the diagnostic as of the start of script execution.

<!-- Script-generated content. Generator: scripts/gen/cmake_builtin_docs.py. -->
