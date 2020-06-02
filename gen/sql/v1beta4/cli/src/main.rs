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
        let mut app = App::new("sql1_beta4")
            .setting(clap::AppSettings::ColoredHelp)
            .author("Sebastian Thiel <byronimo@gmail.com>")
            .version("0.1.0-20200518")
            .about("API for Cloud SQL database instance management")
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
        let mut backup_runs0 = SubCommand::with_name("backup_runs")
            .setting(AppSettings::ColoredHelp)
            .about("methods: delete, get, insert and list");
        {
            let mcmd =
                SubCommand::with_name("delete").about("Deletes the backup taken by a backup run.");
            backup_runs0 = backup_runs0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("get")
                .about("Retrieves a resource containing information about a backup run.");
            backup_runs0 = backup_runs0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("insert").about("Creates a new backup run on demand. This method is applicable only to\nSecond Generation instances.");
            backup_runs0 = backup_runs0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("list").about("Lists all backup runs associated with a given instance and configuration in\nthe reverse chronological order of the backup initiation time.");
            backup_runs0 = backup_runs0.subcommand(mcmd);
        }
        let mut databases0 = SubCommand::with_name("databases")
            .setting(AppSettings::ColoredHelp)
            .about("methods: delete, get, insert, list, patch and update");
        {
            let mcmd = SubCommand::with_name("delete")
                .about("Deletes a database from a Cloud SQL instance.");
            databases0 = databases0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("get").about("Retrieves a resource containing information about a database inside a Cloud\nSQL instance.");
            databases0 = databases0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("insert").about("Inserts a resource containing information about a database inside a Cloud\nSQL instance.");
            databases0 = databases0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("list")
                .about("Lists databases in the specified Cloud SQL instance.");
            databases0 = databases0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("patch").about("Partially updates a resource containing information about a database inside\na Cloud SQL instance. This method supports patch semantics.");
            databases0 = databases0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("update").about("Updates a resource containing information about a database inside a Cloud\nSQL instance.");
            databases0 = databases0.subcommand(mcmd);
        }
        let mut flags0 = SubCommand::with_name("flags")
            .setting(AppSettings::ColoredHelp)
            .about("methods: list");
        {
            let mcmd = SubCommand::with_name("list")
                .about("List all available database flags for Cloud SQL instances.");
            flags0 = flags0.subcommand(mcmd);
        }
        let mut instances0 = SubCommand::with_name("instances")
                        .setting(AppSettings::ColoredHelp)
                        .about("methods: add_server_ca, clone, delete, demote_master, export, failover, get, import, insert, list, list_server_cas, patch, promote_replica, reset_ssl_config, restart, restore_backup, rotate_server_ca, start_replica, stop_replica, truncate_log and update");
        {
            let mcmd = SubCommand::with_name("add_server_ca").about("Add a new trusted Certificate Authority (CA) version for the specified\ninstance. Required to prepare for a certificate rotation. If a CA version\nwas previously added but never used in a certificate rotation, this\noperation replaces that version. There cannot be more than one CA version\nwaiting to be rotated in.");
            instances0 = instances0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("clone").about("Creates a Cloud SQL instance as a clone of the source instance. Using this\noperation might cause your instance to restart.");
            instances0 = instances0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("delete").about("Deletes a Cloud SQL instance.");
            instances0 = instances0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("demote_master").about("Demotes the stand-alone instance to be a Cloud SQL read replica for an\nexternal database server.");
            instances0 = instances0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("export").about("Exports data from a Cloud SQL instance to a Cloud Storage bucket as a SQL\ndump or CSV file.");
            instances0 = instances0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("failover").about("Failover the instance to its failover replica instance. Using this\noperation might cause your instance to restart.");
            instances0 = instances0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("get")
                .about("Retrieves a resource containing information about a Cloud SQL instance.");
            instances0 = instances0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("import").about("Imports data into a Cloud SQL instance from a SQL dump  or CSV file in\nCloud Storage.");
            instances0 = instances0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("insert").about("Creates a new Cloud SQL instance.");
            instances0 = instances0.subcommand(mcmd);
        }
        {
            let mcmd =
                SubCommand::with_name("list").about("Lists instances under a given project.");
            instances0 = instances0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("list_server_cas").about("Lists all of the trusted Certificate Authorities (CAs) for the specified\ninstance. There can be up to three CAs listed: the CA that was used to sign\nthe certificate that is currently in use, a CA that has been added but not\nyet used to sign a certificate, and a CA used to sign a certificate that\nhas previously rotated out.");
            instances0 = instances0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("patch").about(
                "Updates settings of a Cloud SQL instance.\nThis method supports patch semantics.",
            );
            instances0 = instances0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("promote_replica").about("Promotes the read replica instance to be a stand-alone Cloud SQL instance.\nUsing this operation might cause your instance to restart.");
            instances0 = instances0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("reset_ssl_config").about("Deletes all client certificates and generates a new server SSL certificate\nfor the instance.");
            instances0 = instances0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("restart").about("Restarts a Cloud SQL instance.");
            instances0 = instances0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("restore_backup").about("Restores a backup of a Cloud SQL instance. Using this operation might cause\nyour instance to restart.");
            instances0 = instances0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("rotate_server_ca").about("Rotates the server certificate to one signed by the Certificate Authority\n(CA) version previously added with the addServerCA method.");
            instances0 = instances0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("start_replica")
                .about("Starts the replication in the read replica instance.");
            instances0 = instances0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("stop_replica")
                .about("Stops the replication in the read replica instance.");
            instances0 = instances0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("truncate_log")
                .about("Truncate MySQL general and slow query log tables");
            instances0 = instances0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("update").about("Updates settings of a Cloud SQL instance. Using this operation might cause\nyour instance to restart.");
            instances0 = instances0.subcommand(mcmd);
        }
        let mut operations0 = SubCommand::with_name("operations")
            .setting(AppSettings::ColoredHelp)
            .about("methods: get and list");
        {
            let mcmd = SubCommand::with_name("get")
                .about("Retrieves an instance operation that has been performed on an instance.");
            operations0 = operations0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("list").about("Lists all instance operations that have been performed on the given Cloud\nSQL instance in the reverse chronological order of the start time.");
            operations0 = operations0.subcommand(mcmd);
        }
        let mut projects0 = SubCommand::with_name("projects")
            .setting(AppSettings::ColoredHelp)
            .about("sub-resources: instances");
        let mut ssl_certs0 = SubCommand::with_name("ssl_certs")
            .setting(AppSettings::ColoredHelp)
            .about("methods: create_ephemeral, delete, get, insert and list");
        {
            let mcmd = SubCommand::with_name("create_ephemeral").about("Generates a short-lived X509 certificate containing the provided public key\nand signed by a private key specific to the target instance. Users may use\nthe certificate to authenticate as themselves when connecting to the\ndatabase.");
            ssl_certs0 = ssl_certs0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("delete").about("Deletes the SSL certificate. For First Generation instances, the\ncertificate remains valid until the instance is restarted.");
            ssl_certs0 = ssl_certs0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("get").about("Retrieves a particular SSL certificate.  Does not include the private key\n(required for usage).  The private key must be saved from the response to\ninitial creation.");
            ssl_certs0 = ssl_certs0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("insert").about("Creates an SSL certificate and returns it along with the private key and\nserver certificate authority.  The new certificate will not be usable until\nthe instance is restarted.");
            ssl_certs0 = ssl_certs0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("list")
                .about("Lists all of the current SSL certificates for the instance.");
            ssl_certs0 = ssl_certs0.subcommand(mcmd);
        }
        let mut tiers0 = SubCommand::with_name("tiers")
            .setting(AppSettings::ColoredHelp)
            .about("methods: list");
        {
            let mcmd = SubCommand::with_name("list").about("Lists all available machine types (tiers) for Cloud SQL, for example,\ndb-n1-standard-1. For related information, see <a\nhref=\"/sql/pricing\">Pricing</a>.");
            tiers0 = tiers0.subcommand(mcmd);
        }
        let mut users0 = SubCommand::with_name("users")
            .setting(AppSettings::ColoredHelp)
            .about("methods: delete, insert, list and update");
        {
            let mcmd =
                SubCommand::with_name("delete").about("Deletes a user from a Cloud SQL instance.");
            users0 = users0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("insert")
                .about("Creates a new user in a Cloud SQL instance.");
            users0 = users0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("list")
                .about("Lists users in the specified Cloud SQL instance.");
            users0 = users0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("update")
                .about("Updates an existing user in a Cloud SQL instance.");
            users0 = users0.subcommand(mcmd);
        }
        let mut instances1 = SubCommand::with_name("instances")
                        .setting(AppSettings::ColoredHelp)
                        .about("methods: reschedule_maintenance, start_external_sync and verify_external_sync_settings");
        {
            let mcmd = SubCommand::with_name("reschedule_maintenance")
                .about("Reschedules the maintenance on the given instance.");
            instances1 = instances1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("start_external_sync")
                .about("Start External master migration.");
            instances1 = instances1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("verify_external_sync_settings")
                .about("Verify External master external sync settings.");
            instances1 = instances1.subcommand(mcmd);
        }
        projects0 = projects0.subcommand(instances1);
        app = app.subcommand(users0);
        app = app.subcommand(tiers0);
        app = app.subcommand(ssl_certs0);
        app = app.subcommand(projects0);
        app = app.subcommand(operations0);
        app = app.subcommand(instances0);
        app = app.subcommand(flags0);
        app = app.subcommand(databases0);
        app = app.subcommand(backup_runs0);

        Self { app }
    }
}
use google_sql1_beta4 as api;

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
