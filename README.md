

# TeXgen — LaTeX Table Generator [![Status](https://img.shields.io/badge/status-in--development-red)]()

TeXgen is a CLI and API-based tool that generates clean and customizable LaTeX tables from CSV or Excel input files. Designed for academic and professional use, it supports features like multicolumns, multirows, and precise alignment control through a simple JSON config.

---

## 🔧 Features

- Convert `.csv` or `.xlsx` to LaTeX `tabular` environments
- JSON configuration for:
  - `\multicolumn` and `\multirow` spans
  - Horizontal and vertical borders
  - Custom alignments (`l`, `c`, `r`, with or without vertical bars)
- Optional pretty-printing with tabular alignment for readable source
- Lightweight local API server for integration into other workflows

- React frontend available at: [TeXgen-frontend](https://github.com/H3ct0r55/TeXgen-frontend)

---

## 📦 Installation

Clone the repo and build with Cargo:

```bash
git clone https://github.com/H3ct0r55/TeXgen.git
cd TeXgen
cargo build --release
```

---

## 🚀 Usage

### API (localhost)

Start the API:

```bash
./target/release/texgen
```

POST a file and config to `http://localhost:8000/generate`.

---

## 🗂 JSON Config Example

```json
{
  "horizontal_borders": [1, 3],
  "vertical_borders": [1, 2],
  "spans": [
    { "row": 0, "col": 0, "span": 2, "alignment": "|c|" },
    { "row": 2, "col": 1, "span": 2 }
  ]
}
```

---

## 🏗 Roadmap

- [ ] Full Excel support with formula parsing
- [ ] GUI frontend (planned)
- [ ] Advanced formatting (color, bold, etc.)

---