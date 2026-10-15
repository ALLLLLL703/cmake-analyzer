CMake 4.4.4 · [Official documentation](https://cmake.org/cmake/help/v4.4/variable/CTEST_MEMORYCHECK_TYPE.html)

# CTEST_MEMORYCHECK_TYPE

Added in version 3.1.

Specify the CTest `MemoryCheckType` setting in a [`ctest(1)`](https://cmake.org/cmake/help/v4.4/manual/ctest.1.html#manual:ctest(1)) [Dashboard Client](https://cmake.org/cmake/help/v4.4/manual/ctest.1.html#dashboard-client) script, or on the **ctest** command line via the [`-D`](https://cmake.org/cmake/help/v4.4/manual/ctest.1.html#cmdoption-ctest-dashboard-D) option. Valid values are `Valgrind`, `Purify`, `BoundsChecker`, `DrMemory`, `CudaSanitizer`, `ThreadSanitizer`, `AddressSanitizer`, `LeakSanitizer`, `MemorySanitizer` and `UndefinedBehaviorSanitizer`.

<!-- Script-generated content. Generator: scripts/gen/cmake_builtin_docs.py. -->
