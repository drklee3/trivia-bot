use serenity::framework::standard::macros::group;

mod quiz;

use quiz::*;

#[group]
#[commands(quiz)]
struct Quiz;
