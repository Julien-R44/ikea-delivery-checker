# ikea-delivery-checker

Basic script that verify if one or more products are available for home delivery in one or multiple zipcodes by calling the IKEA API. 

A notification can be sent on discord when the availability of a product for delivery changes. A [file database](https://github.com/TheNeikos/rustbreak) is used to send the notification only when the availability status changes.

I did this script mainly to discover Rust language. As a result the code must surely be ugly or badly done.

## Usage

```
USAGE:
    ikea-delivery-checker --products-ids <PRODUCT_ID(S)> --token <TOKEN> --zipcodes <ZIPCODE(s)>

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
    -i, --products-ids <PRODUCT_ID(S)>    The ids of product you want to check, comma-separated
    -t, --token <TOKEN>                   Token ( x-client-id )
    -z, --zipcodes <ZIPCODE(s)>           Home delivery zipcodes, comma-separated

EXAMPLE: 
ikea-delivery-checker --products-ids=1234,5678 --token=12345678 --zipcodes=12345
```