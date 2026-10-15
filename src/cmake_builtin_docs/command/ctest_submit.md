CMake 4.4.4 · [Official documentation](https://cmake.org/cmake/help/v4.4/command/ctest_submit.html)

# ctest_submit

Perform the [CTest Submit Step](https://cmake.org/cmake/help/v4.4/manual/ctest.1.html#ctest-submit-step) as a [Dashboard Client](https://cmake.org/cmake/help/v4.4/manual/ctest.1.html#dashboard-client).

~~~
ctest_submit([PARTS <part>...] [FILES <file>...]
             [SUBMIT_URL <url>]
             [BUILD_ID <result-var>]
             [HTTPHEADER <header>]
             [RETRY_COUNT <count>]
             [RETRY_DELAY <delay>]
             [RETURN_VALUE <result-var>]
             [CAPTURE_CMAKE_ERROR <result-var>]
             [QUIET]
             )
~~~

Submit results to a dashboard server. By default all available parts are submitted.

The options are:

**`PARTS <part>...`**

Specify a subset of parts to submit. Takes precedence over the [`CTEST_SUBMIT_PARTS`](https://cmake.org/cmake/help/v4.4/variable/CTEST_SUBMIT_PARTS.html#variable:CTEST_SUBMIT_PARTS) variable if both are set. Valid part names are:

- `Start` - nothing.

- `Update` - [`ctest_update()`](https://cmake.org/cmake/help/v4.4/command/ctest_update.html#command:ctest_update) results, in `Update.xml`.

- `Configure` - [`ctest_configure()`](https://cmake.org/cmake/help/v4.4/command/ctest_configure.html#command:ctest_configure) results, in `Configure.xml`.

- `Build` - [`ctest_build()`](https://cmake.org/cmake/help/v4.4/command/ctest_build.html#command:ctest_build) results, in `Build.xml`.

- `Test` - [`ctest_test()`](https://cmake.org/cmake/help/v4.4/command/ctest_test.html#command:ctest_test) results, in `Test.xml`.

- `Coverage` - [`ctest_coverage()`](https://cmake.org/cmake/help/v4.4/command/ctest_coverage.html#command:ctest_coverage) results, in `Coverage.xml`.

- `MemCheck` - [`ctest_memcheck()`](https://cmake.org/cmake/help/v4.4/command/ctest_memcheck.html#command:ctest_memcheck) results, in `DynamicAnalysis.xml` and `DynamicAnalysis-Test.xml`.

- `Notes` - Files listed by [`CTEST_NOTES_FILES`](https://cmake.org/cmake/help/v4.4/variable/CTEST_NOTES_FILES.html#variable:CTEST_NOTES_FILES), in `Notes.xml`.

- `ExtraFiles` - Files listed by [`CTEST_EXTRA_SUBMIT_FILES`](https://cmake.org/cmake/help/v4.4/variable/CTEST_EXTRA_SUBMIT_FILES.html#variable:CTEST_EXTRA_SUBMIT_FILES).

- `Upload` - Files prepared for upload by [`ctest_upload()`](https://cmake.org/cmake/help/v4.4/command/ctest_upload.html#command:ctest_upload), in `Upload.xml`.

- `Submit` - nothing.

- `Done` - Build is complete, in `Done.xml`.

**`FILES <file>...`**

Specify an explicit list of specific files to be submitted. Each individual file must exist at the time of the call.

**`SUBMIT_URL <url>`**

Added in version 3.14.

The `http` or `https` URL of the dashboard server to send the submission to. If not given, the [`CTEST_SUBMIT_URL`](https://cmake.org/cmake/help/v4.4/variable/CTEST_SUBMIT_URL.html#variable:CTEST_SUBMIT_URL) variable is used.

**`BUILD_ID <result-var>`**

Added in version 3.15.

Store in the `<result-var>` variable the ID assigned to this build by CDash.

**`HTTPHEADER <HTTP-header>`**

Added in version 3.9.

Specify HTTP header to be included in the request to CDash during submission. For example, CDash can be configured to only accept submissions from authenticated clients. In this case, you should provide a bearer token in your header:

~~~
ctest_submit(HTTPHEADER "Authorization: Bearer <auth-token>")
~~~

This suboption can be repeated several times for multiple headers.

**`RETRY_COUNT <count>`**

Specify how many times to retry a timed-out submission.

**`RETRY_DELAY <delay>`**

Specify how long (in seconds) to wait after a timed-out submission before attempting to re-submit.

**`RETURN_VALUE <result-var>`**

Store in the `<result-var>` variable `0` for success and non-zero on failure.

**`CAPTURE_CMAKE_ERROR <result-var>`**

Added in version 3.13.

Store in the `<result-var>` variable `-1` if there are any errors running the command and prevent [`ctest(1)`](https://cmake.org/cmake/help/v4.4/manual/ctest.1.html#manual:ctest(1)) from returning non-zero if an error occurs.

**`QUIET`**

Added in version 3.3.

Suppress all non-error messages that would have otherwise been printed to the console.

## Submit to CDash Upload API

Added in version 3.2.

~~~
ctest_submit(CDASH_UPLOAD <file> [CDASH_UPLOAD_TYPE <type>]
             [SUBMIT_URL <url>]
             [BUILD_ID <result-var>]
             [HTTPHEADER <header>]
             [RETRY_COUNT <count>]
             [RETRY_DELAY <delay>]
             [RETURN_VALUE <result-var>]
             [QUIET])
~~~

This second signature is used to upload files to CDash via the CDash file upload API. The API first sends a request to upload to CDash along with a content hash of the file. If CDash does not already have the file, then it is uploaded. Along with the file, a CDash type string is specified to tell CDash which handler to use to process the data.

This signature interprets options in the same way as the first one.

Added in version 3.8: Added the `RETRY_COUNT`, `RETRY_DELAY`, `QUIET` options.

Added in version 3.9: Added the `HTTPHEADER` option.

Added in version 3.13: Added the `RETURN_VALUE` option.

Added in version 3.14: Added the `SUBMIT_URL` option.

Added in version 3.15: Added the `BUILD_ID` option.

<!-- Script-generated content. Generator: scripts/gen/cmake_builtin_docs.py. -->
