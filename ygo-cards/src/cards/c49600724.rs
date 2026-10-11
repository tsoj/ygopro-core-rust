//! Crevice Into the Different Dimension
use super::prelude::*;

pub const ID: u32 = 49600724;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Banish 2 monsters from either GY with the declared Attribute
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_REMOVE)
    effect::set_category(d, e1, CATEGORY_REMOVE);
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    // e1:SetProperty(EFFECT_FLAG_CARD_TARGET)
    effect::set_property(d, e1, EFFECT_FLAG_CARD_TARGET, 0);
    // e1:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    // e1:SetTarget(s.target)
    effect::set_target(d, e1, Some(cost_target(target)));
    // e1:SetOperation(s.activate)
    effect::set_operation(d, e1, Some(operation(activate)));
    // e1:SetHintTiming(0,TIMING_STANDBY_PHASE|TIMING_MAIN_END|TIMINGS_CHECK_MONSTER_E)
    effect::set_hint_timing(d, e1, 0, Some(TIMING_STANDBY_PHASE | TIMING_MAIN_END | TIMINGS_CHECK_MONSTER_E));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
/// `s.rmfilter(c,e,opp)`
fn rmfilter(d: &mut Duel, c: CardId, e: EffectId, opp: u8) -> bool {
    // return c:IsMonster() and c:IsAbleToRemove() and c:IsCanBeEffectTarget(e) and (c:IsControler(opp) or aux.SpElimFilter(c,true))
    card::is_monster(d, c)
        && card::is_able_to_remove(d, c, None, None, None)
        && card::is_can_be_effect_target(d, c, Some(e))
        && (card::is_controler(d, c, opp) || aux::sp_elim_filter(d, c, true, false))
}
/// `s.target(e,tp,eg,ep,ev,re,r,rp,chk,chkc)`
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    let (e, tp) = (a.e, a.tp);
    // if chkc then return false end
    if a.chkc().is_some() {
        return false;
    }
    // local g=Duel.GetMatchingGroup(s.rmfilter,tp,LOCATION_MZONE|LOCATION_GRAVE,LOCATION_GRAVE,nil,e,1-tp)
    let g = duel::get_matching_group(d, Some(&|d, c| rmfilter(d, c, e, 1 - tp)), tp, LOCATION_MZONE | LOCATION_GRAVE, LOCATION_GRAVE, NO_EX);
    // if chk==0 then return #g>=2 and g:GetClassCount(Card.GetAttribute)<#g end
    if a.chk == Some(0) {
        return g.len() >= 2 && group::get_class_count(d, &g, &|d, c| card::get_attribute(d, c) as i64) < g.len();
    }
    // local available_attributes=g:GetBitwiseOr(function(c) local attr=c:GetAttribute() return g:IsExists(Card.IsAttribute,1,c,attr) and attr or 0 end)
    let available_attributes = group::get_bitwise_or(d, &g, &|d, c| {
        let attr = card::get_attribute(d, c);
        if group::is_exists(d, &g, &|d, x| card::is_attribute(d, x, attr), 1, c.into()) {
            attr as i64
        } else {
            0
        }
    });
    // local attr=Duel.AnnounceAttribute(tp,1,available_attributes)
    let attr = duel::announce_attribute(d, tp, 1, available_attributes as u32);
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_REMOVE)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_REMOVE as u64);
    // local tg=g:FilterSelect(tp,Card.IsAttribute,2,2,nil,attr)
    let tg = group::filter_select(d, &g, tp, &|d, c| card::is_attribute(d, c, attr), 2, 2, false, NO_EX).nil();
    // Duel.SetTargetCard(tg)
    duel::set_target_card(d, tg.clone());
    // Duel.SetOperationInfo(0,CATEGORY_REMOVE,tg,2,tp,0)
    duel::set_operation_info(d, 0, CATEGORY_REMOVE, Some(tg), 2, tp, 0);
    true
}
/// `s.activate(e,tp,eg,ep,ev,re,r,rp)`
fn activate(d: &mut Duel, a: &EffArgs) {
    // local tg=Duel.GetTargetCards(e)
    let tg = duel::get_target_cards(d, a.e).nil();
    // if #tg>0 then
    if !tg.is_empty() {
        // Duel.Remove(tg,POS_FACEUP,REASON_EFFECT)
        duel::remove(d, &tg, POS_FACEUP as u8, REASON_EFFECT, None, None);
    }
}
