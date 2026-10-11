//! Valkyrie Funfte
use super::prelude::*;

pub const ID: u32 = 46701379;

pub const INFO: ScriptInfo = ScriptInfo { listed_names: &[ID], listed_series: &[SET_VALKYRIE], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // atk boost
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e1, EFFECT_TYPE_FIELD);
    // e1:SetCode(EFFECT_UPDATE_ATTACK)
    effect::set_code(d, e1, EFFECT_UPDATE_ATTACK);
    // e1:SetRange(LOCATION_MZONE)
    effect::set_range(d, e1, LOCATION_MZONE);
    // e1:SetTargetRange(LOCATION_MZONE,0)
    effect::set_target_range(d, e1, LOCATION_MZONE, 0);
    // e1:SetTarget(aux.TargetBoolFunction(Card.IsSetCard,SET_VALKYRIE))
    effect::set_target(d, e1, Some(effect_card_fn(|d, _e, c| card::is_set_card(d, c, &[SET_VALKYRIE as u16]))));
    // e1:SetValue(s.atkval)
    effect::set_value_fn(d, e1, effect_card_value(atkval));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // mill
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e2, aux::stringid(ID, 0));
    // e2:SetCategory(CATEGORY_TOGRAVE)
    effect::set_category(d, e2, CATEGORY_TOGRAVE);
    // e2:SetType(EFFECT_TYPE_IGNITION)
    effect::set_type(d, e2, EFFECT_TYPE_IGNITION);
    // e2:SetRange(LOCATION_MZONE)
    effect::set_range(d, e2, LOCATION_MZONE);
    // e2:SetCountLimit(1,id)
    effect::set_count_limit(d, e2, 1, ID, 0, 0);
    // e2:SetCondition(s.gycon)
    effect::set_condition(d, e2, Some(condition(gycon)));
    // e2:SetTarget(s.gytg)
    effect::set_target(d, e2, Some(cost_target(gytg)));
    // e2:SetOperation(s.gyop)
    effect::set_operation(d, e2, Some(operation(gyop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
}
/// `s.atkval(e,c)`
fn atkval(d: &mut Duel, e: EffectId, _c: CardId) -> i64 {
    // local tp=e:GetHandlerPlayer()
    let tp = effect::get_handler_player(d, e);
    // return Duel.GetMatchingGroupCount(aux.FaceupFilter(Card.IsMonster),tp,0,LOCATION_REMOVED,nil)*200
    duel::get_matching_group_count(d, Some(&aux::faceup_filter(|d, x| card::is_monster(d, x))), tp, 0, LOCATION_REMOVED, NO_EX) as i64 * 200
}
/// `s.gycfilter(c)`
fn gycfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsFaceup() and c:IsSetCard(SET_VALKYRIE) and not c:IsCode(id)
    card::is_faceup(d, c) && card::is_set_card(d, c, &[SET_VALKYRIE as u16]) && !card::is_code(d, c, &[ID])
}
/// `s.gycon(e,tp,eg,ep,ev,re,r,rp)`
fn gycon(d: &mut Duel, a: &EffArgs) -> bool {
    // return Duel.IsExistingMatchingCard(s.gycfilter,tp,LOCATION_MZONE,0,1,nil)
    duel::is_existing_matching_card(d, Some(&gycfilter), a.tp, LOCATION_MZONE, 0, 1, NO_EX)
}
/// `s.gyfilter(c)`
fn gyfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsSpellTrap() and c:IsAbleToGrave()
    card::is_spell_trap(d, c) && card::is_able_to_grave(d, c, None)
}
/// `s.gytg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn gytg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return Duel.IsExistingMatchingCard(s.gyfilter,tp,LOCATION_DECK,0,1,nil) end
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, Some(&gyfilter), a.tp, LOCATION_DECK, 0, 1, NO_EX);
    }
    // Duel.SetOperationInfo(0,CATEGORY_TOGRAVE,nil,1,tp,LOCATION_DECK)
    duel::set_operation_info(d, 0, CATEGORY_TOGRAVE, None, 1, a.tp, LOCATION_DECK as i32);
    true
}
/// `s.gyop(e,tp,eg,ep,ev,re,r,rp)`
fn gyop(d: &mut Duel, a: &EffArgs) {
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_TOGRAVE)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_TOGRAVE as u64);
    // local g=Duel.SelectMatchingCard(tp,s.gyfilter,tp,LOCATION_DECK,0,1,1,nil)
    let g = duel::select_matching_card(d, a.tp, Some(&gyfilter), a.tp, LOCATION_DECK, 0, 1, 1, false, NO_EX).nil();
    // if #g>0 then
    if !g.is_empty() {
        // Duel.SendtoGrave(g,REASON_EFFECT)
        duel::send_to_grave(d, g, REASON_EFFECT, None, None);
    }
}
