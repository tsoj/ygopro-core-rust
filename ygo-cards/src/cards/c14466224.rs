//! The Atmosphere
use super::prelude::*;

pub const ID: u32 = 14466224;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // c:EnableReviveLimit()
    card::enable_revive_limit(d, c);

    // Special Summon this card (from your hand) by banishing 2 monsters you control and 1 monster from your GY
    let e1 = effect::create_effect(d, c);
    effect::set_type(d, e1, EFFECT_TYPE_FIELD);
    effect::set_code(d, e1, EFFECT_SPSUMMON_PROC);
    effect::set_property(d, e1, EFFECT_FLAG_UNCOPYABLE, 0);
    effect::set_range(d, e1, LOCATION_HAND);
    effect::set_condition(d, e1, Some(lua_fn(spcon)));
    effect::set_target(d, e1, Some(lua_fn(sptg)));
    effect::set_operation(d, e1, Some(lua_fn(spop)));
    card::register_effect(d, c, e1, false);

    // Equip a face-up monster your opponent controls to this card
    let e2 = effect::create_effect(d, c);
    effect::set_description(d, e2, aux::stringid(ID, 0));
    effect::set_category(d, e2, CATEGORY_EQUIP);
    effect::set_type(d, e2, EFFECT_TYPE_IGNITION);
    effect::set_property(d, e2, EFFECT_FLAG_CARD_TARGET, 0);
    effect::set_range(d, e2, LOCATION_MZONE);
    effect::set_count_limit(d, e2, 1, 0, 0, 0);
    effect::set_condition(d, e2, Some(condition(eqcon)));
    effect::set_target(d, e2, Some(cost_target(eqtg)));
    effect::set_operation(d, e2, Some(operation(eqop)));
    card::register_effect(d, c, e2, false);

    // aux.AddEREquipLimit(c,s.eqcon,function(ec,_,tp) return ec:IsControler(1-tp) end,s.equipop,e2)
    aux::add_er_equip_limit(
        d,
        c,
        Some(condition(eqcon)),
        lua_fn(|d, a: Args| card::is_controler(d, a.c(0), 1 - a.p(2))),
        lua_fn(|d, a: Args| {
            equipop(d, a.c(0), a.e(1), a.p(2), a.c(3));
            Value::Nil
        }),
        e2,
        None,
        None,
        None,
    );
}
fn gfilter(d: &mut Duel, c: CardId) -> bool {
    card::is_monster(d, c) && card::is_able_to_remove_as_cost(d, c, None)
}
fn removable_mzone(d: &mut Duel, c: CardId) -> bool {
    card::is_able_to_remove_as_cost(d, c, None)
}
/// `s.spcon(e,c)`
fn spcon(d: &mut Duel, a: Args) -> bool {
    let e = a.e(0);
    let c = match a.card(1) {
        None => return true,
        Some(c) => c,
    };
    let tp = card::get_controler(d, c);
    let rg1 = duel::get_matching_group(d, Some(&removable_mzone), tp, LOCATION_MZONE, 0, NO_EX);
    let rg2 = duel::get_matching_group(d, Some(&gfilter), tp, LOCATION_GRAVE, 0, NO_EX);
    if duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 <= -2 {
        return false;
    }
    let chkf = aux::chkf_mmz(1);
    let rs: aux::Rescon = &chkf;
    if duel::is_player_affected_by_effect(d, tp, CARD_SPIRIT_ELIMINATION).is_some() {
        matches!(aux::select_unselect_group(d, &rg1, Some(e), tp, Some(3), Some(3), Some(rs), 0, tp, None, None, None, false), Err(true))
    } else {
        matches!(aux::select_unselect_group(d, &rg1, Some(e), tp, Some(2), Some(2), Some(rs), 0, tp, None, None, None, false), Err(true))
            && matches!(aux::select_unselect_group(d, &rg2, Some(e), tp, Some(1), Some(1), Some(rs), 0, tp, None, None, None, false), Err(true))
    }
}
/// `s.sptg(e,tp,eg,ep,ev,re,r,rp,c)`
fn sptg(d: &mut Duel, a: Args) -> bool {
    let e = a.e(0);
    let tp = a.p(1);
    let rg1 = duel::get_matching_group(d, Some(&removable_mzone), tp, LOCATION_MZONE, 0, NO_EX);
    let rg2 = duel::get_matching_group(d, Some(&gfilter), tp, LOCATION_GRAVE, 0, NO_EX);
    let chkf = aux::chkf_mmz(1);
    let rs: aux::Rescon = &chkf;
    let g1 = if duel::is_player_affected_by_effect(d, tp, CARD_SPIRIT_ELIMINATION).is_some() {
        aux::select_unselect_group(d, &rg1, Some(e), tp, Some(3), Some(3), Some(rs), 1, tp, Some(HINTMSG_REMOVE as u64), None, None, true).ok().nil()
    } else {
        let mut g1 = aux::select_unselect_group(d, &rg1, Some(e), tp, Some(2), Some(2), Some(rs), 1, tp, Some(HINTMSG_REMOVE as u64), None, None, true).ok().nil();
        if !g1.is_empty() {
            let g2 = aux::select_unselect_group(d, &rg2, Some(e), tp, Some(1), Some(1), Some(rs), 1, tp, Some(HINTMSG_REMOVE as u64), None, None, true).ok().nil();
            group::add_card(&mut g1, &g2);
        }
        g1
    };
    if g1.len() > 2 {
        effect::set_label_object(d, e, Value::Group(g1));
        return true;
    }
    false
}
/// `s.spop(e,tp,eg,ep,ev,re,r,rp,c)`
fn spop(d: &mut Duel, a: Args) -> Value {
    let e = a.e(0);
    let g1 = effect::get_label_object(d, e).as_group().cloned();
    if let Some(g1) = g1 {
        duel::remove(d, g1, POS_FACEUP as u8, REASON_COST, None, None);
    }
    Value::Nil
}
/// `s.eqcon(e,tp,eg,ep,ev,re,r,rp)`
fn eqcon(d: &mut Duel, a: &EffArgs) -> bool {
    let h = effect::get_handler(d, a.e).nil();
    let g = card::get_equip_group(d, h);
    group::filter(d, &g, &eqfilter, NO_EX).len() == 0
}
fn eqfilter(d: &mut Duel, c: CardId) -> bool {
    card::get_flag_effect(d, c, ID) != 0
}
fn filter(d: &mut Duel, c: CardId) -> bool {
    card::is_faceup(d, c) && card::is_able_to_change_controler(d, c)
}
fn eqtg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    if let Some(chkc) = a.chkc() {
        return card::is_location(d, chkc, LOCATION_MZONE) && card::is_controler(d, chkc, 1 - tp) && filter(d, chkc);
    }
    if a.chk == Some(0) {
        return duel::get_location_count(d, tp, LOCATION_SZONE as u8, None, None, None).0 > 0
            && duel::is_existing_target(d, Some(&filter), tp, 0, LOCATION_MZONE, 1, NO_EX);
    }
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_EQUIP as u64);
    let g = duel::select_target(d, tp, Some(&filter), tp, 0, LOCATION_MZONE, 1, 1, false, NO_EX);
    duel::set_operation_info(d, 0, CATEGORY_EQUIP, g, 1, tp, 0);
    true
}
/// `s.equipop(c,e,tp,tc)`
fn equipop(d: &mut Duel, c: CardId, e: EffectId, tp: u8, tc: CardId) {
    let mut atk = card::get_text_attack(d, tc);
    let mut def = card::get_text_defense(d, tc);
    if atk < 0 {
        atk = 0;
    }
    if def < 0 {
        def = 0;
    }
    if !card::equip_by_effect_and_limit_register(d, c, e, tp, tc, Some(ID), None) {
        return;
    }
    if atk > 0 {
        let e1 = effect::create_effect(d, c);
        effect::set_type(d, e1, EFFECT_TYPE_EQUIP);
        effect::set_property(d, e1, EFFECT_FLAG_IGNORE_IMMUNE + EFFECT_FLAG_OWNER_RELATE, 0);
        effect::set_code(d, e1, EFFECT_UPDATE_ATTACK);
        effect::set_reset(d, e1, RESET_EVENT | RESETS_STANDARD, 1);
        effect::set_value(d, e1, atk as i64);
        card::register_effect(d, tc, e1, false);
    }
    if def > 0 {
        let e2 = effect::create_effect(d, c);
        effect::set_type(d, e2, EFFECT_TYPE_EQUIP);
        effect::set_property(d, e2, EFFECT_FLAG_IGNORE_IMMUNE + EFFECT_FLAG_OWNER_RELATE, 0);
        effect::set_code(d, e2, EFFECT_UPDATE_DEFENSE);
        effect::set_reset(d, e2, RESET_EVENT | RESETS_STANDARD, 1);
        effect::set_value(d, e2, def as i64);
        card::register_effect(d, tc, e2, false);
    }
}
fn eqop(d: &mut Duel, a: &EffArgs) {
    let c = effect::get_handler(d, a.e).nil();
    let tc = duel::first_target(d);
    if let Some(tc) = tc {
        if card::is_faceup(d, tc) && card::is_relate_to_effect(d, tc, a.e) {
            equipop(d, c, a.e, a.tp, tc);
        }
    }
}
