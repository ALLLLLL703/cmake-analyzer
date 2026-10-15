CMake 4.4.4 · [Official documentation](https://cmake.org/cmake/help/v4.4/variable/CMAKE_LANG_LINK_LIBRARY_USING_FEATURE.html)

# CMAKE_\<LANG\>_LINK_LIBRARY_USING_\<FEATURE\>

Added in version 3.24.

This variable defines how to link a library or framework for the specified `<FEATURE>` when a [`LINK_LIBRARY`](https://cmake.org/cmake/help/v4.4/manual/cmake-generator-expressions.7.html#genex:LINK_LIBRARY) generator expression is used and the link language for the target is `<LANG>`. For this variable to have any effect, the associated [`CMAKE_<LANG>_LINK_LIBRARY_USING_<FEATURE>_SUPPORTED`](https://cmake.org/cmake/help/v4.4/variable/CMAKE_LANG_LINK_LIBRARY_USING_FEATURE_SUPPORTED.html#variable:CMAKE_%3CLANG%3E_LINK_LIBRARY_USING_%3CFEATURE%3E_SUPPORTED) variable must be set to true.

The [`CMAKE_LINK_LIBRARY_USING_<FEATURE>`](https://cmake.org/cmake/help/v4.4/variable/CMAKE_LINK_LIBRARY_USING_FEATURE.html#variable:CMAKE_LINK_LIBRARY_USING_%3CFEATURE%3E) variable should be defined instead for features that are independent of the link language.

Feature names are case-sensitive and may only contain letters, numbers and underscores. Feature names defined in all uppercase are reserved for CMake's own built-in features (see [Predefined Features](https://cmake.org/cmake/help/v4.4/variable/CMAKE_LANG_LINK_LIBRARY_USING_FEATURE.html#predefined-features) further below).

Some aspects of feature behavior can be defined by the [`CMAKE_<LANG>_LINK_LIBRARY_<FEATURE>_ATTRIBUTES`](https://cmake.org/cmake/help/v4.4/variable/CMAKE_LANG_LINK_LIBRARY_FEATURE_ATTRIBUTES.html#variable:CMAKE_%3CLANG%3E_LINK_LIBRARY_%3CFEATURE%3E_ATTRIBUTES) and [`CMAKE_LINK_LIBRARY_<FEATURE>_ATTRIBUTES`](https://cmake.org/cmake/help/v4.4/variable/CMAKE_LINK_LIBRARY_FEATURE_ATTRIBUTES.html#variable:CMAKE_LINK_LIBRARY_%3CFEATURE%3E_ATTRIBUTES) variables.

## Feature Definitions

A library feature definition is a list that contains one or three elements:

~~~
[<PREFIX>] <LIBRARY_EXPRESSION> [<SUFFIX>]
~~~

When `<PREFIX>` and `<SUFFIX>` are specified, they precede and follow respectively the whole list of libraries specified in the [`LINK_LIBRARY`](https://cmake.org/cmake/help/v4.4/manual/cmake-generator-expressions.7.html#genex:LINK_LIBRARY) expression, not each library item individually. There is no guarantee that the list of specified libraries will be kept grouped together though, so the `<PREFIX>` and `<SUFFIX>` may appear more than once if the library list is reorganized by CMake to satisfy other constraints. This means constructs like `--start-group` and `--end-group`, as supported by the GNU `ld` linker, cannot be used in this way. The [`LINK_GROUP`](https://cmake.org/cmake/help/v4.4/manual/cmake-generator-expressions.7.html#genex:LINK_GROUP) generator expression should be used instead for such constructs.

`<LIBRARY_EXPRESSION>` is used to specify the pattern for constructing the corresponding fragment on the linker command line for each library. The following placeholders can be used in the expression:

- `<LIBRARY>` is expanded to the full path to the library for CMake targets, or to a platform-specific value based on the item otherwise (the same as `<LINK_ITEM>` on Windows, or the library base name for other platforms).

- `<LINK_ITEM>` is expanded to how the library would normally be linked on the linker command line.

- `<LIB_ITEM>` is expanded to the full path to the library for CMake targets, or the item itself exactly as specified in the `<LIBRARY_EXPRESSION>` otherwise.

In addition to the above, it is possible to have one pattern for paths (CMake targets and external libraries specified with file paths) and another for other items specified by name only. The `PATH{}` and `NAME{}` wrappers can be used to provide the expansion for those two cases, respectively. When wrappers are used, both must be present. For example:

~~~
set(CMAKE_LINK_LIBRARY_USING_weak_library
    "PATH{-weak_library <LIBRARY>}NAME{LINKER:-weak-l<LIB_ITEM>}"
)
~~~

For all three elements of this variable (`<PREFIX>`, `<LIBRARY_EXPRESSION>`, and `<SUFFIX>`), the `LINKER:` prefix can be used.

To pass options to the linker tool, each compiler driver has its own syntax. The `LINKER:` prefix and `,` separator can be used to specify, in a portable way, options to pass to the linker tool. `LINKER:` is replaced by the appropriate driver option and `,` by the appropriate driver separator. The driver prefix and driver separator are given by the values of the [`CMAKE_<LANG>_LINKER_WRAPPER_FLAG`](https://cmake.org/cmake/help/v4.4/variable/CMAKE_LANG_LINKER_WRAPPER_FLAG.html#variable:CMAKE_%3CLANG%3E_LINKER_WRAPPER_FLAG) and [`CMAKE_<LANG>_LINKER_WRAPPER_FLAG_SEP`](https://cmake.org/cmake/help/v4.4/variable/CMAKE_LANG_LINKER_WRAPPER_FLAG_SEP.html#variable:CMAKE_%3CLANG%3E_LINKER_WRAPPER_FLAG_SEP) variables.

For example, `"LINKER:-z,defs"` becomes `-Xlinker -z -Xlinker defs` for `Clang` and `-Wl,-z,defs` for `GNU GCC`.

The `LINKER:` prefix can be specified as part of a `SHELL:` prefix expression.

The `LINKER:` prefix supports, as an alternative syntax, specification of arguments using the `SHELL:` prefix and space as separator. The previous example then becomes `"LINKER:SHELL:-z defs"`.

Note

Specifying the `SHELL:` prefix anywhere other than at the beginning of the `LINKER:` prefix is not supported.

## Examples

### Loading a whole static library

A common need is to prevent the linker from discarding any symbols from a static library. Different linkers use different syntax for achieving this. The following example shows how this may be implemented for some linkers. Note that this is for illustration purposes only. Projects should use the built-in `WHOLE_ARCHIVE` feature instead (see [Predefined Features](https://cmake.org/cmake/help/v4.4/variable/CMAKE_LANG_LINK_LIBRARY_USING_FEATURE.html#predefined-features)), which provides a more complete and more robust implementation of this functionality.

~~~
set(CMAKE_C_LINK_LIBRARY_USING_load_archive_SUPPORTED TRUE)
if(CMAKE_C_COMPILER_ID STREQUAL "AppleClang")
  set(CMAKE_C_LINK_LIBRARY_USING_load_archive "-force_load <LIB_ITEM>")
elseif(CMAKE_C_COMPILER_ID STREQUAL "GNU" AND CMAKE_SYSTEM_NAME STREQUAL "Linux")
  set(CMAKE_C_LINK_LIBRARY_USING_load_archive
    "LINKER:--push-state,--whole-archive"
    "<LINK_ITEM>"
    "LINKER:--pop-state"
  )
elseif(CMAKE_C_COMPILER_ID STREQUAL "MSVC")
  set(CMAKE_C_LINK_LIBRARY_USING_load_archive "/WHOLEARCHIVE:<LIBRARY>")
else()
  # feature not yet supported for the other environments
  set(CMAKE_C_LINK_LIBRARY_USING_load_archive_SUPPORTED FALSE)
endif()

add_library(lib1 STATIC ...)
add_library(lib2 SHARED ...)

if(CMAKE_C_LINK_LIBRARY_USING_load_archive_SUPPORTED)
  # The -force_load Apple linker option requires a file name
  set(external_lib
    "$<IF:$<LINK_LANG_AND_ID:C,AppleClang>,libexternal.a,external>"
  )
  target_link_libraries(lib2 PRIVATE
    "$<LINK_LIBRARY:load_archive,lib1,${external_lib}>"
  )
else()
  target_link_libraries(lib2 PRIVATE lib1 external)
endif()
~~~

CMake will generate the following link expressions:

- `AppleClang`: `-force_load /path/to/lib1.a -force_load libexternal.a`

- `GNU`: `-Wl,--push-state,--whole-archive /path/to/lib1.a -lexternal -Wl,--pop-state`

- `MSVC`: `/WHOLEARCHIVE:/path/to/lib1.lib /WHOLEARCHIVE:external.lib`

### Linking a library as weak

On macOS, it is possible to link a library in weak mode (the library and all references are marked as weak imports). Different flags must be used for a library specified by file path compared to one specified by name. This constraint can be solved using `PATH{}` and `NAME{}` wrappers. Again, the following example shows how this may be implemented for some linkers, but it is for illustration purposes only. Projects should use the built-in `WEAK_FRAMEWORK` or `WEAK_LIBRARY` features instead (see [Predefined Features](https://cmake.org/cmake/help/v4.4/variable/CMAKE_LANG_LINK_LIBRARY_USING_FEATURE.html#predefined-features)), which provide more complete and more robust implementations of this functionality.

~~~
if (CMAKE_C_COMPILER_ID STREQUAL "AppleClang")
  set(CMAKE_LINK_LIBRARY_USING_weak_library
      "PATH{-weak_library <LIBRARY>}NAME{LINKER:-weak-l<LIB_ITEM>}"
  )
  set(CMAKE_LINK_LIBRARY_USING_weak_library_SUPPORTED TRUE)
endif()

add_library(lib SHARED ...)
add_executable(main ...)
if(CMAKE_LINK_LIBRARY_USING_weak_library_SUPPORTED)
  target_link_libraries(main PRIVATE "$<LINK_LIBRARY:weak_library,lib,external>")
else()
  target_link_libraries(main PRIVATE lib external)
endif()
~~~

CMake will generate the following linker command line fragment when linking `main` using the `AppleClang` toolchain:

`-weak_library /path/to/lib -Xlinker -weak-lexternal`.

## Predefined Features

The following built-in library features are pre-defined by CMake:

**`DEFAULT`**

This feature corresponds to standard linking, essentially equivalent to using no feature at all. It is typically only used with the [`LINK_LIBRARY_OVERRIDE`](https://cmake.org/cmake/help/v4.4/prop_tgt/LINK_LIBRARY_OVERRIDE.html#prop_tgt:LINK_LIBRARY_OVERRIDE) and [`LINK_LIBRARY_OVERRIDE_<LIBRARY>`](https://cmake.org/cmake/help/v4.4/prop_tgt/LINK_LIBRARY_OVERRIDE_LIBRARY.html#prop_tgt:LINK_LIBRARY_OVERRIDE_%3CLIBRARY%3E) target properties.

**`WHOLE_ARCHIVE`**

Force inclusion of all members of a static library when linked as a dependency of consuming [Executables](https://cmake.org/cmake/help/v4.4/manual/cmake-buildsystem.7.html#executables), [Shared Libraries](https://cmake.org/cmake/help/v4.4/manual/cmake-buildsystem.7.html#shared-libraries), and [Module Libraries](https://cmake.org/cmake/help/v4.4/manual/cmake-buildsystem.7.html#module-libraries). This feature is only supported for the following platforms, with limitations as noted:

- Linux.

- All BSD variants.

- SunOS.

- All Apple variants. The library must be specified as a CMake target name, a library file name (such as `libfoo.a`), or a library file path (such as `/path/to/libfoo.a`). Due to a limitation of the Apple linker, it cannot be specified as a plain library name like `foo`, where `foo` is not a CMake target.

- Windows. When using a MSVC or MSVC-like toolchain, the MSVC version must be greater than 1900.

- Cygwin.

- MSYS.

Note

Since [Static Libraries](https://cmake.org/cmake/help/v4.4/manual/cmake-buildsystem.7.html#static-libraries) are archives and not linked binaries, CMake records their link dependencies for transitive use when linking consuming binaries. Therefore `WHOLE_ARCHIVE` does not cause a static library's objects to be included in other static libraries. Use [Object Libraries](https://cmake.org/cmake/help/v4.4/manual/cmake-buildsystem.7.html#object-libraries) for that.

**`FRAMEWORK`**

This option tells the linker to search for the specified framework using the `-framework` linker option. It can only be used on Apple platforms, and only with a linker that understands the option used (i.e. the linker provided with Xcode, or one compatible with it).

The framework can be specified as a CMake framework target, a bare framework name, or a file path. If a target is given, that target must have the [`FRAMEWORK`](https://cmake.org/cmake/help/v4.4/prop_tgt/FRAMEWORK.html#prop_tgt:FRAMEWORK) target property set to true. For a file path, if it contains a directory part, that directory will be added as a framework search path.

~~~
add_library(lib SHARED ...)
target_link_libraries(lib PRIVATE "$<LINK_LIBRARY:FRAMEWORK,/path/to/my_framework>")

# The constructed linker command line will contain:
#   -F/path/to -framework my_framework
~~~

File paths must conform to one of the following patterns (`*` is a wildcard, and optional parts are shown as `[...]`):

- `[/path/to/]FwName[.framework]`

- `[/path/to/]FwName.framework/FwName[suffix]`

- `[/path/to/]FwName.framework/Versions/*/FwName[suffix]`

Note that CMake recognizes and automatically handles framework targets, even without using the [`$<LINK_LIBRARY:FRAMEWORK,...>`](https://cmake.org/cmake/help/v4.4/manual/cmake-generator-expressions.7.html#genex:LINK_LIBRARY) expression. The generator expression can still be used with a CMake target if the project wants to be explicit about it, but it is not required to do so. The linker command line may have some differences between using the generator expression or not, but the final result should be the same. On the other hand, if a file path is given, CMake will recognize some paths automatically, but not all cases. The project may want to use [`$<LINK_LIBRARY:FRAMEWORK,...>`](https://cmake.org/cmake/help/v4.4/manual/cmake-generator-expressions.7.html#genex:LINK_LIBRARY) for file paths so that the expected behavior is clear.

Added in version 3.25: The [`FRAMEWORK_MULTI_CONFIG_POSTFIX_<CONFIG>`](https://cmake.org/cmake/help/v4.4/prop_tgt/FRAMEWORK_MULTI_CONFIG_POSTFIX_CONFIG.html#prop_tgt:FRAMEWORK_MULTI_CONFIG_POSTFIX_%3CCONFIG%3E) target property as well as the `suffix` of the framework library name are now supported by the `FRAMEWORK` features.

**`NEEDED_FRAMEWORK`**

This is similar to the `FRAMEWORK` feature, except it forces the linker to link with the framework even if no symbols are used from it. It uses the `-needed_framework` option and has the same linker constraints as `FRAMEWORK`.

**`REEXPORT_FRAMEWORK`**

This is similar to the `FRAMEWORK` feature, except it tells the linker that the framework should be available to clients linking to the library being created. It uses the `-reexport_framework` option and has the same linker constraints as `FRAMEWORK`.

**`WEAK_FRAMEWORK`**

This is similar to the `FRAMEWORK` feature, except it forces the linker to mark the framework and all references to it as weak imports. It uses the `-weak_framework` option and has the same linker constraints as `FRAMEWORK`.

**`NEEDED_LIBRARY`**

This is similar to the `NEEDED_FRAMEWORK` feature, except it is for use with non-framework targets or libraries (Apple platforms only). It uses the `-needed_library` or `-needed-l` option as appropriate, and has the same linker constraints as `NEEDED_FRAMEWORK`.

**`REEXPORT_LIBRARY`**

This is similar to the `REEXPORT_FRAMEWORK` feature, except it is for use with non-framework targets or libraries (Apple platforms only). It uses the `-reexport_library` or `-reexport-l` option as appropriate, and has the same linker constraints as `REEXPORT_FRAMEWORK`.

**`WEAK_LIBRARY`**

This is similar to the `WEAK_FRAMEWORK` feature, except it is for use with non-framework targets or libraries (Apple platforms only). It uses the `-weak_library` or `-weak-l` option as appropriate, and has the same linker constraints as `WEAK_FRAMEWORK`.

<!-- Script-generated content. Generator: scripts/gen/cmake_builtin_docs.py. -->
