CMake 4.4.4 · [Official documentation](https://cmake.org/cmake/help/v4.4/command/add_test.html)

# add_test

Add a test to the project to be run by [`ctest(1)`](https://cmake.org/cmake/help/v4.4/manual/ctest.1.html#manual:ctest(1)).

~~~
add_test(NAME <name> COMMAND <command> [<arg>...]
         [CONFIGURATIONS <config>...]
         [WORKING_DIRECTORY <dir>]
         [COMMAND_EXPAND_LISTS]
         [BUILD_DEPENDS <dependencies>...])
~~~

Adds a test called `<name>`. The test name may contain arbitrary characters, expressed as a [Quoted Argument](https://cmake.org/cmake/help/v4.4/manual/cmake-language.7.html#quoted-argument) or [Bracket Argument](https://cmake.org/cmake/help/v4.4/manual/cmake-language.7.html#bracket-argument) if necessary. See policy [`CMP0110`](https://cmake.org/cmake/help/v4.4/policy/CMP0110.html#policy:CMP0110).

CMake only generates tests if the [`enable_testing()`](https://cmake.org/cmake/help/v4.4/command/enable_testing.html#command:enable_testing) command has been invoked. The [`CTest`](https://cmake.org/cmake/help/v4.4/module/CTest.html#module:CTest) module invokes `enable_testing` automatically unless [`BUILD_TESTING`](https://cmake.org/cmake/help/v4.4/variable/BUILD_TESTING.html#variable:BUILD_TESTING) is set to `OFF`.

Tests added with the `add_test(NAME)` signature support using [`generator expressions`](https://cmake.org/cmake/help/v4.4/manual/cmake-generator-expressions.7.html#manual:cmake-generator-expressions(7)) in test properties set by [`set_property(TEST)`](https://cmake.org/cmake/help/v4.4/command/set_property.html#command:set_property) or [`set_tests_properties()`](https://cmake.org/cmake/help/v4.4/command/set_tests_properties.html#command:set_tests_properties). Test properties may only be set in the directory the test is created in.

`add_test` options are:

**`COMMAND`**

Specify the test command-line.

If `<command>` specifies an executable target created by [`add_executable()`](https://cmake.org/cmake/help/v4.4/command/add_executable.html#command:add_executable):

- It will automatically be replaced by the location of the executable created at build time.

- Added in version 3.3: The target's [`CROSSCOMPILING_EMULATOR`](https://cmake.org/cmake/help/v4.4/prop_tgt/CROSSCOMPILING_EMULATOR.html#prop_tgt:CROSSCOMPILING_EMULATOR), if set, will be used to run the command on the host:

   

  ~~~
  <emulator> <command>
  ~~~

     

  Changed in version 3.29: The emulator is used only when [`cross-compiling`](https://cmake.org/cmake/help/v4.4/variable/CMAKE_CROSSCOMPILING.html#variable:CMAKE_CROSSCOMPILING). See policy [`CMP0158`](https://cmake.org/cmake/help/v4.4/policy/CMP0158.html#policy:CMP0158).

- Added in version 3.29: The target's [`TEST_LAUNCHER`](https://cmake.org/cmake/help/v4.4/prop_tgt/TEST_LAUNCHER.html#prop_tgt:TEST_LAUNCHER), if set, will be used to launch the command:

   

  ~~~
  <launcher> <command>
  ~~~

    

  If the [`CROSSCOMPILING_EMULATOR`](https://cmake.org/cmake/help/v4.4/prop_tgt/CROSSCOMPILING_EMULATOR.html#prop_tgt:CROSSCOMPILING_EMULATOR) is also set, both are used:

   

  ~~~
  <launcher> <emulator> <command>
  ~~~

- Added in version 4.4: When the [`CMAKE_TEST_BUILD_DEPENDS`](https://cmake.org/cmake/help/v4.4/variable/CMAKE_TEST_BUILD_DEPENDS.html#variable:CMAKE_TEST_BUILD_DEPENDS) variable is enabled, the [Ninja Generators](https://cmake.org/cmake/help/v4.4/manual/cmake-generators.7.html#ninja-generators) generate a convenience build target named `test_prep/<name>` that depends on the test executable target. Building this target ensures the executable is up-to-date before the test runs.

   

  Additionally, targets referenced by the test command via generator expressions are added as dependencies of the `test_prep/<name>` target.

   

  If multiple tests in different directories share the same name, their dependencies are merged into a single `test_prep/<name>` target.

   

  Tests with names that are not valid target names are excluded from this behavior.

   

  The `BUILD_DEPENDS` keyword can be used to add explicit build dependencies.

The command may be specified using [`generator expressions`](https://cmake.org/cmake/help/v4.4/manual/cmake-generator-expressions.7.html#manual:cmake-generator-expressions(7)).

**`CONFIGURATIONS`**

Restrict execution of the test only to the named configurations.

**`WORKING_DIRECTORY`**

Set the test property [`WORKING_DIRECTORY`](https://cmake.org/cmake/help/v4.4/prop_test/WORKING_DIRECTORY.html#prop_test:WORKING_DIRECTORY) in which to execute the test. If not specified, the test will be run in [`CMAKE_CURRENT_BINARY_DIR`](https://cmake.org/cmake/help/v4.4/variable/CMAKE_CURRENT_BINARY_DIR.html#variable:CMAKE_CURRENT_BINARY_DIR). The working directory may be specified using [`generator expressions`](https://cmake.org/cmake/help/v4.4/manual/cmake-generator-expressions.7.html#manual:cmake-generator-expressions(7)).

**`BUILD_DEPENDS`**

Added in version 4.4.

Specify a list of targets or files that must be built before the test can run. Each dependency is added to the `test_prep/<name>` build target described above when [`CMAKE_TEST_BUILD_DEPENDS`](https://cmake.org/cmake/help/v4.4/variable/CMAKE_TEST_BUILD_DEPENDS.html#variable:CMAKE_TEST_BUILD_DEPENDS) is enabled with the [Ninja Generators](https://cmake.org/cmake/help/v4.4/manual/cmake-generators.7.html#ninja-generators). The test name must be a valid target name in order to list build dependencies with this keyword.

**`COMMAND_EXPAND_LISTS`**

Added in version 3.16.

Lists in `COMMAND` arguments will be expanded, including those created with [`generator expressions`](https://cmake.org/cmake/help/v4.4/manual/cmake-generator-expressions.7.html#manual:cmake-generator-expressions(7)).

If the test command exits with code `0` the test passes. Non-zero exit code is a "failed" test. The test property [`WILL_FAIL`](https://cmake.org/cmake/help/v4.4/prop_test/WILL_FAIL.html#prop_test:WILL_FAIL) inverts this logic. Note that system-level test failures such as segmentation faults or heap errors will still fail the test even if `WILL_FAIL` is true. Output written to stdout or stderr is captured by [`ctest(1)`](https://cmake.org/cmake/help/v4.4/manual/ctest.1.html#manual:ctest(1)) and only affects the pass/fail status via the [`PASS_REGULAR_EXPRESSION`](https://cmake.org/cmake/help/v4.4/prop_test/PASS_REGULAR_EXPRESSION.html#prop_test:PASS_REGULAR_EXPRESSION), [`FAIL_REGULAR_EXPRESSION`](https://cmake.org/cmake/help/v4.4/prop_test/FAIL_REGULAR_EXPRESSION.html#prop_test:FAIL_REGULAR_EXPRESSION), or [`SKIP_REGULAR_EXPRESSION`](https://cmake.org/cmake/help/v4.4/prop_test/SKIP_REGULAR_EXPRESSION.html#prop_test:SKIP_REGULAR_EXPRESSION) test properties.

Added in version 3.16: Added [`SKIP_REGULAR_EXPRESSION`](https://cmake.org/cmake/help/v4.4/prop_test/SKIP_REGULAR_EXPRESSION.html#prop_test:SKIP_REGULAR_EXPRESSION) property.

Example usage:

~~~
add_test(NAME mytest
         COMMAND testDriver --config $<CONFIG>
                            --exe $<TARGET_FILE:myexe>)
~~~

This creates a test `mytest` whose command runs a `testDriver` tool passing the configuration name and the full path to the executable file produced by target `myexe`.

 

The command syntax above is recommended over the older, less flexible form:

~~~
add_test(<name> <command> [<arg>...])
~~~

Add a test called `<name>` with the given command-line.

Unlike the above `NAME` signature, target names are not supported in the command-line. Furthermore, tests added with this signature do not support [`generator expressions`](https://cmake.org/cmake/help/v4.4/manual/cmake-generator-expressions.7.html#manual:cmake-generator-expressions(7)) in the command-line or test properties, and the [`TEST_LAUNCHER`](https://cmake.org/cmake/help/v4.4/prop_tgt/TEST_LAUNCHER.html#prop_tgt:TEST_LAUNCHER) and [`CROSSCOMPILING_EMULATOR`](https://cmake.org/cmake/help/v4.4/prop_tgt/CROSSCOMPILING_EMULATOR.html#prop_tgt:CROSSCOMPILING_EMULATOR) target properties are not supported.

<!-- Script-generated content. Generator: scripts/gen/cmake_builtin_docs.py. -->
