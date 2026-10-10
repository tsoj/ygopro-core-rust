//! Tualatin
use super::prelude::*;

pub const ID: u32 = 27769400;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate
    let e1 = effect::create_effect(d, c);
    effect::set_description(d, e1, aux::stringid(ID, 0));
    effect::set_category(d, e1, CATEGORY_SPECIAL_SUMMON);
    effect::set_type(d, e1, EFFECT_TYPE_FIELD + EFFECT_TYPE_TRIGGER_O);
    effect::set_code(d, e1, EVENT_CUSTOM + ID);
    effect::set_property(d, e1, EFFECT_FLAG_DAMAGE_STEP, 0);
    effect::set_range(d, e1, LOCATION_HAND);
    effect::set_condition(d, e1, Some(condition(spcon)));
    effect::set_target(d, e1, Some(cost_target(sptg)));
    effect::set_operation(d, e1, Some(operation(spop)));
    card::register_effect(d, c, e1, false);
    // destroy
    let e2 = effect::create_effect(d, c);
    effect::set_description(d, e2, aux::stringid(ID, 1));
    effect::set_category(d, e2, CATEGORY_DESTROY);
    effect::set_type(d, e2, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_F);
    effect::set_code(d, e2, EVENT_SPSUMMON_SUCCESS);
    effect::set_condition(d, e2, Some(condition(descon)));
    effect::set_operation(d, e2, Some(operation(desop)));
    card::register_effect(d, c, e2, false);
    aux::global_check(d, ID, |d| {
        set_s_var(d, ID, 0, Value::Group(CardSet::new()));
        set_s_var(d, ID, 1, Value::Int(0));
        let ge1 = effect::global_effect(d);
        effect::set_type(d, ge1, EFFECT_TYPE_FIELD + EFFECT_TYPE_CONTINUOUS);
        effect::set_code(d, ge1, EVENT_PHASE_START | PHASE_BATTLE_START);
        effect::set_operation(d, ge1, Some(operation(checkop1)));
        duel::register_effect(d, ge1, 0);
        let ge2 = effect::global_effect(d);
        effect::set_type(d, ge2, EFFECT_TYPE_FIELD + EFFECT_TYPE_CONTINUOUS);
        effect::set_code(d, ge2, EVENT_BATTLE_DESTROYED);
        effect::set_operation(d, ge2, Some(operation(checkop2)));
        duel::register_effect(d, ge2, 0);
    });
}
fn checkop1(d: &mut Duel, _a: &EffArgs) {
    // s[0]:Clear()
    // s[0]:Merge(Duel.GetFieldGroup(Duel.GetTurnPlayer(),0,LOCATION_MZONE))
    let tp = duel::get_turn_player(d);
    let g = duel::get_field_group(d, tp, 0, LOCATION_MZONE);
    // s[1]=s[0]:GetCount()
    let n = g.len() as i64;
    set_s_var(d, ID, 0, Value::Group(g));
    set_s_var(d, ID, 1, Value::Int(n));
}
fn checkop2(d: &mut Duel, a: &EffArgs) {
    // if s[1]<2 or s[0]:GetCount()==0 then return end
    let cnt = s_var(d, ID, 1).int();
    let mut g0 = s_var(d, ID, 0).as_group().cloned().nil();
    if cnt < 2 || g0.is_empty() {
        return;
    }
    // local g=eg:Filter(Card.IsLocation,nil,LOCATION_GRAVE)
    let eg = a.eg.clone().nil();
    let g = group::filter(d, &eg, &|d, c| card::is_location(d, c, LOCATION_GRAVE), NO_EX);
    // s[0]:Sub(g)
    group::remove_card(&mut g0, &g);
    set_s_var(d, ID, 0, Value::Group(g0.clone()));
    // if s[0]:GetCount()==0 then
    if g0.is_empty() {
        // Duel.RaiseEvent(e:GetHandler(),EVENT_CUSTOM+id,e,0,0,0,0)
        let h = effect::get_handler(d, a.e).nil();
        duel::raise_event(d, CardSet::from_card(h), EVENT_CUSTOM + ID, Some(a.e), 0, 0, 0, 0);
    }
}
fn spcon(d: &mut Duel, a: &EffArgs) -> bool {
    duel::is_turn_player(d, 1 - a.tp)
}
fn sptg(d: &mut Duel, a: &EffArgs) -> bool {
    let h = effect::get_handler(d, a.e).nil();
    if a.chk == Some(0) {
        return duel::get_location_count(d, a.tp, LOCATION_MZONE as u8, None, None, None).0 > 0
            && card::is_can_be_special_summoned(d, h, a.e, 0, a.tp, false, false, None, None, None);
    }
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, Some(CardSet::from_card(h)), 1, 0, 0);
    true
}
fn spop(d: &mut Duel, a: &EffArgs) {
    let c = effect::get_handler(d, a.e).nil();
    if !card::is_relate_to_effect(d, c, a.e) {
        return;
    }
    duel::special_summon(d, c, 1, a.tp, a.tp, false, false, POS_FACEUP as u8, None);
}
fn descon(d: &mut Duel, a: &EffArgs) -> bool {
    let h = effect::get_handler(d, a.e).nil();
    card::get_summon_type(d, h) == SUMMON_TYPE_SPECIAL + 1
}
fn sumlimit(d: &mut Duel, e: EffectId, c: CardId) -> bool {
    // return c:IsAttribute(e:GetLabel())
    let attr = effect::get_label(d, e);
    card::is_attribute(d, c, attr as u32)
}
fn desop(d: &mut Duel, a: &EffArgs) {
    let c = effect::get_handler(d, a.e).nil();
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_ATTRIBUTE as u64);
    let rc = duel::announce_attribute(d, a.tp, 1, ATTRIBUTE_ALL);
    let g = duel::get_matching_group(d, Some(&|d, tc| card::is_faceup(d, tc) && card::is_attribute(d, tc, rc)), a.tp, LOCATION_MZONE, LOCATION_MZONE, NO_EX);
    duel::destroy(d, g, REASON_EFFECT, None, None);
    if card::is_relate_to_effect(d, c, a.e) {
        let e1 = effect::create_effect(d, c);
        effect::set_type(d, e1, EFFECT_TYPE_FIELD);
        effect::set_range(d, e1, LOCATION_MZONE);
        effect::set_code(d, e1, EFFECT_CANNOT_SUMMON);
        effect::set_property(d, e1, EFFECT_FLAG_PLAYER_TARGET, 0);
        effect::set_target_range(d, e1, 0, 1);
        effect::set_target(d, e1, Some(effect_card_fn(sumlimit)));
        effect::set_reset(d, e1, RESET_EVENT | RESETS_STANDARD, 1);
        effect::set_label(d, e1, &[rc as i64]);
        card::register_effect(d, c, e1, false);
        let e2 = effect::clone(d, e1);
        effect::set_code(d, e2, EFFECT_CANNOT_SPECIAL_SUMMON);
        card::register_effect(d, c, e2, false);
    }
}
