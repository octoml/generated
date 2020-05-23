use clap::{App, AppSettings, Arg, SubCommand};
use default_boxed::DefaultBoxed;

#[derive(DefaultBoxed)]
struct Outer<'a, 'b> {
    inner: HeapApp<'a, 'b>,
}

struct HeapApp<'a, 'b> {
    app: App<'a, 'b>,
}

impl<'a, 'b> Default for HeapApp<'a, 'b> {
    fn default() -> Self {
        let mut app = App::new("iamcredentials1")
            .setting(clap::AppSettings::ColoredHelp)
            .author("Sebastian Thiel <byronimo@gmail.com>")
            .version("0.1.0-20200515")
            .about("Creates short-lived, limited-privilege credentials for IAM service accounts.")
            .after_help("All documentation details can be found at <TODO figure out URL>")
            .arg(Arg::with_name("scope")
                .long("scope")
                .help("Specify the authentication method should be executed in. Each scope requires the user to grant this application permission to use it. If unset, it defaults to the shortest scope url for a particular method.")
                .multiple(true)
                .takes_value(true))
            .arg(Arg::with_name("folder")
                .long("config-dir")
                .help("A directory into which we will store our persistent data. Defaults to a user-writable directory that we will create during the first invocation." )
                .multiple(false)
                .takes_value(true))
            .arg(Arg::with_name("debug")
                .long("debug")
                .help("Provide more output to aid with debugging")
                .multiple(false)
                .takes_value(false));
        let mut projects0 = SubCommand::with_name("projects")
            .setting(AppSettings::ColoredHelp)
            .about("sub-resources: service_accounts");
        let mut service_accounts1 = SubCommand::with_name("service_accounts")
            .setting(AppSettings::ColoredHelp)
            .about("methods: generate_access_token, generate_id_token, sign_blob and sign_jwt");
        {
            let mcmd = SubCommand::with_name("generate_access_token")
                .about("Generates an OAuth 2.0 access token for a service account.");
            service_accounts1 = service_accounts1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("generate_id_token")
                .about("Generates an OpenID Connect ID token for a service account.");
            service_accounts1 = service_accounts1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("sign_blob")
                .about("Signs a blob using a service account\'s system-managed private key.");
            service_accounts1 = service_accounts1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("sign_jwt")
                .about("Signs a JWT using a service account\'s system-managed private key.");
            service_accounts1 = service_accounts1.subcommand(mcmd);
        }
        projects0 = projects0.subcommand(service_accounts1);
        app = app.subcommand(projects0);

        Self { app }
    }
}
use google_iamcredentials1 as api;

fn main() {
    // TODO: set homedir afterwards, once the address is unmovable, or use Pin for the very first time
    // to allow a self-referential structure :D!
    let _home_dir = dirs::config_dir()
        .expect("configuration directory can be obtained")
        .join("google-service-cli");
    let outer = Outer::default_boxed();
    let app = outer.inner.app;
    let _matches = app.get_matches();
}
