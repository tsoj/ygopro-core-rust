//! Mecha Phantom Beast Jaculuslan
use super::prelude::*;

pub const ID: u32 = 26949946;

pub const INFO: ScriptInfo = ScriptInfo {
    listed_names: &[TOKEN_MECHA_PHANTOM_BEAST],
    listed_series: &[SET_MECHA_PHANTOM_BEAST],
    ..ScriptInfo::DEFAULT
};

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // synchro summon
    // Synchro.AddProcedure(c,aux.FilterBoolFunctionEx(Card.IsSetCard,SET_MECHA_PHANTOM_BEAST),1,1,Synchro.NonTunerEx(Card.IsSetCard,SET_MECHA_PHANTOM_BEAST),1,99)
    synchro::add_procedure(
        d,
        c,
        Some(mat_filter(|d, c, sc, st, tp| card::is_set_card_ctx(d, c, &[SET_MECHA_PHANTOM_BEAST as u16], sc, st, Some(tp)))),
        1,
        1,
        Some(synchro::non_tuner_ex(|d, c, sc, st, tp| card::is_set_card_ctx(d, c, &[SET_MECHA_PHANTOM_BEAST as u16], sc, st, Some(tp)))),
        1,
        99,
        None,
        None,
        None,
        None,
        None,
    );
    // c:EnableReviveLimit()
    card::enable_revive_limit(d, c);
    // handes
    let e1 = effect::create_effect(d, c);
    effect::set_description(d, e1, aux::stringid(ID, 0));
    effect::set_category(d, e1, CATEGORY_HANDES);
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_O);
    effect::set_code(d, e1, EVENT_SPSUMMON_SUCCESS);
    effect::set_condition(d, e1, Some(condition(hdcon)));
    effect::set_cost(d, e1, Some(cost_target(hdcost)));
    effect::set_target(d, e1, Some(cost_target(hdtg)));
    effect::set_operation(d, e1, Some(operation(hdop)));
    card::register_effect(d, c, e1, false);
    // indes
    let e2 = effect::create_effect(d, c);
    effect::set_type(d, e2, EFFECT_TYPE_FIELD);
    effect::set_code(d, e2, EFFECT_INDESTRUCTABLE_BATTLE);
    effect::set_range(d, e2, LOCATION_MZONE);
    effect::set_target_range(d, e2, LOCATION_MZONE, 0);
    effect::set_target(d, e2, Some(effect_card_fn(indtg)));
    effect::set_value(d, e2, 1);
    card::register_effect(d, c, e2, false);
    let e3 = effect::clone(d, e2);
    effect::set_code(d, e3, EFFECT_INDESTRUCTABLE_EFFECT);
    card::register_effect(d, c, e3, false);
    // set
    let e4 = effect::create_effect(d, c);
    effect::set_description(d, e4, aux::stringid(ID, 1));
    effect::set_category(d, e4, CATEGORY_SET);
    effect::set_type(d, e4, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_O);
    effect::set_property(d, e4, EFFECT_FLAG_DELAY + EFFECT_FLAG_DAMAGE_STEP, 0);
    effect::set_code(d, e4, EVENT_DESTROYED);
    effect::set_condition(d, e4, Some(condition(setcon)));
    effect::set_target(d, e4, Some(cost_target(settg)));
    effect::set_operation(d, e4, Some(operation(setop)));
    card::register_effect(d, c, e4, false);
}
/// `s.hdcon(e,tp,eg,ep,ev,re,r,rp)`
fn hdcon(d: &mut Duel, a: &EffArgs) -> bool {
    let h = effect::get_handler(d, a.e).nil();
    card::is_synchro_summoned(d, h)
}
/// `s.hdcost(e,tp,eg,ep,ev,re,r,rp,chk)`
fn hdcost(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    if a.chk == Some(0) {
        return duel::check_release_group_cost(d, tp, Some(&|d, c| card::is_code(d, c, &[TOKEN_MECHA_PHANTOM_BEAST])), 1, 1, false, None, NO_EX)
            && duel::get_field_group_count(d, tp, 0, LOCATION_HAND) > 0;
    }
    // local ct=Duel.GetFieldGroupCount(tp,0,LOCATION_HAND)
    let ct = duel::get_field_group_count(d, tp, 0, LOCATION_HAND);
    // local g=Duel.SelectReleaseGroupCost(tp,Card.IsCode,1,ct,false,nil,nil,TOKEN_MECHA_PHANTOM_BEAST)
    let g = duel::select_release_group_cost(d, tp, Some(&|d, c| card::is_code(d, c, &[TOKEN_MECHA_PHANTOM_BEAST])), 1, ct as usize, false, None, NO_EX);
    // e:SetLabel(#g)
    effect::set_label(d, a.e, &[g.len() as i64]);
    // Duel.Release(g,REASON_COST)
    duel::release(d, g, REASON_COST, None);
    true
}
/// `s.hdtg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn hdtg(d: &mut Duel, a: &EffArgs) -> bool {
    if a.chk == Some(0) {
        return true;
    }
    let label = effect::get_label(d, a.e);
    duel::set_operation_info(d, 0, CATEGORY_HANDES, None, 0, 1 - a.tp, label as i32);
    true
}
/// `s.hdop(e,tp,eg,ep,ev,re,r,rp)`
fn hdop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // local g=Duel.GetFieldGroup(tp,0,LOCATION_HAND)
    let g = duel::get_field_group(d, tp, 0, LOCATION_HAND);
    // if #g>0 then
    if !g.is_empty() {
        // local sg=g:RandomSelect(1-tp,e:GetLabel())
        let label = effect::get_label(d, a.e);
        let sg = group::random_select(d, &g, 1 - tp, label as usize);
        // Duel.SendtoGrave(sg,REASON_DISCARD|REASON_EFFECT)
        duel::send_to_grave(d, sg, REASON_DISCARD | REASON_EFFECT, None, None);
    }
}
/// `s.indtg(e,c)`: `c:IsSetCard(SET_MECHA_PHANTOM_BEAST) and c~=e:GetHandler()`
fn indtg(d: &mut Duel, e: EffectId, c: CardId) -> bool {
    card::is_set_card(d, c, &[SET_MECHA_PHANTOM_BEAST as u16]) && c != effect::get_handler(d, e).nil()
}
/// `s.setcon(e,tp,eg,ep,ev,re,r,rp)`: `rp~=tp and e:GetHandler():IsPreviousControler(tp)`
fn setcon(d: &mut Duel, a: &EffArgs) -> bool {
    let h = effect::get_handler(d, a.e).nil();
    a.rp != a.tp && card::is_previous_controler(d, h, a.tp)
}
/// `s.filter(c)`: `c:IsQuickPlaySpell() and c:IsSSetable()`
fn filter(d: &mut Duel, c: CardId) -> bool {
    card::is_quick_play_spell(d, c) && card::is_s_setable(d, c, false, None)
}
fn settg(d: &mut Duel, a: &EffArgs) -> bool {
    if a.chk == Some(0) {
        return duel::get_location_count(d, a.tp, LOCATION_SZONE as u8, None, None, None).0 > 0 && duel::is_existing_matching_card(d, Some(&filter), a.tp, LOCATION_DECK, 0, 1, NO_EX);
    }
    true
}
fn setop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_SET as u64);
    let g = duel::select_matching_card(d, tp, Some(&filter), tp, LOCATION_DECK, 0, 1, 1, false, NO_EX).nil();
    if !g.is_empty() {
        let tc = group::get_first(d, &g).nil();
        duel::s_set(d, tp, tc, None, true);
    }
}
