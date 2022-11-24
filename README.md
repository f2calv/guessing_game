# Rust Playground

Learning Rust via;

- https://www.rust-lang.org/learn;

    - [the official book](https://doc.rust-lang.org/book/)
    - [the rustlings course](https://github.com/rust-lang/rustlings/)
    - [rust by example](https://doc.rust-lang.org/stable/rust-by-example/)

- [@letsgetrusty](https://www.youtube.com/@letsgetrusty)
- [@codetothemoon](https://www.youtube.com/@codetothemoon)

## Local Development via vscode devcontainer

- Install vscode
- Install [Remote Development Extension Pack for vscode](https://marketplace.visualstudio.com/items?itemName=ms-vscode-remote.vscode-remote-extensionpack)
- Rust has loads of small files which will give you performance problems when working in a container with a mounted volume, so mount the repository in a virtual volume;

    - In vscode press F1 and select 'Dev Containers: Clone Repository in Container Volume', enter this repo URL.

- After the container image has built you are ready to Rust!
