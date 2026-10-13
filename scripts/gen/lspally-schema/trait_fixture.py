#!/usr/bin/env python3
# AI-authored generator.
"""Generate an explicit unsupported server fixture outside target/. Adapt it for smoke checks."""
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
output = [
    "// Script-generated tests by scripts/gen/lspally-schema/trait_fixture.py.",
    "use lspally::server::model::*;",
    "pub struct Server;",
    "impl ILanguageServer for Server {",
]
for name, params, result in methods:
    params = params.strip().rstrip(",")
    output.append(
        f"async fn {name}(&self, context: Context, params: {params})"
        f" -> ServerResult<{result}> {{"
        " let _ = (context, params); Err(ServerError::unsupported()) }"
    )
output.append("}")
args.output.write_text("\n".join(output) + "\n")
print(f"Generated {len(methods)} explicit methods in {args.output}")
