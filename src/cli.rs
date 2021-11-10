use clap::{App, Arg, ArgMatches};

pub fn parse_arguments() -> ArgMatches<'static> {
    return App::new("ikea-delivery-checker")
        .version("0.1.0")
        .arg(
            Arg::with_name("product-ids")
                .required(true)
                .short("i")
                .long("products-ids")
                .value_name("PRODUCT_ID(S)")
                .help("The id of product you want to check")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("zipcodes")
                .required(true)
                .short("z")
                .long("zipcodes")
                .value_name("ZIPCODE(s)")
                .help("Home delivery zipcode"),
        )
        .arg(
            Arg::with_name("token")
                .required(true)
                .short("t")
                .long("token")
                .value_name("TOKEN")
                .help("Token ( x-client-id )"),
        )
        .get_matches();
}