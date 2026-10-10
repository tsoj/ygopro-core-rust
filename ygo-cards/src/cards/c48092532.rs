//! D.D. Survivor
use super::prelude::*;

pub const ID: u32 = 48092532;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // removed
    let e1 = effect::create_effect(d, c);
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE | EFFECT_TYPE_CONTINUOUS);
    effect::set_code(d, e1, EVENT_REMOVE);
    effect::set_condition(d, e1, Some(condition(rmcon)));
    effect::set_operation(d, e1, Some(operation(rmop)));
    card::register_effect(d, c, e1, false);
    // special summon
    let e2 = effect::create_effect(d, c);
    effect::set_description(d, e2, aux::stringid(ID, 0));
    effect::set_category(d, e2, CATEGORY_SPECIAL_SUMMON);
    effect::set_type(d, e2, EFFECT_TYPE_FIELD | EFFECT_TYPE_TRIGGER_F);
    effect::set_code(d, e2, EVENT_PHASE | PHASE_END);
    effect::set_range(d, e2, LOCATION_REMOVED);
    effect::set_condition(d, e2, Some(condition(cond)));
    effect::set_target(d, e2, Some(cost_target(target)));
    effect::set_operation(d, e2, Some(operation(op)));
    card::register_effect(d, c, e2, false);
}
/// `s.rmcon(e,tp,eg,ep,ev,re,r,rp)`
fn rmcon(d: &mut Duel, a: &EffArgs) -> bool {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // return c:IsFaceup() and c:IsPreviousPosition(POS_FACEUP) and c:IsPreviousLocation(LOCATION_ONFIELD) and c:IsPreviousControler(tp)
    card::is_faceup(d, c)
        && card::is_previous_position(d, c, POS_FACEUP)
        && card::is_previous_location(d, c, LOCATION_ONFIELD)
        && card::is_previous_controler(d, c, a.tp)
}
/// `s.rmop(e,tp,eg,ep,ev,re,r,rp)`
fn rmop(d: &mut Duel, a: &EffArgs) {
    // e:GetHandler():RegisterFlagEffect(id,RESETS_STANDARD_PHASE_END,0,1)
    let c = effect::get_handler(d, a.e).nil();
    card::register_flag_effect(d, c, ID, RESETS_STANDARD_PHASE_END, 0, 1, 0, 0);
}
/// `s.condition(e,tp,eg,ep,ev,re,r,rp)`
fn cond(d: &mut Duel, a: &EffArgs) -> bool {
    // return e:GetHandler():GetFlagEffect(id)~=0
    let c = effect::get_handler(d, a.e).nil();
    card::get_flag_effect(d, c, ID) != 0
}
/// `s.target(e,tp,eg,ep,ev,re,r,rp,chk)`
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    let h = effect::get_handler(d, a.e);
    // if chk==0 then return e:GetHandler():GetFlagEffect(id+1)==0 end
    if a.chk == Some(0) {
        return card::get_flag_effect(d, h.nil(), ID + 1) == 0;
    }
    // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,e:GetHandler(),1,0,0)
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, h.map(CardSet::from_card), 1, 0, 0);
    // e:GetHandler():RegisterFlagEffect(id+1,RESET_EVENT|RESET_TURN_SET|RESET_TOGRAVE|RESET_TEMP_REMOVE|RESET_TOHAND|RESET_TODECK|RESET_OVERLAY|RESET_PHASE|PHASE_END,0,1)
    card::register_flag_effect(
        d,
        h.nil(),
        ID + 1,
        RESET_EVENT | RESET_TURN_SET | RESET_TOGRAVE | RESET_TEMP_REMOVE | RESET_TOHAND | RESET_TODECK | RESET_OVERLAY | RESET_PHASE | PHASE_END,
        0,
        1,
        0,
        0,
    );
    true
}
/// `s.operation(e,tp,eg,ep,ev,re,r,rp)`
fn op(d: &mut Duel, a: &EffArgs) {
    let (e, tp) = (a.e, a.tp);
    // local c=e:GetHandler()
    let c = effect::get_handler(d, e).nil();
    // if c:IsRelateToEffect(e) then
    if card::is_relate_to_effect(d, c, e) {
        // if Duel.GetLocationCount(tp,LOCATION_MZONE)<=0 then
        if duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 <= 0 {
            // Duel.SendtoGrave(e:GetHandler(),REASON_EFFECT)
            duel::send_to_grave(d, c, REASON_EFFECT, None, None);
            // return
            return;
        }
        // Duel.SpecialSummon(c,0,tp,tp,false,false,POS_FACEUP)
        duel::special_summon(d, c, 0, tp, tp, false, false, POS_FACEUP as u8, None);
    }
}
