CMake 4.4.4 · [Official documentation](https://cmake.org/cmake/help/v4.4/variable/CMAKE_AUTOMOC_MACRO_NAMES.html)

# CMAKE_AUTOMOC_MACRO_NAMES

Added in version 3.10.

[Semicolon-separated list](https://cmake.org/cmake/help/v4.4/manual/cmake-language.7.html#cmake-language-lists) list of macro names used by [`CMAKE_AUTOMOC`](https://cmake.org/cmake/help/v4.4/variable/CMAKE_AUTOMOC.html#variable:CMAKE_AUTOMOC) to determine if a C++ file needs to be processed by `moc`.

This variable is used to initialize the [`AUTOMOC_MACRO_NAMES`](https://cmake.org/cmake/help/v4.4/prop_tgt/AUTOMOC_MACRO_NAMES.html#prop_tgt:AUTOMOC_MACRO_NAMES) property on all the targets. See that target property for additional information.

The default value is `Q_OBJECT;Q_GADGET;Q_NAMESPACE;Q_NAMESPACE_EXPORT`.

## Example

Let CMake know that source files that contain `CUSTOM_MACRO` must be `moc` processed as well:

~~~
set(CMAKE_AUTOMOC ON)
list(APPEND CMAKE_AUTOMOC_MACRO_NAMES "CUSTOM_MACRO")
~~~

<!-- Script-generated content. Generator: scripts/gen/cmake_builtin_docs.py. -->
