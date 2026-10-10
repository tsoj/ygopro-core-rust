//! Tearlaments Perlegia
use super::prelude::*;

pub const ID: u32 = 34225426;

pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_TEARLAMENTS], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    // e1:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // Increase ATK
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e2, EFFECT_TYPE_FIELD);
    // e2:SetCode(EFFECT_UPDATE_ATTACK)
    effect::set_code(d, e2, EFFECT_UPDATE_ATTACK);
    // e2:SetTargetRange(LOCATION_MZONE,0)
    effect::set_target_range(d, e2, LOCATION_MZONE, 0);
    // e2:SetRange(LOCATION_FZONE)
    effect::set_range(d, e2, LOCATION_FZONE);
    // e2:SetTarget(function(_,c) return c:IsSetCard(SET_TEARLAMENTS) or c:IsType(TYPE_FUSION) end)
    effect::set_target(d, e2, Some(effect_card_fn(|d, _e, c| card::is_set_card(d, c, &[SET_TEARLAMENTS as u16]) || card::is_type(d, c, TYPE_FUSION))));
    // e2:SetValue(500)
    effect::set_value(d, e2, 500);
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
    // Send 1 Aqua monster from Deck to GY
    // local e3=Effect.CreateEffect(c)
    let e3 = effect::create_effect(d, c);
    // e3:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e3, aux::stringid(ID, 1));
    // e3:SetCategory(CATEGORY_TOGRAVE)
    effect::set_category(d, e3, CATEGORY_TOGRAVE);
    // e3:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e3, EFFECT_TYPE_FIELD + EFFECT_TYPE_TRIGGER_O);
    // e3:SetProperty(EFFECT_FLAG_DELAY)
    effect::set_property(d, e3, EFFECT_FLAG_DELAY, 0);
    // e3:SetCode(EVENT_TO_GRAVE)
    effect::set_code(d, e3, EVENT_TO_GRAVE);
    // e3:SetRange(LOCATION_FZONE)
    effect::set_range(d, e3, LOCATION_FZONE);
    // e3:SetCountLimit(1,id)
    effect::set_count_limit(d, e3, 1, ID, 0, 0);
    // e3:SetCondition(s.grcon)
    effect::set_condition(d, e3, Some(condition(grcon)));
    // e3:SetTarget(s.grtg)
    effect::set_target(d, e3, Some(cost_target(grtg)));
    // e3:SetOperation(s.grop)
    effect::set_operation(d, e3, Some(operation(grop)));
    // c:RegisterEffect(e3)
    card::register_effect(d, c, e3, false);
}
/// `s.cfilter(c,tp)`
fn cfilter(d: &mut Duel, c: CardId, tp: u8) -> bool {
    // return c:IsRace(RACE_AQUA) and c:IsSetCard(SET_TEARLAMENTS) and c:IsControler(tp) and c:IsReason(REASON_EFFECT)
    card::is_race(d, c, RACE_AQUA) && card::is_set_card(d, c, &[SET_TEARLAMENTS as u16]) && card::is_controler(d, c, tp) && card::is_reason(d, c, REASON_EFFECT)
}
/// `s.grcon(e,tp,eg,ep,ev,re,r,rp)`
fn grcon(d: &mut Duel, a: &EffArgs) -> bool {
    // return eg:IsExists(s.cfilter,1,nil,tp)
    let eg = a.eg.clone().nil();
    group::is_exists(d, &eg, &|d, c| cfilter(d, c, a.tp), 1, NO_EX)
}
/// `s.grvfilter(c)`
fn grvfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsRace(RACE_AQUA) and c:IsLevelBelow(4) and c:IsAbleToGrave()
    card::is_race(d, c, RACE_AQUA) && card::is_level_below(d, c, 4) && card::is_able_to_grave(d, c, None)
}
/// `s.grtg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn grtg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return Duel.IsExistingMatchingCard(s.grvfilter,tp,LOCATION_DECK,0,1,nil) end
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, Some(&grvfilter), a.tp, LOCATION_DECK, 0, 1, NO_EX);
    }
    // Duel.SetOperationInfo(0,CATEGORY_TOGRAVE,nil,1,tp,LOCATION_DECK)
    duel::set_operation_info(d, 0, CATEGORY_TOGRAVE, None, 1, a.tp, LOCATION_DECK as i32);
    true
}
/// `s.grop(e,tp,eg,ep,ev,re,r,rp)`
fn grop(d: &mut Duel, a: &EffArgs) {
    let e = a.e;
    let tp = a.tp;
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_TOGRAVE)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_TOGRAVE as u64);
    // local g=Duel.SelectMatchingCard(tp,s.grvfilter,tp,LOCATION_DECK,0,1,1,nil)
    let g = duel::select_matching_card(d, tp, Some(&grvfilter), tp, LOCATION_DECK, 0, 1, 1, false, NO_EX).nil();
    // if #g>0 and Duel.SendtoGrave(g,REASON_EFFECT)>0 then
    if g.is_empty() || duel::send_to_grave(d, g, REASON_EFFECT, None, None) <= 0 {
        return;
    }
    // local tc=Duel.GetOperatedGroup():GetFirst()
    let op = duel::get_operated_group(d);
    let tc = group::get_first(d, &op).nil();
    // if tc:IsLocation(LOCATION_GRAVE) and not tc:IsSetCard(SET_TEARLAMENTS) then
    if card::is_location(d, tc, LOCATION_GRAVE) && !card::is_set_card(d, tc, &[SET_TEARLAMENTS as u16]) {
        // Cannot activate card or effects with the same name if it's not a "Tearlaments"
        // local e1=Effect.CreateEffect(e:GetHandler())
        let h = effect::get_handler(d, e).nil();
        let e1 = effect::create_effect(d, h);
        // e1:SetDescription(aux.Stringid(id,2))
        effect::set_description(d, e1, aux::stringid(ID, 2));
        // e1:SetType(EFFECT_TYPE_FIELD)
        effect::set_type(d, e1, EFFECT_TYPE_FIELD);
        // e1:SetProperty(EFFECT_FLAG_PLAYER_TARGET+EFFECT_FLAG_CLIENT_HINT)
        effect::set_property(d, e1, EFFECT_FLAG_PLAYER_TARGET + EFFECT_FLAG_CLIENT_HINT, 0);
        // e1:SetCode(EFFECT_CANNOT_ACTIVATE)
        effect::set_code(d, e1, EFFECT_CANNOT_ACTIVATE);
        // e1:SetTargetRange(1,0)
        effect::set_target_range(d, e1, 1, 0);
        // e1:SetValue(s.aclimit)
        effect::set_value_fn(d, e1, lua_fn(aclimit));
        // e1:SetLabel(tc:GetCode())
        let code = card::get_code(d, tc)[0];
        effect::set_label(d, e1, &[code as i64]);
        // e1:SetReset(RESET_PHASE|PHASE_END)
        effect::set_reset(d, e1, RESET_PHASE | PHASE_END, 1);
        // Duel.RegisterEffect(e1,tp)
        duel::register_effect(d, e1, tp);
    }
}
/// `s.aclimit(e,re,tp)`
fn aclimit(d: &mut Duel, a: Args) -> bool {
    let e = a.e(0);
    let re = a.e(1);
    // return re:GetHandler():IsCode(e:GetLabel())
    let h = effect::get_handler(d, re).nil();
    let label = effect::get_label(d, e);
    card::is_code(d, h, &[label as u32])
}
