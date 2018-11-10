extern crate cursive;
extern crate drs_0x01;
extern crate serialport;
#[macro_use]
extern crate clap;

mod ui;

use crate::ui::init;

use clap::{Arg, App};

fn main() {


    let mut app = App::new("Herkulex Manager")
        .version(crate_version!())
        .author("Paul Florence <perso@florencepaul.com>")
        .about("Communicate with drs0101 and drs0201 servomotors from command line.")
        .arg(Arg::with_name("gui")
            .short("g")
            .long("gui")
            .help("Start the application in graphical mode"))
        .arg(Arg::with_name("id")
            .short("i")
            .long("id")
            .takes_value(true)
            .help("Initialize the ID of the servomotors")
            .use_delimiter(false)
            .validator(|s| match s.parse::<u8>() {
                Ok(_) => Ok(()),
                Err(e) => Err(format!("{}",e))

            })
        );

    let matches = app.clone().get_matches();

    match matches.value_of("gui") {
        Some(_) => {
            let mut ui = init();
            ui.run()
        }
        None => {
            app.print_help();
        }
    }
    //ui.run();
}
