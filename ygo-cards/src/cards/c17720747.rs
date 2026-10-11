//! Witch of the Black Rose
//! 黒薔薇の魔女
use super::prelude::*;

pub const ID: u32 = 17720747;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // cannot special summon
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetProperty(EFFECT_FLAG_CANNOT_DISABLE+EFFECT_FLAG_UNCOPYABLE)
    effect::set_property(d, e1, EFFECT_FLAG_CANNOT_DISABLE + EFFECT_FLAG_UNCOPYABLE, 0);
    // e1:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
    // e1:SetCode(EFFECT_SPSUMMON_CONDITION)
    effect::set_code(d, e1, EFFECT_SPSUMMON_CONDITION);
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // Draw
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e2, aux::stringid(ID, 0));
    // e2:SetCategory(CATEGORY_DRAW)
    effect::set_category(d, e2, CATEGORY_DRAW);
    // e2:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_TRIGGER_F)
    effect::set_type(d, e2, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_F);
    // e2:SetCode(EVENT_SUMMON_SUCCESS)
    effect::set_code(d, e2, EVENT_SUMMON_SUCCESS);
    // e2:SetCondition(s.condition)
    effect::set_condition(d, e2, Some(condition(cond)));
    // e2:SetTarget(s.target)
    effect::set_target(d, e2, Some(cost_target(target)));
    // e2:SetOperation(s.operation)
    effect::set_operation(d, e2, Some(operation(op)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
}
// function s.condition(e,tp,eg,ep,ev,re,r,rp)
fn cond(d: &mut Duel, a: &EffArgs) -> bool {
    // return Duel.GetFieldGroupCount(tp,LOCATION_ONFIELD,0)<=1
    duel::get_field_group_count(d, a.tp, LOCATION_ONFIELD, 0) <= 1
}
// function s.target(e,tp,eg,ep,ev,re,r,rp,chk)
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return true end
    if a.chk == Some(0) {
        return true;
    }
    // Duel.SetOperationInfo(0,CATEGORY_DRAW,nil,0,tp,1)
    duel::set_operation_info(d, 0, CATEGORY_DRAW, None, 0, a.tp, 1);
    true
}
// function s.operation(e,tp,eg,ep,ev,re,r,rp)
fn op(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // local g=Duel.GetDecktopGroup(tp,1)
    let g = duel::get_decktop_group(d, tp, 1);
    // local tc=g:GetFirst()
    let tc = group::get_first(d, &g);
    // Duel.Draw(tp,1,REASON_EFFECT)
    duel::draw(d, tp, 1, REASON_EFFECT);
    // if tc then
    if let Some(tc) = tc {
        // Duel.ConfirmCards(1-tp,tc)
        duel::confirm_cards(d, 1 - tp, tc);
        // if not tc:IsMonster() then
        if !card::is_monster(d, tc) {
            // Duel.BreakEffect()
            duel::break_effect(d);
            // Duel.SendtoGrave(tc,REASON_EFFECT)
            duel::send_to_grave(d, tc, REASON_EFFECT, None, None);
            // if e:GetHandler():IsRelateToEffect(e) then
            let h = effect::get_handler(d, a.e).nil();
            if card::is_relate_to_effect(d, h, a.e) {
                // Duel.Destroy(e:GetHandler(),REASON_EFFECT)
                duel::destroy(d, h, REASON_EFFECT, None, None);
            }
        }
        // Duel.ShuffleHand(tp)
        duel::shuffle_hand(d, tp);
    }
}
