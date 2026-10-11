//! Lunalight Cat Dancer
use super::prelude::*;

pub const ID: u32 = 51777272;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // fusion material
    // Fusion.AddProcMixN(c,true,true,aux.FilterBoolFunctionEx(Card.IsSetCard,SET_LUNALIGHT),2)
    fusion::add_proc_mix_n(
        d,
        c,
        true,
        true,
        vec![(
            fusion::Mat::F(fusion::fus_mat(|d, c, fc, st, tp| card::is_set_card_ctx(d, c, &[SET_LUNALIGHT as u16], Some(fc), st, Some(tp)))),
            2,
        )],
    );
    card::enable_revive_limit(d, c);

    // battle indestructable
    let e1 = effect::create_effect(d, c);
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
    effect::set_code(d, e1, EFFECT_INDESTRUCTABLE_BATTLE);
    effect::set_value(d, e1, 1);
    card::register_effect(d, c, e1, false);

    // Multiple attacks
    let e2 = effect::create_effect(d, c);
    effect::set_description(d, e2, aux::stringid(ID, 0));
    effect::set_type(d, e2, EFFECT_TYPE_IGNITION);
    effect::set_range(d, e2, LOCATION_MZONE);
    effect::set_count_limit(d, e2, 1, 0, 0, 0);
    effect::set_condition(d, e2, Some(condition(cond)));
    effect::set_cost(d, e2, Some(cost_target(cost_fn)));
    effect::set_operation(d, e2, Some(operation(op)));
    card::register_effect(d, c, e2, false);

    // atk
    let e3 = effect::create_effect(d, c);
    effect::set_description(d, e3, aux::stringid(ID, 1));
    effect::set_category(d, e3, CATEGORY_DAMAGE);
    effect::set_type(d, e3, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_F);
    effect::set_code(d, e3, EVENT_ATTACK_ANNOUNCE);
    effect::set_target(d, e3, Some(cost_target(damtg)));
    effect::set_operation(d, e3, Some(operation(damop)));
    card::register_effect(d, c, e3, false);
}

pub const INFO: ScriptInfo = ScriptInfo {
    listed_series: &[SET_LUNALIGHT],
    material_setcode: &[SET_LUNALIGHT as u32],
    ..ScriptInfo::DEFAULT
};

fn cond(d: &mut Duel, _a: &EffArgs) -> bool {
    duel::is_able_to_enter_bp(d)
}

fn cost_fn(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    let h = effect::get_handler(d, a.e).nil();
    if a.chk == Some(0) {
        return duel::check_release_group_cost(
            d,
            tp,
            Some(&|d, c| card::is_set_card(d, c, &[SET_LUNALIGHT as u16])),
            1,
            1,
            false,
            None,
            h.into(),
        );
    }
    let g = duel::select_release_group_cost(
        d,
        tp,
        Some(&|d, c| card::is_set_card(d, c, &[SET_LUNALIGHT as u16])),
        1,
        1,
        false,
        None,
        h.into(),
    );
    duel::release(d, g, REASON_COST, None);
    true
}

fn op(d: &mut Duel, a: &EffArgs) {
    let (e, tp) = (a.e, a.tp);
    let c = effect::get_handler(d, e).nil();
    let e1 = effect::create_effect(d, c);
    effect::set_type(d, e1, EFFECT_TYPE_FIELD);
    effect::set_code(d, e1, EFFECT_INDESTRUCTABLE_COUNT);
    effect::set_target_range(d, e1, 0, LOCATION_MZONE);
    effect::set_value_fn(d, e1, lua_fn(indct));
    effect::set_reset(d, e1, RESET_PHASE | PHASE_END, 1);
    duel::register_effect(d, e1, tp);
    if card::is_relate_to_effect(d, c, e) {
        let e2 = effect::create_effect(d, c);
        effect::set_type(d, e2, EFFECT_TYPE_SINGLE);
        effect::set_code(d, e2, EFFECT_ATTACK_ALL);
        effect::set_property(d, e2, EFFECT_FLAG_CANNOT_DISABLE, 0);
        effect::set_reset(d, e2, RESETS_STANDARD_PHASE_END, 1);
        effect::set_value(d, e2, 2);
        card::register_effect(d, c, e2, false);
    }
}

/// `s.indct(e,re,r,rp)`
fn indct(_d: &mut Duel, a: Args) -> i64 {
    let r = a.int(2) as u32;
    if (r & REASON_BATTLE) != 0 {
        1
    } else {
        0
    }
}

fn damtg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    if a.chk == Some(0) {
        return true;
    }
    duel::set_target_player(d, 1 - tp);
    duel::set_target_param(d, 100);
    duel::set_operation_info(d, 0, CATEGORY_DAMAGE, None, 0, 1 - tp, 100);
    true
}

fn damop(d: &mut Duel, _a: &EffArgs) {
    let info = duel::get_chain_info(d, 0, &[CHAININFO_TARGET_PLAYER, CHAININFO_TARGET_PARAM]);
    let p = info[0].int() as u8;
    let dam = info[1].int();
    duel::damage(d, p, dam, REASON_EFFECT, false, None);
}
