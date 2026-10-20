#!/usr/bin/env python3
"""Project-local validation for published Markdown and Zensical navigation."""

from __future__ import annotations

import re
import sys
import tomllib
from pathlib import Path
from urllib.parse import unquote

ROOT = Path(__file__).resolve().parents[1]
DOCS = ROOT / "project-docs"
EXCLUDED = ("harness/", "backlog/", "evidence/", "documentation/manifests/", "adrs/")
LINK = re.compile(r"(?<!!)\[[^\]]*\]\(([^)]+)\)|<((?:\.\.?/|/)[^>]+)>")
FENCE = re.compile(r"^```(\w+)\s*$")


def published_markdown() -> list[Path]:
    return [
        path
        for path in DOCS.rglob("*.md")
        if not any(path.relative_to(DOCS).as_posix().startswith(prefix) for prefix in EXCLUDED)
        and not path.name == "manifest.yml"
    ]


def links() -> list[str]:
    errors: list[str] = []
    for source in published_markdown():
        for match in LINK.finditer(source.read_text(encoding="utf-8")):
            target = (match.group(1) or match.group(2)).strip()
            if not target or target.startswith(("#", "https://", "http://", "mailto:", "tel:")):
                continue
            target = unquote(target.split("#", 1)[0].split("?", 1)[0])
            if not target:
                continue
            resolved = (source.parent / target).resolve()
            if not resolved.is_file() and not resolved.is_dir():
                errors.append(f"{source.relative_to(ROOT)}: missing link target {target}")
    return errors


def navigation() -> list[str]:
    config = tomllib.loads((ROOT / "zensical.toml").read_text(encoding="utf-8"))
    docs_dir = ROOT / config["project"].get("docs_dir", "docs")
    entries = config["project"].get("nav", [])
    paths: list[str] = []
    for entry in entries:
        if isinstance(entry, str):
            paths.append(entry)
        elif isinstance(entry, dict):
            for value in entry.values():
                if isinstance(value, str):
                    paths.append(value)
                elif isinstance(value, list):
                    paths.extend(item for item in value if isinstance(item, str))
    if not entries:
        return ["zensical.toml: project.nav must contain at least one page"]
    return [f"zensical.toml: navigation target does not exist: {path}" for path in paths if not (docs_dir / path).is_file()]


def redaction() -> list[str]:
    patterns = {
        "bearer credential": re.compile(r"(?i)\bBearer\s+[A-Za-z0-9._~+/=-]{16,}"),
        "private key": re.compile(r"-----BEGIN (?:RSA |EC |OPENSSH )?PRIVATE KEY-----"),
        "credential assignment": re.compile(r"(?i)\b(?:access[_-]?token|client[_-]?secret|api[_-]?key)\s*[:=]\s*[\"']?[A-Za-z0-9._~+/=-]{20,}"),
    }
    errors: list[str] = []
    for source in published_markdown():
        content = source.read_text(encoding="utf-8")
        for name, pattern in patterns.items():
            if pattern.search(content):
                errors.append(f"{source.relative_to(ROOT)}: possible {name}")
    return errors


def mermaid() -> list[str]:
    errors: list[str] = []
    accepted = ("flowchart ", "graph ", "sequenceDiagram", "classDiagram", "stateDiagram", "erDiagram", "journey", "gantt", "pie", "mindmap", "timeline", "gitGraph", "quadrantChart", "xychart", "block-beta", "architecture-beta")
    for source in DOCS.rglob("*.md"):
        in_mermaid = False
        body: list[str] = []
        start = 0
        for number, line in enumerate(source.read_text(encoding="utf-8").splitlines(), 1):
            match = FENCE.match(line)
            if match and not in_mermaid and match.group(1).lower() == "mermaid":
                in_mermaid, body, start = True, [], number
            elif in_mermaid and line.startswith("```"):
                diagram = "\n".join(body).strip()
                if not diagram or not diagram.startswith(accepted):
                    errors.append(f"{source.relative_to(ROOT)}:{start}: invalid or empty Mermaid diagram")
                in_mermaid = False
            elif in_mermaid:
                body.append(line.strip())
        if in_mermaid:
            errors.append(f"{source.relative_to(ROOT)}:{start}: unterminated Mermaid fence")
    return errors


CHECKS = {"documentation_links": links, "documentation_navigation": navigation, "documentation_redaction": redaction, "mermaid_validation": mermaid}

if len(sys.argv) != 2 or sys.argv[1] not in CHECKS:
    raise SystemExit(f"usage: {Path(sys.argv[0]).name} {{{'|'.join(CHECKS)}}}")
problems = CHECKS[sys.argv[1]]()
if problems:
    print("\n".join(problems), file=sys.stderr)
    raise SystemExit(1)
print(f"{sys.argv[1]}: passed")
