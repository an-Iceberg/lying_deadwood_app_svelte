# Lying Deadwood App

This project uses [Tauri](https://tauri.app/) as the app framework, [Svelte](https://svelte.dev/) for the frontend
and [Rust](https://www.rust-lang.org/) for the backend. The data is stored in [Polars](https://pola.rs/) dataframes.

[rustup](https://rustup.rs/)

[NodeJS](https://nodejs.org/en)

[Deno](https://deno.com/)

## Recommended IDE Setup

[VS Code](https://code.visualstudio.com/) + [Svelte](https://marketplace.visualstudio.com/items?itemName=svelte.svelte-vscode) + [Tauri](https://marketplace.visualstudio.com/items?itemName=tauri-apps.tauri-vscode) + [rust-analyzer](https://marketplace.visualstudio.com/items?itemName=rust-lang.rust-analyzer).

---
---
---

useful commands:

`cargo tauri android init`

`cargo tauri dev`

`cargo tauri build`

`cargo tauri android dev`

`cargo tauri android build --debug`

`cargo tauri android build --split-per-abi --apk`

## Todo:
- [x] ability to swap min with max
- [ ] part id same as in the field: 1.1, 1.2, 2.1, 2.2, 3.1, 3.2, 3.3, etc.
- [ ] directory for each area group and subarea. Put lookup data into the specific subdirectory asw.
- [ ] display largest ID in input somewhere
- [ ] if files for team and tree species don't exist, create ones
- [ ] {maybe} display input restraints (make it toggleable in settings)
- [ ] create folders and file button (upon entering area data)
- [ ] show list of present files (tapping on one should open it)
- [ ] after creating a file, the area info has to become immutable
- [ ] submit data to backend
- [ ] upon receiving data from frontend, backend needs to update frontend with dataframe data
- [ ] edit data
- [ ] delete data
- [ ] {at the very end} adjust colors in `bootstrap.css`
- [ ] installation and dev instructions (with references to sources) for all platforms
  - [ ] android device in debug mode
  - [ ] `adb`
  - [ ] `android-sdk`
  - [ ] `android-ndk`
  - [ ] `gradle`
  - [ ] `kotlin`
  - [ ] `java`
  - [ ] `sdkmanager`
  - [ ] `deno`
  - [ ] `rust`
  - [ ] [dependencies listed on tauri website](https://tauri.app/start/prerequisites/)
  - [ ] environment variables
  - [ ] proper vscode F5 commands
