# Docs contribution

[![Open in Gitpod](https://gitpod.io/button/open-in-gitpod.svg)](https://gitpod.io/#https://GITHUB.com/trisasnava/koifish)

1. Install rust

```shell script
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

2. Install mdbook 

```shell script
cargo install mdbook
``` 

3. Clone source code from GitHub

```shell script
git clone git@GITHUB.com:trisasnava/koifish.git
```
 
4.  Write DOCS under the `DOCS/src` using Markdown

```
> tree DOCS/src

KOIFISH\DOCS\SRC
    ├─contribution
    ├─getting-started
    ├─images
    └─roadmap
    └─...
```

5. Read [![Code of conduct](https://img.shields.io/badge/code%20of%20conduct-orange?style=for-the-badge&color=%23E5531A)](./CODE_OF_CONDUCT.md)
   and [![Sign the CLA](https://img.shields.io/badge/Sign%20the%20CLA-orange?style=for-the-badge&color=%23E5531A)](./CLA.md)

6. Run `mdbook serve DOCS` and to preview DOCS

7. Submit a pull request(PR)

## Related Links

- [Code of conduct](./CODE_OF_CONDUCT.md)

- [Sign the CLA](./CLA.md)