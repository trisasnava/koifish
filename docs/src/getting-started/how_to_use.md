# How to use

## CLI help info

```shell script
> koi -h

    █▄▀ █▀█ ░ █▀▀ ░ █▀ █░█
    █░█ █▄█ █ █▀▀ █ ▄█ █▀█   0.0.6

USAGE:
    koi [FLAGS] <SUBCOMMAND>

FLAGS:
    -d, --debug      Activate debug mode
    -h, --help       Prints help information
    -V, --version    Prints version information

SUBCOMMANDS:
    help       Prints this message or the help of the given subcommand(s)
    join       Join our slack/discord channel,default is slack
    login      Verify login via GitHub Oauth
    meet       Start a meeting  at https://meet.jit.si/koig
    open       Open Koifish github/site/docs
    upgrade    Upgrade Koifish from github release

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
    <channel>     [default: docs]

``` 

## join

Join Koifish SLACK channel

```shell script
> koi join -h 

USAGE:
    koi join [channel]

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

ARGS:
    <channel>    Join our slack/discord channel [default: slack]
```

## meet

Start a meeting with https://MEET.jit.si/koi

```shell script
> koi meet -h

USAGE:
    koi meet

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information
```

## upgrade

Upgrade Koifish to latest version

```shell script
> koi upgrade -h

USAGE:
    koi upgrade [FLAGS]

FLAGS:
    -h, --help        Prints help information
    -r, --re-oauth    Re-oauth via github Oauth
    -V, --version     Prints version information
    -v, --verbose     Release notes verbose output
```

## Related Links

- [Code of conduct](../contribution/CODE_OF_CONDUCT.md)

- [Sign the CLA](../contribution/CLA.md)

- [Contribute code](../contribution/code.md)

- [Contribute docs](../contribution/docs.md)

- [FAQ](../FAQ.md)