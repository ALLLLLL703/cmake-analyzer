CMake 4.4.4 · [Official documentation](https://cmake.org/cmake/help/v4.4/variable/CMAKE_POLICY_WARNING_CMPNNNN.html)

# CMAKE_POLICY_WARNING_CMP\<NNNN\>

Explicitly enable or disable the warning when CMake Policy `CMP<NNNN>` has not been set explicitly by [`cmake_policy()`](https://cmake.org/cmake/help/v4.4/command/cmake_policy.html#command:cmake_policy) or implicitly by [`cmake_minimum_required()`](https://cmake.org/cmake/help/v4.4/command/cmake_minimum_required.html#command:cmake_minimum_required). This is meaningful only for the policies that do not warn by default:

- `CMAKE_POLICY_WARNING_CMP0025` controlled the warning for policy [`CMP0025`](https://cmake.org/cmake/help/v4.4/policy/CMP0025.html#policy:CMP0025) in CMake versions before 4.0.

- `CMAKE_POLICY_WARNING_CMP0047` controlled the warning for policy [`CMP0047`](https://cmake.org/cmake/help/v4.4/policy/CMP0047.html#policy:CMP0047) in CMake versions before 4.0.

- `CMAKE_POLICY_WARNING_CMP0056` controlled the warning for policy [`CMP0056`](https://cmake.org/cmake/help/v4.4/policy/CMP0056.html#policy:CMP0056) in CMake versions before 4.0.

- `CMAKE_POLICY_WARNING_CMP0060` controlled the warning for policy [`CMP0060`](https://cmake.org/cmake/help/v4.4/policy/CMP0060.html#policy:CMP0060) in CMake versions before 4.0.

- `CMAKE_POLICY_WARNING_CMP0065` controlled the warning for policy [`CMP0065`](https://cmake.org/cmake/help/v4.4/policy/CMP0065.html#policy:CMP0065) in CMake versions before 4.0.

- `CMAKE_POLICY_WARNING_CMP0066` controls the warning for policy [`CMP0066`](https://cmake.org/cmake/help/v4.4/policy/CMP0066.html#policy:CMP0066).

- `CMAKE_POLICY_WARNING_CMP0067` controls the warning for policy [`CMP0067`](https://cmake.org/cmake/help/v4.4/policy/CMP0067.html#policy:CMP0067).

- `CMAKE_POLICY_WARNING_CMP0082` controls the warning for policy [`CMP0082`](https://cmake.org/cmake/help/v4.4/policy/CMP0082.html#policy:CMP0082).

- `CMAKE_POLICY_WARNING_CMP0089` controls the warning for policy [`CMP0089`](https://cmake.org/cmake/help/v4.4/policy/CMP0089.html#policy:CMP0089).

- `CMAKE_POLICY_WARNING_CMP0102` controls the warning for policy [`CMP0102`](https://cmake.org/cmake/help/v4.4/policy/CMP0102.html#policy:CMP0102).

- `CMAKE_POLICY_WARNING_CMP0112` controls the warning for policy [`CMP0112`](https://cmake.org/cmake/help/v4.4/policy/CMP0112.html#policy:CMP0112).

- `CMAKE_POLICY_WARNING_CMP0116` controls the warning for policy [`CMP0116`](https://cmake.org/cmake/help/v4.4/policy/CMP0116.html#policy:CMP0116).

- `CMAKE_POLICY_WARNING_CMP0126` controls the warning for policy [`CMP0126`](https://cmake.org/cmake/help/v4.4/policy/CMP0126.html#policy:CMP0126).

- `CMAKE_POLICY_WARNING_CMP0128` controls the warning for policy [`CMP0128`](https://cmake.org/cmake/help/v4.4/policy/CMP0128.html#policy:CMP0128).

- `CMAKE_POLICY_WARNING_CMP0129` controls the warning for policy [`CMP0129`](https://cmake.org/cmake/help/v4.4/policy/CMP0129.html#policy:CMP0129).

- `CMAKE_POLICY_WARNING_CMP0133` controls the warning for policy [`CMP0133`](https://cmake.org/cmake/help/v4.4/policy/CMP0133.html#policy:CMP0133).

- `CMAKE_POLICY_WARNING_CMP0172` controls the warning for policy [`CMP0172`](https://cmake.org/cmake/help/v4.4/policy/CMP0172.html#policy:CMP0172).

- `CMAKE_POLICY_WARNING_CMP0206` controls the warning for policy [`CMP0206`](https://cmake.org/cmake/help/v4.4/policy/CMP0206.html#policy:CMP0206).

This variable should not be set by a project in CMake code. Project developers running CMake may set this variable in their cache to enable the warning (e.g. `-DCMAKE_POLICY_WARNING_CMP<NNNN>=ON`). Alternatively, running [`cmake(1)`](https://cmake.org/cmake/help/v4.4/manual/cmake.1.html#manual:cmake(1)) with the [`--debug-output`](https://cmake.org/cmake/help/v4.4/manual/cmake.1.html#cmdoption-cmake-debug-output), [`--trace`](https://cmake.org/cmake/help/v4.4/manual/cmake.1.html#cmdoption-cmake-trace), or [`--trace-expand`](https://cmake.org/cmake/help/v4.4/manual/cmake.1.html#cmdoption-cmake-trace-expand) option will also enable the warning.

<!-- Script-generated content. Generator: scripts/gen/cmake_builtin_docs.py. -->
