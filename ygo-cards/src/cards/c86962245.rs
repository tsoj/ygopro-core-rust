//! Salamangreat Foxer
//! scripted by Larry126
use super::prelude::*;

pub const ID: u32 = 86962245;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // destroy
    let e1 = effect::create_effect(d, c);
    effect::set_description(d, e1, aux::stringid(ID, 0));
    effect::set_category(d, e1, CATEGORY_DESTROY + CATEGORY_TODECK);
    effect::set_property(d, e1, EFFECT_FLAG_CARD_TARGET, 0);
    effect::set_type(d, e1, EFFECT_TYPE_IGNITION);
    effect::set_range(d, e1, LOCATION_HAND);
    effect::set_condition(d, e1, Some(condition(cond)));
    effect::set_count_limit(d, e1, 1, ID, 0, 0);
    effect::set_cost(d, e1, Some(cost::self_to_grave()));
    effect::set_target(d, e1, Some(cost_target(target)));
    effect::set_operation(d, e1, Some(operation(operation_fn)));
    card::register_effect(d, c, e1, false);

    // spsummon
    let e2 = effect::create_effect(d, c);
    effect::set_description(d, e2, aux::stringid(ID, 1));
    effect::set_category(d, e2, CATEGORY_SPECIAL_SUMMON);
    effect::set_type(d, e2, EFFECT_TYPE_FIELD + EFFECT_TYPE_TRIGGER_O);
    effect::set_property(d, e2, EFFECT_FLAG_DELAY, 0);
    effect::set_code(d, e2, EVENT_TO_GRAVE);
    effect::set_range(d, e2, LOCATION_GRAVE);
    effect::set_condition(d, e2, Some(condition(spcon)));
    effect::set_count_limit(d, e2, 1, ID, 0, 0);
    effect::set_target(d, e2, Some(cost_target(sptg)));
    effect::set_operation(d, e2, Some(operation(spop)));
    card::register_effect(d, c, e2, false);
}

pub const INFO: ScriptInfo = ScriptInfo {
    listed_series: &[SET_SALAMANGREAT],
    ..ScriptInfo::DEFAULT
};

fn cfilter(d: &mut Duel, c: CardId) -> bool {
    card::is_monster(d, c) && card::is_set_card(d, c, &[SET_SALAMANGREAT as u16])
}

fn cond(d: &mut Duel, a: &EffArgs) -> bool {
    duel::is_existing_matching_card(d, Some(&cfilter), a.tp, LOCATION_GRAVE, 0, 3, NO_EX)
}

fn filter(d: &mut Duel, c: CardId) -> bool {
    card::get_sequence(d, c) < 5
}

fn tdfilter(d: &mut Duel, c: CardId) -> bool {
    card::is_monster(d, c) && card::is_set_card(d, c, &[SET_SALAMANGREAT as u16]) && card::is_link_monster(d, c) && card::is_able_to_extra(d, c)
}

fn target(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    let e = a.e;
    if a.chkc().is_some() {
        return false;
    }
    if a.chk == Some(0) {
        return duel::is_existing_target(d, Some(&filter), tp, 0, LOCATION_SZONE, 1, NO_EX)
            && duel::is_existing_target(d, Some(&tdfilter), tp, LOCATION_GRAVE, 0, 1, NO_EX);
    }
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_TODECK as u64);
    let lg = duel::select_target(d, tp, Some(&tdfilter), tp, LOCATION_GRAVE, 0, 1, 1, false, NO_EX);
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_TARGET as u64);
    let g = duel::select_target(d, tp, Some(&filter), tp, 0, LOCATION_SZONE, 1, 1, false, NO_EX).nil();
    let first = group::get_first(d, &g);
    effect::set_label_object(d, e, first.map_or(Value::Nil, Value::Card));
    duel::set_operation_info(d, 0, CATEGORY_DESTROY, Some(g), 1, 1 - tp, LOCATION_SZONE as i32);
    duel::set_operation_info(d, 0, CATEGORY_TODECK, lg, 1, tp, LOCATION_GRAVE as i32);
    true
}

fn operation_fn(d: &mut Duel, a: &EffArgs) {
    let e = a.e;
    let sc = effect::get_label_object(d, e).as_card();
    let info = duel::get_chain_info(d, 0, &[CHAININFO_TARGET_CARDS]);
    let g = info[0].as_group().cloned().nil();
    let mut lc = group::get_first(d, &g);
    if lc == sc {
        lc = group::iter(d, &g).get(1).copied();
    }
    if let Some(lc) = lc {
        if card::is_relate_to_effect(d, lc, e) && duel::send_to_deck(d, lc, None, SEQ_DECKSHUFFLE as i32, REASON_EFFECT, None) > 0 {
            if let Some(sc) = sc {
                if card::is_relate_to_effect(d, sc, e) {
                    duel::destroy(d, sc, REASON_EFFECT, None, None);
                }
            }
        }
    }
}

fn spfilter(d: &mut Duel, c: CardId, tp: u8) -> bool {
    card::is_reason(d, c, REASON_DESTROY)
        && card::is_previous_location(d, c, LOCATION_SZONE)
        && !card::is_previous_controler(d, c, tp)
        && card::get_previous_sequence(d, c) < 5
}

fn spcon(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    let eg = a.eg.clone().nil();
    group::is_exists(d, &eg, &|d, c| spfilter(d, c, tp), 1, NO_EX)
}

fn sptg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    let h = effect::get_handler(d, a.e).nil();
    if a.chk == Some(0) {
        return duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 > 0
            && card::is_can_be_special_summoned(d, h, a.e, 0, tp, false, false, None, None, None);
    }
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, Some(CardSet::from_card(h)), 1, tp, LOCATION_GRAVE as i32);
    true
}

fn spop(d: &mut Duel, a: &EffArgs) {
    let c = effect::get_handler(d, a.e).nil();
    if card::is_relate_to_effect(d, c, a.e) {
        duel::special_summon(d, c, 0, a.tp, a.tp, false, false, POS_FACEUP_DEFENSE as u8, None);
    }
}
