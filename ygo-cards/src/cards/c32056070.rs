//! You and A.I.
use super::prelude::*;

pub const ID: u32 = 32056070;

pub const INFO: ScriptInfo = ScriptInfo { listed_names: &[TOKEN_IGNISTER], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate
    let e1 = effect::create_effect(d, c);
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    card::register_effect(d, c, e1, false);
    // Apply effect based on the attribute of the summoned monster
    let e2 = effect::create_effect(d, c);
    effect::set_description(d, e2, aux::stringid(ID, 0));
    effect::set_type(d, e2, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_O);
    effect::set_code(d, e2, EVENT_CUSTOM + ID);
    effect::set_property(d, e2, EFFECT_FLAG_DELAY, 0);
    effect::set_range(d, e2, LOCATION_SZONE);
    effect::set_target(d, e2, Some(cost_target(target)));
    card::register_effect(d, c, e2, false);
    // local g=Group.CreateGroup() g:KeepAlive() e2:SetLabelObject(g): the group lives in the label object of e2
    effect::set_label_object(d, e2, Value::Group(CardSet::new()));
    // Resgister summons
    let e3 = effect::create_effect(d, c);
    effect::set_description(d, e3, aux::stringid(ID, 1));
    effect::set_type(d, e3, EFFECT_TYPE_FIELD + EFFECT_TYPE_CONTINUOUS);
    effect::set_code(d, e3, EVENT_SPSUMMON_SUCCESS);
    effect::set_property(d, e3, EFFECT_FLAG_CANNOT_DISABLE, 0);
    effect::set_range(d, e3, LOCATION_SZONE);
    effect::set_label_object(d, e3, Value::Effect(e2));
    effect::set_operation(d, e3, Some(operation(regop)));
    card::register_effect(d, c, e3, false);
    // Register attributes
    aux::global_check(d, ID, |d| {
        // s.attr_list={} s.attr_list[0]=0 s.attr_list[1]=0
        set_s_var(d, ID, 0, Value::Int(0));
        set_s_var(d, ID, 1, Value::Int(0));
        aux::add_values_reset(d, reset_attr_list);
    });
}
/// `aux.AddValuesReset(function() s.attr_list[0]=0 s.attr_list[1]=0 end)`
fn reset_attr_list(d: &mut Duel) {
    set_s_var(d, ID, 0, Value::Int(0));
    set_s_var(d, ID, 1, Value::Int(0));
}
/// `s.attr_list[tp]`
fn attr_list(d: &Duel, tp: u8) -> i64 {
    s_var(d, ID, tp as u64).int()
}
/// `s.filter(c,e,tp)`
fn filter(d: &mut Duel, c: CardId, e: EffectId, tp: u8) -> bool {
    let attr = card::get_attribute(d, c) as i64;
    if !(card::is_faceup(d, c) && card::is_race(d, c, RACE_CYBERSE) && card::get_base_attack(d, c) == 2300 && attr_list(d, tp) & attr == 0) {
        return false;
    }
    let _ = e;
    if attr & (ATTRIBUTE_EARTH | ATTRIBUTE_WATER) as i64 != 0 {
        atktg(d, tp, 0)
    } else if attr & (ATTRIBUTE_WIND | ATTRIBUTE_LIGHT) as i64 != 0 {
        distg(d, tp, 0)
    } else if attr & (ATTRIBUTE_DARK | ATTRIBUTE_FIRE) as i64 != 0 {
        tktg(d, tp, 0)
    } else {
        false
    }
}
/// `s.regop(e,tp,eg,ep,ev,re,r,rp)`
fn regop(d: &mut Duel, a: &EffArgs) {
    let (e, tp) = (a.e, a.tp);
    let eg = a.eg.clone().nil();
    let tg = group::filter(d, &eg, &|d, c| filter(d, c, e, tp), NO_EX);
    if !tg.is_empty() {
        for tc in group::iter(d, &tg) {
            card::register_flag_effect(d, tc, ID, RESET_CHAIN, 0, 1, 0, 0);
        }
        // local g=e:GetLabelObject():GetLabelObject()
        let e2 = effect::get_label_object(d, e).as_effect().nil();
        let mut g = effect::get_label_object(d, e2).as_group().cloned().nil();
        // if Duel.GetCurrentChain()==0 then g:Clear() end
        if duel::get_current_chain(d, false) == 0 {
            g = CardSet::new();
        }
        // g:Merge(tg)
        group::add_card(&mut g, &tg);
        // g:Remove(function(c) return c:GetFlagEffect(id)==0 end,nil)
        group::remove(d, &mut g, &|d, c| card::get_flag_effect(d, c, ID) == 0, NO_EX);
        // e:GetLabelObject():SetLabelObject(g)
        effect::set_label_object(d, e2, Value::Group(g));
        // Duel.RaiseSingleEvent(e:GetHandler(),EVENT_CUSTOM+id,e,0,tp,tp,0)
        let h = effect::get_handler(d, e).nil();
        duel::raise_single_event(d, h, EVENT_CUSTOM + ID, Some(e), 0, tp, tp, 0);
    }
}
/// `s.target(e,tp,eg,ep,ev,re,r,rp,chk)`
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    let (e, tp) = (a.e, a.tp);
    let set = effect::get_label_object(d, e).as_group().cloned().nil();
    let g = group::filter(d, &set, &|d, c| filter(d, c, e, tp), NO_EX);
    if a.chk == Some(0) {
        return !g.is_empty() && duel::get_flag_effect(d, tp, ID) == 0;
    }
    duel::register_flag_effect(d, tp, ID, RESET_CHAIN, 0, 1, 0);
    let sc = if g.len() == 1 {
        group::get_first(d, &g)
    } else {
        duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_FACEUP as u64);
        let sel = group::select(d, &g, tp, 1, 1, false, NO_EX).nil();
        group::get_first(d, &sel)
    }
    .nil();
    let attr = card::get_attribute(d, sc) as i64;
    effect::set_label(d, e, &[attr]);
    if attr & (ATTRIBUTE_EARTH | ATTRIBUTE_WATER) as i64 != 0 {
        effect::set_category(d, e, CATEGORY_ATKCHANGE);
        effect::set_operation(d, e, Some(operation(atkop)));
        atktg(d, tp, 1);
    } else if attr & (ATTRIBUTE_WIND | ATTRIBUTE_LIGHT) as i64 != 0 {
        effect::set_category(d, e, CATEGORY_DISABLE);
        effect::set_operation(d, e, Some(operation(disop)));
        distg(d, tp, 1);
    } else if attr & (ATTRIBUTE_DARK | ATTRIBUTE_FIRE) as i64 != 0 {
        effect::set_category(d, e, CATEGORY_SPECIAL_SUMMON + CATEGORY_TOKEN);
        effect::set_operation(d, e, Some(operation(tkop)));
        tktg(d, tp, 1);
    }
    true
}
/// `s.operation(c,tp,att)`
fn set_attr(d: &mut Duel, c: CardId, tp: u8, att: i64) {
    let cur = attr_list(d, tp);
    set_s_var(d, ID, tp as u64, Value::Int(cur | att));
    for s in aux::get_attribute_strings(att as u64) {
        card::register_flag_effect(d, c, 0, RESETS_STANDARD_PHASE_END, EFFECT_FLAG_CLIENT_HINT, 1, 0, s);
    }
}
/// `s.atktg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn atktg(d: &mut Duel, tp: u8, chk: i64) -> bool {
    if chk == 0 {
        return duel::is_existing_matching_card(d, Some(&|d, c| card::has_non_zero_attack(d, c)), tp, LOCATION_MZONE, LOCATION_MZONE, 1, NO_EX);
    }
    false
}
/// `s.atkop(e,tp,eg,ep,ev,re,r,rp)`
fn atkop(d: &mut Duel, a: &EffArgs) {
    let (e, tp) = (a.e, a.tp);
    let c = effect::get_handler(d, e).nil();
    set_attr(d, c, tp, effect::get_label(d, e));
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_FACEUP as u64);
    let sel = duel::select_matching_card(d, tp, Some(&|d, x| card::has_non_zero_attack(d, x)), tp, LOCATION_MZONE, LOCATION_MZONE, 1, 1, false, NO_EX).nil();
    if let Some(tc) = group::get_first(d, &sel) {
        let atk = card::get_attack(d, tc) as f64;
        let e1 = effect::create_effect(d, c);
        effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
        effect::set_code(d, e1, EFFECT_SET_ATTACK_FINAL);
        effect::set_reset(d, e1, RESETS_STANDARD_PHASE_END, 1);
        // math.ceil(atk/2)
        effect::set_value(d, e1, (atk / 2.0).ceil() as i64);
        card::register_effect(d, tc, e1, false);
    }
}
/// `s.distg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn distg(d: &mut Duel, tp: u8, chk: i64) -> bool {
    if chk == 0 {
        return duel::is_existing_matching_card(d, Some(&|d, c| card::is_negatable(d, c)), tp, LOCATION_ONFIELD, LOCATION_ONFIELD, 1, NO_EX);
    }
    duel::set_operation_info(d, 0, CATEGORY_DISABLE, None, 1, PLAYER_ALL as u8, LOCATION_ONFIELD as i32);
    false
}
/// `s.disop(e,tp,eg,ep,ev,re,r,rp)`
fn disop(d: &mut Duel, a: &EffArgs) {
    let (e, tp) = (a.e, a.tp);
    let c = effect::get_handler(d, e).nil();
    set_attr(d, c, tp, effect::get_label(d, e));
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_NEGATE as u64);
    let sel = duel::select_matching_card(d, tp, Some(&|d, x| card::is_negatable(d, x)), tp, LOCATION_ONFIELD, LOCATION_ONFIELD, 1, 1, false, NO_EX).nil();
    if let Some(tc) = group::get_first(d, &sel) {
        let usable = card::is_faceup(d, tc) && !card::is_disabled(d, tc);
        if usable || card::is_type(d, tc, TYPE_TRAPMONSTER) {
            duel::negate_related_chain(d, tc, RESET_TURN_SET);
            let e1 = effect::create_effect(d, c);
            effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
            effect::set_property(d, e1, EFFECT_FLAG_CANNOT_DISABLE, 0);
            effect::set_code(d, e1, EFFECT_DISABLE);
            effect::set_reset(d, e1, RESETS_STANDARD_PHASE_END, 1);
            card::register_effect(d, tc, e1, false);
            let e2 = effect::create_effect(d, c);
            effect::set_type(d, e2, EFFECT_TYPE_SINGLE);
            effect::set_property(d, e2, EFFECT_FLAG_CANNOT_DISABLE, 0);
            effect::set_code(d, e2, EFFECT_DISABLE_EFFECT);
            effect::set_value(d, e2, RESET_TURN_SET as i64);
            effect::set_reset(d, e2, RESETS_STANDARD_PHASE_END, 1);
            card::register_effect(d, tc, e2, false);
            if card::is_type(d, tc, TYPE_TRAPMONSTER) {
                let e3 = effect::create_effect(d, c);
                effect::set_type(d, e3, EFFECT_TYPE_SINGLE);
                effect::set_property(d, e3, EFFECT_FLAG_CANNOT_DISABLE, 0);
                effect::set_code(d, e3, EFFECT_DISABLE_TRAPMONSTER);
                effect::set_reset(d, e3, RESETS_STANDARD_PHASE_END, 1);
                card::register_effect(d, tc, e3, false);
            }
        }
    }
}
/// `s.tktg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn tktg(d: &mut Duel, tp: u8, chk: i64) -> bool {
    if chk == 0 {
        return duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 > 0 && duel::is_player_can_special_summon_monster(d, tp, TOKEN_IGNISTER, ignister_token_data(), None, None, 0);
    }
    duel::set_operation_info(d, 0, CATEGORY_TOKEN, None, 1, 0, 0);
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, None, 1, tp, 0);
    false
}
/// `TOKEN_IGNISTER` as `Duel.IsPlayerCanSpecialSummonMonster(tp,TOKEN_IGNISTER,SET_IGNISTER,TYPES_TOKEN,0,0,1,RACE_CYBERSE,ATTRIBUTE_DARK)`
fn ignister_token_data() -> duel::MonsterData {
    duel::MonsterData {
        setcodes: Some(vec![SET_IGNISTER as u16]),
        type_: Some(TYPES_TOKEN),
        attack: Some(0),
        defense: Some(0),
        level: Some(1),
        race: Some(RACE_CYBERSE),
        attribute: Some(ATTRIBUTE_DARK),
    }
}
/// `s.tkop(e,tp,eg,ep,ev,re,r,rp)`
fn tkop(d: &mut Duel, a: &EffArgs) {
    let (e, tp) = (a.e, a.tp);
    let c = effect::get_handler(d, e).nil();
    set_attr(d, c, tp, effect::get_label(d, e));
    if duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 > 0 && duel::is_player_can_special_summon_monster(d, tp, TOKEN_IGNISTER, ignister_token_data(), None, None, 0) {
        let token = duel::create_token(d, tp, TOKEN_IGNISTER).nil();
        duel::special_summon(d, token, 0, tp, tp, false, false, POS_FACEUP as u8, None);
    }
}
