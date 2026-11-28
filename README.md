![Lorebook](resources/lorebook-small.png)

---
---

# Lorebook

An experiment in programming as storytelling and storytelling as code.

## Reference

* [Changelog](CHANGELOG.md)
* [To Do List](TODO.md)
* [Design Decisions](DESIGN_DECISIONS.md)
* [Language Definition](LANGUAGE_DEFINITION.md)

## Usage

```
Usage: lorebook [(OPTIONS)]

  An experiment in programming as storytelling and storytelling as code.

Options:

  --mode <MODE>                        Mode to run in (default: cli, choices: cli, help)

  --intro <INTRO>                      Introduction to interactive storytelling default:

  Long ago, when the dew of creation was fresh upon the fresh upon the heather, and the mists had not yet parted, and the stars in the skup were still few and bright...

  Let us tell a tale together!

  --prompt <PROMPT>                    Prompt template for interactive storytelling (default: [Lorebook] {}> )

  --help | -h                          Display this help message
  --version | -V                       Display version
```

## Syntax Highlighting

You can find the syntax highlighting files in [syntax_files/](syntax_files/), and can install them automatically on systems that have posix compatible shells (like sh, bash, and ksh) with:

```bash
./scripts/install-syntax.sh
```

---
---

©2026 Bethany Davis <ellis@despoena.com>.

