![](https://github.com/Pencilcaseman/librapid/blob/master/branding/LibRapid_light.png)
[![Discord](https://img.shields.io/discord/848914274105557043?color=green&label=Discord&logo=Discord)](https://discord.gg/cau7zy7zBE)
[![Crates.io](https://img.shields.io/crates/v/lib_rapid?color=green&label=Latest&logo=Rust&logoColor=orange)](https://crates.io/crates/lib_rapid/)
[![docs.rs](https://img.shields.io/docsrs/lib_rapid?color=green&label=Docs%20%28latest%29&logo=Rust&logoColor=orange)](https://docs.rs/crate/lib_rapid/latest)
[![Crates.io](https://img.shields.io/crates/d/lib_rapid?color=green&label=Downloads&logo=Rust&logoColor=orange)](https://crates.io/crates/lib_rapid)
----

LibRapid for Rust - Fast. Reliable. Lightweight.
============
Current Statistics
-----

| **Item**     | **Count** | **Used RegEx**       | **Used Method**     |
|--------------|-----------|----------------------|---------------------|
| Examples     | 313       |(No RegEx required)   | `cargo test`        |
| Constants    | 187       |`pub const [^f][^n]`  | VSCode RegEx-Search |
| Functions    | 131       |`pub c?o?n?s?t? ?fn`  | VSCode RegEx-Search |
| Modules      | 31        |`pub mod`             | VSCode RegEx-Search |
| Traits       | 14        |`pub trait`           | VSCode RegEx-Search |
| Structs      | 13        |`pub struct`          | VSCode RegEx-Search |
| Macros       | 7         |`macro_rules`         | VSCode RegEx-Search |
| Dependencies | 4         |(No RegEx required)   | `Cargo.toml`        |

**Changelog**
-----
0.4.2
_____
- Added Public API: `math::equations::polynomial`.
- Added Public API: `math::general::round_to_n_mult()`.
- Added Public API: `math::general::NumTools::pow()`.
- Added Public API: `math::general::factorial()`.
- Added Public API: `math::general::div_factorials()`.
- Added Public API: `compsci::stringhelpers::StringUtils::to_isize()`.
- Added Public API: `compsci::general::SliceOps::position_of()`.
-----

Why X? Why Y?
-----

Sometimes I have to yank a version. To stay updated and to chat with us, please consider joining our Discord! https://discord.gg/cGxTFTgCAC

Contributing
-----

You want to contribute? Sure! You can contribute with:

- 🚩 Issues
- 🙇 Pull Requests (See `FORMATRULES.md` for more information)
- 💡 Feature Suggestions (Via Discussions or Issues)

Be sure to add documentation to any new public API with examples, and running `cargo test` to ensure that your changes are valid.

Documentation
-----

Depending on the current published LibRapid version and how many crates are in queue on docs.rs, you may need to build the docs yourself.
To do that, simply git-clone this repository and then run `cargo doc --open`. After that, your documentation should open automatically.

How does it work?
-----

The Rust-port of LibRapid is a derivative from the main library made for C++ and Python. It's goal is to be optimised for speedy calculations, mathematical and
scientific applications.