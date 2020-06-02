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
        let mut app = App::new("docs1")
            .setting(clap::AppSettings::ColoredHelp)
            .author("Sebastian Thiel <byronimo@gmail.com>")
            .version("0.1.0-20200527")
            .about("Reads and writes Google Docs documents.")
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
        let mut documents0 = SubCommand::with_name("documents")
            .setting(AppSettings::ColoredHelp)
            .about("methods: batch_update, create and get");
        {
            let mcmd = SubCommand::with_name("batch_update").about("Applies one or more updates to the document.\n\nEach request is validated before\nbeing applied. If any request is not valid, then the entire request will\nfail and nothing will be applied.\n\nSome requests have replies to\ngive you some information about how they are applied. Other requests do\nnot need to return information; these each return an empty reply.\nThe order of replies matches that of the requests.\n\nFor example, suppose you call batchUpdate with four updates, and only the\nthird one returns information. The response would have two empty replies,\nthe reply to the third request, and another empty reply, in that order.\n\nBecause other users may be editing the document, the document\nmight not exactly reflect your changes: your changes may\nbe altered with respect to collaborator changes. If there are no\ncollaborators, the document should reflect your changes. In any case,\nthe updates in your request are guaranteed to be applied together\natomically.");
            documents0 = documents0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("create").about("Creates a blank document using the title given in the request. Other fields\nin the request, including any provided content, are ignored.\n\nReturns the created document.");
            documents0 = documents0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("get")
                .about("Gets the latest version of the specified document.");
            documents0 = documents0.subcommand(mcmd);
        }
        app = app.subcommand(documents0);

        Self { app }
    }
}
use google_docs1 as api;

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
