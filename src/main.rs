use clap::{arg, Command, value_parser};
use colored::*;
use std::process;

mod util;
mod build;
mod clean;
mod pull;
mod deploy;
mod types;
mod debug;


fn run(cli: clap::ArgMatches) -> Result<(), String> {
    // TODO do stuff

    match cli.subcommand() {
        Some(("pull", sub_m)) => pull::run(sub_m),
        Some(("build", sub_m)) => build::run(sub_m),
        Some(("deploy", sub_m)) => deploy::run(sub_m),
        Some(("clean", sub_m)) => clean::run(sub_m),
        Some(("debug", sub_m)) => debug::run(sub_m),
        Some(("", sub_m)) => {
            util::stdout("warning", "Please provide a subcommand. You can call this tool without arguments or with the --help flag for more information.")
        }
        _ => panic!(
            "I before E, except when your foreign neighbor Keith received eight counterfeit beige sleights from feisty caffeinated weightlifters. Weird."
        ),
    }
    // HACK add error handling
    Ok(())
}

fn main() {
    #[cfg(not(feature = "debug"))]
    let debug_command = Command::new("");
    #[cfg(feature = "debug")]
    let debug_command = Command::new("debug")
    .subcommand_required(true)
        .arg_required_else_help(true)
    .about("Runs the debug mode")
    .subcommand(Command::new("table")
        .about("table debug stuff")
        .subcommand_required(true)
        .arg_required_else_help(true)
        .subcommand(Command::new("parse")
            .about("Directly parse tables in the machinegen folder and output the parsed structs to stdout")
        )
        .subcommand(Command::new("process")
            .about("Directly parse tables in the machinegen folder, build the internal representation of the data, and otputs the parsed structs to stdout")
        )
    );

    let matches = Command::new("machinegen")
        .version("0.1.0")
        .author("Agata Ordano - aordano@protonmail.com")
        .about("Utility that prepares, builds and deploys a KVM virtual machine with a certain configuration.")
        .long_about(concat! ("This utility helps with the steps needed to deploy a custom machine under KVM.\n", 
        "It can be used to prepare a machine, build it and deploy it to a KVM host using Terraform.\n\n",
        "Using this can be thought of as in three different parts; the fetching, the processing and the deployment.\n", 
        "The fetching part lets you gather all the runtime dependencies, templates, and configuration from a remote source. ",
        "As this is intended to be used in the host where the machines will be deployed, is important to have a way of gathering ", 
        "all the required stuff from elsewhere.\n", "The processing part uses some of the dependencies, templates, and configuration to build a", 
        "image and metadata that will let you construct a Terraform plan compatible with your needs.\n", "The deployment part is the last step ", 
        "in the process, where this tool will ease the process of invoking Terraform with the right parameters, and launching the guest."))
        .subcommand_required(true)
        .arg_required_else_help(true)
        .subcommand(
            Command::new("pull")
                .alias("fetch")
            .about("Manages fetching dependencies, templates and other files from the internet.")
            .long_about(util::string_to_sstr(format!("{}\n {}", "This command gathers needed files from remote resources.".yellow(), concat!("Configuration data can be categorized ",
            "in two separate groups; the user-provided config data (a single file with specific details to include in the machine deployment ",
            "process), and the machine config data (a folder containing lower-level confguration that defines how the user config will be digested ", 
            "and processed, and specific information about how to perform the deployment).\n", "You can pull either or both of those. ",
            "Check the help on the arguments for more information."))))
            .arg_required_else_help(true)
            .subcommand_required(true)
            .subcommand(Command::new("deps")
                .alias("dependencies")
                .about("Fetches dependencies from the internet.")
                .long_about(concat!("This subcommand fetches dependencies from the internet.\n", "Dependencies can be categorized ",
                "in two separate groups; the runtime dependencies (needed for the subsequent steps to function), and the machine image ", 
                "(needed to build the virtual machine using cloud-init).\n", "You can pull either or both of those. Check the help on the ",
                "arguments for more information."))
                .arg_required_else_help(true)
                .arg(
                    arg!(-f --force "Force the download of dependencies, even if they are already present.")
                        .long_help(concat! ("This will pull dependencies using wget with the -O flag and no continue.", 
                        "It will overwrite any existing file with the same name")))
                .arg(
                    arg!(-a --all "Downloads all dependencies.").exclusive(true))
                .arg(
                    arg!(-i --image "Downloads machine image.").exclusive(true)
                    .long_help(concat! ("This will pull a Ubuntu Jammy Jellyfish (22.04) image from the ubuntu cloud image releases.\n", 
                    "It's a ~600MiB download. In the future (i hope) this flag will allow to fetch an arbitrary image.")))
                .arg(
                    arg!(-r --runtime "Downloads all runtime dependencies.").exclusive(true)
                    .long_help(concat! ("This will pull the runtime dependencies.\n", 
                    "Those include the Terraform binary, cloud-init tools, and some other minor stuff.")))
                )
            .subcommand(Command::new("config")
                .alias("configuration")
                .about("This command pulls user or machine config file from a remote host.")
                .long_about(concat!("This subcommand fetches configuration data from the internet.\n", "Configuration data can be categorized ",
                "in two separate groups; the user-provided config data (a single file with specific details to include in the machine deployment ",
                "process), and the machine config data (a folder containing lower-level confguration that defines how the user config will be digested ", 
                "and processed, and specific information about how to perform the deployment).\n", "You can pull either or both of those. ",
                "Check the help on the arguments for more information."))
                .arg_required_else_help(true)
                .arg(
                    arg!(-f --force "Force the download of config data, even if it is already present.")
                        .long_help(concat! ("This will pull the data using wget with the -O flag and no continue (in case of user config) ",
                        "or will clean git before calling pulling (in case of machine config).\n", 
                        "It will overwrite any existing file with the same name")))
                .arg(
                    arg!(-u --user "Pulls a user config file from a remote source.")
                        .long_help(concat!("Provide a URI from where to fetch the config file. Config files can be JSON, JSONC and JSON5.\n", 
                        "If you don't have a config file, you can generate a commentated skeleton with schema using the --skeleton flag"))
                        .exclusive(true)
                        .takes_value(true)
                        .multiple_values(false)
                        .value_parser(value_parser!(String)))   

                .arg(
                    arg!(-m --machine "Pulls a machine config folder from a remote source.")
                        .long_help(concat!("Provide a URI from where to fetch the config folder. Machine config is defined as a set of ",
                        "tables that contain information about how to process templates, and the template files (both for building the cloud-init)",
                        "image and the required Terraform project.\n", 
                        "This is not intended to be managed by an user. By default, it will pull the default machine config, from the source of ", 
                        "this program.\n", "You can learn more at https://github.com/nodoambiental/machinegen/tree/master/config"))
                        .default_value("https://github.com/nodoambiental/machinegen-config")
                        .exclusive(true)
                        .takes_value(true)
                        .multiple_values(false)
                        .value_parser(value_parser!(String)))
                )
            )
        .subcommand(
            Command::new("build")
                .alias("process")
                .about("Performs the processing of the user config and machine config into a ready-to deploy Terraform project.")
                .long_about(concat!("This subcommand uses the previously fetched configuration data to build both a cloud-init image customized", 
                "according to the configuration and a Terraform project that defines a KVM virtual machine that includes that image and further configuration."))
                .arg_required_else_help(true)
                .arg(
                    arg!(-f --force "Force building the machine configuration, even if files are already present.")
                    .long_help(concat! ("This will clean every generated file before reattempting the build process."))
                )
                .arg(
                    arg!(-c --cloud "Builds the cloud-init image with the specified configuration.")
                        .exclusive(true)
                        .long_help(concat! ("This will use the previously fetched machine and user-provided config to build the cloud-init image."))
                )
                .arg(
                    arg!(-t --terraform "Builds and plans the Terraform project with the specified configuration.")
                        .exclusive(true)
                        .long_help(concat! ("This will use the previously fetched machine, user-provided config and cloud-init image to build ", 
                        "the terraform project, initialize it, and plan it to be ready to deploy."))
                )
                .arg(
                    arg!(-a --all "Builds both the cloud init image and the Terraform project.")
                        .exclusive(true)
                        .long_help(concat! ("This will use the previously fetched machine and user-provided config to do everything needed to ", 
                        "have the Terraform project ready to deploy."))
                )

        )
        .subcommand(
            Command::new("deploy")
                .about("This subcommand deploys a previously processed Terraform project.")
                .long_about(
                    util::string_to_sstr(format!("This takes a successfully built Terraform project and uses {} to deploy it", "terraform apply".italic().green()))) 
                .arg_required_else_help(false)
        )
        .subcommand(
            Command::new("clean")
                .about("This subcommand removes pulled and/or generated files.")
                .long_about(
                    util::string_to_sstr(String::from(format!("This takes a successfully built Terraform project and uses {} to deploy it", "terraform apply".italic())
                    .as_str()))) 
                .arg_required_else_help(true)
                .arg(
                    arg!(-c --config "Clean pulled config files.")
                    .long_help(concat! ("This will clean all config files. To discriminate between user or machine config, use --user or --machine flags."))
                    .exclusive(true)
                    .takes_value(false)
                )
                .arg(
                    arg!(-u --user "Clean pulled user-provided config files.")
                    .exclusive(false)
                    .requires("config")
                    .takes_value(false)
                )
                .arg(
                    arg!(-m --machine "Clean pulled machine config files.")
                    .exclusive(false)
                    .requires("config")
                    .takes_value(false)
                )
                .arg(
                    arg!(-d --deps "Clean pulled dependencies.")
                    .long_help(concat! ("This will clean all dependencies. To discriminate between runtime or image dependencies, use --runtime or --image flags."))
                    .exclusive(true)
                    .takes_value(false)
                )
                .arg(
                    arg!(-i --image "Clean pulled image files.")
                    .exclusive(false)
                    .requires("deps")
                    .takes_value(false)
                )
                .arg(
                    arg!(-r --runtime "Clean pulled image files.")
                    .exclusive(false)
                    .requires("deps")
                    .takes_value(false)
                )
                .arg(
                    arg!(-g --generated "Clean generated files.")
                    .long_help(concat! ("This will clean every generated file in the build process."))
                    .exclusive(true)
                    .takes_value(false)
                )
                .arg(
                    arg!(-a --all "Clean everything; config files, dependencies and generated files.")
                    .exclusive(true)
                    .takes_value(false)
                )
        )
        .subcommand(debug_command)
        .get_matches();
        if let Err(error) = run(matches) {
            println!("Application error: {}", error);
            process::exit(1);
        }

}
