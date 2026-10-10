//! Necrovalley Temple
use super::prelude::*;

pub const ID: u32 = 56773577;

pub const INFO: ScriptInfo = ScriptInfo {
    listed_names: &[CARD_NECROVALLEY, ID],
    listed_series: &[SET_GRAVEKEEPERS, SET_NECROVALLEY],
    ..ScriptInfo::DEFAULT
};

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate
    let e0 = effect::create_effect(d, c);
    effect::set_type(d, e0, EFFECT_TYPE_ACTIVATE);
    effect::set_code(d, e0, EVENT_FREE_CHAIN);
    effect::set_property(d, e0, EFFECT_FLAG_DAMAGE_STEP, 0);
    effect::set_hint_timing(d, e0, TIMING_DAMAGE_STEP, Some(TIMING_DAMAGE_STEP));
    effect::set_condition(d, e0, Some(condition(dmgstep_cond)));
    card::register_effect(d, c, e0, false);
    // While a "Gravekeeper's" monster(s) and "Necrovalley" are both on the field, monsters your opponent controls lose 500 ATK/DEF
    let e1 = effect::create_effect(d, c);
    effect::set_type(d, e1, EFFECT_TYPE_FIELD);
    effect::set_code(d, e1, EFFECT_UPDATE_ATTACK);
    effect::set_range(d, e1, LOCATION_SZONE);
    effect::set_target_range(d, e1, 0, LOCATION_MZONE);
    effect::set_condition(d, e1, Some(condition(atkcon)));
    effect::set_value(d, e1, -500);
    card::register_effect(d, c, e1, false);
    // Decrease DEF
    let e2 = effect::clone(d, e1);
    effect::set_code(d, e2, EFFECT_UPDATE_DEFENSE);
    card::register_effect(d, c, e2, false);
    // Once per turn, during the Main Phase, if you control no card in your Field Zone: You can activate 1 "Necrovalley" directly from your hand or GY
    let e3 = effect::create_effect(d, c);
    effect::set_description(d, e3, aux::stringid(ID, 0));
    effect::set_type(d, e3, EFFECT_TYPE_QUICK_O);
    effect::set_range(d, e3, LOCATION_SZONE);
    effect::set_code(d, e3, EVENT_FREE_CHAIN);
    effect::set_count_limit(d, e3, 1, 0, 0, 0);
    effect::set_condition(d, e3, Some(condition(accon)));
    effect::set_operation(d, e3, Some(operation(acop)));
    card::register_effect(d, c, e3, false);
    // f this card in your possession is destroyed by an opponent's effect and sent to your GY: You can Set 1 "Necrovalley" Spell/Trap directly from your Deck, except "Necrovalley Temple"
    let e4 = effect::create_effect(d, c);
    effect::set_description(d, e4, aux::stringid(ID, 1));
    effect::set_category(d, e4, CATEGORY_SET);
    effect::set_type(d, e4, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_O);
    effect::set_code(d, e4, EVENT_TO_GRAVE);
    effect::set_property(d, e4, EFFECT_FLAG_DELAY, 0);
    effect::set_condition(d, e4, Some(condition(setcon)));
    effect::set_target(d, e4, Some(cost_target(settg)));
    effect::set_operation(d, e4, Some(operation(setop)));
    card::register_effect(d, c, e4, false);
}
/// `aux.StatChangeDamageStepCondition`
fn dmgstep_cond(d: &mut Duel, _a: &EffArgs) -> bool {
    aux::stat_change_damage_step_condition(d)
}
fn atkcon(d: &mut Duel, a: &EffArgs) -> bool {
    // return Duel.IsExistingMatchingCard(aux.FaceupFilter(Card.IsSetCard,SET_GRAVEKEEPERS),e:GetHandlerPlayer(),LOCATION_MZONE,LOCATION_MZONE,1,nil) and Duel.IsEnvironment(CARD_NECROVALLEY)
    let hp = effect::get_handler_player(d, a.e);
    duel::is_existing_matching_card(
        d,
        Some(&aux::faceup_filter(|d, c| card::is_set_card(d, c, &[SET_GRAVEKEEPERS as u16]))),
        hp,
        LOCATION_MZONE,
        LOCATION_MZONE,
        1,
        NO_EX,
    ) && duel::is_environment(d, CARD_NECROVALLEY, None, None)
}
fn accon(d: &mut Duel, a: &EffArgs) -> bool {
    // return Duel.IsMainPhase() and Duel.IsExistingMatchingCard(s.filter,tp,LOCATION_HAND|LOCATION_GRAVE,0,1,nil,tp)
    //     and not Duel.IsExistingMatchingCard(aux.TRUE,tp,LOCATION_FZONE,0,1,nil)
    let tp = a.tp;
    duel::is_main_phase(d, None)
        && duel::is_existing_matching_card(d, Some(&|d, c| filter(d, c, tp)), tp, LOCATION_HAND | LOCATION_GRAVE, 0, 1, NO_EX)
        && !duel::is_existing_matching_card(d, Some(&|_d, _c| true), tp, LOCATION_FZONE, 0, 1, NO_EX)
}
/// `s.filter(c,tp)`
fn filter(d: &mut Duel, c: CardId, tp: u8) -> bool {
    // return c:IsCode(CARD_NECROVALLEY) and c:GetActivateEffect() and c:GetActivateEffect():IsActivatable(tp,true,true)
    if !card::is_code(d, c, &[CARD_NECROVALLEY]) {
        return false;
    }
    match card::get_activate_effect(d, c).first().copied() {
        Some(ae) => effect::is_activatable(d, ae, tp, true, true),
        None => false,
    }
}
fn acop(d: &mut Duel, a: &EffArgs) {
    // if not e:GetHandler():IsRelateToEffect(e) then return end
    let h = effect::get_handler(d, a.e).nil();
    if !card::is_relate_to_effect(d, h, a.e) {
        return;
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_TOFIELD)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_TOFIELD as u64);
    // local tc=Duel.SelectMatchingCard(tp,aux.NecroValleyFilter(s.filter),tp,LOCATION_HAND|LOCATION_GRAVE,0,1,1,nil,tp):GetFirst()
    let tp = a.tp;
    let g = duel::select_matching_card(
        d,
        tp,
        Some(&aux::necro_valley_filter(|d, c| filter(d, c, tp))),
        tp,
        LOCATION_HAND | LOCATION_GRAVE,
        0,
        1,
        1,
        false,
        NO_EX,
    )
    .nil();
    let tc = group::get_first(d, &g);
    // Duel.ActivateFieldSpell(tc,e,tp,eg,ep,ev,re,r,rp)
    duel::activate_field_spell(d, tc, a, None);
}
fn setcon(d: &mut Duel, a: &EffArgs) -> bool {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // return rp==1-tp and c:IsReason(REASON_DESTROY) and c:IsReason(REASON_EFFECT) and c:IsPreviousControler(tp)
    a.rp == 1 - a.tp
        && card::is_reason(d, c, REASON_DESTROY)
        && card::is_reason(d, c, REASON_EFFECT)
        && card::is_previous_controler(d, c, a.tp)
}
/// `s.setfilter(c)`
fn setfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsSetCard(SET_NECROVALLEY) and not c:IsCode(id) and c:IsSSetable()
    card::is_set_card(d, c, &[SET_NECROVALLEY as u16]) && !card::is_code(d, c, &[ID]) && card::is_s_setable(d, c, false, None)
}
fn settg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return Duel.IsExistingMatchingCard(s.setfilter,tp,LOCATION_DECK,0,1,nil) end
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, Some(&setfilter), a.tp, LOCATION_DECK, 0, 1, NO_EX);
    }
    true
}
fn setop(d: &mut Duel, a: &EffArgs) {
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_SET)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_SET as u64);
    // local tc=Duel.SelectMatchingCard(tp,s.setfilter,tp,LOCATION_DECK,0,1,1,nil):GetFirst()
    let g = duel::select_matching_card(d, a.tp, Some(&setfilter), a.tp, LOCATION_DECK, 0, 1, 1, false, NO_EX).nil();
    let tc = group::get_first(d, &g);
    // if tc then Duel.SSet(tp,tc) end
    if let Some(tc) = tc {
        duel::s_set(d, a.tp, tc, None, true);
    }
}
