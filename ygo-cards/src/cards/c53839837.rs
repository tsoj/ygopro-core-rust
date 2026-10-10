//! Vampire Lord
use super::prelude::*;

pub const ID: u32 = 53839837;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // spsummon
    let e1 = effect::create_effect(d, c);
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE + EFFECT_TYPE_CONTINUOUS);
    effect::set_property(d, e1, EFFECT_FLAG_CANNOT_DISABLE, 0);
    effect::set_code(d, e1, EVENT_TO_GRAVE);
    effect::set_operation(d, e1, Some(operation(spr)));
    card::register_effect(d, c, e1, false);
    let e2 = effect::create_effect(d, c);
    effect::set_description(d, e2, aux::stringid(ID, 4));
    effect::set_category(d, e2, CATEGORY_SPECIAL_SUMMON);
    effect::set_type(d, e2, EFFECT_TYPE_FIELD + EFFECT_TYPE_TRIGGER_F);
    effect::set_range(d, e2, LOCATION_GRAVE);
    effect::set_code(d, e2, EVENT_PHASE | PHASE_STANDBY);
    effect::set_condition(d, e2, Some(condition(spcon)));
    effect::set_target(d, e2, Some(cost_target(sptg)));
    effect::set_operation(d, e2, Some(operation(spop)));
    effect::set_label_object(d, e2, Value::Effect(e1));
    card::register_effect(d, c, e2, false);
    // to grave
    let e3 = effect::create_effect(d, c);
    effect::set_description(d, e3, aux::stringid(ID, 0));
    effect::set_category(d, e3, CATEGORY_TOGRAVE);
    effect::set_type(d, e3, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_F);
    effect::set_code(d, e3, EVENT_BATTLE_DAMAGE);
    effect::set_condition(d, e3, Some(condition(tgcon)));
    effect::set_target(d, e3, Some(cost_target(tgtg)));
    effect::set_operation(d, e3, Some(operation(tgop)));
    card::register_effect(d, c, e3, false);
}
fn spr(d: &mut Duel, a: &EffArgs) {
    let c = effect::get_handler(d, a.e).nil();
    if (a.r & (REASON_DESTROY | REASON_EFFECT)) != (REASON_DESTROY | REASON_EFFECT) || a.rp == a.tp || !card::is_previous_controler(d, c, a.tp) {
        return;
    }
    let reset = RESET_EVENT | RESETS_STANDARD | RESET_PHASE | PHASE_STANDBY | RESET_SELF_TURN;
    if duel::is_turn_player(d, a.tp) && duel::is_phase(d, PHASE_STANDBY) {
        let turn = duel::get_turn_count(d, None);
        effect::set_label(d, a.e, &[turn as i64]);
        card::register_flag_effect(d, c, ID, reset, 0, 2, 0, 0);
    } else {
        effect::set_label(d, a.e, &[0]);
        card::register_flag_effect(d, c, ID, reset, 0, 1, 0, 0);
    }
}
fn spcon(d: &mut Duel, a: &EffArgs) -> bool {
    // return e:GetLabelObject():GetLabel()~=Duel.GetTurnCount() and tp==Duel.GetTurnPlayer() and c:GetFlagEffect(id)>0
    let c = effect::get_handler(d, a.e).nil();
    let e1 = effect::get_label_object(d, a.e).as_effect().nil();
    effect::get_label(d, e1) != duel::get_turn_count(d, None) as i64 && a.tp == duel::get_turn_player(d) && card::get_flag_effect(d, c, ID) > 0
}
fn sptg(d: &mut Duel, a: &EffArgs) -> bool {
    if a.chk == Some(0) {
        return true;
    }
    let c = effect::get_handler(d, a.e).nil();
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, Some(c.into()), 1, 0, 0);
    card::reset_flag_effect(d, c, ID);
    true
}
fn spop(d: &mut Duel, a: &EffArgs) {
    let c = effect::get_handler(d, a.e).nil();
    if card::is_relate_to_effect(d, c, a.e) {
        duel::special_summon(d, c, 0, a.tp, a.tp, false, false, POS_FACEUP as u8, None);
    }
}
fn tgcon(_d: &mut Duel, a: &EffArgs) -> bool {
    a.ep != a.tp
}
fn tgtg(d: &mut Duel, a: &EffArgs) -> bool {
    if a.chk == Some(0) {
        return true;
    }
    let (e, tp) = (a.e, a.tp);
    duel::hint(d, HINT_SELECTMSG as u8, tp, aux::stringid(ID, 5));
    let op = duel::select_option(d, tp, true, &[aux::stringid(ID, 1), aux::stringid(ID, 2), aux::stringid(ID, 3)]);
    effect::set_label(d, e, &[op as i64]);
    duel::set_operation_info(d, 0, CATEGORY_TOGRAVE, None, 1, 1 - tp, LOCATION_DECK as i32);
    true
}
fn tgfilter(d: &mut Duel, c: CardId, ty: u32) -> bool {
    card::is_type(d, c, ty) && card::is_able_to_grave(d, c, None)
}
fn tgop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    duel::hint(d, HINT_SELECTMSG as u8, 1 - tp, HINTMSG_TOGRAVE as u64);
    let label = effect::get_label(d, a.e);
    let g = if label == 0 {
        duel::select_matching_card(d, 1 - tp, Some(&|d, c| tgfilter(d, c, TYPE_MONSTER)), 1 - tp, LOCATION_DECK, 0, 1, 1, false, NO_EX).nil()
    } else if label == 1 {
        duel::select_matching_card(d, 1 - tp, Some(&|d, c| tgfilter(d, c, TYPE_SPELL)), 1 - tp, LOCATION_DECK, 0, 1, 1, false, NO_EX).nil()
    } else {
        duel::select_matching_card(d, 1 - tp, Some(&|d, c| tgfilter(d, c, TYPE_TRAP)), 1 - tp, LOCATION_DECK, 0, 1, 1, false, NO_EX).nil()
    };
    duel::send_to_grave(d, g, REASON_EFFECT, None, None);
}
