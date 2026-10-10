//! Rune-Eyes Pendulum Dragon
use super::prelude::*;

pub const ID: u32 = 1516510;

pub const INFO: ScriptInfo = ScriptInfo {
    listed_names: &[16178681],
    material_setcode: &[SET_ODD_EYES, SET_PENDULUM, SET_PENDULUM_DRAGON],
    ..ScriptInfo::DEFAULT
};

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // c:EnableReviveLimit()
    card::enable_revive_limit(d, c);
    // Fusion.AddProcMix(c,true,true,16178681,aux.FilterBoolFunctionEx(Card.IsRace,RACE_SPELLCASTER))
    fusion::add_proc_mix(
        d,
        c,
        true,
        true,
        vec![
            fusion::Mat::Code(16178681),
            fusion::Mat::F(fusion::fus_mat(|d, c, fc, st, tp| card::is_race_ctx(d, c, RACE_SPELLCASTER, Some(fc), st, Some(tp)))),
        ],
    );
    // multi attack
    let e1 = effect::create_effect(d, c);
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE + EFFECT_TYPE_CONTINUOUS);
    effect::set_code(d, e1, EVENT_SPSUMMON_SUCCESS);
    effect::set_condition(d, e1, Some(condition(cond)));
    effect::set_operation(d, e1, Some(operation(op)));
    card::register_effect(d, c, e1, false);
    let e2 = effect::create_effect(d, c);
    effect::set_type(d, e2, EFFECT_TYPE_SINGLE);
    effect::set_code(d, e2, EFFECT_MATERIAL_CHECK);
    effect::set_value_fn(d, e2, effect_card_value(valcheck));
    effect::set_label_object(d, e2, Value::Effect(e1));
    card::register_effect(d, c, e2, false);
}
fn lvfilter(d: &mut Duel, c: CardId, fc: CardId) -> bool {
    card::is_code(d, c, &[16178681]) || card::check_fusion_substitute(d, c, fc)
}
fn imfilter(d: &mut Duel, c: CardId) -> bool {
    card::is_location(d, c, LOCATION_MZONE) && card::is_pendulum_summoned(d, c)
}
/// `s.valcheck(e,c)`
fn valcheck(d: &mut Duel, e: EffectId, c: CardId) -> i64 {
    let g = card::get_material(d, c);
    let mut flag: i64 = 0;
    if g.len() == 2 {
        let mut lv: u32;
        let lg1 = group::filter(d, &g, &|d, x| lvfilter(d, x, c), NO_EX);
        let lg2 = group::filter(d, &g, &|d, x| card::is_race_ctx(d, x, RACE_SPELLCASTER, Some(c), SUMMON_TYPE_FUSION as u64, None), NO_EX);
        if lg1.len() == 2 {
            let first2 = group::get_first(d, &lg2).nil();
            lv = card::get_original_level(d, first2);
            // local lc=lg2:GetNext()
            if let Some(lc) = group::iter(d, &lg2).get(1).copied() {
                lv = lv.max(card::get_original_level(d, lc));
            }
        } else {
            let mut lc = group::get_first(d, &g);
            if lc == group::get_first(d, &lg1) {
                lc = group::iter(d, &g).get(1).copied();
            }
            lv = card::get_original_level(d, lc.nil());
        }
        if lv > 4 {
            flag = 0x2;
        } else if lv > 0 {
            flag = 0x1;
        }
    }
    if group::is_exists(d, &g, &imfilter, 1, NO_EX) {
        flag += 0x4;
    }
    let lo = effect::get_label_object(d, e).as_effect().nil();
    effect::set_label(d, lo, &[flag]);
    0
}
fn cond(d: &mut Duel, a: &EffArgs) -> bool {
    let c = effect::get_handler(d, a.e).nil();
    card::is_fusion_summoned(d, c)
}
fn op(d: &mut Duel, a: &EffArgs) {
    let c = effect::get_handler(d, a.e).nil();
    let flag = effect::get_label(d, a.e);
    if (flag & 0x3) != 0 {
        let e1 = effect::create_effect(d, c);
        effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
        effect::set_code(d, e1, EFFECT_EXTRA_ATTACK_MONSTER);
        effect::set_property(d, e1, EFFECT_FLAG_CLIENT_HINT, 0);
        if (flag & 0x1) != 0 {
            effect::set_description(d, e1, aux::stringid(ID, 0));
            effect::set_value(d, e1, 1);
        } else {
            effect::set_description(d, e1, aux::stringid(ID, 1));
            effect::set_value(d, e1, 2);
        }
        effect::set_reset(d, e1, RESET_EVENT | RESETS_STANDARD, 1);
        card::register_effect(d, c, e1, false);
    }
    if (flag & 0x4) != 0 {
        let e4 = effect::create_effect(d, c);
        effect::set_type(d, e4, EFFECT_TYPE_SINGLE);
        effect::set_code(d, e4, EFFECT_IMMUNE_EFFECT);
        effect::set_value_fn(d, e4, lua_fn(efilter));
        effect::set_owner_player(d, e4, a.tp);
        effect::set_reset(d, e4, RESETS_STANDARD_PHASE_END, 1);
        card::register_effect(d, c, e4, false);
    }
}
fn efilter(d: &mut Duel, a: Args) -> bool {
    effect::get_owner_player(d, a.e(0)) != effect::get_owner_player(d, a.e(1))
}
