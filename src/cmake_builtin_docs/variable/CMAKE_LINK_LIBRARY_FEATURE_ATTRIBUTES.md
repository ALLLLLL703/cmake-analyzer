CMake 4.4.4 · [Official documentation](https://cmake.org/cmake/help/v4.4/variable/CMAKE_LINK_LIBRARY_FEATURE_ATTRIBUTES.html)

# CMAKE_LINK_LIBRARY_\<FEATURE\>_ATTRIBUTES

Added in version 3.30.

This variable defines the behavior of the specified link library `<FEATURE>`. It specifies how the `<FEATURE>` interacts with other features, when the `<FEATURE>` should be applied, and aspects of how the `<FEATURE>` should be handled when CMake assembles the final linker command line (e.g. de-duplication).

The syntax of the linker flags for the `<FEATURE>` are controlled by the [`CMAKE_<LANG>_LINK_LIBRARY_USING_<FEATURE>`](https://cmake.org/cmake/help/v4.4/variable/CMAKE_LANG_LINK_LIBRARY_USING_FEATURE.html#variable:CMAKE_%3CLANG%3E_LINK_LIBRARY_USING_%3CFEATURE%3E) and [`CMAKE_LINK_LIBRARY_USING_<FEATURE>`](https://cmake.org/cmake/help/v4.4/variable/CMAKE_LINK_LIBRARY_USING_FEATURE.html#variable:CMAKE_LINK_LIBRARY_USING_%3CFEATURE%3E) variables. The [`CMAKE_<LANG>_LINK_LIBRARY_USING_<FEATURE>_SUPPORTED`](https://cmake.org/cmake/help/v4.4/variable/CMAKE_LANG_LINK_LIBRARY_USING_FEATURE_SUPPORTED.html#variable:CMAKE_%3CLANG%3E_LINK_LIBRARY_USING_%3CFEATURE%3E_SUPPORTED) and [`CMAKE_LINK_LIBRARY_USING_<FEATURE>_SUPPORTED`](https://cmake.org/cmake/help/v4.4/variable/CMAKE_LINK_LIBRARY_USING_FEATURE_SUPPORTED.html#variable:CMAKE_LINK_LIBRARY_USING_%3CFEATURE%3E_SUPPORTED) variables control whether the `<FEATURE>` is available at all.

When linking for a particular language `<LANG>`, `CMAKE_LINK_LIBRARY_<FEATURE>_ATTRIBUTES` is ignored if the [`CMAKE_<LANG>_LINK_LIBRARY_<FEATURE>_ATTRIBUTES`](https://cmake.org/cmake/help/v4.4/variable/CMAKE_LANG_LINK_LIBRARY_FEATURE_ATTRIBUTES.html#variable:CMAKE_%3CLANG%3E_LINK_LIBRARY_%3CFEATURE%3E_ATTRIBUTES) variable is also defined for the same `<FEATURE>`.

The value of `CMAKE_LINK_LIBRARY_<FEATURE>_ATTRIBUTES` and [`CMAKE_<LANG>_LINK_LIBRARY_<FEATURE>_ATTRIBUTES`](https://cmake.org/cmake/help/v4.4/variable/CMAKE_LANG_LINK_LIBRARY_FEATURE_ATTRIBUTES.html#variable:CMAKE_%3CLANG%3E_LINK_LIBRARY_%3CFEATURE%3E_ATTRIBUTES) at the end of the directory scope in which a target is defined is what matters.

## Feature Attributes Definition

A feature attributes definition is a [semicolon-separated list](https://cmake.org/cmake/help/v4.4/manual/cmake-language.7.html#cmake-language-lists) of `attribute=value(s)` items. If an attribute has multiple values, those values must be comma-separated.

The following attributes are supported:

**`LIBRARY_TYPE=<library-type-list>`**

Specify the library types supported by the feature. Supported values are: `STATIC`, `SHARED`, `MODULE`, and `EXECUTABLE`.

If this attribute is not specified, the default is `LIBRARY_TYPE=STATIC,SHARED,MODULE,EXECUTABLE`.

If the feature is used with an unsupported library type, CMake will emit a developer warning and the feature will be ignored.

**`OVERRIDE=<feature-list>`**

Specify which features this one replaces in the event of a conflict. This override mechanism is superseded by [`LINK_LIBRARY_OVERRIDE`](https://cmake.org/cmake/help/v4.4/prop_tgt/LINK_LIBRARY_OVERRIDE.html#prop_tgt:LINK_LIBRARY_OVERRIDE) or [`LINK_LIBRARY_OVERRIDE_<LIBRARY>`](https://cmake.org/cmake/help/v4.4/prop_tgt/LINK_LIBRARY_OVERRIDE_LIBRARY.html#prop_tgt:LINK_LIBRARY_OVERRIDE_%3CLIBRARY%3E) target property definitions, if defined.

If this attribute is not specified, the default is an empty list.

**`DEDUPLICATION=YES|NO|DEFAULT`**

Specify the de-duplication strategy for a library using this feature.

**`YES`**

The library is always de-duplicated. The default strategy CMake would normally apply is ignored.

**`NO`**

The library is never de-duplicated. The default strategy CMake would normally apply is ignored.

**`DEFAULT`**

Let CMake determine a de-duplication strategy automatically.

If this attribute is not specified, `DEFAULT` will be used.

## Example

A common need is the loading of a full archive as part of the creation of a shared library. The built-in `WHOLE_ARCHIVE` feature can be used for that purpose. The implementation of that built-in feature sets the following link library feature attributes:

~~~
set(CMAKE_LINK_LIBRARY_WHOLE_ARCHIVE_ATTRIBUTES
  LIBRARY_TYPE=STATIC
  OVERRIDE=DEFAULT
  DEDUPLICATION=YES
)
~~~

**`LIBRARY_TYPE=STATIC`**

This feature is only meaningful for static libraries.

**`OVERRIDE=DEFAULT`**

The `DEFAULT` feature will be overridden by the `WHOLE_ARCHIVE` feature because they are compatible and enhance the user's experience: standard library specification and `$<LINK_LIBRARY:WHOLE_ARCHIVE>` can be used freely.

**`DEDUPLICATION=YES`**

When this feature is used, the linker loads all symbols from the static library, so there is no need to repeat the library on the linker command line.

The `WHOLE_ARCHIVE` feature can be used like so:

~~~
add_library(A STATIC ...)
add_library(B STATIC ...)

target_link_libraries(B PUBLIC A)
target_link_libraries(A PUBLIC B)

add_library(global SHARED ...)
target_link_libraries(global PRIVATE $<LINK_LIBRARY:WHOLE_ARCHIVE,A>)
~~~

The resulting link command will only have one instance of the `A` library specified, and the linker flags will ensure that all symbols are loaded from the `A` library.

<!-- Script-generated content. Generator: scripts/gen/cmake_builtin_docs.py. -->
