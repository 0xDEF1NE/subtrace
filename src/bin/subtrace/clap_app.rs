use clap::{Arg, ArgAction, ColorChoice, Command};

pub fn build_app(interactive_output: bool) -> Command {
    let color_when = if interactive_output && !std::env::var("NO_COLOR").is_ok() {
        ColorChoice::Auto
    } else {
        ColorChoice::Never
    };

    Command::new("subtrace")
        .version("1.0")
        .color(color_when)
        .args_conflicts_with_subcommands(true)
        .allow_external_subcommands(true)
        .disable_help_subcommand(true)
        .disable_version_flag(true)
        .disable_help_flag(true)
        .max_term_width(100)
        .about("Subdomain scanner tool")
        .arg(
            Arg::new("domain")
                .short('d')
                .long("domain")
                .value_name("DOMAIN")
                .help("Target URL/host to scan")
                .long_help("Specify the main domain to search for subdomains.")
                .num_args(1)
                .required(true)
                .help_heading("TARGET"),
        )
        .arg(
            Arg::new("templates")
                .short('t')
                .long("templates")
                .required(true)
                .value_name("TEMPLATE_DIRECTORY")
                .help("List of template or template directory to run")
                .long_help("Specify the directory with templates for subdomain scanning.")
                .num_args(1)
                .help_heading("TEMPLATE"),
        )
        .arg(
            Arg::new("listtemplates")
            .long("list-templates")
            .value_name("List Templates")
            .help("List all templates.")
            .required(false)
            .action(ArgAction::SetTrue)
            .requires("templates")
            .help_heading("TEMPLATE"),
        )
        .arg(
            Arg::new("output")
                .short('o')
                .long("output")
                .value_name("OUTPUT_FILE")
                .help("Output to the given filename")
                .long_help("Specify the filename to write the output to.")
                .num_args(1)
                .help_heading("OUTPUT"),
        )
        .arg(
            Arg::new("silent")
                .long("silent")
                .help("Display findings only")
                .long_help("Suppress verbose output and display only the findings.")
                .required(false)
                .action(ArgAction::SetTrue)
                .help_heading("OUTPUT"),
        )
        .arg(
            Arg::new("concurrency")
                .short('c')
                .long("concurrency")
                .hide_default_value(true)
                .value_name("CONCURRENCY")
                .default_value("12")
                .value_parser(clap::value_parser!(i32))
                .help("Maximum number of templates to be executed in parallel (Default:12).")
                .long_help("Set the maximum number of templates to be executed in parallel (Default:12).")
                .num_args(1)
                .help_heading("OPTIMIZATIONS"),
        )
        .arg(
            Arg::new("debug")
                .long("debug")
                .value_name("DEBUG")
                .hide_default_value(true)
                .default_value("3")
                .help("Display errors and warnings. The default level is 0.")
                .long_help("Set the debug level: 0 = ERROR, 1 = WARN, 2 = INFO, 3 = DEBUG")
                .num_args(1)
                .help_heading("DEBUG"),
        )
        .arg(
            Arg::new("help")
                .short('h')
                .long("help")
                .help("Show this help message and exit.")
                .action(ArgAction::Help)
                .help_heading("OPTIONS"),
        )
}
