//! Ambulanceroid
use super::prelude::*;

pub const ID: u32 = 36378213;
pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_ROID], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e1, EFFECT_TYPE_FIELD + EFFECT_TYPE_TRIGGER_O);
    // e1:SetCategory(CATEGORY_SPECIAL_SUMMON)
    effect::set_category(d, e1, CATEGORY_SPECIAL_SUMMON);
    // e1:SetCode(EVENT_TO_HAND)
    effect::set_code(d, e1, EVENT_TO_HAND);
    // e1:SetRange(LOCATION_MZONE)
    effect::set_range(d, e1, LOCATION_MZONE);
    // e1:SetTarget(s.target)
    effect::set_target(d, e1, Some(cost_target(target)));
    // e1:SetOperation(s.activate)
    effect::set_operation(d, e1, Some(operation(activate)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
/// `s.filter(c,e,tp)`
fn filter(d: &mut Duel, c: CardId, e: EffectId, tp: u8) -> bool {
    // return c:IsPreviousLocation(LOCATION_GRAVE) and c:IsPreviousControler(tp) and c:IsSetCard(SET_ROID)
    //     and c:IsCanBeSpecialSummoned(e,0,tp,false,false)
    card::is_previous_location(d, c, LOCATION_GRAVE)
        && card::is_previous_controler(d, c, tp)
        && card::is_set_card(d, c, &[SET_ROID as u16])
        && card::is_can_be_special_summoned(d, c, e, 0, tp, false, false, None, None, None)
}
/// `s.target(e,tp,eg,ep,ev,re,r,rp,chk)`
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    // local g=eg:Filter(s.filter,nil,e,tp)
    let eg = a.eg.clone().nil();
    let g = group::filter(d, &eg, &|d, c| filter(d, c, a.e, a.tp), NO_EX);
    // if chk==0 then return #g~=0 and Duel.GetLocationCount(tp,LOCATION_MZONE)>0 end
    if a.chk == Some(0) {
        return !g.is_empty() && duel::get_location_count(d, a.tp, LOCATION_MZONE as u8, None, None, None).0 > 0;
    }
    // Duel.SetTargetCard(eg)
    duel::set_target_card(d, eg.clone());
    // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,g,1,0,0)
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, Some(g), 1, 0, 0);
    true
}
/// `s.spfilter(c,e,tp)`
fn spfilter(d: &mut Duel, c: CardId, e: EffectId, tp: u8) -> bool {
    // return c:IsPreviousLocation(LOCATION_GRAVE) and c:IsPreviousControler(tp) and c:IsSetCard(SET_ROID)
    //     and c:IsCanBeSpecialSummoned(e,0,tp,false,false) and c:IsRelateToEffect(e)
    card::is_previous_location(d, c, LOCATION_GRAVE)
        && card::is_previous_controler(d, c, tp)
        && card::is_set_card(d, c, &[SET_ROID as u16])
        && card::is_can_be_special_summoned(d, c, e, 0, tp, false, false, None, None, None)
        && card::is_relate_to_effect(d, c, e)
}
/// `s.activate(e,tp,eg,ep,ev,re,r,rp)`
fn activate(d: &mut Duel, a: &EffArgs) {
    // local ft=Duel.GetLocationCount(tp,LOCATION_MZONE)
    let ft = duel::get_location_count(d, a.tp, LOCATION_MZONE as u8, None, None, None).0;
    // if ft<=0 then return end
    if ft <= 0 {
        return;
    }
    // local g=eg:Filter(s.spfilter,nil,e,tp)
    let eg = a.eg.clone().nil();
    let mut g = group::filter(d, &eg, &|d, c| spfilter(d, c, a.e, a.tp), NO_EX);
    // if #g==0 then return end
    if g.is_empty() {
        return;
    }
    // if #g>1 and Duel.IsPlayerAffectedByEffect(tp,CARD_BLUEEYES_SPIRIT) then return end
    if g.len() > 1 && duel::is_player_affected_by_effect(d, a.tp, CARD_BLUEEYES_SPIRIT).is_some() {
        return;
    }
    // if #g>ft then
    if g.len() as i32 > ft {
        // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_SPSUMMON)
        duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_SPSUMMON as u64);
        // g=g:Select(tp,ft,ft,nil)
        g = group::select(d, &g, a.tp, ft as u16, ft as u16, false, NO_EX).nil();
    }
    // Duel.SpecialSummon(g,0,tp,tp,false,false,POS_FACEUP)
    duel::special_summon(d, g, 0, a.tp, a.tp, false, false, POS_FACEUP as u8, None);
}
