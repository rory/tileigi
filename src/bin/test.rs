extern crate slippy_map_tiles;
extern crate clap;

extern crate tilegen;

use std::time::Instant;

use clap::{Arg, App, AppSettings};
use slippy_map_tiles::BBox;

use tilegen::*;

fn fmt_duration(dur: &std::time::Duration) -> String {
    format!("{:.2}s", duration_to_float_secs(dur))
}

fn duration_to_float_secs(dur: &std::time::Duration) -> f64 {
    (dur.as_secs() as f64) + (dur.subsec_nanos() as f64 / 1e9)
}

fn main() {

    let matches = App::new("test")
        .setting(AppSettings::AllowLeadingHyphen)
        .arg(Arg::with_name("data_yml").long("data-yml").takes_value(true).required(true))
        .arg(Arg::with_name("dest_dir").long("dest").takes_value(true).required(true))
        .arg(Arg::with_name("minzoom").long("minzoom").default_value("0"))
        .arg(Arg::with_name("maxzoom").long("maxzoom").default_value("14"))
        .arg(Arg::with_name("bbox").long("bbox").default_value("90,-180,-90,180"))
        .arg(Arg::with_name("if_not_exists").long("if-not-exists"))
        .arg(Arg::with_name("no_compress").long("no-compress"))
        .get_matches();

    let data_yml = matches.value_of("data_yml").unwrap();
    let dest_dir = matches.value_of("dest_dir").unwrap();
    let minzoom: u8 = matches.value_of("minzoom").unwrap().parse().unwrap();
    let maxzoom: u8 = matches.value_of("maxzoom").unwrap().parse().unwrap();
    let if_not_exists = matches.is_present("if_not_exists");
    let compress = ! matches.is_present("no_compress");

    let bbox = BBox::new_from_string(matches.value_of("bbox").expect("bbox not provided")).expect("Invalid bbox");

    let start = Instant::now();

    generate_all(&data_yml, minzoom, maxzoom, &bbox, &dest_dir, if_not_exists, compress);

    println!("Finished in {}", fmt_duration(&start.elapsed()));

}
