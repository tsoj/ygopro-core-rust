//! Battle Survivor
use super::prelude::*;

pub const ID: u32 = 64178868;

pub const INFO: ScriptInfo = ScriptInfo { listed_names: &[ID], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    let e1 = effect::create_effect(d, c);
    effect::set_description(d, e1, aux::stringid(ID, 0));
    effect::set_category(d, e1, CATEGORY_TOHAND);
    effect::set_type(d, e1, EFFECT_TYPE_FIELD + EFFECT_TYPE_TRIGGER_O);
    effect::set_code(d, e1, EVENT_PHASE | PHASE_BATTLE);
    effect::set_range(d, e1, LOCATION_MZONE);
    effect::set_count_limit(d, e1, 1, ID, 0, 0);
    effect::set_target(d, e1, Some(cost_target(thtg)));
    effect::set_operation(d, e1, Some(operation(thop)));
    card::register_effect(d, c, e1, false);
    let e2 = effect::create_effect(d, c);
    effect::set_type(d, e2, EFFECT_TYPE_CONTINUOUS + EFFECT_TYPE_FIELD);
    effect::set_property(d, e2, EFFECT_FLAG_DAMAGE_STEP + EFFECT_FLAG_CANNOT_DISABLE, 0);
    effect::set_code(d, e2, EVENT_TO_GRAVE);
    effect::set_range(d, e2, LOCATION_MZONE);
    effect::set_condition(d, e2, Some(condition(chkcond)));
    effect::set_operation(d, e2, Some(operation(checkop)));
    card::register_effect(d, c, e2, false);
}
// function s.filter(c,fid)
fn filter(d: &mut Duel, c: CardId, fid: u32) -> bool {
    if card::is_code(d, c, &[ID]) || !card::is_able_to_hand(d, c, None) {
        return false;
    }
    // local eff={c:GetCardEffect(id)}
    for te in card::get_card_effect(d, c, ID) {
        // if te:GetValue()==fid then return true end
        if matches!(effect::get_value(d, te), Value::Int(v) if v == fid as i64) {
            return true;
        }
    }
    false
}
// function s.thtg(e,tp,eg,ep,ev,re,r,rp,chk)
fn thtg(d: &mut Duel, a: &EffArgs) -> bool {
    if a.chk == Some(0) {
        let h = effect::get_handler(d, a.e).nil();
        let fid = card::get_field_id(d, h);
        return duel::is_existing_matching_card(d, Some(&|d, c| filter(d, c, fid)), a.tp, LOCATION_GRAVE, 0, 1, NO_EX);
    }
    duel::set_operation_info(d, 0, CATEGORY_TOHAND, None, 1, a.tp, LOCATION_GRAVE as i32);
    true
}
// function s.thop(e,tp,eg,ep,ev,re,r,rp)
fn thop(d: &mut Duel, a: &EffArgs) {
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_ATOHAND as u64);
    let h = effect::get_handler(d, a.e).nil();
    let fid = card::get_field_id(d, h);
    let g = duel::select_matching_card(d, a.tp, Some(&|d, c| filter(d, c, fid)), a.tp, LOCATION_GRAVE, 0, 1, 1, false, NO_EX).nil();
    if !g.is_empty() {
        duel::hint_selection(d, g.clone(), true);
        duel::send_to_hand(d, g.clone(), None, REASON_EFFECT, None);
        duel::confirm_cards(d, 1 - a.tp, g);
    }
}
// function s.chkcond(e,tp,eg,ep,ev,re,r,rp)
fn chkcond(d: &mut Duel, _a: &EffArgs) -> bool {
    duel::is_battle_phase(d, None)
}
// function s.checkop(e,tp,eg,ep,ev,re,r,rp)
fn checkop(d: &mut Duel, a: &EffArgs) {
    let c = effect::get_handler(d, a.e).nil();
    let fid = card::get_field_id(d, c);
    // eg:ForEach(function(tc) ... end)
    let eg = a.eg.clone().nil();
    for tc in group::iter(d, &eg) {
        let e1 = effect::create_effect(d, c);
        effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
        effect::set_code(d, e1, ID);
        effect::set_value(d, e1, fid as i64);
        effect::set_reset(d, e1, RESET_EVENT + RESETS_STANDARD + RESET_PHASE + PHASE_BATTLE, 1);
        card::register_effect(d, tc, e1, false);
    }
}
