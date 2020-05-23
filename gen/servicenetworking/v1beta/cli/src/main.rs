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
        let mut app = App::new("servicenetworking1_beta")
            .setting(clap::AppSettings::ColoredHelp)
            .author("Sebastian Thiel <byronimo@gmail.com>")
            .version("0.1.0-20200520")
            .about("Provides automatic management of network configurations necessary for certain services.")
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
        let mut operations0 = SubCommand::with_name("operations")
            .setting(AppSettings::ColoredHelp)
            .about("methods: get");
        {
            let mcmd = SubCommand::with_name("get").about("Gets the latest state of a long-running operation.  Clients can use this\nmethod to poll the operation result at intervals as recommended by the API\nservice.");
            operations0 = operations0.subcommand(mcmd);
        }
        let mut services0 = SubCommand::with_name("services")
            .setting(AppSettings::ColoredHelp)
            .about("methods: add_subnetwork, search_range and update_connections");
        {
            let mcmd = SubCommand::with_name("add_subnetwork").about("For service producers, provisions a new subnet in a\npeered service\'s shared VPC network in the requested region and with the\nrequested size that\'s expressed as a CIDR range (number of leading bits of\nipV4 network mask). The method checks against the assigned allocated ranges\nto find a non-conflicting IP address range. The method will reuse a subnet\nif subsequent calls contain the same subnet name, region, and prefix\nlength. This method will make producer\'s tenant project to be a shared VPC\nservice project as needed. The response from the `get` operation will be of\ntype `Subnetwork` if the operation successfully completes.");
            services0 = services0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("search_range").about("Service producers can use this method to find a currently unused range\nwithin consumer allocated ranges.   This returned range is not reserved,\nand not guaranteed to remain unused.\nIt will validate previously provided allocated ranges, find\nnon-conflicting sub-range of requested size (expressed in\nnumber of leading bits of ipv4 network mask, as in CIDR range\nnotation).\nOperation<response: Range>");
            services0 = services0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("update_connections").about("Updates the allocated ranges that are assigned to a connection.\nThe response from the `get` operation will be of type `Connection` if the\noperation successfully completes.");
            services0 = services0.subcommand(mcmd);
        }
        let mut connections1 = SubCommand::with_name("connections")
            .setting(AppSettings::ColoredHelp)
            .about("methods: create and list");
        {
            let mcmd = SubCommand::with_name("create").about("Creates a private connection that establishes a VPC Network Peering\nconnection to a VPC network in the service producer\'s organization.\nThe administrator of the service consumer\'s VPC network invokes this\nmethod. The administrator must assign one or more allocated IP ranges for\nprovisioning subnetworks in the service producer\'s VPC network. This\nconnection is used for all supported services in the service producer\'s\norganization, so it only needs to be invoked once. The response from the\n`get` operation will be of type `Connection` if the operation successfully\ncompletes.");
            connections1 = connections1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("list").about("List the private connections that are configured in a service consumer\'s\nVPC network.");
            connections1 = connections1.subcommand(mcmd);
        }
        services0 = services0.subcommand(connections1);
        app = app.subcommand(services0);
        app = app.subcommand(operations0);

        Self { app }
    }
}
use google_servicenetworking1_beta as api;

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
