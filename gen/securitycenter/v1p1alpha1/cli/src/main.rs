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
        let mut app = App::new("securitycenter1_p1alpha1")
            .setting(clap::AppSettings::ColoredHelp)
            .author("Sebastian Thiel <byronimo@gmail.com>")
            .version("0.1.0-20200522")
            .about("Security Command Center API provides access to temporal views of assets and findings within an organization.")
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
        let mut organizations0 = SubCommand::with_name("organizations")
            .setting(AppSettings::ColoredHelp)
            .about("sub-resources: operations");
        let mut operations1 = SubCommand::with_name("operations")
            .setting(AppSettings::ColoredHelp)
            .about("methods: cancel, delete, get and list");
        {
            let mcmd = SubCommand::with_name("cancel").about("Starts asynchronous cancellation on a long-running operation.  The server\nmakes a best effort to cancel the operation, but success is not\nguaranteed.  If the server doesn\'t support this method, it returns\n`google.rpc.Code.UNIMPLEMENTED`.  Clients can use\nOperations.GetOperation or\nother methods to check whether the cancellation succeeded or whether the\noperation completed despite cancellation. On successful cancellation,\nthe operation is not deleted; instead, it becomes an operation with\nan Operation.error value with a google.rpc.Status.code of 1,\ncorresponding to `Code.CANCELLED`.");
            operations1 = operations1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("delete").about("Deletes a long-running operation. This method indicates that the client is\nno longer interested in the operation result. It does not cancel the\noperation. If the server doesn\'t support this method, it returns\n`google.rpc.Code.UNIMPLEMENTED`.");
            operations1 = operations1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("get").about("Gets the latest state of a long-running operation.  Clients can use this\nmethod to poll the operation result at intervals as recommended by the API\nservice.");
            operations1 = operations1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("list").about("Lists operations that match the specified filter in the request. If the\nserver doesn\'t support this method, it returns `UNIMPLEMENTED`.\n\nNOTE: the `name` binding allows API services to override the binding\nto use different resource name schemes, such as `users/*/operations`. To\noverride the binding, API services can add a binding such as\n`\"/v1/{name=users/*}/operations\"` to their service configuration.\nFor backwards compatibility, the default name includes the operations\ncollection id, however overriding users must ensure the name binding\nis the parent resource, without the operations collection id.");
            operations1 = operations1.subcommand(mcmd);
        }
        organizations0 = organizations0.subcommand(operations1);
        app = app.subcommand(organizations0);

        Self { app }
    }
}
use google_securitycenter1_p1alpha1 as api;

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
