//! Pendulum Fusion
use super::prelude::*;
use std::sync::Arc;

pub const ID: u32 = 65646587;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate
    // local e1=Fusion.CreateSummonEff(c,nil,Fusion.OnFieldMat,s.fextra)
    let e1 = fusion::create_summon_eff(
        d,
        c,
        fusion::SummonEff {
            matfilter: Some(fusion::on_field_mat()),
            extrafil: Some(Arc::new(fextra)),
            ..Default::default()
        },
    );
    // e1:SetCountLimit(1,id,EFFECT_COUNT_CODE_OATH)
    effect::set_count_limit(d, e1, 1, ID, 0, EFFECT_COUNT_CODE_OATH as u8);
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
// s.fextra(e,tp,mg)
fn fextra(d: &mut Duel, _e: EffectId, tp: u8, _mg: &CardSet) -> (Option<CardSet>, Option<LuaFn>) {
    // if Duel.GetFieldGroupCount(tp,LOCATION_PZONE,0)==2 then
    if duel::get_field_group_count(d, tp, LOCATION_PZONE, 0) == 2 {
        // return Duel.GetMatchingGroup(Card.IsAbleToGrave,tp,LOCATION_PZONE,0,nil)
        return (Some(duel::get_matching_group(d, Some(&|d, c| card::is_able_to_grave(d, c, None)), tp, LOCATION_PZONE, 0, NO_EX)), None);
    }
    (None, None)
}
