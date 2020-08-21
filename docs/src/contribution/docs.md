# Docs contribution

[![Open in Gitpod](https://gitpod.io/button/open-in-gitpod.svg)](https://gitpod.io/#https://github.com/trisasnava/koifish)

1. Install rust-lang environment

```shell script
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

2. Install Mdbook 

```shell script
cargo install mdbook
``` 

3. Clone source code from GitHub

```shell script
git clone git@github.com:trisasnava/koifish.git
```
 
4.  Write docs under the `docs/src` using Markdown

```tree
    ├─docs
    │  ├─src
    │  │  ├─commands
    │  │  ├─contribution
    │  │  ├─getting-started
    │  │  ├─guide
    │  │  ├─reference
    │  │  └─roadmap
    │  └─theme
```

5. Read [contributor covenant code of conduct](CODE_OF_CONDUCT.md) and [sign the CLA](https://cla-assistant.io/trisasnava/koifish)

6. Run `mdbook serve` and to preview docs

7. Submit a pull request(PR)