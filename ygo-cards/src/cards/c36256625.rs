//! Super Vehicroid Jumbo Drill
use super::prelude::*;

pub const ID: u32 = 36256625;

pub const INFO: ScriptInfo = ScriptInfo { material_setcode: &[SET_ROID], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // c:EnableReviveLimit()
    card::enable_revive_limit(d, c);
    // Fusion.AddProcMix(c,false,false,44729197,71218746,99861526)
    fusion::add_proc_mix(d, c, false, false, vec![fusion::Mat::Code(44729197), fusion::Mat::Code(71218746), fusion::Mat::Code(99861526)]);
    // pierce
    let e1 = effect::create_effect(d, c);
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
    effect::set_code(d, e1, EFFECT_PIERCE);
    card::register_effect(d, c, e1, false);
}
