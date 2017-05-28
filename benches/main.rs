#![feature(test)]
#![feature(try_from)]
use std::convert::TryFrom;

#[macro_use]
extern crate enum_proc_derive;
extern crate rand;
extern crate test;

use test::Bencher;
use std::mem::transmute;
use rand::Rng;

#[derive(Debug)]
struct InvalidEnumValue(());

#[derive(PartialEq,Eq,Debug,FromPrimitive)]
#[FromPrimitiveType="u8"]
enum Foo {
    ZerothFoo = 0,
    FirstFoo,
    SecondFoo,
    ThirdFoo,
}

#[bench]
fn bench_raw(b: &mut Bencher) {
    let mut values = (0..255).map(|x| x % 4).collect::<Vec<u8>>();
    let mut rng = rand::thread_rng();
    rng.shuffle(&mut values);

    b.iter(|| {
        values.iter().map(|x| {
            if *x > 3 as u8 {panic!("x out of range!")};
            unsafe{transmute::<u8, Foo>(*x)}
        }).collect::<Vec<Foo>>()
    })
}

#[bench]
fn bench_from(b: &mut Bencher) {
    let mut values = (0..255).map(|x| x % 4).collect::<Vec<u8>>();
    let mut rng = rand::thread_rng();
    rng.shuffle(&mut values);

    b.iter(|| {
        values.iter().map(|x| Foo::try_from(*x).unwrap()).collect::<Vec<Foo>>()
    })
}
