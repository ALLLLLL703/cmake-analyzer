> AI-generated content.

# Completion source catalogues

Workspace provides protocol-independent catalogues; the server matches schema roles
before requesting them. No command-name/variable-only query enumerates package trees.
Directory reads are async, local hint replay uses a joined blocking worker, and
cancellation is checked between commands, directory entries, and source visits.

- `model.rs`: requests, shared name lists, bounded cache state, and target-source visits.
- `context.rs`: capture owner directory/scope state and restore known local search
  assignments preceding the active argument. Unknown dynamic values remain unknown.
- `discovery.rs`: module/package filename discovery in configured and known roots.
- `targets.rs`: read literal conditional target imports beneath full-indexed package
  visits, without adding execution claims to the ordinary dependency/reference graph.

ModulePath maps to the Module completion role. Bare module names omit .cmake and
use the resolver's module paths, builtin directory, and known CMAKE_MODULE_PATH.
PackageName maps to the Package role. Find<Name>.cmake supplies package names;
<Name>Config.cmake and <name>-config.cmake supply config-package names. ConfigVersion
files do not become packages. Prefix discovery checks fixed conventional directory
levels, including lib/share/cmake and multiarch layouts, not arbitrary recursive
source trees. Known package DIR/ROOT values and CMAKE_PREFIX_PATH can supply roots.
These are available source names, not proof of version/component/compiler support.

Name lists are shared through Arc slices. One cache per Workspace retains the current
root lists and module/package catalogues, replacing old roots instead of growing a
history. Reload, path setters, explicit disk invalidation, and new/closed overlays
invalidate discovery; changed known search roots select new lists. There is no new
watcher: newly installed files otherwise require explicit configuration reload.
modules.enable=false suppresses both module and package catalogues.

## Imported library targets

The existing target_link_libraries Target operand schema already completes library,
object/interface, alias, and other supported target names. The same typed candidate
collector now also sees package export files reached through literal conditional
includes, a common Config.cmake -> Targets.cmake pattern. Nested includes and cycles
retain caller directory context and imported-target visibility.

This supplements completion only. It neither evaluates conditions nor upgrades a
possible declaration into a resolved reference, diagnostic fact, or executed target.
Variable names/values from these supplemental files are not exported. Unknown dynamic
include paths, computed target names, and raw linker flags/libraries without target
declarations are not inferred. Function/macro-body includes are not traversed.

Only full-indexed package visits seed this walk; external include/full policies apply
to children. functions/none do not acquire library targets through completion. Cached
immutable snapshots are reused, with existing Workspace file/depth/invocation budgets
bounding additional visits. The latest bounded supplemental source set survives ordinary
flushes to avoid reparsing exports on every keystroke; replacing that set or removing
full-indexed package visits releases unused data. No configure, project command, download, or native help
process is executed.
