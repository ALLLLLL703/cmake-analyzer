CMake 4.4.4 · [Official documentation](https://cmake.org/cmake/help/v4.4/variable/PackageName_ROOT.html)

# \<PackageName\>_ROOT

Added in version 3.12.

Calls to [`find_package(<PackageName>)`](https://cmake.org/cmake/help/v4.4/command/find_package.html#command:find_package) will search in prefixes specified by the `<PackageName>_ROOT` CMake variable, where `<PackageName>` is the (case-preserved) name given to the [`find_package()`](https://cmake.org/cmake/help/v4.4/command/find_package.html#command:find_package) call and `_ROOT` is literal. For example, `find_package(Foo)` will search prefixes specified in the `Foo_ROOT` CMake variable (if set). See policy [`CMP0074`](https://cmake.org/cmake/help/v4.4/policy/CMP0074.html#policy:CMP0074).

This variable may hold a single prefix or a [semicolon-separated list](https://cmake.org/cmake/help/v4.4/manual/cmake-language.7.html#cmake-language-lists) of multiple prefixes.

See also the [`<PackageName>_ROOT`](https://cmake.org/cmake/help/v4.4/envvar/PackageName_ROOT.html#envvar:%3CPackageName%3E_ROOT) environment variable.

**\<PACKAGENAME\>_ROOT**

Added in version 3.27.

Calls to [`find_package(<PackageName>)`](https://cmake.org/cmake/help/v4.4/command/find_package.html#command:find_package) will also search in prefixes specified by the upper-case `<PACKAGENAME>_ROOT` CMake variable. See policy [`CMP0144`](https://cmake.org/cmake/help/v4.4/policy/CMP0144.html#policy:CMP0144).

<!-- Script-generated content. Generator: scripts/gen/cmake_builtin_docs.py. -->
