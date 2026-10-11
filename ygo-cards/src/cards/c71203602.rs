//! Evigishki Levianima
use super::prelude::*;

pub const ID: u32 = 71203602;
pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_GISHKI], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // c:EnableReviveLimit()
    card::enable_revive_limit(d, c);
    // confirm
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_DRAW)
    effect::set_category(d, e1, CATEGORY_DRAW);
    // e1:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_TRIGGER_F)
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_F);
    // e1:SetCode(EVENT_ATTACK_ANNOUNCE)
    effect::set_code(d, e1, EVENT_ATTACK_ANNOUNCE);
    // e1:SetTarget(s.target)
    effect::set_target(d, e1, Some(cost_target(target)));
    // e1:SetOperation(s.operation)
    effect::set_operation(d, e1, Some(operation(op)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
/// `s.target(e,tp,eg,ep,ev,re,r,rp,chk)`
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return true end
    if a.chk == Some(0) {
        return true;
    }
    // Duel.SetOperationInfo(0,CATEGORY_DRAW,nil,0,tp,1)
    duel::set_operation_info(d, 0, CATEGORY_DRAW, None, 0, a.tp, 1);
    true
}
/// `s.operation(e,tp,eg,ep,ev,re,r,rp)`
fn op(d: &mut Duel, a: &EffArgs) {
    // local ct=Duel.Draw(tp,1,REASON_EFFECT)
    let ct = duel::draw(d, a.tp, 1, REASON_EFFECT);
    // if ct==0 then return end
    if ct == 0 {
        return;
    }
    // local tc=Duel.GetOperatedGroup():GetFirst()
    let tc = group::get_first(d, &duel::get_operated_group(d)).nil();
    // Duel.ConfirmCards(1-tp,tc)
    duel::confirm_cards(d, 1 - a.tp, tc);
    // Duel.ShuffleHand(tp)
    duel::shuffle_hand(d, a.tp);
    // if tc:IsSetCard(SET_GISHKI) and tc:IsMonster() then
    if card::is_set_card(d, tc, &[SET_GISHKI as u16]) && card::is_monster(d, tc) {
        // Duel.BreakEffect()
        duel::break_effect(d);
        // local g=Duel.GetFieldGroup(tp,0,LOCATION_HAND)
        let g = duel::get_field_group(d, a.tp, 0, LOCATION_HAND);
        // if #g>0 then
        if !g.is_empty() {
            // local sg=g:RandomSelect(tp,1)
            let sg = group::random_select(d, &g, a.tp, 1);
            // Duel.ConfirmCards(tp,sg)
            duel::confirm_cards(d, a.tp, sg);
        }
        // Duel.ShuffleHand(1-tp)
        duel::shuffle_hand(d, 1 - a.tp);
    }
}
