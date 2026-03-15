# bud 

A minimal CLI tool for quickly scaffolding folders and files using a simple shell-like syntax.

## Usage
```bash
bud <path>
```

## Syntax

- **Word without extension** → folder
- **Word with extension** → file  
- **`: `** → enter folder (go deeper)
- **`;`** → go back one level

## Example
```
MYPROJECT:
  ↳ src:
      ↳ main.rs
      ↳ lib.rs;
  ↳ Cargo.toml
  ↳ .gitignore;
```

This creates:
```
MYPROJECT/
├── src/
│   ├── main.rs
│   └── lib.rs
├── Cargo.toml
└── .gitignore
```