//! Spellbook of Knowledge
use super::prelude::*;

pub const ID: u32 = 23314220;

pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_SPELLBOOK], listed_names: &[ID], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // activate
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetCategory(CATEGORY_TOGRAVE+CATEGORY_DRAW)
    effect::set_category(d, e1, CATEGORY_TOGRAVE + CATEGORY_DRAW);
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    // e1:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    // e1:SetCountLimit(1,id,EFFECT_COUNT_CODE_OATH)
    effect::set_count_limit(d, e1, 1, ID, 0, EFFECT_COUNT_CODE_OATH as u8);
    // e1:SetTarget(s.target)
    effect::set_target(d, e1, Some(cost_target(target)));
    // e1:SetOperation(s.activate)
    effect::set_operation(d, e1, Some(operation(activate)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
/// `s.filter(c)`
fn filter(d: &mut Duel, c: CardId) -> bool {
    // return (((c:IsFaceup() or c:IsLocation(LOCATION_HAND)) and c:IsSetCard(SET_SPELLBOOK))
    //     or (c:IsFaceup() and c:IsLocation(LOCATION_MZONE) and c:IsRace(RACE_SPELLCASTER)))
    //     and not c:IsCode(id) and c:IsAbleToGrave()
    let a = (card::is_faceup(d, c) || card::is_location(d, c, LOCATION_HAND)) && card::is_set_card(d, c, &[SET_SPELLBOOK as u16]);
    let b = card::is_faceup(d, c) && card::is_location(d, c, LOCATION_MZONE) && card::is_race(d, c, RACE_SPELLCASTER);
    (a || b) && !card::is_code(d, c, &[ID]) && card::is_able_to_grave(d, c, None)
}
/// `s.target(e,tp,eg,ep,ev,re,r,rp,chk)`
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return Duel.IsPlayerCanDraw(tp,2) and Duel.IsExistingMatchingCard(s.filter,tp,LOCATION_HAND|LOCATION_ONFIELD,0,1,e:GetHandler()) end
    if a.chk == Some(0) {
        let h = effect::get_handler(d, a.e).nil();
        return duel::is_player_can_draw(d, a.tp, 2) && duel::is_existing_matching_card(d, Some(&filter), a.tp, LOCATION_HAND | LOCATION_ONFIELD, 0, 1, h.into());
    }
    // Duel.SetOperationInfo(0,CATEGORY_TOGRAVE,nil,1,tp,LOCATION_HAND|LOCATION_ONFIELD)
    duel::set_operation_info(d, 0, CATEGORY_TOGRAVE, None, 1, a.tp, (LOCATION_HAND | LOCATION_ONFIELD) as i32);
    // Duel.SetOperationInfo(0,CATEGORY_DRAW,nil,0,tp,2)
    duel::set_operation_info(d, 0, CATEGORY_DRAW, None, 0, a.tp, 2);
    true
}
/// `s.activate(e,tp,eg,ep,ev,re,r,rp)`
fn activate(d: &mut Duel, a: &EffArgs) {
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_TOGRAVE)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_TOGRAVE as u64);
    // local g=Duel.SelectMatchingCard(tp,s.filter,tp,LOCATION_HAND|LOCATION_ONFIELD,0,1,1,e:GetHandler())
    let h = effect::get_handler(d, a.e).nil();
    let g = duel::select_matching_card(d, a.tp, Some(&filter), a.tp, LOCATION_HAND | LOCATION_ONFIELD, 0, 1, 1, false, h.into()).nil();
    // local tc=g:GetFirst()
    let tc = group::get_first(d, &g);
    // if tc and Duel.SendtoGrave(tc,REASON_EFFECT)>0 and tc:IsLocation(LOCATION_GRAVE) then
    if let Some(tc) = tc {
        if duel::send_to_grave(d, tc, REASON_EFFECT, None, None) > 0 && card::is_location(d, tc, LOCATION_GRAVE) {
            // Duel.Draw(tp,2,REASON_EFFECT)
            duel::draw(d, a.tp, 2, REASON_EFFECT);
        }
    }
}
