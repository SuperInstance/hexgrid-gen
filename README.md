# hexgrid-gen

**Code generator for Eisenstein hex grid lookup tables. Outputs Rust, C, Python, JS, or JSON.**

Precompute hex disk coordinates, rotation tables, and D₆ symmetry maps once and emit them as static arrays in whatever language you need. The tables are exact — generated from Eisenstein integer arithmetic, not floating-point approximation.

## Quickstart

```bash
# Generate a hex disk table as Rust
hexgrid-gen disk --radius 20 --lang rust > hex_disk_20.rs

# Generate rotation table as C
hexgrid-gen rotate --lang c > rotation_table.h

# Generate D₆ symmetry table as Python
hexgrid-gen symmetry --lang python > symmetry_table.py
```

## Supported Outputs

| Language | Use Case |
|----------|----------|
| Rust | `static` arrays for no_std environments |
| C | Embedded lookup tables for microcontrollers |
| Python | Prototyping and simulation |
| JavaScript | Browser-based hex grids |
| JSON | Interchange format for cross-language pipelines |

## Why Code Generation?

For embedded and safety-critical systems, you want the tables at compile time, not computed at startup. `hexgrid-gen` produces the same hex coordinates the Rust crate computes at runtime, but as static data — no allocations, no init code, no runtime cost.

## License

MIT OR Apache-2.0
