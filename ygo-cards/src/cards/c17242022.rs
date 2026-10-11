//! Red-Eyes Black Dragon Exceed
use super::prelude::*;

pub const ID: u32 = 17242022;

pub const INFO: ScriptInfo = ScriptInfo {
    listed_names: &[CARD_REDEYES_B_DRAGON, CARD_DARK_TIME_WIZARD, ID],
    material: &[CARD_REDEYES_B_DRAGON],
    material_setcode: &[SET_RED_EYES],
    ..ScriptInfo::DEFAULT
};

pub fn initial_effect(d: &mut Duel, c: CardId) {
    card::enable_revive_limit(d, c);
    // Fusion Materials: "Red-Eyes Black Dragon" + 1 monster that mentions "Dark Time Wizard"
    fusion::add_proc_mix(
        d,
        c,
        true,
        true,
        vec![
            fusion::Mat::Code(CARD_REDEYES_B_DRAGON),
            fusion::Mat::F(fusion::fus_mat(|d, c, _fc, _st, _tp| card::lists_code(d, c, &[CARD_DARK_TIME_WIZARD]))),
        ],
    );
    card::add_must_be_fusion_summoned(d, c);
    // Must be either Fusion Summoned, or Special Summoned (from your Extra Deck) by Tributing 1 face-up monster on either field during the turn a monster(s) was destroyed by the effect of "Dark Time Wizard"
    let e0a = effect::create_effect(d, c);
    effect::set_description(d, e0a, aux::stringid(ID, 0));
    effect::set_type(d, e0a, EFFECT_TYPE_FIELD);
    effect::set_property(d, e0a, EFFECT_FLAG_CANNOT_DISABLE + EFFECT_FLAG_UNCOPYABLE, 0);
    effect::set_code(d, e0a, EFFECT_SPSUMMON_PROC);
    effect::set_range(d, e0a, LOCATION_EXTRA);
    effect::set_condition(d, e0a, Some(lua_fn(selfspcon)));
    effect::set_target(d, e0a, Some(lua_fn(selfsptg)));
    effect::set_operation(d, e0a, Some(operation(selfspop)));
    effect::set_value(d, e0a, 1);
    card::register_effect(d, c, e0a, false);
    // You can only Special Summon "Red-Eyes Black Dragon Exceed" once per turn this way, no matter which method you use
    let e0b = effect::create_effect(d, c);
    effect::set_type(d, e0b, EFFECT_TYPE_SINGLE + EFFECT_TYPE_CONTINUOUS);
    effect::set_property(d, e0b, EFFECT_FLAG_CANNOT_DISABLE, 0);
    effect::set_code(d, e0b, EVENT_SPSUMMON_SUCCESS);
    effect::set_condition(d, e0b, Some(condition(regcon)));
    effect::set_operation(d, e0b, Some(operation(regop)));
    card::register_effect(d, c, e0b, false);
    // Keep track of a monster being destroyed by the effect of "Dark Time Wizard"
    aux::global_check(d, ID, |d| {
        let ge1 = effect::create_effect(d, c);
        effect::set_type(d, ge1, EFFECT_TYPE_FIELD + EFFECT_TYPE_CONTINUOUS);
        effect::set_code(d, ge1, EVENT_DESTROYED);
        effect::set_operation(d, ge1, Some(operation(ge1_op)));
        duel::register_effect(d, ge1, 0);
    });
    // If this card is Special Summoned: You can Special Summon 1 Level 8 or lower monster from your hand or GY
    let e1 = effect::create_effect(d, c);
    effect::set_description(d, e1, aux::stringid(ID, 1));
    effect::set_category(d, e1, CATEGORY_SPECIAL_SUMMON);
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_O);
    effect::set_property(d, e1, EFFECT_FLAG_DELAY, 0);
    effect::set_code(d, e1, EVENT_SPSUMMON_SUCCESS);
    effect::set_target(d, e1, Some(cost_target(lv8sptg)));
    effect::set_operation(d, e1, Some(operation(lv8spop)));
    card::register_effect(d, c, e1, false);
    // Unaffected by your opponent's activated monster and Spell effects
    let e2 = effect::create_effect(d, c);
    effect::set_type(d, e2, EFFECT_TYPE_SINGLE);
    effect::set_property(d, e2, EFFECT_FLAG_SINGLE_RANGE, 0);
    effect::set_code(d, e2, EFFECT_IMMUNE_EFFECT);
    effect::set_range(d, e2, LOCATION_MZONE);
    effect::set_value_fn(d, e2, lua_fn(immune_value));
    card::register_effect(d, c, e2, false);
}
fn selfspcostfilter(d: &mut Duel, c: CardId, tp: u8, fc: CardId) -> bool {
    card::is_releasable(d, c, None, None)
        && card::is_faceup(d, c)
        && card::is_can_be_fusion_material(d, c, Some(fc), Some(MATERIAL_FUSION as u64), Some(tp))
        && duel::get_location_count_from_ex(d, tp, Some(tp), c.into(), duel::ExtraCard::Card(fc), None).0 > 0
}
fn selfspcon(d: &mut Duel, a: Args) -> bool {
    // if c==nil then return true end
    let Some(c) = a.card(1) else {
        return true;
    };
    let tp = card::get_controler(d, c);
    let ra = duel::ReleaseArgs {
        use_hand: Some(false),
        max: Some(1),
        check_field: true,
        to_check: Some(c),
        toplayer: Some(tp),
        use_oppo: true,
        ..Default::default()
    };
    duel::has_flag_effect(d, 0, ID, None) && duel::check_release_group(d, tp, Some(&|d, x| selfspcostfilter(d, x, tp, c)), 1, ra, NO_EX, false)
}
fn selfsptg(d: &mut Duel, a: Args) -> bool {
    let e = a.e(0);
    let tp = a.p(1);
    let c = a.c(9);
    let ra = duel::ReleaseArgs {
        use_hand: Some(false),
        cancelable: true,
        check_field: true,
        to_check: Some(c),
        toplayer: Some(tp),
        use_oppo: true,
        ..Default::default()
    };
    let g = duel::select_release_group(d, tp, Some(&|d, x| selfspcostfilter(d, x, tp, c)), 1, 1, ra, NO_EX, false);
    if let Some(g) = g {
        if !g.is_empty() {
            effect::set_label_object(d, e, Value::Group(g));
            return true;
        }
    }
    false
}
fn selfspop(d: &mut Duel, a: &EffArgs) {
    if let Some(g) = effect::get_label_object(d, a.e).as_group().cloned() {
        if !g.is_empty() {
            duel::release(d, g, REASON_COST | REASON_MATERIAL, None);
        }
    }
}
fn regcon(d: &mut Duel, a: &EffArgs) -> bool {
    let c = effect::get_handler(d, a.e).nil();
    card::is_fusion_summoned(d, c) || card::is_summon_type(d, c, &[SUMMON_TYPE_SPECIAL + 1])
}
fn regop(d: &mut Duel, a: &EffArgs) {
    // You can only Special Summon "Red-Eyes Black Dragon Exceed" once per turn this way, no matter which method you use
    let h = effect::get_handler(d, a.e).nil();
    let e1 = effect::create_effect(d, h);
    effect::set_type(d, e1, EFFECT_TYPE_FIELD);
    effect::set_property(d, e1, EFFECT_FLAG_PLAYER_TARGET, 0);
    effect::set_code(d, e1, EFFECT_CANNOT_SPECIAL_SUMMON);
    effect::set_target_range(d, e1, 1, 0);
    effect::set_target(d, e1, Some(lua_fn(regop_target)));
    effect::set_reset(d, e1, RESET_PHASE | PHASE_END, 1);
    duel::register_effect(d, e1, a.tp);
}
fn regop_target(d: &mut Duel, a: Args) -> bool {
    let c = a.c(1);
    let st = a.int(3);
    let fus = SUMMON_TYPE_FUSION as i64;
    let sp = (SUMMON_TYPE_SPECIAL + 1) as i64;
    card::is_original_code_rule(d, c, &[ID]) && ((st & fus) == fus || (st & sp) == sp)
}
fn ge1_op(d: &mut Duel, a: &EffArgs) {
    if (a.r as u32 & REASON_EFFECT) > 0 {
        let h = effect::get_handler(d, a.re.nil()).nil();
        if card::is_code(d, h, &[CARD_DARK_TIME_WIZARD]) {
            duel::register_flag_effect(d, 0, ID, RESET_PHASE | PHASE_END, 0, 1, 0);
        }
    }
}
fn lv8spfilter(d: &mut Duel, c: CardId, e: EffectId, tp: u8) -> bool {
    card::is_level_below(d, c, 8) && card::is_can_be_special_summoned(d, c, e, 0, tp, false, false, None, None, None)
}
fn lv8sptg(d: &mut Duel, a: &EffArgs) -> bool {
    if a.chk == Some(0) {
        return duel::get_location_count(d, a.tp, LOCATION_MZONE as u8, None, None, None).0 > 0
            && duel::is_existing_matching_card(d, Some(&|d, c| lv8spfilter(d, c, a.e, a.tp)), a.tp, LOCATION_HAND | LOCATION_GRAVE, 0, 1, NO_EX);
    }
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, None, 1, a.tp, (LOCATION_HAND | LOCATION_GRAVE) as i32);
    true
}
fn lv8spop(d: &mut Duel, a: &EffArgs) {
    if duel::get_location_count(d, a.tp, LOCATION_MZONE as u8, None, None, None).0 < 1 {
        return;
    }
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_SPSUMMON as u64);
    let f = aux::necro_valley_filter(|d, c| lv8spfilter(d, c, a.e, a.tp));
    let g = duel::select_matching_card(d, a.tp, Some(&f), a.tp, LOCATION_HAND | LOCATION_GRAVE, 0, 1, 1, false, NO_EX).nil();
    if !g.is_empty() {
        duel::special_summon(d, g, 0, a.tp, a.tp, false, false, POS_FACEUP as u8, None);
    }
}
fn immune_value(d: &mut Duel, a: Args) -> bool {
    // return te:GetOwnerPlayer()==1-e:GetHandlerPlayer() and te:IsActivated() and (te:IsMonsterEffect() or te:IsSpellEffect())
    let te = a.e(1);
    effect::get_owner_player(d, te) == 1 - effect::get_handler_player(d, a.e(0))
        && effect::is_activated(d, te)
        && (effect::is_monster_effect(d, te) || effect::is_spell_effect(d, te))
}
