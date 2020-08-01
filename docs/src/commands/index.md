# Koifish Commands

```shell script
> koifish -h

    █▄▀ █▀█ ░ █▀▀ ░ █▀ █░█
    █░█ █▄█ █ █▀▀ █ ▄█ █▀█   0.0.1

USAGE:
    koifish <SUBCOMMAND>

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

SUBCOMMANDS:
    help        Prints this message or the help of the given subcommand(s)
    issues      Get GitHub issues info in your repo
    join        Join our slack | github | website
    login       Login to GitHub
    online      Run a online Koifish in https://webassembly.sh
    prs         Get GitHub prs info for your repo
    repo        Get GitHub repo info
    trending    Get GitHub trending repo info
    user        Get GitHub user info
    web         Start a web Koifish in local with your port
```

## login

When you execute this command, koi will open your default browser. 
And it will oauth your GitHub token to save `$HOME/.koi` file.
  
```shell script
koifish login 
``` 

When login successfully,it will redirect to this page.  

![](https://user-images.githubusercontent.com/25944814/89096743-62784780-d40b-11ea-8a50-8ec50e1ea550.png)

## open

When you execute this command, koi will open this channel 
using your default browser 

```shell script
koifish open 
koifish open github | website | docs | slack(default)  
``` 