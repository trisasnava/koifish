# How to use

## CLI help info

```shell script
> koi -h

    █▄▀ █▀█ ░ █▀▀ ░ █▀ █░█
    █░█ █▄█ █ █▀▀ █ ▄█ █▀█   0.0.7

USAGE:
    koi [FLAGS] <SUBCOMMAND>

FLAGS:
    -d, --debug      Activate debug mode
    -h, --help       Prints help information
    -V, --version    Prints version information

SUBCOMMANDS:
    clone     Speed up git clone via `https://github.com.cnpmjs.org`
    help      Prints this message or the help of the given subcommand(s)
    login     Log in via GitHub Oauth
    meet      Start a online meeting via https://meet.jit.si/
    open      Open channels "docs,github,site,slack,discord"
    update    Update from GitHub release

```

## login

When you execute this command, koi will open your default browser. 
And it will oauth your GitHub token to save `$HOME/.koi` file.
  
```shell script
> koi login -h

USAGE:
    koi login [FLAGS]

FLAGS:
    -h, --help        Prints help information
    -r, --re-oauth    Re-oauth via github Oauth
    -V, --version     Prints version information
``` 

When login successfully,it will redirect to this page.  

![](https://user-images.githubusercontent.com/25944814/89096743-62784780-d40b-11ea-8a50-8ec50e1ea550.png)

## Clone

Speed up git clone via `https://github.com.cnpmjs.org`

```shell
> koi clone -h

USAGE:
    koi clone <url> [path]

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

ARGS:
    <url>     Git repo url
    <path>    A path to save [default: .]

```

## open

When you execute this command, koi will open this channel 
using your default browser 

```shell script
> koi open -h

USAGE:
    koi open [channel]

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

ARGS:
    <channel>    Open channels "docs,github,site,slack,discord" [default: docs]

```

## meet

Start a online meeting via https://meet.jit.si/

```shell script
> koi meet -h

USAGE:
    koi meet

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information
```

## update

Update Koifish

```shell script
> koi update -h

USAGE:
    koi update [FLAGS]

FLAGS:
    -h, --help        Prints help information
    -r, --re-oauth    Re-oauth via GitHub Oauth
    -V, --version     Prints version information
    -v, --verbose     Release notes verbose output
    
```

## Related Links

- [Code of conduct](../contribution/CODE_OF_CONDUCT.md)

- [Sign the CLA](../contribution/CLA.md)

- [Contribute code](../contribution/code.md)

- [Contribute docs](../contribution/docs.md)

- [FAQ](../FAQ.md)