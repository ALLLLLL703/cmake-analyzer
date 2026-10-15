> AI-generated content.

# Embedded CMake help

This directory contains all 132 command and 810 variable pages from the locally
installed CMake 4.4.4 HTML documentation. Markdown is script-generated from the
rendered page bodies, including expanded upstream include files and substitutions.
Each page records its version, official URL, and generator. This version is fixed
independently of the analyzed project's selected CMake version.

Regenerate from an installed documentation tree:

```sh
python3 scripts/gen/cmake_builtin_docs.py --html-root /usr/share/doc/cmake/html
```

The tree must have a sibling LICENSE.rst. Python's standard library performs the
conversion; rustfmt formats the generated index. No network or CMake project
execution is needed. `src/server/hovers/include.rs` embeds every page using
include_str!, so hover does not read these files at runtime. Exact variable names
precede template matches; function argument variables reuse the function page.

Documentation is copyright Kitware, Inc. and Contributors, not AI-authored.
See LICENSE.txt for the upstream BSD terms. Binary redistributors must retain
that notice and disclaimer with their distribution.
