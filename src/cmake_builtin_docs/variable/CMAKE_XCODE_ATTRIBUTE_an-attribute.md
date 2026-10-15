CMake 4.4.4 · [Official documentation](https://cmake.org/cmake/help/v4.4/variable/CMAKE_XCODE_ATTRIBUTE_an-attribute.html)

# CMAKE_XCODE_ATTRIBUTE_\<an-attribute\>

Added in version 3.1.

Set Xcode target attributes directly.

Tell the [`Xcode`](https://cmake.org/cmake/help/v4.4/generator/Xcode.html#generator:Xcode) generator to set `<an-attribute>` to a given value in the generated Xcode project. Ignored on other generators.

This offers low-level control over the generated Xcode project file. It is meant as a last resort for specifying settings that CMake does not otherwise have a way to control. Although this can override a setting CMake normally produces on its own, doing so bypasses CMake's model of the project and can break things.

See the [`XCODE_ATTRIBUTE_<an-attribute>`](https://cmake.org/cmake/help/v4.4/prop_tgt/XCODE_ATTRIBUTE_an-attribute.html#prop_tgt:XCODE_ATTRIBUTE_%3Can-attribute%3E) target property to set attributes on a specific target.

Contents of `CMAKE_XCODE_ATTRIBUTE_<an-attribute>` may use "generator expressions" with the syntax `$<...>`. See the [`cmake-generator-expressions(7)`](https://cmake.org/cmake/help/v4.4/manual/cmake-generator-expressions.7.html#manual:cmake-generator-expressions(7)) manual for available expressions. See the [`cmake-buildsystem(7)`](https://cmake.org/cmake/help/v4.4/manual/cmake-buildsystem.7.html#manual:cmake-buildsystem(7)) manual for more on defining buildsystem properties.

<!-- Script-generated content. Generator: scripts/gen/cmake_builtin_docs.py. -->
