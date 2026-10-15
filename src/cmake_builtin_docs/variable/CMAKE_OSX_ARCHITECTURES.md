CMake 4.4.4 · [Official documentation](https://cmake.org/cmake/help/v4.4/variable/CMAKE_OSX_ARCHITECTURES.html)

# CMAKE_OSX_ARCHITECTURES

Target specific architectures for macOS and iOS.

This variable is used to initialize the [`OSX_ARCHITECTURES`](https://cmake.org/cmake/help/v4.4/prop_tgt/OSX_ARCHITECTURES.html#prop_tgt:OSX_ARCHITECTURES) property on each target as it is created. See that target property for additional information.

If `CMAKE_OSX_ARCHITECTURES` is not set, the compiler's default target architecture is used. For compilers provided by Xcode, this is the host machine's architecture.

The value of this variable should be set prior to the first [`project()`](https://cmake.org/cmake/help/v4.4/command/project.html#command:project) or [`enable_language()`](https://cmake.org/cmake/help/v4.4/command/enable_language.html#command:enable_language) command invocation because it may influence configuration of the toolchain and flags. It is intended to be set locally by the user creating a build tree. This variable should be set as a `CACHE` entry (or else CMake may remove it while initializing a cache entry of the same name) unless policy [`CMP0126`](https://cmake.org/cmake/help/v4.4/policy/CMP0126.html#policy:CMP0126) is set to `NEW`.

Despite the `OSX` part in the variable name(s) they apply also to other SDKs than macOS like iOS, tvOS, visionOS, or watchOS.

This variable is ignored on platforms other than Apple.

<!-- Script-generated content. Generator: scripts/gen/cmake_builtin_docs.py. -->
