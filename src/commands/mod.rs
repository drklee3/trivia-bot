use serenity::framework::standard::macros::group;

mod points;
mod quiz;

use self::{points::*, quiz::*};

#[group]
#[commands(quiz, points)]
struct Quiz;
