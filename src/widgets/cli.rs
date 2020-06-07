use structopt::StructOpt;

#[derive(Debug, PartialEq, StructOpt)]
#[structopt(about = "
█▄▀ █▀█ ░ █▀▀ ░ █▀ █░█
█░█ █▄█ █ █▀▀ █ ▄█ █▀█")]
pub struct Koifish {
    /// Run a online Koifish in https://webassembly.sh
    #[structopt(short = "o", long = "oline", default_value = "")]
    online: String,

    /// Join our Slack Channel or WeChat Group(QR-Code)
    #[structopt(short = "j", long = "join", default_value = "slack")]
    join: String,

    /// Start a web Koifish in local with your port.
    #[structopt(short = "w", long = "web", default_value = "2121")]
    web: String,

    #[structopt(subcommand)]
    fish: Option<Fish>,
}

#[derive(Debug, PartialEq, StructOpt)]
enum Fish {
    /// Login to GitHub.
    Login { user: String, password: String },
    /// Get GitHub user info.
    User {
        #[structopt(default_value = "trisasnava")]
        user_or_org: String,
    },
    /// Get GitHub repo info.
    Repo {
        #[structopt(default_value = "trisasnava")]
        user_or_org: String,
        #[structopt(default_value = "koifish")]
        repo: String,
    },
    /// Get GitHub issues info in your repo.
    Issues {
        #[structopt(default_value = "trisasnava")]
        user_or_org: String,
        #[structopt(default_value = "koifish")]
        repo: String,
    },
    /// Get GitHub prs info for your repo.
    Prs {
        #[structopt(default_value = "trisasnava")]
        user_or_org: String,
        #[structopt(default_value = "koifish")]
        repo: String,
    },
    /// Get GitHub trending repo info.
    #[structopt(help = "Fitter by daily|weekly|monthly,The default is daily.")]
    Trending {
        #[structopt(default_value = "daily")]
        date: String,
    },
}

impl Koifish {
    /// Match command line args
    pub fn match_args() -> Self {
        Koifish::from_args()
    }
}
