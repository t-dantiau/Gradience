# Command-Line Help for `gradience-cli`

This document contains the help content for the `gradience-cli` command-line program.

**Command Overview:**

* [`gradience-cli`↴](#gradience-cli)
* [`gradience-cli shell`↴](#gradience-cli-shell)
* [`gradience-cli gtk`↴](#gradience-cli-gtk)
* [`gradience-cli store`↴](#gradience-cli-store)
* [`gradience-cli store add`↴](#gradience-cli-store-add)
* [`gradience-cli store remove`↴](#gradience-cli-store-remove)
* [`gradience-cli store online-list`↴](#gradience-cli-store-online-list)
* [`gradience-cli store local-list`↴](#gradience-cli-store-local-list)
* [`gradience-cli store download`↴](#gradience-cli-store-download)

## `gradience-cli`

Change the look of GNOME, with ease

**Usage:** `gradience-cli [OPTIONS] <COMMAND>`

###### **Subcommands:**

* `shell` — Apply the theme to the shell, it will create a new theme with a shell theme and GTK theme
* `gtk` — Apply the theme to the GTK theme
* `store` — Manage the store of presets

###### **Options:**

* `-m`, `--mode <MODE>` — The mode of the theme

  Possible values: `light`, `dark`

* `-a`, `--accent <ACCENT>` — The accent color of the theme

  Possible values: `blue`, `teal`, `green`, `yellow`, `orange`, `red`, `pink`, `purple`, `slate`

* `-s`, `--store <STORE>` — The path to the store where presets are stored
* `--shell-source <SHELL_SOURCE>` — The path to the shell source directory which contains templates for the shell theme
* `-p`, `--preset <PRESET>` — The name of the preset to apply



## `gradience-cli shell`

Apply the theme to the shell, it will create a new theme with a shell theme and GTK theme

**Usage:** `gradience-cli shell [TEMP_DIR] [THEME_DIR]`

###### **Arguments:**

* `<TEMP_DIR>` — The path to the temporary directory where the theme will be created and build
* `<THEME_DIR>` — The path to the directory where the theme will be stored



## `gradience-cli gtk`

Apply the theme to the GTK theme

**Usage:** `gradience-cli gtk`



## `gradience-cli store`

Manage the store of presets

**Usage:** `gradience-cli store <COMMAND>`

###### **Subcommands:**

* `add` — Add a new preset to the store
* `remove` — Remove a preset from the store
* `online-list` — List the online presets
* `local-list` — List the local presets
* `download` — Download a preset from the online store



## `gradience-cli store add`

Add a new preset to the store

**Usage:** `gradience-cli store add <PATH>`

###### **Arguments:**

* `<PATH>` — The path to the preset file



## `gradience-cli store remove`

Remove a preset from the store

**Usage:** `gradience-cli store remove <NAME>`

###### **Arguments:**

* `<NAME>` — The name of the preset to remove



## `gradience-cli store online-list`

List the online presets

**Usage:** `gradience-cli store online-list`



## `gradience-cli store local-list`

List the local presets

**Usage:** `gradience-cli store local-list`



## `gradience-cli store download`

Download a preset from the online store

**Usage:** `gradience-cli store download <NAME>`

###### **Arguments:**

* `<NAME>` — The name of the preset to download



<hr/>

<small><i>
    This document was generated automatically by
    <a href="https://crates.io/crates/clap-markdown"><code>clap-markdown</code></a>.
</i></small>

