//! D.D. Scout Plane
use super::prelude::*;

pub const ID: u32 = 3773196;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // removed
    let e1 = effect::create_effect(d, c);
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE + EFFECT_TYPE_CONTINUOUS);
    effect::set_code(d, e1, EVENT_REMOVE);
    effect::set_operation(d, e1, Some(operation(rmop)));
    card::register_effect(d, c, e1, false);
    // special summon
    let e2 = effect::create_effect(d, c);
    effect::set_description(d, e2, aux::stringid(ID, 0));
    effect::set_category(d, e2, CATEGORY_SPECIAL_SUMMON);
    effect::set_type(d, e2, EFFECT_TYPE_FIELD + EFFECT_TYPE_TRIGGER_F);
    effect::set_code(d, e2, EVENT_PHASE + PHASE_END);
    effect::set_range(d, e2, LOCATION_REMOVED);
    effect::set_condition(d, e2, Some(condition(cond)));
    effect::set_target(d, e2, Some(cost_target(target)));
    effect::set_operation(d, e2, Some(operation(operation_sp)));
    card::register_effect(d, c, e2, false);
}
// function s.rmop(e,tp,eg,ep,ev,re,r,rp)
fn rmop(d: &mut Duel, a: &EffArgs) {
    // if e:GetHandler():IsFacedown() then return end
    let c = effect::get_handler(d, a.e).nil();
    if card::is_facedown(d, c) {
        return;
    }
    // e:GetHandler():RegisterFlagEffect(id,RESETS_STANDARD_PHASE_END,0,1)
    card::register_flag_effect(d, c, ID, RESETS_STANDARD_PHASE_END, 0, 1, 0, 0);
}
// function s.condition(e,tp,eg,ep,ev,re,r,rp)
fn cond(d: &mut Duel, a: &EffArgs) -> bool {
    // return e:GetHandler():GetFlagEffect(id)~=0
    let c = effect::get_handler(d, a.e).nil();
    card::get_flag_effect(d, c, ID) != 0
}
// function s.target(e,tp,eg,ep,ev,re,r,rp,chk)
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    let c = effect::get_handler(d, a.e).nil();
    if a.chk == Some(0) {
        // return e:GetHandler():GetFlagEffect(id+1)==0
        return card::get_flag_effect(d, c, ID + 1) == 0;
    }
    // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,e:GetHandler(),1,0,0)
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, Some(CardSet::from_card(c)), 1, 0, 0);
    // e:GetHandler():RegisterFlagEffect(id+1,RESET_EVENT|RESET_TURN_SET|RESET_TOGRAVE|RESET_TEMP_REMOVE|RESET_TOHAND|RESET_TODECK|RESET_OVERLAY|RESET_PHASE|PHASE_END,0,1)
    card::register_flag_effect(
        d,
        c,
        ID + 1,
        RESET_EVENT | RESET_TURN_SET | RESET_TOGRAVE | RESET_TEMP_REMOVE | RESET_TOHAND | RESET_TODECK | RESET_OVERLAY | RESET_PHASE | PHASE_END,
        0,
        1,
        0,
        0,
    );
    true
}
// function s.operation(e,tp,eg,ep,ev,re,r,rp)
fn operation_sp(d: &mut Duel, a: &EffArgs) {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if c:IsRelateToEffect(e) then
    if card::is_relate_to_effect(d, c, a.e) {
        // if Duel.GetLocationCount(tp,LOCATION_MZONE)<=0 then
        if duel::get_location_count(d, a.tp, LOCATION_MZONE as u8, None, None, None).0 <= 0 {
            // Duel.SendtoGrave(e:GetHandler(),REASON_EFFECT)
            duel::send_to_grave(d, c, REASON_EFFECT, None, None);
            // return
            return;
        }
        // Duel.SpecialSummon(c,0,tp,tp,false,false,POS_FACEUP_ATTACK)
        duel::special_summon(d, c, 0, a.tp, a.tp, false, false, POS_FACEUP_ATTACK as u8, None);
    }
}
