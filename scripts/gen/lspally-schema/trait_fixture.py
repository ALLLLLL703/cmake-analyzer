#!/usr/bin/env python3
# AI-authored generator.
"""Write a documented, all-unsupported ILanguageServer seed outside target/.

Usage: python scripts/gen/lspally-schema/trait_fixture.py /tmp/unsupported-server.rs
Read the local trait inventory and overwrite the requested seed file. Every body,
including initialization, returns MethodNotFound. This is not a regeneration of
crates/lspally/tests/integration/fixture.rs: that fixture has a manually added
event field, successful/callback hooks, assertions and behavior-specific Rustdoc.
Keep its script-origin and separate manual test/documentation labels, module/type/
field docs and all 73 hook contracts when maintaining it. Never copy its success
or event/callback documentation onto this seed's still-unsupported bodies, or
replace the adapted fixture with this seed. Inspect generation only in scratch.
"""
import argparse
from pathlib import Path
import re

parser = argparse.ArgumentParser(description=__doc__)
parser.add_argument("output", type=Path)
args = parser.parse_args()
if "target" in args.output.resolve().parts:
    parser.error("Fixture sources must stay outside target/")
root = Path(__file__).resolve().parents[3]
source = (root / "crates/lspally/src/server/model.rs").read_text()
trait = source.split("pub trait ILanguageServer", 1)[1].split("#[derive(Debug)]", 1)[0]
methods = re.findall(
    r"fn (\w+)\(\s*&self,\s*context: Context,\s*params: (.*?)\s*\)"
    r" -> impl Future<Output = ServerResult<(.*?)>> \+ Send;",
    trait,
    re.S,
)

def unsupported_docs(name, params, result):
    """Describe one unchanged unsupported seed body, not an adapted fixture hook.

    Args:
        name: Exact trait method spelling for the link and local call example.
        params: Rust input type; the generated body discards this entire value.
        result: Rust success type; the generated body never produces this value.
    Returns:
        Rustdoc lines for the declaration; no mutation or file I/O occurs here.
    Example:
        unsupported_docs("shutdown", "()", "()") documents Err, never Ok(()).
    """
    lines = [
        f"Reject [`ILanguageServer::{name}`] with MethodNotFound; no side effects.",
        "This seed ignores the payload and context. It makes no client callbacks,",
        "performs no I/O and changes no state; even lifecycle hooks are unsupported.",
        "", "# Arguments",
        "- `self` `[in]`: stateless seed server; no fields are read or written.",
        "- `context` `[in]`: consumed call context, ignored without client or cancellation access.",
        f"- `params` `[in]`: consumed `{params}` payload; every field is ignored.",
        "# Returns",
        "Always `Err(ServerError::unsupported())` with code -32601.",
        f"No successful `{result}` value is produced; no output parameter is overwritten or appended.",
        "# Errors",
        "Every call returns MethodNotFound, regardless of input or lifecycle state.",
        "# Examples",
        f"Seed-module context with a prepared `Context` and `{params}` payload:",
        "```text",
        "let server = Server;",
        f"let result = server.{name}(context, params).await;",
        "assert!(matches!(result, Err(error) if error.code == -32601));",
        "```",
    ]
    return "\n".join("    ///" + (" " + line if line else "") for line in lines)


output = [
    "// Script-generated tests by scripts/gen/lspally-schema/trait_fixture.py.",
    "//! Provide a stateless, all-unsupported language-server seed for later adaptation.",
    "//! Every hook, including initialization, returns MethodNotFound without I/O.",
    "//! This is not the hand-adapted duplex integration fixture: do not overwrite its",
    "//! event log, successful/callback bodies, assertions or manual documentation.",
    "use lspally::server::model::*;",
    "/// Stateless placeholder for smoke-check authors implementing [`ILanguageServer`].",
    "/// Construction has no side effects. [`Context`] supplies each hook's call services,",
    "/// but this seed ignores them. A [`Wrapper`] can own it; initialization will fail",
    "/// until the author replaces the corresponding unsupported body and its docs.",
    "///",
    "/// # Examples",
    "/// Seed-module context:",
    "/// ```text",
    "/// let server = Server;",
    "/// ```",
    "pub struct Server;",
    "impl ILanguageServer for Server {",
]
for name, params, result in methods:
    params = params.strip().rstrip(",")
    output.append(unsupported_docs(name, params, result))
    output.append(
        f"async fn {name}(&self, context: Context, params: {params})"
        f" -> ServerResult<{result}> {{"
        " let _ = (context, params); Err(ServerError::unsupported()) }"
    )
output.append("}")
args.output.write_text("\n".join(output) + "\n")
print(f"Generated {len(methods)} explicit methods in {args.output}")
