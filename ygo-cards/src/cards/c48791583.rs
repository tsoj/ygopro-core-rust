//! Invoked Magellanica
use super::prelude::*;

pub const ID: u32 = 48791583;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // fusion material
    // c:EnableReviveLimit()
    card::enable_revive_limit(d, c);
    // Fusion.AddProcMix(c,true,true,86120751,aux.FilterBoolFunctionEx(Card.IsAttribute,ATTRIBUTE_EARTH))
    fusion::add_proc_mix(
        d,
        c,
        true,
        true,
        vec![
            fusion::Mat::Code(86120751),
            fusion::Mat::F(fusion::fus_mat(|d, x, fc, st, tp| card::is_attribute_ctx(d, x, ATTRIBUTE_EARTH, Some(fc), st, Some(tp)))),
        ],
    );
}
