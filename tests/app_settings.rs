extern crate clap;
extern crate regex;

use clap::{App, Arg, SubCommand, AppSettings, ErrorKind};

include!("../clap-test.rs");

static GLOBAL_VERSION: &'static str = "clap-test-sub1 v1.1";

static DONT_COLLAPSE_ARGS: &'static str = "clap-test v1.4.8

USAGE:
    clap-test [arg1] [arg2] [arg3]

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

ARGS:
    <arg1>    some
    <arg2>    some
    <arg3>    some";

static REQUIRE_EQUALS: &'static str = "clap-test v1.4.8

USAGE:
    clap-test --opt=<FILE>

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
    -o, --opt=<FILE>    some";

static UNIFIED_HELP: &'static str = "test 1.3
Kevin K.
tests stuff

USAGE:
    test [OPTIONS] [arg1]

OPTIONS:
    -f, --flag            some flag
    -h, --help            Prints help information
        --option <opt>    some option
    -V, --version         Prints version information

ARGS:
    <arg1>    some pos arg";

static SKIP_POS_VALS: &'static str = "test 1.3
Kevin K.
tests stuff

USAGE:
    test [OPTIONS] [arg1]

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
    -o, --opt <opt>    some option

ARGS:
    <arg1>    some pos arg";

#[test]
fn sub_command_negate_required() {
    let res = App::new("sub_command_negate")
        .setting(AppSettings::SubcommandsNegateReqs)
        .arg(Arg::new("test").required(true).index(1))
        .subcommand(App::new("sub1"))
        .get_matches_from_safe(vec!["myprog", "sub1"]);

    assert!(res.is_ok());
}

#[test]
fn global_version() {
    let app = App::new("global_version")
        .setting(AppSettings::GlobalVersion)
        .version("1.1")
        .subcommand(App::new("sub1"));
    assert!(test::compare_output(app, "test sub1 --version", GLOBAL_VERSION, false));
}

#[test]
fn sub_command_negate_required_2() {
    let result = App::new("sub_command_negate")
        .setting(AppSettings::SubcommandsNegateReqs)
        .arg(Arg::new("test").required(true).index(1))
        .subcommand(App::new("sub1"))
        .get_matches_from_safe(vec![""]);
    assert!(result.is_err());
    let err = result.err().unwrap();
    assert_eq!(err.kind, ErrorKind::MissingRequiredArgument);
}

#[test]
fn sub_command_required() {
    let result = App::new("sc_required")
        .setting(AppSettings::SubcommandRequired)
        .subcommand(App::new("sub1"))
        .get_matches_from_safe(vec![""]);
    assert!(result.is_err());
    let err = result.err().unwrap();
    assert_eq!(err.kind, ErrorKind::MissingSubcommand);
}

#[test]
fn arg_required_else_help() {
    let result = App::new("arg_required")
        .setting(AppSettings::ArgRequiredElseHelp)
        .arg(Arg::new("test").index(1))
        .get_matches_from_safe(vec![""]);
    assert!(result.is_err());
    let err = result.err().unwrap();
    assert_eq!(err.kind, ErrorKind::MissingArgumentOrSubcommand);
}

#[test]
fn arg_required_else_help_over_reqs() {
    let result = App::new("arg_required")
        .setting(AppSettings::ArgRequiredElseHelp)
        .arg(Arg::new("test").index(1).required(true))
        .get_matches_from_safe(vec![""]);
    assert!(result.is_err());
    let err = result.err().unwrap();
    assert_eq!(err.kind, ErrorKind::MissingArgumentOrSubcommand);
}

#[cfg(not(feature = "suggestions"))]
#[test]
fn infer_subcommands_fail_no_args() {
    let m = App::new("prog")
        .setting(AppSettings::InferSubcommands)
        .subcommand(App::new("test"))
        .subcommand(App::new("temp"))
        .get_matches_from_safe(vec!["prog", "te"]);
    assert!(m.is_err(), "{:#?}", m.unwrap());
    assert_eq!(m.unwrap_err().kind, ErrorKind::UnrecognizedSubcommand);
}

