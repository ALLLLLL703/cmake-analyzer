> AI-generated content.

# Builtin argument grammars

`builtin.rs` registers the 132 command names present in the CMake 4.4 command
catalogue, including legacy commands, with version additions for CMake 3.0–3.31
and 4.0–4.4. Scalar operands remain roles; LanguageType has an explicit supported-language catalogue.

The final version layer supplies structured keyword grammars:

- `builtin/all_transform.rs`: file, string, list, cmake_path, and math operations.
- `builtin/all_build.rs`: construction, custom commands, tests, install/export,
  try_compile/try_run, source/file sets, and legacy installation forms.
- `builtin/all_process.rs`: process execution, CTest, host queries, language/file
  API/pkg-config, instrumentation, diagnostics, and test discovery.
- `builtin/all_lookup/`: find commands, property scopes, loops, messages,
  conditions, cache setters, source groups, and cache loading.
- `builtin/project.rs` and `builtin/find_package.rs`: project metadata and
  package module/config signatures, including components and search options.
- Remaining simple command grammars retain their existing complete base/version
  layers; no effective registered schema has an Unmodeled/Action/Unknown tail.

`keyword_schemas.rs` caches only requested versions in OnceLock slots. Its final
layer replaces earlier signatures without introducing commands before their
registration version. Analysis and completion consume the same effective layers;
command completion borrows these trees. No runtime manual parsing or global
keyword fallback is used.

## Matching

Keyword/Optional/Repeat/Choice/AllOf express signatures and typed operands.
Grammar wraps a structured multi-token operand while retaining a single snippet
placeholder (for example, condition). CacheEntry recognizes CACHE{name} dispatch
in the CMake 4.2 set signature without declaring an ordinary variable write.

Full-call output/path/reference matching keeps its literal preference and
intersection policy. Keyword completion first ranks the exact completed prefix,
then probes canonical replacements without giving proposed words extra weight.
A weak opaque/scalar-repeat interpretation cannot steal a required enum/scalar
slot from a stronger completed branch. Only the current payload is replaced.

Search hints, runtime dependency directories, installed paths, and mixed system
header tokens are not universally required existing source inputs. Their generic
Path/Value roles avoid speculative missing-file diagnostics.

## Language names

LanguageType marks project LANGUAGES/legacy language lists, enable_language, and
existing compile/link language operands. It remains an open scalar for custom
language modules; it does not reject unrecognized language names. languages.rs
supplies C, CXX, CSharp, CUDA, OBJC, OBJCXX, Fortran, HIP, ISPC, Swift, ASM,
ASM_NASM, ASM_MARMASM, ASM_MASM, ASM_POASM, and ASM-ATT, filtered by introduction
version. Project language slots also suggest NONE. Completion uses plain-text
EnumMember items and retains argument delimiters; it never probes compilers.

## Evidence and limits

Grammars were checked against local CMake 4.4 command manuals and release notes;
older official versioned manuals establish experimental PACKAGE_INFO history.
Grouped grammar tests derive literal-node witnesses across all 37 supported versions
and check required slots, mode boundaries, output/target/path roles, and LSP version
changes. A registry-wide check rejects remaining opaque keyword tails.

These are structural schemas, not full CMake validators. Numeric/regex/JSON
validity, every option dependency or duplicate, platform/policy availability,
and runtime execution remain CMake's responsibility. Experimental syntax is
exposed from its known introduction without enabling or evaluating feature gates.
Host DISTRIB_<name> queries retain an open scalar alternative; only documented
literal query keys are suggested. Condition grammar covers linear operators and
predicates, not general recursive parenthesized expression evaluation. Dynamic
function/macro keyword vocabularies and arbitrary scalar values are not inferred.

No completion request configures a project, runs a script, inspects native help,
or fetches network documentation.
