CMake 4.4.4 · [Official documentation](https://cmake.org/cmake/help/v4.4/command/enable_language.html)

# enable_language

Enable languages (CXX/C/OBJC/OBJCXX/Fortran/etc)

~~~
enable_language(<lang>... [OPTIONAL])
~~~

Enables support for the named languages in CMake. This is the same as the [`project()`](https://cmake.org/cmake/help/v4.4/command/project.html#command:project) command but does not create any of the extra variables that are created by the [`project()`](https://cmake.org/cmake/help/v4.4/command/project.html#command:project) command.

The supported languages are:

`C`

**`CXX`**

C++

**`CSharp`**

Added in version 3.8.

C#

**`CUDA`**

Added in version 3.8.

**`OBJC`**

Added in version 3.16.

Objective-C

**`OBJCXX`**

Added in version 3.16.

Objective-C++

`Fortran`

**`HIP`**

Added in version 3.21.

**`ISPC`**

Added in version 3.18.

**`Swift`**

Added in version 3.15.

**`ASM`**

Assembly language supported by the C compiler.

If enabling `ASM`, list it last so that CMake can check whether the `C` or `CXX` compiler supports assembly.

**`ASM_NASM`**

Netwide Assembler

**`ASM_MARMASM`**

Added in version 3.26.

Microsoft Assembler (ARM, ARM64)

**`ASM_MASM`**

Microsoft Assembler (x86, x64)

**`ASM_POASM`**

Added in version 4.4.

Pelles C toolchain assembler.

`ASM-ATT`

The following restrictions apply to where `enable_language()` may be called:

- It must be called in file scope, not in a [`function()`](https://cmake.org/cmake/help/v4.4/command/function.html#command:function) call nor inside a [`block()`](https://cmake.org/cmake/help/v4.4/command/block.html#command:block).

- It must not be called before the first call to [`project()`](https://cmake.org/cmake/help/v4.4/command/project.html#command:project). See policy [`CMP0165`](https://cmake.org/cmake/help/v4.4/policy/CMP0165.html#policy:CMP0165).

- It must be called in the highest directory common to all targets using the named language directly for compiling sources or indirectly through link dependencies. It is simplest to enable all needed languages in the top-level directory of a project.

The `OPTIONAL` keyword is a placeholder for future implementation and does not currently work. Instead you can use the [`CheckLanguage`](https://cmake.org/cmake/help/v4.4/module/CheckLanguage.html#module:CheckLanguage) module to verify support before enabling.

<!-- Script-generated content. Generator: scripts/gen/cmake_builtin_docs.py. -->
