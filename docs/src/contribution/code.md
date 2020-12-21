# Codes contribution

[![Open in Gitpod](https://gitpod.io/button/open-in-gitpod.svg)](https://gitpod.io/#https://github.com/trisasnava/koifish)

1. Install rust

```shell script
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

2. Clone source code from GitHub

```shell script
git clone git@github.com:trisasnava/koifish.git
``` 

3. Write your rust code under `src`

```
> tree src

KOIFISH\SRC
    ├─handler
    ├─model
    ├─static
    ├─utils
    └─widgets
    └─...
```

4. Read [![Code of conduct](https://img.shields.io/badge/code%20of%20conduct-orange?style=for-the-badge&color=%23E5531A)](./CODE_OF_CONDUCT.md)
and [![Sign the CLA](https://img.shields.io/badge/Sign%20the%20CLA-orange?style=for-the-badge&color=%23E5531A)](./CLA.md)

5. build and code format

```shell
cargo fmt && cargo build
```

6. Submit a pull request(PR)

## Related Links

- [Code of conduct](./CODE_OF_CONDUCT.md)

- [Sign the CLA](./CLA.md)