#[cfg(feature = "suggestions")]
#[test]
fn infer_subcommands_fail_no_args() {
    let m = App::new("prog")
        .setting(AppSettings::InferSubcommands)
        .subcommand(App::new("test"))
        .subcommand(App::new("temp"))
        .get_matches_from_safe(vec!["prog", "te"]);
    assert!(m.is_err(), "{:#?}", m.unwrap());
    assert_eq!(m.unwrap_err().kind, ErrorKind::InvalidSubcommand);
}

#[test]
fn infer_subcommands_fail_with_args() {
    let m = App::new("prog")
        .setting(AppSettings::InferSubcommands)
        .arg(Arg::new("some"))
        .subcommand(App::new("test"))
        .subcommand(App::new("temp"))
        .get_matches_from_safe(vec!["prog", "t"]);
    assert!(m.is_ok(), "{:?}", m.unwrap_err().kind);
    assert_eq!(m.unwrap().value_of("some"), Some("t"));
}

#[test]
fn infer_subcommands_fail_with_args2() {
    let m = App::new("prog")
        .setting(AppSettings::InferSubcommands)
        .arg(Arg::new("some"))
        .subcommand(App::new("test"))
        .subcommand(App::new("temp"))
        .get_matches_from_safe(vec!["prog", "te"]);
    assert!(m.is_ok(), "{:?}", m.unwrap_err().kind);
    assert_eq!(m.unwrap().value_of("some"), Some("te"));
}

#[test]
fn infer_subcommands_pass() {
    let m = App::new("prog")
        .setting(AppSettings::InferSubcommands)
        .subcommand(App::new("test"))
        .get_matches_from(vec!["prog", "te"]);
    assert_eq!(m.subcommand_name(), Some("test"));
}

#[test]
fn infer_subcommands_pass_close() {
    let m = App::new("prog")
        .setting(AppSettings::InferSubcommands)
        .subcommand(App::new("test"))
        .subcommand(App::new("temp"))
        .get_matches_from(vec!["prog", "tes"]);
    assert_eq!(m.subcommand_name(), Some("test"));
}

#[cfg(feature = "suggestions")]
#[test]
fn infer_subcommands_fail_suggestions() {
    let m = App::new("prog")
        .setting(AppSettings::InferSubcommands)
        .subcommand(App::new("test"))
        .subcommand(App::new("temp"))
        .get_matches_from_safe(vec!["prog", "temps"]);
    assert!(m.is_err(), "{:#?}", m.unwrap());
    assert_eq!(m.unwrap_err().kind, ErrorKind::InvalidSubcommand);
}

#[cfg(not(feature = "suggestions"))]
#[test]
fn infer_subcommands_fail_suggestions() {
    let m = App::new("prog")
        .setting(AppSettings::InferSubcommands)
        .subcommand(App::new("test"))
        .subcommand(App::new("temp"))
        .get_matches_from_safe(vec!["prog", "temps"]);
    assert!(m.is_err(), "{:#?}", m.unwrap());
    assert_eq!(m.unwrap_err().kind, ErrorKind::UnrecognizedSubcommand);
}

#[test]
fn no_bin_name() {
    let result = App::new("arg_required")
        .setting(AppSettings::NoBinaryName)
        .arg(Arg::new("test").required(true).index(1))
        .get_matches_from_safe(vec!["testing"]);
    assert!(result.is_ok());
    let matches = result.unwrap();
    assert_eq!(matches.value_of("test").unwrap(), "testing");
}

