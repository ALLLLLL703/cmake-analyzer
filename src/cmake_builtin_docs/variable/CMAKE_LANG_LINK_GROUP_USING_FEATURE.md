CMake 4.4.4 · [Official documentation](https://cmake.org/cmake/help/v4.4/variable/CMAKE_LANG_LINK_GROUP_USING_FEATURE.html)

# CMAKE_\<LANG\>_LINK_GROUP_USING_\<FEATURE\>

Added in version 3.24.

This variable defines how to link a group of libraries for the specified `<FEATURE>` when a [`LINK_GROUP`](https://cmake.org/cmake/help/v4.4/manual/cmake-generator-expressions.7.html#genex:LINK_GROUP) generator expression is used and the link language for the target is `<LANG>`. For this variable to have any effect, the associated [`CMAKE_<LANG>_LINK_GROUP_USING_<FEATURE>_SUPPORTED`](https://cmake.org/cmake/help/v4.4/variable/CMAKE_LANG_LINK_GROUP_USING_FEATURE_SUPPORTED.html#variable:CMAKE_%3CLANG%3E_LINK_GROUP_USING_%3CFEATURE%3E_SUPPORTED) variable must be set to true.

The [`CMAKE_LINK_GROUP_USING_<FEATURE>`](https://cmake.org/cmake/help/v4.4/variable/CMAKE_LINK_GROUP_USING_FEATURE.html#variable:CMAKE_LINK_GROUP_USING_%3CFEATURE%3E) variable should be defined instead for features that are independent of the link language.

Feature names are case-sensitive and may only contain letters, numbers and underscores. Feature names defined in all uppercase are reserved for CMake's own built-in features (see [Predefined Features](https://cmake.org/cmake/help/v4.4/variable/CMAKE_LANG_LINK_GROUP_USING_FEATURE.html#predefined-features) further below).

## Feature Definitions

A group feature definition is a list that contains exactly two elements:

~~~
<PREFIX> <SUFFIX>
~~~

On the linker command line, `<PREFIX>` will precede the list of libraries in the group and `<SUFFIX>` will follow after.

For the elements of this variable, the `LINKER:` prefix can be used.

To pass options to the linker tool, each compiler driver has its own syntax. The `LINKER:` prefix and `,` separator can be used to specify, in a portable way, options to pass to the linker tool. `LINKER:` is replaced by the appropriate driver option and `,` by the appropriate driver separator. The driver prefix and driver separator are given by the values of the [`CMAKE_<LANG>_LINKER_WRAPPER_FLAG`](https://cmake.org/cmake/help/v4.4/variable/CMAKE_LANG_LINKER_WRAPPER_FLAG.html#variable:CMAKE_%3CLANG%3E_LINKER_WRAPPER_FLAG) and [`CMAKE_<LANG>_LINKER_WRAPPER_FLAG_SEP`](https://cmake.org/cmake/help/v4.4/variable/CMAKE_LANG_LINKER_WRAPPER_FLAG_SEP.html#variable:CMAKE_%3CLANG%3E_LINKER_WRAPPER_FLAG_SEP) variables.

For example, `"LINKER:-z,defs"` becomes `-Xlinker -z -Xlinker defs` for `Clang` and `-Wl,-z,defs` for `GNU GCC`.

The `LINKER:` prefix can be specified as part of a `SHELL:` prefix expression.

The `LINKER:` prefix supports, as an alternative syntax, specification of arguments using the `SHELL:` prefix and space as separator. The previous example then becomes `"LINKER:SHELL:-z defs"`.

Note

Specifying the `SHELL:` prefix anywhere other than at the beginning of the `LINKER:` prefix is not supported.

## Examples

### Solving cross-references between two static libraries

A project may define two or more static libraries which have circular dependencies between them. In order for the linker to resolve all symbols at link time, it may need to search repeatedly among the libraries until no new undefined references are created. Different linkers use different syntax for achieving this. The following example shows how this may be implemented for some linkers. Note that this is for illustration purposes only. Projects should use the built-in `RESCAN` group feature instead (see [Predefined Features](https://cmake.org/cmake/help/v4.4/variable/CMAKE_LANG_LINK_GROUP_USING_FEATURE.html#predefined-features)), which provides a more complete and more robust implementation of this functionality.

~~~
set(CMAKE_C_LINK_GROUP_USING_cross_refs_SUPPORTED TRUE)
if(CMAKE_C_COMPILER_ID STREQUAL "GNU" AND CMAKE_SYSTEM_NAME STREQUAL "Linux")
  set(CMAKE_C_LINK_GROUP_USING_cross_refs
    "LINKER:--start-group"
    "LINKER:--end-group"
  )
elseif(CMAKE_C_COMPILER_ID STREQUAL "SunPro" AND CMAKE_SYSTEM_NAME STREQUAL "SunOS")
  set(CMAKE_C_LINK_GROUP_USING_cross_refs
    "LINKER:-z,rescan-start"
    "LINKER:-z,rescan-end"
  )
else()
  # feature not yet supported for the other environments
  set(CMAKE_C_LINK_GROUP_USING_cross_refs_SUPPORTED FALSE)
endif()

add_library(lib1 STATIC ...)
add_library(lib2 SHARED ...)

if(CMAKE_C_LINK_GROUP_USING_cross_refs_SUPPORTED)
  target_link_libraries(lib2 PRIVATE "$<LINK_GROUP:cross_refs,lib1,external>")
else()
  target_link_libraries(lib2 PRIVATE lib1 external)
endif()
~~~

CMake will generate the following linker command line fragments when linking `lib2`:

- `GNU`: `-Wl,--start-group /path/to/lib1.a -lexternal -Wl,--end-group`

- `SunPro`: `-Wl,-z,rescan-start /path/to/lib1.a -lexternal -Wl,-z,rescan-end`

## Predefined Features

The following built-in group features are pre-defined by CMake:

**`RESCAN`**

Some linkers are single-pass only. For such linkers, circular references between libraries typically result in unresolved symbols. This feature instructs the linker to search the specified static libraries repeatedly until no new undefined references are created.

Normally, a static library is searched only once in the order that it is specified on the command line. If a symbol in that library is needed to resolve an undefined symbol referred to by an object in a library that appears later on the command line, the linker would not be able to resolve that reference. By grouping the static libraries with the `RESCAN` feature, they will all be searched repeatedly until all possible references are resolved. This will use linker options like `--start-group` and `--end-group`, or on SunOS, `-z rescan-start` and `-z rescan-end`.

Using this feature has a significant performance cost. It is best to use it only when there are unavoidable circular references between two or more static libraries.

This feature is available when using toolchains that target Linux, BSD, and SunOS. It can also be used when targeting Windows platforms if the GNU toolchain is used.

Added in version 4.4: `RESCAN` is additionally available when using toolchains targeting Haiku.

<!-- Script-generated content. Generator: scripts/gen/cmake_builtin_docs.py. -->
