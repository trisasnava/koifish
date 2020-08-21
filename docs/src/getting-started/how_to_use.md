# How to use

# help

```shell script
> koi -h

    █▄▀ █▀█ ░ █▀▀ ░ █▀ █░█
    █░█ █▄█ █ █▀▀ █ ▄█ █▀█   0.0.1

USAGE:
    koi <SUBCOMMAND>

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

SUBCOMMANDS:
    help       Prints this message or the help of the given subcommand(s)
    join       Join our slack channel
    login      Verify login via GitHub Oauth
    meet       Start a meeting with https://meet.jit.si/koi
    open       Open koifish github|website|docs
    upgrade    Upgrade tool for Koifish
```

## login

When you execute this command, koi will open your default browser. 
And it will oauth your GitHub token to save `$HOME/.koi` file.
  
```shell script
koi login 
``` 

When login successfully,it will redirect to this page.  

![](https://user-images.githubusercontent.com/25944814/89096743-62784780-d40b-11ea-8a50-8ec50e1ea550.png)

## open

When you execute this command, koi will open this channel 
using your default browser 

```shell script
koi open 
koi open github | website | docs
``` 

## join

Join Koifish slack channel

```shell script
koi join
```

## meet

Start a meeting with https://meet.jit.si/koi

```shell script
koi meet 
```

## upgrade

Upgrade Koifish to latest version

```shell script
koi upgrade
```