//! Predaprime Fusion
use super::prelude::*;

pub const ID: u32 = 8148322;

pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_PREDAPLANT], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate
    // local e1=Fusion.CreateSummonEff(c,aux.FilterBoolFunction(Card.IsAttribute,ATTRIBUTE_DARK),Fusion.OnFieldMat,s.fextra)
    let e1 = fusion::create_summon_eff(
        d,
        c,
        fusion::SummonEff {
            fusfilter: Some(fusion::fus_filter(|d, c, _tp| card::is_attribute(d, c, ATTRIBUTE_DARK))),
            matfilter: Some(fusion::on_field_mat()),
            extrafil: Some(std::sync::Arc::new(fextra)),
            ..Default::default()
        },
    );
    // e1:SetCountLimit(1,id,EFFECT_COUNT_CODE_OATH)
    effect::set_count_limit(d, e1, 1, ID, 0, EFFECT_COUNT_CODE_OATH as u8);
    // e1:SetHintTiming(0,TIMINGS_CHECK_MONSTER_E)
    effect::set_hint_timing(d, e1, 0, Some(TIMINGS_CHECK_MONSTER_E as u32));
    // e1:SetCondition(s.condition)
    effect::set_condition(d, e1, Some(condition(cond)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
/// `s.condition(e,tp,eg,ep,ev,re,r,rp)`
fn cond(d: &mut Duel, a: &EffArgs) -> bool {
    // return Duel.IsExistingMatchingCard(aux.FaceupFilter(Card.IsSetCard,SET_PREDAPLANT),tp,LOCATION_MZONE,LOCATION_MZONE,1,nil)
    duel::is_existing_matching_card(
        d,
        Some(&aux::faceup_filter(|d, c| card::is_set_card(d, c, &[SET_PREDAPLANT as u16]))),
        a.tp,
        LOCATION_MZONE,
        LOCATION_MZONE,
        1,
        NO_EX,
    )
}
/// `s.checkextra(tp,sg,fc)`
fn checkextra(d: &mut Duel, a: Args) -> bool {
    // return sg:IsExists(aux.AND(aux.FilterBoolFunction(Card.IsAttribute,ATTRIBUTE_DARK),aux.FilterBoolFunction(Card.IsControler,tp)),2,nil)
    let tp = a.p(0);
    let sg = a.g(1);
    group::is_exists(d, &sg, &|d, c| card::is_attribute(d, c, ATTRIBUTE_DARK) && card::is_controler(d, c, tp), 2, NO_EX)
}
/// `s.fextra(e,tp,mg)`
fn fextra(d: &mut Duel, _e: EffectId, tp: u8, _mg: &CardSet) -> (Option<CardSet>, Option<LuaFn>) {
    // return Duel.GetMatchingGroup(Fusion.IsMonsterFilter(Card.IsFaceup),tp,0,LOCATION_ONFIELD,nil),s.checkextra
    let g = duel::get_matching_group(d, Some(&|d, c| card::is_monster(d, c) && card::is_faceup(d, c)), tp, 0, LOCATION_ONFIELD, NO_EX);
    (Some(g), Some(lua_fn(checkextra)))
}
