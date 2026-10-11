//! Metalfoes Crimsonite
//! メタルフォーゼ・カーディナル
use super::prelude::*;

pub const ID: u32 = 54401832;

pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_METALFOES], material_setcode: &[SET_METALFOES], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // c:EnableReviveLimit()
    card::enable_revive_limit(d, c);
    // Fusion.AddProcMixN(c,true,true,aux.FilterBoolFunctionEx(Card.IsSetCard,SET_METALFOES),1,aux.FilterBoolFunction(Card.IsAttackBelow,3000),2)
    fusion::add_proc_mix_n(
        d,
        c,
        true,
        true,
        vec![
            (fusion::Mat::F(fusion::fus_mat(|d, c, fc, st, tp| card::is_set_card_ctx(d, c, &[SET_METALFOES as u16], Some(fc), st, Some(tp)))), 1),
            (fusion::Mat::F(fusion::fus_mat(|d, c, _fc, _st, _tp| card::is_attack_below(d, c, 3000))), 2),
        ],
    );
}
