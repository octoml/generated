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
        let mut app = App::new("gamesManagement1_management")
            .setting(clap::AppSettings::ColoredHelp)
            .author("Sebastian Thiel <byronimo@gmail.com>")
            .version("0.1.0-20200519")
            .about("The Google Play Game Management API allows developers to manage resources from the Google\n     Play Game service.")
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
        let mut achievements0 = SubCommand::with_name("achievements")
                        .setting(AppSettings::ColoredHelp)
                        .about("methods: reset, reset_all, reset_all_for_all_players, reset_for_all_players and reset_multiple_for_all_players");
        {
            let mcmd = SubCommand::with_name("reset").about("Resets the achievement with the given ID for the currently authenticated\nplayer. This method is only accessible to whitelisted tester accounts for\nyour application.");
            achievements0 = achievements0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("reset_all").about("Resets all achievements for the currently authenticated player for your\napplication. This method is only accessible to whitelisted tester accounts\nfor your application.");
            achievements0 = achievements0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("reset_all_for_all_players").about("Resets all draft achievements for all players. This method is only\navailable to user accounts for your developer console.");
            achievements0 = achievements0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("reset_for_all_players").about("Resets the achievement with the given ID for all players. This method is\nonly available to user accounts for your developer console. Only draft\nachievements can be reset.");
            achievements0 = achievements0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("reset_multiple_for_all_players").about("Resets achievements with the given IDs for all players. This method is only\navailable to user accounts for your developer console. Only draft\nachievements may be reset.");
            achievements0 = achievements0.subcommand(mcmd);
        }
        let mut applications0 = SubCommand::with_name("applications")
            .setting(AppSettings::ColoredHelp)
            .about("methods: list_hidden");
        {
            let mcmd = SubCommand::with_name("list_hidden").about("Get the list of players hidden from the given application. This method is\nonly available to user accounts for your developer console.");
            applications0 = applications0.subcommand(mcmd);
        }
        let mut events0 = SubCommand::with_name("events")
                        .setting(AppSettings::ColoredHelp)
                        .about("methods: reset, reset_all, reset_all_for_all_players, reset_for_all_players and reset_multiple_for_all_players");
        {
            let mcmd = SubCommand::with_name("reset").about("Resets all player progress on the event with the given ID for the currently\nauthenticated player. This method is only accessible to whitelisted tester\naccounts for your application.");
            events0 = events0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("reset_all").about("Resets all player progress on all events for the currently authenticated\nplayer. This method is only accessible to whitelisted tester accounts for\nyour application.");
            events0 = events0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("reset_all_for_all_players").about("Resets all draft events for all players. This method is only available to\nuser accounts for your developer console.");
            events0 = events0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("reset_for_all_players").about("Resets the event with the given ID for all players. This method is only\navailable to user accounts for your developer console. Only draft events\ncan be reset.");
            events0 = events0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("reset_multiple_for_all_players").about("Resets events with the given IDs for all players. This method is only\navailable to user accounts for your developer console. Only draft events\nmay be reset.");
            events0 = events0.subcommand(mcmd);
        }
        let mut players0 = SubCommand::with_name("players")
            .setting(AppSettings::ColoredHelp)
            .about("methods: hide and unhide");
        {
            let mcmd = SubCommand::with_name("hide").about("Hide the given player\'s leaderboard scores from the given application. This\nmethod is only available to user accounts for your developer console.");
            players0 = players0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("unhide").about("Unhide the given player\'s leaderboard scores from the given application.\nThis method is only available to user accounts for your developer console.");
            players0 = players0.subcommand(mcmd);
        }
        let mut scores0 = SubCommand::with_name("scores")
                        .setting(AppSettings::ColoredHelp)
                        .about("methods: reset, reset_all, reset_all_for_all_players, reset_for_all_players and reset_multiple_for_all_players");
        {
            let mcmd = SubCommand::with_name("reset").about("Resets scores for the leaderboard with the given ID for the currently\nauthenticated player. This method is only accessible to whitelisted tester\naccounts for your application.");
            scores0 = scores0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("reset_all").about("Resets all scores for all leaderboards for the currently authenticated\nplayers. This method is only accessible to whitelisted tester accounts for\nyour application.");
            scores0 = scores0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("reset_all_for_all_players").about("Resets scores for all draft leaderboards for all players. This method is\nonly available to user accounts for your developer console.");
            scores0 = scores0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("reset_for_all_players").about("Resets scores for the leaderboard with the given ID for all players. This\nmethod is only available to user accounts for your developer console. Only\ndraft leaderboards can be reset.");
            scores0 = scores0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("reset_multiple_for_all_players").about("Resets scores for the leaderboards with the given IDs for all players. This\nmethod is only available to user accounts for your developer console. Only\ndraft leaderboards may be reset.");
            scores0 = scores0.subcommand(mcmd);
        }
        app = app.subcommand(scores0);
        app = app.subcommand(players0);
        app = app.subcommand(events0);
        app = app.subcommand(applications0);
        app = app.subcommand(achievements0);

        Self { app }
    }
}
use google_gamesManagement1_management as api;

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
