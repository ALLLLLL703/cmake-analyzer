CMake 4.4.4 · [Official documentation](https://cmake.org/cmake/help/v4.4/variable/CMAKE_Swift_COMPILATION_MODE.html)

# CMAKE_Swift_COMPILATION_MODE

Added in version 3.29.

Specify how Swift compiles a target. This variable is used to initialize the [`Swift_COMPILATION_MODE`](https://cmake.org/cmake/help/v4.4/prop_tgt/Swift_COMPILATION_MODE.html#prop_tgt:Swift_COMPILATION_MODE) property on targets as they are created.

The allowed values are:

**`incremental`**

Compiles each Swift source in the module separately, resulting in better parallelism in the build. The compiler emits additional information into the build directory improving rebuild performance when small changes are made to the source between rebuilds. This is the best option to use while iterating on changes in a project.

**`wholemodule`**

Whole-module optimizations are slowest to compile, but results in the most optimized library. The entire context is loaded into once instance of the compiler, so there is no parallelism across source files in the module.

**`singlefile`**

Compiles each source in a Swift modules separately, resulting in better parallelism. Unlike the `incremental` build mode, no additional information is emitted by the compiler during the build, so rebuilding after making small changes to the source file will not run faster. This option should be used sparingly, preferring `incremental` builds, unless working around a compiler bug.

Use [`generator expressions`](https://cmake.org/cmake/help/v4.4/manual/cmake-generator-expressions.7.html#manual:cmake-generator-expressions(7)) to support per-configuration specification. For example, the code:

~~~
set(CMAKE_Swift_COMPILATION_MODE
  "$<IF:$<CONFIG:Release>,wholemodule,incremental>")
~~~

sets the default Swift compilation mode to wholemodule mode when building a release configuration and to incremental mode in other configurations.

If this variable is not set then the [`Swift_COMPILATION_MODE`](https://cmake.org/cmake/help/v4.4/prop_tgt/Swift_COMPILATION_MODE.html#prop_tgt:Swift_COMPILATION_MODE) target property will not be set automatically. If that property is unset then CMake uses the default value `incremental` to build the Swift source files.

Note

This property only has effect when policy [`CMP0157`](https://cmake.org/cmake/help/v4.4/policy/CMP0157.html#policy:CMP0157) is set to `NEW` prior to the first [`project()`](https://cmake.org/cmake/help/v4.4/command/project.html#command:project) or [`enable_language()`](https://cmake.org/cmake/help/v4.4/command/enable_language.html#command:enable_language) command that enables the Swift language.

<!-- Script-generated content. Generator: scripts/gen/cmake_builtin_docs.py. -->
