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
        let mut app = App::new("pubsub1_beta2")
            .setting(clap::AppSettings::ColoredHelp)
            .author("Sebastian Thiel <byronimo@gmail.com>")
            .version("0.1.0-20200505")
            .about("Provides reliable, many-to-many, asynchronous messaging between applications.\n")
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
            .about("sub-resources: subscriptions and topics");
        let mut subscriptions1 = SubCommand::with_name("subscriptions")
                        .setting(AppSettings::ColoredHelp)
                        .about("methods: acknowledge, create, delete, get, get_iam_policy, list, modify_ack_deadline, modify_push_config, pull, set_iam_policy and test_iam_permissions");
        {
            let mcmd = SubCommand::with_name("acknowledge").about("Acknowledges the messages associated with the `ack_ids` in the\n`AcknowledgeRequest`. The Pub/Sub system can remove the relevant messages\nfrom the subscription.\n\nAcknowledging a message whose ack deadline has expired may succeed,\nbut such a message may be redelivered later. Acknowledging a message more\nthan once will not result in an error.");
            subscriptions1 = subscriptions1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("create").about("Creates a subscription to a given topic.\nIf the subscription already exists, returns `ALREADY_EXISTS`.\nIf the corresponding topic doesn\'t exist, returns `NOT_FOUND`.\n\nIf the name is not provided in the request, the server will assign a random\nname for this subscription on the same project as the topic. Note that\nfor REST API requests, you must specify a name.");
            subscriptions1 = subscriptions1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("delete").about("Deletes an existing subscription. All pending messages in the subscription\nare immediately dropped. Calls to `Pull` after deletion will return\n`NOT_FOUND`. After a subscription is deleted, a new one may be created with\nthe same name, but the new one has no association with the old\nsubscription, or its topic unless the same topic is specified.");
            subscriptions1 = subscriptions1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("get")
                .about("Gets the configuration details of a subscription.");
            subscriptions1 = subscriptions1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("get_iam_policy").about("Gets the access control policy for a resource.\nReturns an empty policy if the resource exists and does not have a policy\nset.");
            subscriptions1 = subscriptions1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("list").about("Lists matching subscriptions.");
            subscriptions1 = subscriptions1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("modify_ack_deadline").about("Modifies the ack deadline for a specific message. This method is useful\nto indicate that more time is needed to process a message by the\nsubscriber, or to make the message available for redelivery if the\nprocessing was interrupted. Note that this does not modify the\nsubscription-level `ackDeadlineSeconds` used for subsequent messages.");
            subscriptions1 = subscriptions1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("modify_push_config").about("Modifies the `PushConfig` for a specified subscription.\n\nThis may be used to change a push subscription to a pull one (signified by\nan empty `PushConfig`) or vice versa, or change the endpoint URL and other\nattributes of a push subscription. Messages will accumulate for delivery\ncontinuously through the call regardless of changes to the `PushConfig`.");
            subscriptions1 = subscriptions1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("pull").about("Pulls messages from the server. Returns an empty list if there are no\nmessages available in the backlog. The server may return `UNAVAILABLE` if\nthere are too many concurrent pull requests pending for the given\nsubscription.");
            subscriptions1 = subscriptions1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("set_iam_policy").about("Sets the access control policy on the specified resource. Replaces any\nexisting policy.\n\nCan return `NOT_FOUND`, `INVALID_ARGUMENT`, and `PERMISSION_DENIED` errors.");
            subscriptions1 = subscriptions1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("test_iam_permissions").about("Returns permissions that a caller has on the specified resource.\nIf the resource does not exist, this will return an empty set of\npermissions, not a `NOT_FOUND` error.\n\nNote: This operation is designed to be used for building permission-aware\nUIs and command-line tools, not for authorization checking. This operation\nmay \"fail open\" without warning.");
            subscriptions1 = subscriptions1.subcommand(mcmd);
        }
        let mut topics1 = SubCommand::with_name("topics")
                        .setting(AppSettings::ColoredHelp)
                        .about("methods: create, delete, get, get_iam_policy, list, publish, set_iam_policy and test_iam_permissions");
        {
            let mcmd = SubCommand::with_name("create")
                .about("Creates the given topic with the given name.");
            topics1 = topics1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("delete").about("Deletes the topic with the given name. Returns `NOT_FOUND` if the topic\ndoes not exist. After a topic is deleted, a new topic may be created with\nthe same name; this is an entirely new topic with none of the old\nconfiguration or subscriptions. Existing subscriptions to this topic are\nnot deleted, but their `topic` field is set to `_deleted-topic_`.");
            topics1 = topics1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("get").about("Gets the configuration of a topic.");
            topics1 = topics1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("get_iam_policy").about("Gets the access control policy for a resource.\nReturns an empty policy if the resource exists and does not have a policy\nset.");
            topics1 = topics1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("list").about("Lists matching topics.");
            topics1 = topics1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("publish").about("Adds one or more messages to the topic. Returns `NOT_FOUND` if the topic\ndoes not exist. The message payload must not be empty; it must contain\n either a non-empty data field, or at least one attribute.");
            topics1 = topics1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("set_iam_policy").about("Sets the access control policy on the specified resource. Replaces any\nexisting policy.\n\nCan return `NOT_FOUND`, `INVALID_ARGUMENT`, and `PERMISSION_DENIED` errors.");
            topics1 = topics1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("test_iam_permissions").about("Returns permissions that a caller has on the specified resource.\nIf the resource does not exist, this will return an empty set of\npermissions, not a `NOT_FOUND` error.\n\nNote: This operation is designed to be used for building permission-aware\nUIs and command-line tools, not for authorization checking. This operation\nmay \"fail open\" without warning.");
            topics1 = topics1.subcommand(mcmd);
        }
        let mut subscriptions2 = SubCommand::with_name("subscriptions")
            .setting(AppSettings::ColoredHelp)
            .about("methods: list");
        {
            let mcmd = SubCommand::with_name("list")
                .about("Lists the name of the subscriptions for this topic.");
            subscriptions2 = subscriptions2.subcommand(mcmd);
        }
        topics1 = topics1.subcommand(subscriptions2);
        projects0 = projects0.subcommand(topics1);
        projects0 = projects0.subcommand(subscriptions1);
        app = app.subcommand(projects0);

        Self { app }
    }
}
use google_pubsub1_beta2 as api;

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
