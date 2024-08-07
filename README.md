# ✏️ Ren

> A tiny spellchecker.

## ⬇️ Installation

At the moment, you can only install Ren by source:

```sh
git clone https://github.com/interrrp/ren
cd ren
cargo install .
```

## ⚙️ Usage

Read the help text:

```plaintext
Usage: ren [OPTIONS] [PATTERN]

Arguments:
  [PATTERN]  The glob pattern to search for files to spell check [default: **/*.md]

Options:
  -l, --langs <LANGS>  The language codes of the wordlists to use for spell checking [default: en_us]
  -h, --help           Print help
```

## 🔑 License

Ren is licensed under [GPLv3](./LICENSE).
