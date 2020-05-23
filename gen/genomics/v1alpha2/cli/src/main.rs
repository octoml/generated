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
        let mut app = App::new("genomics1_alpha2")
            .setting(clap::AppSettings::ColoredHelp)
            .author("Sebastian Thiel <byronimo@gmail.com>")
            .version("0.1.0-20200519")
            .about("Uploads, processes, queries, and searches Genomics data in the cloud.")
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
            .about("methods: cancel, get and list");
        {
            let mcmd = SubCommand::with_name("cancel").about("Starts asynchronous cancellation on a long-running operation.\nThe server makes a best effort to cancel the operation, but success is not\nguaranteed. Clients may use Operations.GetOperation\nor Operations.ListOperations\nto check whether the cancellation succeeded or the operation completed\ndespite cancellation.\nAuthorization requires the following [Google IAM](https://cloud.google.com/iam) permission&#58;\n\n* `genomics.operations.cancel`");
            operations0 = operations0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("get").about("Gets the latest state of a long-running operation.\nClients can use this method to poll the operation result at intervals as\nrecommended by the API service.\nAuthorization requires the following [Google IAM](https://cloud.google.com/iam) permission&#58;\n\n* `genomics.operations.get`");
            operations0 = operations0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("list").about("Lists operations that match the specified filter in the request.\nAuthorization requires the following [Google IAM](https://cloud.google.com/iam) permission&#58;\n\n* `genomics.operations.list`");
            operations0 = operations0.subcommand(mcmd);
        }
        let mut pipelines0 = SubCommand::with_name("pipelines")
                        .setting(AppSettings::ColoredHelp)
                        .about("methods: create, delete, get, get_controller_config, list, run and set_operation_status");
        {
            let mcmd = SubCommand::with_name("create").about("Creates a pipeline that can be run later. Create takes a Pipeline that\nhas all fields other than `pipelineId` populated, and then returns\nthe same pipeline with `pipelineId` populated. This id can be used\nto run the pipeline.\n\nCaller must have WRITE permission to the project.");
            pipelines0 = pipelines0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("delete").about("Deletes a pipeline based on ID.\n\nCaller must have WRITE permission to the project.");
            pipelines0 = pipelines0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("get").about("Retrieves a pipeline based on ID.\n\nCaller must have READ permission to the project.");
            pipelines0 = pipelines0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("get_controller_config").about("Gets controller configuration information. Should only be called\nby VMs created by the Pipelines Service and not by end users.");
            pipelines0 = pipelines0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("list")
                .about("Lists pipelines.\n\nCaller must have READ permission to the project.");
            pipelines0 = pipelines0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("run").about("Runs a pipeline. If `pipelineId` is specified in the request, then\nrun a saved pipeline. If `ephemeralPipeline` is specified, then run\nthat pipeline once without saving a copy.\n\nThe caller must have READ permission to the project where the pipeline\nis stored and WRITE permission to the project where the pipeline will be\nrun, as VMs will be created and storage will be used.\n\nIf a pipeline operation is still running after 6 days, it will be canceled.");
            pipelines0 = pipelines0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("set_operation_status").about("Sets status of a given operation. Any new timestamps (as determined by\ndescription) are appended to TimestampEvents. Should only be called by VMs\ncreated by the Pipelines Service and not by end users.");
            pipelines0 = pipelines0.subcommand(mcmd);
        }
        app = app.subcommand(pipelines0);
        app = app.subcommand(operations0);

        Self { app }
    }
}
use google_genomics1_alpha2 as api;

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
