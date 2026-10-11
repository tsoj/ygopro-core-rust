//! Metalfoes Adamante
use super::prelude::*;

pub const ID: u32 = 81612598;

pub const INFO: ScriptInfo = ScriptInfo {
    // s.listed_series={SET_METALFOES}
    listed_series: &[SET_METALFOES],
    // s.material_setcode=SET_METALFOES
    material_setcode: &[SET_METALFOES],
    ..ScriptInfo::DEFAULT
};

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // fusion material
    // c:EnableReviveLimit()
    card::enable_revive_limit(d, c);
    // Fusion.AddProcMix(c,true,true,aux.FilterBoolFunctionEx(Card.IsSetCard,SET_METALFOES),aux.FilterBoolFunction(Card.IsAttackBelow,2500))
    fusion::add_proc_mix(
        d,
        c,
        true,
        true,
        vec![
            fusion::Mat::F(fusion::fus_mat(|d, c, fc, st, tp| card::is_set_card_ctx(d, c, &[SET_METALFOES as u16], Some(fc), st, Some(tp)))),
            fusion::Mat::F(fusion::fus_mat(|d, c, _fc, _st, _tp| card::is_attack_below(d, c, 2500))),
        ],
    );
}
