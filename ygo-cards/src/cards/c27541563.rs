//! Altergeist Protocol
use super::prelude::*;

pub const ID: u32 = 27541563;

pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_ALTERGEIST], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // activate
    let e1 = effect::create_effect(d, c);
    effect::set_description(d, e1, aux::stringid(ID, 0));
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    effect::set_condition(d, e1, Some(condition(stat_change_cond)));
    card::register_effect(d, c, e1, false);
    // negate
    let e2 = effect::create_effect(d, c);
    effect::set_description(d, e2, aux::stringid(ID, 1));
    effect::set_category(d, e2, CATEGORY_NEGATE + CATEGORY_DESTROY);
    effect::set_type(d, e2, EFFECT_TYPE_ACTIVATE);
    effect::set_code(d, e2, EVENT_CHAINING);
    effect::set_property(d, e2, EFFECT_FLAG_DAMAGE_STEP + EFFECT_FLAG_DAMAGE_CAL, 0);
    effect::set_count_limit(d, e2, 1, ID, 0, 0);
    effect::set_condition(d, e2, Some(condition(discon)));
    effect::set_cost(d, e2, Some(cost_target(discost)));
    effect::set_target(d, e2, Some(cost_target(distg)));
    effect::set_operation(d, e2, Some(operation(disop)));
    card::register_effect(d, c, e2, false);
    let e3 = effect::clone(d, e2);
    effect::set_type(d, e3, EFFECT_TYPE_QUICK_O);
    effect::set_range(d, e3, LOCATION_SZONE);
    card::register_effect(d, c, e3, false);
    // inactivatable
    let e4 = effect::create_effect(d, c);
    effect::set_type(d, e4, EFFECT_TYPE_FIELD);
    effect::set_code(d, e4, EFFECT_CANNOT_INACTIVATE);
    effect::set_range(d, e4, LOCATION_SZONE);
    effect::set_value_fn(d, e4, lua_fn(effectfilter));
    card::register_effect(d, c, e4, false);
    let e5 = effect::create_effect(d, c);
    effect::set_type(d, e5, EFFECT_TYPE_FIELD);
    effect::set_code(d, e5, EFFECT_CANNOT_DISEFFECT);
    effect::set_range(d, e5, LOCATION_SZONE);
    effect::set_value_fn(d, e5, lua_fn(effectfilter));
    card::register_effect(d, c, e5, false);
}

fn stat_change_cond(d: &mut Duel, _a: &EffArgs) -> bool {
    aux::stat_change_damage_step_condition(d)
}

fn discon(d: &mut Duel, a: &EffArgs) -> bool {
    let re = a.re.nil();
    a.ep == 1 - a.tp && effect::is_monster_effect(d, re) && duel::is_chain_negatable(d, a.ev as u8)
}

fn discfilter(d: &mut Duel, c: CardId) -> bool {
    card::is_faceup(d, c)
        && card::is_set_card(d, c, &[SET_ALTERGEIST as u16])
        && card::is_able_to_grave_as_cost(d, c)
        && !card::is_status(d, c, STATUS_BATTLE_DESTROYED)
}

fn discost(d: &mut Duel, a: &EffArgs) -> bool {
    let c = effect::get_handler(d, a.e).nil();
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, Some(&discfilter), a.tp, LOCATION_ONFIELD, 0, 1, c.into());
    }
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_TOGRAVE as u64);
    let g = duel::select_matching_card(d, a.tp, Some(&discfilter), a.tp, LOCATION_ONFIELD, 0, 1, 1, false, c.into()).nil();
    duel::send_to_grave(d, g, REASON_COST, None, None);
    true
}

fn distg(d: &mut Duel, a: &EffArgs) -> bool {
    if a.chk == Some(0) {
        return true;
    }
    duel::set_operation_info(d, 0, CATEGORY_NEGATE, a.eg.clone(), 1, 0, 0);
    let re = a.re.nil();
    let rh = effect::get_handler(d, re).nil();
    if card::is_relate_to_effect(d, rh, re) && card::is_destructable(d, rh, None) {
        duel::set_operation_info(d, 0, CATEGORY_DESTROY, a.eg.clone(), 1, 0, 0);
    }
    true
}

fn disop(d: &mut Duel, a: &EffArgs) {
    let c = effect::get_handler(d, a.e).nil();
    if !card::is_relate_to_effect(d, c, a.e) {
        return;
    }
    if duel::negate_activation(d, a.ev as u8) {
        let re = a.re.nil();
        let rh = effect::get_handler(d, re).nil();
        if card::is_relate_to_effect(d, rh, re) {
            duel::destroy(d, a.eg(), REASON_EFFECT, None, None);
        }
    }
}

fn effectfilter(d: &mut Duel, a: Args) -> bool {
    let h = effect::get_handler(d, a.e(0)).nil();
    let p = card::get_controler(d, h);
    let ct = a.int(1) as u8;
    let info = duel::get_chain_info(d, ct, &[CHAININFO_TRIGGERING_EFFECT, CHAININFO_TRIGGERING_PLAYER, CHAININFO_TRIGGERING_LOCATION]);
    let te = info[0].as_effect();
    let tp = info[1].int() as u8;
    let loc = info[2].int();
    if p != tp {
        return false;
    }
    let teh = effect::get_handler(d, te.nil()).nil();
    card::is_set_card(d, teh, &[SET_ALTERGEIST as u16]) && (loc & LOCATION_ONFIELD as i64) != 0
}
