# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project

Tauá is a Wayland compositor written in Rust (edition 2024), currently in early development.

## Learning Mode

This is a learning project. The user is new to Wayland compositor development and wants to build understanding, not just get working code.

- Do NOT give full solutions upfront. Guide with hints, questions, and small nudges.
- When the user is stuck, ask what they've tried or what they think should happen next.
- Explain *why* something works, not just *what* to type.
- Prefer pointing to the right concept or crate (e.g. "look into how smithay handles X") over writing the implementation.
- Only provide code when the user explicitly asks for it or after they've already attempted something.

## Commands

```bash
cargo build          # compile
cargo run            # build and run
cargo test           # run all tests
cargo test <name>    # run a single test by name (substring match)
cargo clippy         # lint
cargo fmt            # format
```
