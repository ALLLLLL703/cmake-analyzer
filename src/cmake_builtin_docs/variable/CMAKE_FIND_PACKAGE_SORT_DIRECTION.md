CMake 4.4.4 · [Official documentation](https://cmake.org/cmake/help/v4.4/variable/CMAKE_FIND_PACKAGE_SORT_DIRECTION.html)

# CMAKE_FIND_PACKAGE_SORT_DIRECTION

Added in version 3.7.

Changed in version 4.2: The default sort direction has changed from `DEC` to `ASC`.

The sorting direction used by [`CMAKE_FIND_PACKAGE_SORT_ORDER`](https://cmake.org/cmake/help/v4.4/variable/CMAKE_FIND_PACKAGE_SORT_ORDER.html#variable:CMAKE_FIND_PACKAGE_SORT_ORDER). It can assume one of the following values:

**`ASC`**

Ordering is done in ascending mode. The lowest folder found will be tested first.

**`DEC`**

Default. Ordering is done in descending mode. The highest folder found will be tested first.

If [`CMAKE_FIND_PACKAGE_SORT_ORDER`](https://cmake.org/cmake/help/v4.4/variable/CMAKE_FIND_PACKAGE_SORT_ORDER.html#variable:CMAKE_FIND_PACKAGE_SORT_ORDER) is set to `NONE` this variable has no effect.

<!-- Script-generated content. Generator: scripts/gen/cmake_builtin_docs.py. -->
