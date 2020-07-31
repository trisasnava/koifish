use webbrowser;

pub fn join(channel: String) {
    let slack = "https://trisasnava.slack.com/join/shared_invite/enQtODg1NjI0NTc1NzAzLTBjYTM1YjQxZWZkMTExYTBlNTcxNjQzYTc0MjRmNDNjMmIxZmMwZjM5ODFkZWExNjJkNWMwZWRjOGJlODdiM2Q";
    let github = "https://github.com/trisasnava";
    let website = "https://trisasnava.org";

    if channel.as_str() == "slack" {
        if webbrowser::open(slack).is_ok() {
            println!("Open slack successful !");
        };
    } else if channel.as_str() == "github" {
        if webbrowser::open(github).is_ok() {
            println!("Open github successful !");
        };
    } else if channel.as_str() == "website" {
        if webbrowser::open(website).is_ok() {
            println!("Open website successful !");
        };
    } else {
        println!("Open {:?} failure !", channel);
    }
}