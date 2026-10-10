//! Crackdown
use super::prelude::*;

pub const ID: u32 = 36975314;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate
    let e1 = effect::create_effect(d, c);
    effect::set_category(d, e1, CATEGORY_CONTROL);
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    effect::set_property(d, e1, EFFECT_FLAG_CARD_TARGET, 0);
    effect::set_target(d, e1, Some(cost_target(target)));
    card::register_effect(d, c, e1, false);
    let e2 = effect::create_effect(d, c);
    effect::set_type(d, e2, EFFECT_TYPE_FIELD + EFFECT_TYPE_CONTINUOUS);
    effect::set_property(d, e2, EFFECT_FLAG_CANNOT_DISABLE, 0);
    effect::set_range(d, e2, LOCATION_SZONE);
    effect::set_code(d, e2, EVENT_CHAIN_SOLVED);
    effect::set_label_object(d, e2, Value::Effect(e1));
    effect::set_condition(d, e2, Some(aux::persistent_tg_con()));
    effect::set_operation(d, e2, Some(operation(tgop)));
    card::register_effect(d, c, e2, false);
    // control
    let e3 = effect::create_effect(d, c);
    effect::set_property(d, e3, EFFECT_FLAG_SET_AVAILABLE, 0);
    effect::set_type(d, e3, EFFECT_TYPE_FIELD);
    effect::set_code(d, e3, EFFECT_SET_CONTROL);
    effect::set_range(d, e3, LOCATION_SZONE);
    effect::set_target_range(d, e3, LOCATION_MZONE, LOCATION_MZONE);
    effect::set_target(d, e3, Some(aux::persistent_target_filter()));
    effect::set_value_fn(d, e3, effect_card_value(tg));
    card::register_effect(d, c, e3, false);
    // cannot attack
    let e4 = effect::create_effect(d, c);
    effect::set_type(d, e4, EFFECT_TYPE_FIELD);
    effect::set_code(d, e4, EFFECT_CANNOT_ATTACK);
    effect::set_range(d, e4, LOCATION_SZONE);
    effect::set_target_range(d, e4, LOCATION_MZONE, 0);
    effect::set_target(d, e4, Some(aux::persistent_target_filter()));
    card::register_effect(d, c, e4, false);
    // cannot activate
    let e5 = effect::create_effect(d, c);
    effect::set_type(d, e5, EFFECT_TYPE_FIELD);
    effect::set_code(d, e5, EFFECT_CANNOT_TRIGGER);
    effect::set_range(d, e5, LOCATION_SZONE);
    effect::set_target_range(d, e5, LOCATION_MZONE, 0);
    effect::set_target(d, e5, Some(aux::persistent_target_filter()));
    card::register_effect(d, c, e5, false);
    // destroy
    let e6 = effect::create_effect(d, c);
    effect::set_type(d, e6, EFFECT_TYPE_CONTINUOUS + EFFECT_TYPE_FIELD);
    effect::set_range(d, e6, LOCATION_SZONE);
    effect::set_code(d, e6, EVENT_LEAVE_FIELD);
    effect::set_condition(d, e6, Some(condition(descon)));
    effect::set_operation(d, e6, Some(operation(desop)));
    card::register_effect(d, c, e6, false);
}
fn filter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsFaceup() and c:IsControlerCanBeChanged()
    card::is_faceup(d, c) && card::is_controler_can_be_changed(d, c, false, None)
}
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    if let Some(chkc) = a.chkc() {
        return card::is_location(d, chkc, LOCATION_MZONE) && card::is_controler(d, chkc, 1 - a.tp) && filter(d, chkc);
    }
    if a.chk == Some(0) {
        return duel::is_existing_target(d, Some(&filter), a.tp, 0, LOCATION_MZONE, 1, NO_EX);
    }
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_CONTROL as u64);
    let g = duel::select_target(d, a.tp, Some(&filter), a.tp, 0, LOCATION_MZONE, 1, 1, false, NO_EX);
    duel::set_operation_info(d, 0, CATEGORY_CONTROL, g, 1, 0, 0);
    true
}
fn tgop(d: &mut Duel, a: &EffArgs) {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // local tc=Duel.GetChainInfo(ev,CHAININFO_TARGET_CARDS):GetFirst()
    let info = duel::get_chain_info(d, a.ev as u8, &[CHAININFO_TARGET_CARDS]);
    let tcs = info[0].as_group().cloned().nil();
    let tc = group::get_first(d, &tcs);
    // if c:IsRelateToEffect(re) and tc and tc:IsRelateToEffect(re) then
    if !card::is_relate_to_effect(d, c, a.re.nil()) {
        return;
    }
    let Some(tc) = tc else {
        return;
    };
    if card::is_relate_to_effect(d, tc, a.re.nil()) {
        // c:SetCardTarget(tc)
        card::set_card_target(d, c, tc);
        // local e1=Effect.CreateEffect(c)
        let e1 = effect::create_effect(d, c);
        effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
        effect::set_code(d, e1, EFFECT_SET_CONTROL);
        effect::set_property(d, e1, EFFECT_FLAG_SINGLE_RANGE + EFFECT_FLAG_OWNER_RELATE, 0);
        effect::set_range(d, e1, LOCATION_MZONE);
        effect::set_value(d, e1, a.tp as i64);
        effect::set_reset(d, e1, RESET_EVENT | (RESETS_STANDARD & !RESET_TURN_SET), 1);
        effect::set_condition(d, e1, Some(condition(con)));
        card::register_effect(d, tc, e1, false);
    }
}
fn con(d: &mut Duel, a: &EffArgs) -> bool {
    // local c=e:GetOwner()
    // local h=e:GetHandler()
    // return c:IsHasCardTarget(h)
    let c = effect::get_owner(d, a.e).nil();
    let h = effect::get_handler(d, a.e).nil();
    card::is_has_card_target(d, c, h)
}
fn tg(d: &mut Duel, e: EffectId, _c: CardId) -> i64 {
    // return e:GetHandlerPlayer()
    effect::get_handler_player(d, e) as i64
}
fn descon(d: &mut Duel, a: &EffArgs) -> bool {
    // local tc=e:GetHandler():GetFirstCardTarget()
    // return tc and eg:IsContains(tc)
    let h = effect::get_handler(d, a.e).nil();
    match card::get_first_card_target(d, h) {
        Some(tc) => {
            let eg = a.eg.clone().nil();
            group::is_contains(&eg, tc)
        }
        None => false,
    }
}
fn desop(d: &mut Duel, a: &EffArgs) {
    // Duel.Destroy(e:GetHandler(),REASON_EFFECT)
    let h = effect::get_handler(d, a.e).nil();
    duel::destroy(d, h, REASON_EFFECT, None, None);
}
