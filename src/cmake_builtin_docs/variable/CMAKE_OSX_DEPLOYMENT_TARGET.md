CMake 4.4.4 · [Official documentation](https://cmake.org/cmake/help/v4.4/variable/CMAKE_OSX_DEPLOYMENT_TARGET.html)

# CMAKE_OSX_DEPLOYMENT_TARGET

Specify the minimum version of the target platform, e.g., macOS or iOS, on which the target binaries are to be deployed.

For builds targeting macOS ([`CMAKE_SYSTEM_NAME`](https://cmake.org/cmake/help/v4.4/variable/CMAKE_SYSTEM_NAME.html#variable:CMAKE_SYSTEM_NAME) is `Darwin`), if `CMAKE_OSX_DEPLOYMENT_TARGET` is not explicitly set, a default is set:

- If the `MACOSX_DEPLOYMENT_TARGET` environment variable is non-empty, its value is the default.

- Otherwise, if using the [`Xcode`](https://cmake.org/cmake/help/v4.4/generator/Xcode.html#generator:Xcode) generator, and the host's macOS version is older than the macOS SDK ([`CMAKE_OSX_SYSROOT`](https://cmake.org/cmake/help/v4.4/variable/CMAKE_OSX_SYSROOT.html#variable:CMAKE_OSX_SYSROOT), if set, or Xcode's default SDK), the host's macOS version is the default.

    

  Changed in version 4.0: Previously this was done for all generators, not just Xcode.

- Otherwise, the default is empty.

The effects of `CMAKE_OSX_DEPLOYMENT_TARGET` depend on the generator:

[`Xcode`](https://cmake.org/cmake/help/v4.4/generator/Xcode.html#generator:Xcode)

If `CMAKE_OSX_DEPLOYMENT_TARGET` is set to a non-empty value, it is added to the generated Xcode project as the `MACOSX_DEPLOYMENT_TARGET` setting. Otherwise, no such setting is added, so Xcode's default deployed target is used, typically based on the SDK version.

Other Generators

If `CMAKE_OSX_DEPLOYMENT_TARGET` is set to a non-empty value, it is passed to the compiler via the `-mmacosx-version-min` flag or equivalent. Otherwise, no such flag is added, so the compiler's default deployment target is used.

The value of this variable should be set prior to the first [`project()`](https://cmake.org/cmake/help/v4.4/command/project.html#command:project) or [`enable_language()`](https://cmake.org/cmake/help/v4.4/command/enable_language.html#command:enable_language) command invocation because it may influence configuration of the toolchain and flags. It is intended to be set locally by the user creating a build tree. This variable should be set as a `CACHE` entry (or else CMake may remove it while initializing a cache entry of the same name) unless policy [`CMP0126`](https://cmake.org/cmake/help/v4.4/policy/CMP0126.html#policy:CMP0126) is set to `NEW`.

Despite the `OSX` part in the variable name(s) they apply also to other SDKs than macOS like iOS, tvOS, visionOS, or watchOS.

This variable is ignored on platforms other than Apple.

<!-- Script-generated content. Generator: scripts/gen/cmake_builtin_docs.py. -->
