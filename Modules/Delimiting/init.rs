extern crate rustling;
use rustling::*;
use rustling::moment::Grain;
use rustling::training::train;
use rustling::RustlingClassifier;

fn main() {
    let classifier = train::<RustlingClassifier, _>(&[
        Example {
            tokens: &["in", "two", "days"],
            value: &|_| RustlingValue::new(MomentValue::new(Grain::Day, 2)),
            latent: false,
        },
    ]);

    let result = classifier.parse("in two days", &ParseOptions::default());
    println!("{:?}", result);
}