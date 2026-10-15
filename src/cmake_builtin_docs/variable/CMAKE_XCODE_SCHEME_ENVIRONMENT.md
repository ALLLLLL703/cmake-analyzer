CMake 4.4.4 · [Official documentation](https://cmake.org/cmake/help/v4.4/variable/CMAKE_XCODE_SCHEME_ENVIRONMENT.html)

# CMAKE_XCODE_SCHEME_ENVIRONMENT

Added in version 3.17.

Specify environment variables that should be added to the Arguments section of the generated Xcode scheme.

If set to a list of environment variables and values of the form `MYVAR=value` those environment variables will be added to the scheme.

This variable initializes the [`XCODE_SCHEME_ENVIRONMENT`](https://cmake.org/cmake/help/v4.4/prop_tgt/XCODE_SCHEME_ENVIRONMENT.html#prop_tgt:XCODE_SCHEME_ENVIRONMENT) property on all targets.

Please refer to the [`XCODE_GENERATE_SCHEME`](https://cmake.org/cmake/help/v4.4/prop_tgt/XCODE_GENERATE_SCHEME.html#prop_tgt:XCODE_GENERATE_SCHEME) target property documentation to see all Xcode schema related properties.

<!-- Script-generated content. Generator: scripts/gen/cmake_builtin_docs.py. -->
