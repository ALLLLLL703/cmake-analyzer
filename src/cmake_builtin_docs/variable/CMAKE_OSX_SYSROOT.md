CMake 4.4.4 · [Official documentation](https://cmake.org/cmake/help/v4.4/variable/CMAKE_OSX_SYSROOT.html)

# CMAKE_OSX_SYSROOT

Specify the location or name of the macOS platform SDK to be used.

If not set explicitly, the value is initialized by the `SDKROOT` environment variable, if set. Otherwise, the value defaults to empty, and the compiler is expected to choose a default macOS SDK on its own.

Changed in version 4.0: The default is now empty. Previously a default was computed based on the [`CMAKE_OSX_DEPLOYMENT_TARGET`](https://cmake.org/cmake/help/v4.4/variable/CMAKE_OSX_DEPLOYMENT_TARGET.html#variable:CMAKE_OSX_DEPLOYMENT_TARGET) or the host platform.

In order to pass an explicit macOS SDK via the compiler's `-isysroot` flag, users may configure their build tree with `-DCMAKE_OSX_SYSROOT=macosx`, or `export SDKROOT="$(xcrun --sdk macosx --show-sdk-path)"` in their environment.

Notes:

- macOS compilers in `/usr/bin`, when not invoked with `-isysroot`, search for headers in `/usr/local/include` before system SDK paths, matching the convention on many platforms. Users on macOS-x86_64 hosts with Homebrew installed in `/usr/local` should pass an explicit SDK, as described above, when not building with Homebrew tools.

- Some Clang compilers have no default macOS SDK selection. For these, if [`CMAKE_OSX_SYSROOT`](https://cmake.org/cmake/help/v4.4/variable/CMAKE_OSX_SYSROOT.html#variable:CMAKE_OSX_SYSROOT) is empty, CMake will automatically pass `-isysroot` with the macOS SDK printed by `xcrun --show-sdk-path`.

The value of this variable should be set prior to the first [`project()`](https://cmake.org/cmake/help/v4.4/command/project.html#command:project) or [`enable_language()`](https://cmake.org/cmake/help/v4.4/command/enable_language.html#command:enable_language) command invocation because it may influence configuration of the toolchain and flags. It is intended to be set locally by the user creating a build tree. This variable should be set as a `CACHE` entry (or else CMake may remove it while initializing a cache entry of the same name) unless policy [`CMP0126`](https://cmake.org/cmake/help/v4.4/policy/CMP0126.html#policy:CMP0126) is set to `NEW`.

Despite the `OSX` part in the variable name(s) they apply also to other SDKs than macOS like iOS, tvOS, visionOS, or watchOS.

This variable is ignored on platforms other than Apple.

<!-- Script-generated content. Generator: scripts/gen/cmake_builtin_docs.py. -->
