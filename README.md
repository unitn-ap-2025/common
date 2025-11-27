[![Image Dev](https://github.com/unitn-ap-2025/common/actions/workflows/image-dev.yaml/badge.svg)](https://github.com/unitn-ap-2025/common/actions/workflows/image-dev.yaml) [![CI](https://github.com/unitn-ap-2025/common/actions/workflows/ci.yaml/badge.svg)](https://github.com/unitn-ap-2025/common/actions/workflows/ci.yaml) [![CD](https://github.com/unitn-ap-2025/common/actions/workflows/cd.yaml/badge.svg)](https://github.com/unitn-ap-2025/common/actions/workflows/cd.yaml) [![Docs](https://github.com/unitn-ap-2025/common/actions/workflows/docs.yaml/badge.svg)](https://github.com/unitn-ap-2025/common/actions/workflows/docs.yaml)

# Common

## Introduction

This is where things actually work.

Working in progress...

### About the project

- [Main document](https://didatticaonline.unitn.it/dol/pluginfile.php/1983232/mod_resource/content/2/main.pdf)
- [Protocol](https://didatticaonline.unitn.it/dol/pluginfile.php/1983233/mod_resource/content/2/protocol.pdf)
- [Organization](https://didatticaonline.unitn.it/dol/pluginfile.php/1983234/mod_resource/content/1/org.pdf)

## Contribution

### Understand how do Git & GitHub work

- [GitHub & Git -- A brief introduction](https://didatticaonline.unitn.it/dol/pluginfile.php/1987335/mod_resource/content/2/GitHub%20%20Git.pdf)

### Permission

- Team "[reviewer](https://github.com/orgs/unitn-ap-2025/teams/reviewer)": The permission role "write".
- Team "[leader](https://github.com/orgs/unitn-ap-2025/teams/leader)": The permission role "Triage".
- Members from other teams only have the permission to read.

About permission role:

- [Repository roles for an organization](https://docs.github.com/en/organizations/managing-user-access-to-your-organizations-repositories/managing-repository-roles/repository-roles-for-an-organization)

### Guide for Developer

#### Enter the developer environment

For developing, VSCode with `devcontainer` is recommended. Steps as following ([full docs](https://code.visualstudio.com/docs/devcontainers/containers)):

1. To get start with, please have [VSCode](https://code.visualstudio.com) and [Docker](https://www.docker.com) prepared.
2. Install the extension named "Dev Containers" on VSCode (ID: [ms-vscode-remote.remote-containers](https://marketplace.visualstudio.com/items?itemName=ms-vscode-remote.remote-containers)).
3. Enter the directory of the repo, then click "Reopen in Container".
4. If you have network connection, it should be done in minutes.

> INFO: To conclude, `devcontainer` provides a standard dev env for every developer, regardless of the OS or the hardware, which is why you should use it. No one knows what's the Neovim config or the `/.idea` behind your screenshot afterall.

To use `devcontainer` in RustRover, please check the official guide here:

- [Dev Container overview - RustRover Documentation](https://www.jetbrains.com/help/rust/connect-to-devcontainer.html)

> WARNING: Because I ([yifen9](https://github.com/yifen9)) don't use RustRover, I didn't justify it to suit RustRover's env, lacking of extensions for example, though it should work. If anyone has the motivation, please feel free to config [`/.devcontainer/devcontainer.json`](https://github.com/unitn-ap-2025/common/blob/main/.devcontainer/devcontainer.json).

If you use Neovim or something else, you should be able to figure it out by yourself.

By the way, if you insist on not using `devcontainer`, then please check and install the packages listed in [`/ops/apt`](https://github.com/unitn-ap-2025/common/tree/main/ops/apt), and remember to install [`rust`](https://rust-lang.org) and [`just`](https://just.systems/man/en).

#### Meet `just` and `justfile`

`just` and `justfile` are very useful for manipulating projects ([full docs](https://just.systems/man/en)).

To understand it, I will provide an easy example:

> Normally if we want to format a Rust project, we do this:
> 
> ```
> cargo fmt
> ```
> And with a `justfile` as this:
>
> ```
> # /justfile
>
> fmt:
>     cargo fmt
> ```
>
> We can do this:
>
> ```
> just fmt
> ```
>
> Which is equivalent to `cargo fmt`. OK but what's the point? Now think of this:
>
> ```
> cargo fmt && cargo clippy && cargo test
> ```
>
> But with a `justfile` like this:
>
> ```
> # /justfile
>
> fmt:
>     cargo fmt
>
> lint:
>     cargo clippy
>
> test:
>     cargo test
>
> ci:
>     just fmt && just lint && just test
> ```
>
> You would only need to use `just ci`, which is better.
>
> What's more, if you have a `justfile` like this:
>
> ```
> # /justfile
>
> fmt:
>     cargo fmt
>
> lint:
>     cargo clippy -- -D warnings
>
> test:
>     cargo test
>
> ci:
>     just fmt && just lint && just test
> ```
>
> Then everytime you run `just ci`, it's actually equivalent to:
>
> ```
> cargo fmt && cargo clippy -- -D warnings && cargo test
> ```
>
> Now I hope you understand why it's good and why you should use it.

In fact, `just` and `justfile` are much powerful than what the example presented above. It's kind of similar to `make` and `Makefile` but better.

By default, `just` is included in the `devcontainer`, so once you have entered the `devcontainer`, `just` is ready to go. One can also check the [`/justfile`](https://github.com/unitn-ap-2025/common/blob/main/justfile)

#### Continuous Integration

A commit to the `main` branch will trigger the GitHub Action called `CI`, which will do the same as what the `just ci` will do locally. This is for ensuring the quality of the code.

Please use `just ci` locally and make sure all green before committing any changes. Otherwise, the [![CI](https://github.com/unitn-ap-2025/common/actions/workflows/ci.yaml/badge.svg)](https://github.com/unitn-ap-2025/common/actions/workflows/ci.yaml) might fail, and the PR will be rejected.
