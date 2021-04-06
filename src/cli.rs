use clap::{App, Arg, ArgGroup, SubCommand};

pub fn build_cli<'a>() -> App<'a, 'a> {
    App::new("repolocli")
        .version("0.1")
        .author("Matthias Beyer <mail@beyermatthias.de>")
        .about("Query repology.org and postprocess its output")

        .arg(Arg::with_name("config")
            .long("config")
            .value_name("PATH")
            .required(false)
            .multiple(false)
            .takes_value(true)
            .help("Override default configuration file path")

        )

        .arg(Arg::with_name("verbose")
                 .long("verbose")
                 .short("v")
                 .required(false)
                 .multiple(true)
                 .takes_value(false)
                 .help("Increase verbosity. Default = Info, -v = Debug, -vv = Trace")
        )

        .arg(Arg::with_name("quiet")
            .long("quiet")
            .short("q")
            .required(false)
            .multiple(true)
            .takes_value(false)
            .help("Decrease verbosity. Default = Info, -q = Warn, -qq = Error")
        )

        .arg(Arg::with_name("output")
            .long("output")
            .short("o")
            .required(false)
            .multiple(false)
            .takes_value(true)
            .possible_values(&["table", "json", "lines"])
            .default_value("lines")
            .help("Output format")
        )

        .arg(Arg::with_name("input_stdin")
            .long("stdin")
            .short("I")
            .required(false)
            .multiple(false)
            .takes_value(false)
            .help("Read data (JSON) from stdin.")
        )

        .subcommand(SubCommand::with_name("project")
            .arg(Arg::with_name("project_name")
                .index(1)
                .required(false) // TODO: Make required, is not required currently when --stdin is passed.
                .multiple(false)
                .takes_value(true)
                .help("Query data about a project")
            )

            .arg(Arg::with_name("sort-version")
                .long("sort-version")
                .required(false)
                .multiple(false)
                .takes_value(false)
                .help("Sort output by version")
                .conflicts_with("sort-repo")
            )
            .arg(Arg::with_name("sort-repo")
                .long("sort-repo")
                .required(false)
                .multiple(false)
                .takes_value(false)
                .help("Sort output by repository")
                .conflicts_with("sort-version")
            )
            .arg(Arg::with_name("latest")
                .long("latest")
                .required(false)
                .multiple(false)
                .takes_value(false)
                .help("Try to find the lastest version (version is string-compared if not used with --semver)")
                .conflicts_with("sort-version")
                .conflicts_with("sort-repo")
            )
            .arg(Arg::with_name("semver")
                .long("semver")
                .required(false)
                .multiple(false)
                .takes_value(false)
                .requires("latest")
                .help("Try to find latest version using semver. If semver could not be parsed, equality is assumed, which might yield bogus results.")
            )
        )

        .subcommand(SubCommand::with_name("problems")
            .arg(Arg::with_name("repo")
                .short("r")
                .long("repo")
                .alias("repository")
                .required(false)
                .multiple(false)
                .takes_value(true)
                .help("The repository to get problems for")
            )

            .arg(Arg::with_name("maintainer")
                .short("m")
                .long("maintainer")
                .alias("maint")
                .required(false)
                .multiple(false)
                .takes_value(true)
                .help("The maintainer to get problems for")
            )

            .group(ArgGroup::with_name("problems-args")
                .args(&["repo", "maintainer"])
                .required(true))


            .arg(Arg::with_name("sort-maintainer")
                .long("sort-maintainer")
                .required(false)
                .multiple(false)
                .takes_value(false)
                .help("Sort output by maintainer")
                .conflicts_with("sort-repo")
            )
            .arg(Arg::with_name("sort-repo")
                .long("sort-repo")
                .required(false)
                .multiple(false)
                .takes_value(false)
                .help("Sort output by repository")
                .conflicts_with("sort-maintainer")
            )
        )

        .subcommand(SubCommand::with_name("compare")
            .about("Compare a list of packages to distro repositories")
            .arg(Arg::with_name("compare-list")
                .index(1)
                .required(true)
                .multiple(false)
                .takes_value(true)
                .value_name("FILE")
                .help("Compare the data from this list to a list of distros out there. Supports JSON and CSV, based on file extension (.json / .csv)"))
            .arg(Arg::with_name("compare-distros")
                .index(2)
                .required(true)
                .multiple(true)
                .takes_value(true)
                .value_name("DIST")
                .help("A list of repology distribution names to compare to"))

            .after_help(r#"
            Compare a list of packages to all supplied repology distributions.
            The list of packages shall have the following format:

            * CSV:
                Header: name;version;comment

            * JSON:
                { "name": "...", "version": "...", "comment": "..." }

            "#)
        )

        .after_help(r#"
        repolocli can read data from stdin, if you want to postprocess repology.org data you already
        fetched from repology.org/api/v1 via curl (or some other method).
        In this case, repolocli is only a easier-to-use 'jq' (if you don't know jq, look it up NOW!).
        "#)

}
