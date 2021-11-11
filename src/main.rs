mod delivery_checker;
use delivery_checker::check_if_product_is_home_deliverable;
mod cli;
use cli::parse_arguments;
mod utils;
use utils::send_discord_message;
mod file_database;
use file_database::FileDatabase;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let matches = parse_arguments();

    let db = FileDatabase::new(None).unwrap();

    let product_ids = matches.value_of("product-ids").unwrap().split(",");
    let token = matches.value_of("token").unwrap();
    let webhook_url = matches.value_of("discord-webhook").unwrap();

    for product_id in product_ids {
        println!("Checking product {}...", product_id);
        let zipcodes = matches.value_of("zipcodes").unwrap().split(",");

        for zipcode in zipcodes {
            let is_available =
                check_if_product_is_home_deliverable(product_id, zipcode, token).await?;

            let output = match is_available {
                true => "is now available",
                false => "is not anymore available",
            };

            let product_db_key = format!("{}-{}", product_id, zipcode);
            let previous_product_availability = db.get_key_value(&product_db_key).unwrap();

            let message = format!(
                "Product {} {} for home delivery in {}",
                product_id, output, zipcode
            );

            println!("{}", message);

            if previous_product_availability != is_available {
                send_discord_message(
                    &message,
                    match is_available {
                        true => 0x00FF00,
                        false => 0xFF0000,
                    },
                    webhook_url,
                )
                .await?;
            }

            db.save_key_value(&product_db_key, is_available)?;
        }
    }

    Ok(())
}
