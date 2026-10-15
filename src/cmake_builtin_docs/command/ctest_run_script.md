CMake 4.4.4 · [Official documentation](https://cmake.org/cmake/help/v4.4/command/ctest_run_script.html)

# ctest_run_script

Run a [`ctest -S`](https://cmake.org/cmake/help/v4.4/manual/ctest.1.html#cmdoption-ctest-S) script.

~~~
ctest_run_script([NEW_PROCESS]
                 <script-file>...
                 [RETURN_VALUE <result-var>]
                 )
~~~

Runs a script or scripts much like if it was run from [`ctest -S`](https://cmake.org/cmake/help/v4.4/manual/ctest.1.html#cmdoption-ctest-S). The options are:

**`NEW_PROCESS`**

Run each script in a separate process.

**`RETURN_VALUE <result-var>`**

Store in the `<result-var>` variable `0` for success and non-zero on failure.

<!-- Script-generated content. Generator: scripts/gen/cmake_builtin_docs.py. -->
