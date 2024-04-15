use store::shop;
use store::buy;

fn main() {
    shop::take_item();
    buy::claims::request();
    buy::refund::request();    
}
