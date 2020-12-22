# How to use

## CLI help info

```shell script
> koi -h

    █▄▀ █▀█ ░ █▀▀ ░ █▀ █░█
    █░█ █▄█ █ █▀▀ █ ▄█ █▀█   0.0.5

USAGE:
    koi <SUBCOMMAND>

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

SUBCOMMANDS:
    help       Prints this message or the help of the given subcommand(s)
    join       join our SLACK channel
    login      verify login via GitHub Oauth
    MEET       start a meeting with https://MEET.jit.si/koi
    open       open koifish GITHUB|WEBSITE|DOCS
    upgrade    upgrade tool for Koifish
```

## login

When you execute this command, koi will open your default browser. 
And it will oauth your GitHub token to save `$HOME/.koi` file.
  
```shell script
koi login -h

USAGE:
    koi login [FLAGS]

FLAGS:
    -h, --help        Prints help information
    -r, --re-oauth    re-oauth with GitHub
    -V, --version     Prints version information
``` 

When login successfully,it will redirect to this page.  

![](https://user-images.githubusercontent.com/25944814/89096743-62784780-d40b-11ea-8a50-8ec50e1ea550.png)

## open

When you execute this command, koi will open this channel 
using your default browser 

```shell script
koi open -h

USAGE:
    koi open [GITHUB | WEBSITE | DOCS] 

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

ARGS:
    <channel>     [default: DOCS]

``` 

## join

Join Koifish SLACK channel

```shell script
koi join -h 

USAGE:
    koi join

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

```

## MEET

Start a meeting with https://MEET.jit.si/koi

```shell script
koi MEET -h

USAGE:
    koi MEET

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information
```

## upgrade

Upgrade Koifish to latest version

```shell script
koi upgrade -h

USAGE:
    koi upgrade [FLAGS]

FLAGS:
    -h, --help        Prints help information
    -r, --re-oauth    re-oauth with GitHub
    -V, --version     Prints version information
    -v, --verbose     release notes verbose output
```

## Related Links

- [Code of conduct](../contribution/CODE_OF_CONDUCT.md)

- [Sign the CLA](../contribution/CLA.md)

- [Contribute code](../contribution/code.md)

- [Contribute DOCS](../contribution/DOCS.md)

- [FAQ](../FAQ.md)