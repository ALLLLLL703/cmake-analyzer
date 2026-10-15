CMake 4.4.4 · [Official documentation](https://cmake.org/cmake/help/v4.4/variable/CTEST_SUBMIT_PARTS.html)

# CTEST_SUBMIT_PARTS

Added in version 4.4.

Specify a [semicolon-separated list](https://cmake.org/cmake/help/v4.4/manual/cmake-language.7.html#cmake-language-lists) of parts for the [`ctest_submit()`](https://cmake.org/cmake/help/v4.4/command/ctest_submit.html#command:ctest_submit) command to submit in a [`ctest(1)`](https://cmake.org/cmake/help/v4.4/manual/ctest.1.html#manual:ctest(1)) [Dashboard Client](https://cmake.org/cmake/help/v4.4/manual/ctest.1.html#dashboard-client) script, or on the **ctest** command line via the [`-D`](https://cmake.org/cmake/help/v4.4/manual/ctest.1.html#cmdoption-ctest-dashboard-D) option.

Valid part names are the same as those accepted by the `PARTS` option of [`ctest_submit()`](https://cmake.org/cmake/help/v4.4/command/ctest_submit.html#command:ctest_submit): `Start`, `Update`, `Configure`, `Build`, `Test`, `Coverage`, `MemCheck`, `Submit`, `Notes`, `ExtraFiles`, `Upload`, and `Done`.

If the [`ctest_submit()`](https://cmake.org/cmake/help/v4.4/command/ctest_submit.html#command:ctest_submit) command is given an explicit `PARTS` argument, that argument takes precedence and this variable is ignored.

When this variable is empty or unset (and no explicit `PARTS` argument is given), all available parts are submitted (the default behavior).

Example --- submit only the Configure result in a single `ctest` invocation:

~~~
ctest -T Configure -T Submit -D CTEST_SUBMIT_PARTS=Configure
~~~

<!-- Script-generated content. Generator: scripts/gen/cmake_builtin_docs.py. -->