#[test]
fn unified_help() {
    let app = App::new("myTest")
        .name("test")
        .author("Kevin K.")
        .about("tests stuff")
        .version("1.3")
        .setting(AppSettings::UnifiedHelpMessage)
        .args_from_usage("-f, --flag 'some flag'
                          [arg1] 'some pos arg'
                          --option [opt] 'some option'");

    assert!(test::compare_output(app, "test --help", UNIFIED_HELP, false));
}

#[test]
fn skip_possible_values() {
    let app =
        App::new("test")
            .author("Kevin K.")
            .about("tests stuff")
            .version("1.3")
            .setting(AppSettings::HidePossibleValuesInHelp)
            .args(&[Arg::from("-o, --opt [opt] 'some option'").possible_values(&["one",
                                                                                       "two"]),
                    Arg::from("[arg1] 'some pos arg'").possible_values(&["three", "four"])]);

    assert!(test::compare_output(app, "test --help", SKIP_POS_VALS, false));
}


// @TEST @TODO-v3-release: test via DisableVersion or something
// #[test]
// fn global_setting() {
// }
// fn global_settings() {
// }

#[test]
fn stop_delim_values_only_pos_follows() {
    let r = App::new("onlypos")
        .setting(AppSettings::DontDelimitTrailingValues)
        .args(&[Arg::from("-f [flag] 'some opt'"),
                Arg::from("[arg]... 'some arg'")])
        .get_matches_from_safe(vec!["", "--", "-f", "-g,x"]);
    assert!(r.is_ok());
    let m = r.unwrap();
    assert!(m.is_present("arg"));
    assert!(!m.is_present("f"));
    assert_eq!(m.values_of("arg").unwrap().collect::<Vec<_>>(),
               &["-f", "-g,x"]);
}

#[test]
fn dont_delim_values_trailingvararg() {
    let m = App::new("positional")
        .setting(AppSettings::TrailingVarArg)
        .setting(AppSettings::DontDelimitTrailingValues)
        .arg(Arg::from("[opt]... 'some pos'"))
        .get_matches_from(vec!["", "test", "--foo", "-Wl,-bar"]);
    assert!(m.is_present("opt"));
    assert_eq!(m.values_of("opt").unwrap().collect::<Vec<_>>(),
               &["test", "--foo", "-Wl,-bar"]);
}

#[test]
fn delim_values_only_pos_follows() {
    let r = App::new("onlypos")
        .args(&[Arg::from("-f [flag] 'some opt'"),
                Arg::from("[arg]... 'some arg'")])
        .get_matches_from_safe(vec!["", "--", "-f", "-g,x"]);
    assert!(r.is_ok());
    let m = r.unwrap();
    assert!(m.is_present("arg"));
    assert!(!m.is_present("f"));
    assert_eq!(m.values_of("arg").unwrap().collect::<Vec<_>>(),
               &["-f", "-g,x"]);
}

#[test]
fn delim_values_trailingvararg() {
    let m = App::new("positional")
        .setting(AppSettings::TrailingVarArg)
        .arg(Arg::from("[opt]... 'some pos'"))
        .get_matches_from(vec!["", "test", "--foo", "-Wl,-bar"]);
    assert!(m.is_present("opt"));
    assert_eq!(m.values_of("opt").unwrap().collect::<Vec<_>>(),
               &["test", "--foo", "-Wl,-bar"]);
}

#[test]
fn delim_values_only_pos_follows_with_delim() {
    let r = App::new("onlypos")
        .args(&[Arg::from("-f [flag] 'some opt'"),
                Arg::from("[arg]... 'some arg'").use_delimiter(true)])
        .get_matches_from_safe(vec!["", "--", "-f", "-g,x"]);
    assert!(r.is_ok());
    let m = r.unwrap();
    assert!(m.is_present("arg"));
    assert!(!m.is_present("f"));
    assert_eq!(m.values_of("arg").unwrap().collect::<Vec<_>>(),
               &["-f", "-g", "x"]);
}

#[test]
fn delim_values_trailingvararg_with_delim() {
    let m = App::new("positional")
        .setting(AppSettings::TrailingVarArg)
        .arg(Arg::from("[opt]... 'some pos'").use_delimiter(true))
        .get_matches_from(vec!["", "test", "--foo", "-Wl,-bar"]);
    assert!(m.is_present("opt"));
    assert_eq!(m.values_of("opt").unwrap().collect::<Vec<_>>(),
               &["test", "--foo", "-Wl", "-bar"]);
}

#[test]
fn leading_hyphen_short() {
    let res = App::new("leadhy")
        .setting(AppSettings::AllowLeadingHyphen)
        .arg(Arg::new("some"))
        .arg(Arg::new("other").short("o"))
        .get_matches_from_safe(vec!["", "-bar", "-o"]);
    assert!(res.is_ok(), "Error: {:?}", res.unwrap_err().kind);
    let m = res.unwrap();
    assert!(m.is_present("some"));
    assert!(m.is_present("other"));
    assert_eq!(m.value_of("some").unwrap(), "-bar");
}

#[test]
fn leading_hyphen_long() {
    let res = App::new("leadhy")
        .setting(AppSettings::AllowLeadingHyphen)
        .arg(Arg::new("some"))
        .arg(Arg::new("other").short("o"))
        .get_matches_from_safe(vec!["", "--bar", "-o"]);
    assert!(res.is_ok(), "Error: {:?}", res.unwrap_err().kind);
    let m = res.unwrap();
    assert!(m.is_present("some"));
    assert!(m.is_present("other"));
    assert_eq!(m.value_of("some").unwrap(), "--bar");
}

#[test]
fn leading_hyphen_opt() {
    let res = App::new("leadhy")
        .setting(AppSettings::AllowLeadingHyphen)
        .arg(Arg::new("some").takes_value(true).long("opt"))
        .arg(Arg::new("other").short("o"))
        .get_matches_from_safe(vec!["", "--opt", "--bar", "-o"]);
    assert!(res.is_ok(), "Error: {:?}", res.unwrap_err().kind);
    let m = res.unwrap();
    assert!(m.is_present("some"));
    assert!(m.is_present("other"));
    assert_eq!(m.value_of("some").unwrap(), "--bar");
}

#[test]
fn allow_negative_numbers() {
    let res = App::new("negnum")
        .setting(AppSettings::AllowNegativeNumbers)
        .arg(Arg::new("panum"))
        .arg(Arg::new("onum").short("o").takes_value(true))
        .get_matches_from_safe(vec!["negnum", "-20", "-o", "-1.2"]);
    assert!(res.is_ok(), "Error: {:?}", res.unwrap_err().kind);
    let m = res.unwrap();
    assert_eq!(m.value_of("panum").unwrap(), "-20");
    assert_eq!(m.value_of("onum").unwrap(), "-1.2");
}

#[test]
fn allow_negative_numbers_fail() {
    let res = App::new("negnum")
        .setting(AppSettings::AllowNegativeNumbers)
        .arg(Arg::new("panum"))
        .arg(Arg::new("onum").short("o").takes_value(true))
        .get_matches_from_safe(vec!["negnum", "--foo", "-o", "-1.2"]);
    assert!(res.is_err());
    assert_eq!(res.unwrap_err().kind, ErrorKind::UnknownArgument)
}

#[test]
fn leading_double_hyphen_trailingvararg() {
    let m = App::new("positional")
        .setting(AppSettings::TrailingVarArg)
        .setting(AppSettings::AllowLeadingHyphen)
        .arg(Arg::from("[opt]... 'some pos'"))
        .get_matches_from(vec!["", "--foo", "-Wl", "bar"]);
    assert!(m.is_present("opt"));
    assert_eq!(m.values_of("opt").unwrap().collect::<Vec<_>>(),
               &["--foo", "-Wl", "bar"]);
}

#[test]
fn disable_help_subcommand() {
    let result = App::new("disablehelp")
        .setting(AppSettings::DisableHelpSubcommand)
        .subcommand(App::new("sub1"))
        .get_matches_from_safe(vec!["", "help"]);
    assert!(result.is_err());
    let err = result.err().unwrap();
    assert_eq!(err.kind, ErrorKind::UnknownArgument);
}

#[test]
fn dont_collapse_args() {
    let app = App::new("clap-test")
        .version("v1.4.8")
        .setting(AppSettings::DontCollapseArgsInUsage)
        .args(&[Arg::new("arg1").help("some"),
                Arg::new("arg2").help("some"),
                Arg::new("arg3").help("some")]);
    assert!(test::compare_output(app, "clap-test --help", DONT_COLLAPSE_ARGS, false));
}

#[test]
fn require_eq() {
    let app = App::new("clap-test")
        .version("v1.4.8")
        .arg(Arg::new("opt")
                 .long("opt")
                 .short("o")
                 .required(true)
                 .require_equals(true)
                 .value_name("FILE")
                 .help("some"));
    assert!(test::compare_output(app, "clap-test --help", REQUIRE_EQUALS, false));
}

#[test]
fn args_negate_subcommands_one_level() {
    let res = App::new("disablehelp")
        .setting(AppSettings::ArgsNegateSubcommands)
        .setting(AppSettings::SubcommandsNegateReqs)
        .arg_from_usage("<arg1> 'some arg'")
        .arg_from_usage("<arg2> 'some arg'")
        .subcommand(App::new("sub1")
                        .subcommand(App::new("sub2")
                                        .subcommand(App::new("sub3"))))
        .get_matches_from_safe(vec!["", "pickles", "sub1"]);
    assert!(res.is_ok(), "error: {:?}", res.unwrap_err().kind);
    let m = res.unwrap();
    assert_eq!(m.value_of("arg2"), Some("sub1"));
}

#[test]
fn args_negate_subcommands_two_levels() {
    let res = App::new("disablehelp")
        .global_setting(AppSettings::ArgsNegateSubcommands)
        .global_setting(AppSettings::SubcommandsNegateReqs)
        .arg_from_usage("<arg1> 'some arg'")
        .arg_from_usage("<arg2> 'some arg'")
        .subcommand(App::new("sub1")
                        .arg_from_usage("<arg> 'some'")
                        .arg_from_usage("<arg2> 'some'")
                        .subcommand(App::new("sub2")
                                        .subcommand(App::new("sub3"))))
        .get_matches_from_safe(vec!["", "sub1", "arg", "sub2"]);
    assert!(res.is_ok(), "error: {:?}", res.unwrap_err().kind);
    let m = res.unwrap();
    assert_eq!(m.subcommand_matches("sub1").unwrap().value_of("arg2"),
               Some("sub2"));
}


#[test]
fn propagate_vals_down() {
    let m = App::new("myprog")
        .setting(AppSettings::PropagateGlobalValuesDown)
        .arg(Arg::from("[cmd] 'command to run'").global(true))
        .subcommand(App::new("foo"))
        .get_matches_from_safe(vec!["myprog", "set", "foo"]);
    assert!(m.is_ok(), "{:?}", m.unwrap_err().kind);
    let m = m.unwrap();
    assert_eq!(m.value_of("cmd"), Some("set"));
    let sub_m = m.subcommand_matches("foo").unwrap();
    assert_eq!(sub_m.value_of("cmd"), Some("set"));
}

#[test]
fn allow_missing_positional() {
    let m = App::new("test")
        .setting(AppSettings::AllowMissingPositional)
        .arg(Arg::from("[src] 'some file'").default_value("src"))
        .arg_from_usage("<dest> 'some file'")
        .get_matches_from_safe(vec!["test", "file"]);
    assert!(m.is_ok(), "{:?}", m.unwrap_err().kind);
    let m = m.unwrap();
    assert_eq!(m.value_of("src"), Some("src"));
    assert_eq!(m.value_of("dest"), Some("file"));
}
