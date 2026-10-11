//! Myutant Cry
use super::prelude::*;

pub const ID: u32 = 31855260;

pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_MYUTANT], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Fusion summon 1 "Myutant" fusion monster by shuffling fusion materials from field/GY/face-up banished into deck
    // local e1=Fusion.CreateSummonEff(c,aux.FilterBoolFunction(Card.IsSetCard,SET_MYUTANT),Fusion.OnFieldMat(Card.IsAbleToDeck),s.fextra,Fusion.ShuffleMaterial,nil,nil,nil,nil,nil,nil,nil,nil,nil,s.extratg)
    let e1 = fusion::create_summon_eff(
        d,
        c,
        fusion::SummonEff {
            fusfilter: Some(fusion::fus_filter(|d, c, _tp| card::is_set_card(d, c, &[SET_MYUTANT as u16]))),
            // Fusion.OnFieldMat(Card.IsAbleToDeck): the C function takes only the card
            matfilter: Some(fusion::on_field_mat_with(std::sync::Arc::new(|d, c, _e, _tp, _chk| card::is_able_to_deck(d, c)))),
            extrafil: Some(std::sync::Arc::new(fextra)),
            extraop: Some(fusion::shuffle_material()),
            extratg: Some(cost_target(extratg)),
            ..Default::default()
        },
    );
    // e1:SetCountLimit(1,id,EFFECT_COUNT_CODE_OATH)
    effect::set_count_limit(d, e1, 1, ID, 0, EFFECT_COUNT_CODE_OATH as u8);
    // e1:SetCondition(function() return Duel.IsMainPhase() end)
    effect::set_condition(d, e1, Some(condition(e1_condition)));
    // e1:SetHintTiming(0,TIMING_MAIN_END)
    effect::set_hint_timing(d, e1, 0, Some(TIMING_MAIN_END));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
/// `function() return Duel.IsMainPhase() end`
fn e1_condition(d: &mut Duel, _a: &EffArgs) -> bool {
    duel::is_main_phase(d, None)
}
/// `s.fextra(e,tp,mg)`
fn fextra(d: &mut Duel, _e: EffectId, tp: u8, _mg: &CardSet) -> (Option<CardSet>, Option<LuaFn>) {
    // return Duel.GetMatchingGroup(Fusion.IsMonsterFilter(aux.NecroValleyFilter(Card.IsFaceup,Card.IsAbleToDeck)),tp,LOCATION_GRAVE|LOCATION_REMOVED,0,nil)
    let f = aux::necro_valley_filter(|d, c| card::is_faceup(d, c) && card::is_able_to_deck(d, c));
    let g = duel::get_matching_group(d, Some(&|d, c| card::is_monster(d, c) && f(d, c)), tp, LOCATION_GRAVE | LOCATION_REMOVED, 0, NO_EX);
    (Some(g), None)
}
/// `s.extratg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn extratg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return true end
    if a.chk == Some(0) {
        return true;
    }
    // Duel.SetOperationInfo(0,CATEGORY_TODECK,nil,0,tp,LOCATION_PUBLIC)
    duel::set_operation_info(d, 0, CATEGORY_TODECK, None, 0, a.tp, LOCATION_PUBLIC as i32);
    true
}
