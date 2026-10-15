#!/usr/bin/env python3
# AI-authored generator. Documentation text is Kitware's, not AI-authored.
"""Vendor installed CMake HTML help as Markdown and a compile-time Rust index."""
import argparse
from dataclasses import dataclass, field
from html.parser import HTMLParser
from pathlib import Path
import re
import subprocess
from urllib.parse import urljoin


@dataclass
class Node:
    tag: str
    attrs: dict = field(default_factory=dict)
    children: list = field(default_factory=list)


class Body(HTMLParser):
    def __init__(self):
        super().__init__(convert_charrefs=True)
        self.root = None
        self.stack = []

    def handle_starttag(self, tag, attrs):
        attrs = dict(attrs)
        if not self.stack:
            if self.root is not None or tag != "div" or attrs.get("role") != "main":
                return
            self.root = Node(tag, attrs)
            self.stack.append(self.root)
            return
        node = Node(tag, attrs)
        self.stack[-1].children.append(node)
        if tag not in {"br", "hr", "img", "input", "meta", "link", "wbr"}:
            self.stack.append(node)

    def handle_endtag(self, tag):
        if self.stack and self.stack[-1].tag == tag:
            self.stack.pop()

    def handle_data(self, data):
        if self.stack:
            self.stack[-1].children.append(data)


def plain(node):
    if isinstance(node, str):
        return node
    if "headerlink" in node.attrs.get("class", ""):
        return ""
    return "".join(plain(child) for child in node.children)


def render(node, base):
    if isinstance(node, str):
        return re.sub(r"\s+", " ", node).replace("<", "\\<").replace(">", "\\>")
    tag = node.tag
    if tag in {"script", "style"} or "headerlink" in node.attrs.get("class", ""):
        return ""
    if tag == "pre":
        text = plain(node).strip("\n")
        fence = "~" * max(3, max((len(m[0]) + 1 for m in re.finditer(r"~+", text)), default=0))
        return f"\n\n{fence}\n{text}\n{fence}\n\n"
    text = "".join(render(child, base) for child in node.children)
    if tag in {"h1", "h2", "h3", "h4", "h5", "h6"}:
        return f"\n\n{'#' * int(tag[1])} {text.strip()}\n\n"
    if tag == "code":
        text = plain(node)
        fence = "`" * max(1, max((len(m[0]) + 1 for m in re.finditer(r"`+", text)), default=0))
        return f"{fence}{text}{fence}"
    if tag == "a" and node.attrs.get("href"):
        href = urljoin(base, node.attrs["href"]).replace(" ", "%20").replace("<", "%3C").replace(">", "%3E")
        return f"[{text}]({href})" if text else ""
    if tag == "img":
        return f"![{node.attrs.get('alt', '')}]({urljoin(base, node.attrs.get('src', ''))})"
    if tag in {"strong", "b"}:
        return f"**{text}**"
    if tag in {"em", "i"}:
        return f"*{text}*"
    if tag == "li":
        return "\n- " + text.strip().replace("\n", "\n  ") + "\n"
    if tag in {"p", "ul", "ol", "dl", "table", "section", "blockquote"}:
        return "\n\n" + text.strip() + "\n\n"
    if tag == "dt":
        return "\n\n**" + text.strip() + "**\n"
    if tag in {"dd", "tr"}:
        return "\n" + text.strip() + "\n"
    if tag in {"th", "td"}:
        return text.strip() + " | "
    if tag in {"br", "hr"}:
        return "\n"
    return text


def walk(node):
    if isinstance(node, Node):
        yield node
        for child in node.children:
            yield from walk(child)


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--html-root", type=Path, default=Path("/usr/share/doc/cmake/html"))
    args = parser.parse_args()
    repo = Path(__file__).resolve().parents[2]
    output = repo / "src/cmake_builtin_docs"
    index = repo / "src/server/hovers/include.rs"
    version = re.search(r"CMake ([0-9.]+)", (args.html_root / "index.html").read_text()).group(1)
    records = {}
    provenance = "\n\n<!-- Script-generated content. Generator: scripts/gen/cmake_builtin_docs.py. -->\n"
    for kind in ("command", "variable"):
        directory = output / kind
        directory.mkdir(parents=True, exist_ok=True)
        records[kind] = []
        for source in sorted((args.html_root / kind).glob("*.html")):
            document = Body()
            document.feed(source.read_text())
            if document.root is None:
                raise ValueError(f"No documentation body: {source}")
            heading = next(node for node in walk(document.root) if node.tag == "h1")
            name = plain(heading).strip()
            url = f"https://cmake.org/cmake/help/v{'.'.join(version.split('.')[:2])}/{kind}/{source.name}"
            content = render(document.root, url).strip()
            content = re.sub(r"\n[ \t]+\n", "\n\n", content)
            content = re.sub(r"\n{3,}", "\n\n", content)
            target = directory / (source.stem + ".md")
            target.write_text(f"CMake {version} · [Official documentation]({url})\n\n" + content + provenance)
            records[kind].append((name, target))
    license_source = args.html_root.parent / "LICENSE.rst"
    (output / "LICENSE.txt").write_text("Script-generated by scripts/gen/cmake_builtin_docs.py. Upstream CMake license follows.\n\n" + license_source.read_text())
    lines = ["// Script-generated by scripts/gen/cmake_builtin_docs.py.", f'pub const VERSION: &str = "{version}";']
    for kind, entries in records.items():
        lines.append(f"pub static {kind.upper()}S: &[(&str, &str)] = &[")
        for name, path in sorted(entries):
            name_literal = '"' + name.replace('\\', '\\\\').replace('"', '\\"') + '"'
            relative = path.relative_to(repo / "src").as_posix()
            lines.append(f'({name_literal}, include_str!("../../{relative}")),')
        lines.append("];")
    index.write_text("\n".join(lines) + "\n")
    subprocess.run(["rustfmt", "--edition=2024", str(index)], check=True)
    print(f"CMake {version}: {len(records['command'])} commands, {len(records['variable'])} variables")


if __name__ == "__main__":
    main()
