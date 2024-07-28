<div align="center">
  <a href="https://github.com/UndefinedProjectMC/UndefinedRedstone">
    <img src="urgithub.png" alt="Logo">
  </a>
  <h3 align="center">The next-gen Minecraft: Bedrock Edition server software written in Rust Programming Language</h3>

  <a href="https://github.com/UndefinedProjectMC/UndefinedRedstone/actions"><img     src="https://github.com/UndefinedProjectMC/UndefinedRedstone/actions/workflows/rust.yml/badge.svg" alt="Build"/></a>
  <a href="https://feedback.minecraft.net/hc/en-us/articles/28105668043661-Minecraft-1-21-2-Bedrock"><img src="https://img.shields.io/badge/minecraft-v1.21.2%20(Bedrock)-green" /></a>
  <a href="https://github.com/UndefinedProjectMC/UndefinedRedstone/tree/main/crates/undefined_redstone_network/src/protocol"><img src="https://img.shields.io/badge/protocol-686-green" /></a>

  ### English | [简体中文(中国大陆)](README_zh_CN.md) | [正體中文(台灣地區)](README_zh_TW.md) | [Русский](README_ru.md)
</div>

## 🎉Introduction Undefined Redstone🎉
### Undefined Redstone is a third-party server software for Minecraft: Bedrock Edition written in Rust Programming Language.
Undefined Redstone utilized the **ECS (Entity-Component-System)** architecture for coding, while extensively employing asynchronous operations. Thanks to the Plugin System of Bevy ECS, he could easily construct modularly, which has many advantages.
> [!IMPORTANT]
> Please be aware that this project is currently in a very early stage of development and may contain numerous **unknown bugs**, **unresolved security vulnerabilities**, and **many features that have yet to be implemented**.
> 
> Furthermore, **there is no official release version of the project** at this time, so all beta versions will be built using **Github Actions**.
> 
> We strongly advise against using this project in a **production environment**, even if certain parts may appear to be complete, until an official release has been issued.
## 🎶Features🎶
- **Modularity**: The meticulously designed modularity of UndefinedRedstone allows developers to easily modify the code.

- **Humanized version management**:
  - Based on the modular design concept, UndefinedRedstone does not specify any Minecraft version.
  - You only need to drag the version package into the version_control folder to automatically load the specified version's mobs, items, recipes, etc.
  - In the future, we will further support a higher degree of modularity and customization.

- **ECS architecture**: Following Minecraft: Bedrock Edition, we have used the ECS architecture to manage the game, which results in less repetitive code, more efficient memory management, and a high degree of customization and modularity.

- **Security, high performance, and low footprint**:
  - Thanks to the Rust programming language, we have brought memory safety and high performance to the Minecraft: Bedrock Edition server software.
  - Compared to server software developed in PHP and Java, we have higher performance, lower footprint, and better memory safety.

- **Extensive support for asynchronous and multi-threading**: We have used a large amount of asynchronous and multi-threading to make the most of multi-core CPUs and reduce performance waste.

## 🎆Getting Started🎆
#### UndefinedRedstone is written in Rust language, so you need to use Cargo to build it.
[Download Rust](https://www.rust-lang.org/en-US/learn/get-started)

**Run directly**
```shell
cargo run --package undefined_redstone_startup --bin undefined_redstone_startup
```

**Build**
```shell
cargo build --package undefined_redstone_startup --bin undefined_redstone_startup
```

## 👉Feedback👈
#### We need your help in reporting any bugs or vulnerabilities that you encounter, and we also welcome any suggestions you may have.

[Issues page](https://github.com/UndefinedProjectMC/UndefinedRedstone/issues)

You are also welcome to join our Discord group or contact us via email at dev@iruanp.com

## 📄License📄

**Copyright 2024 UndefinedProject, all rights reserved.**

If not otherwise specified, project content is open source under the GPL-3.0 license.

The contents of the following folders are open-sourced under the Apache-2.0 license:
- undefined_redstone_plugin
