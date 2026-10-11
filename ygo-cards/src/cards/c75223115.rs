//! Malefic Territory
use super::prelude::*;

pub const ID: u32 = 75223115;

pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_MALEFIC], listed_names: &[27564031], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // activate
    let e1 = effect::create_effect(d, c);
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    effect::set_operation(d, e1, Some(operation(activate)));
    card::register_effect(d, c, e1, false);
    // malefic change
    let e2 = effect::create_effect(d, c);
    effect::set_type(d, e2, EFFECT_TYPE_FIELD);
    effect::set_code(d, e2, ID);
    effect::set_property(d, e2, EFFECT_FLAG_PLAYER_TARGET, 0);
    effect::set_target_range(d, e2, 1, 1);
    effect::set_range(d, e2, LOCATION_SZONE);
    card::register_effect(d, c, e2, false);
    // adjust
    let e3 = effect::create_effect(d, c);
    effect::set_type(d, e3, EFFECT_TYPE_FIELD + EFFECT_TYPE_CONTINUOUS);
    effect::set_code(d, e3, EVENT_ADJUST);
    effect::set_range(d, e3, LOCATION_SZONE);
    effect::set_operation(d, e3, Some(operation(adjustop)));
    effect::set_condition(d, e3, Some(lua_fn(|d, _a| validitycheck(d))));
    card::register_effect(d, c, e3, false);
    // cannot summon,spsummon,flipsummon
    let e4 = effect::create_effect(d, c);
    effect::set_type(d, e4, EFFECT_TYPE_FIELD);
    effect::set_range(d, e4, LOCATION_SZONE);
    effect::set_code(d, e4, EFFECT_FORCE_SPSUMMON_POSITION);
    effect::set_property(d, e4, EFFECT_FLAG_PLAYER_TARGET, 0);
    effect::set_target_range(d, e4, 1, 1);
    effect::set_target(d, e4, Some(lua_fn(sumlimit)));
    effect::set_value(d, e4, POS_FACEDOWN as i64);
    card::register_effect(d, c, e4, false);
    let e5 = effect::clone(d, e4);
    effect::set_code(d, e5, EFFECT_CANNOT_SUMMON);
    card::register_effect(d, c, e5, false);
    let e6 = effect::clone(d, e4);
    effect::set_code(d, e6, EFFECT_CANNOT_FLIP_SUMMON);
    card::register_effect(d, c, e6, false);
    // disable
    let e7 = effect::create_effect(d, c);
    effect::set_type(d, e7, EFFECT_TYPE_FIELD);
    effect::set_code(d, e7, EFFECT_DISABLE);
    effect::set_range(d, e7, LOCATION_SZONE);
    effect::set_target_range(d, e7, LOCATION_MZONE, LOCATION_MZONE);
    effect::set_condition(d, e7, Some(lua_fn(|d, _a| duel::is_battle_phase(d, None))));
    effect::set_target(d, e7, Some(effect_card_fn(|d, _e, c| card::is_set_card(d, c, &[SET_MALEFIC as u16]))));
    card::register_effect(d, c, e7, false);
}
// function s.filter(c,tp)
fn filter(d: &mut Duel, c: CardId, tp: u8) -> bool {
    // return c:IsCode(27564031) and c:GetActivateEffect() and c:GetActivateEffect():IsActivatable(tp,true,true)
    card::is_code(d, c, &[27564031])
        && match card::get_activate_effect(d, c).first().copied() {
            Some(te) => effect::is_activatable(d, te, tp, true, true),
            None => false,
        }
}
// function s.activate(e,tp,eg,ep,ev,re,r,rp)
fn activate(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    let h = effect::get_handler(d, a.e).nil();
    // if not e:GetHandler():IsRelateToEffect(e) then return end
    if !card::is_relate_to_effect(d, h, a.e) {
        return;
    }
    // local g=Duel.GetMatchingGroup(s.filter,tp,LOCATION_DECK,0,nil,tp)
    let g = duel::get_matching_group(d, Some(&|d, c| filter(d, c, tp)), tp, LOCATION_DECK, 0, NO_EX);
    // if #g>0 and Duel.SelectYesNo(tp,aux.Stringid(id,0)) then
    if !g.is_empty() && duel::select_yes_no(d, tp, aux::stringid(ID, 0)) {
        // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_TOFIELD)
        duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_TOFIELD as u64);
        // local tc=g:Select(tp,1,1,nil):GetFirst()
        let sg = group::select(d, &g, tp, 1, 1, false, NO_EX).nil();
        let tc = group::get_first(d, &sg);
        // if Duel.ActivateFieldSpell(tc,e,tp,eg,ep,ev,re,r,rp) then
        if duel::activate_field_spell(d, tc, a, None) {
            let tc = tc.nil();
            let e1 = effect::create_effect(d, h);
            effect::set_type(d, e1, EFFECT_TYPE_FIELD);
            effect::set_code(d, e1, EFFECT_CANNOT_BE_EFFECT_TARGET);
            effect::set_property(d, e1, EFFECT_FLAG_IGNORE_IMMUNE, 0);
            effect::set_range(d, e1, LOCATION_FZONE);
            effect::set_target_range(d, e1, LOCATION_FZONE, LOCATION_FZONE);
            effect::set_value(d, e1, 1);
            effect::set_reset(d, e1, RESET_EVENT | RESETS_STANDARD, 1);
            card::register_effect(d, tc, e1, true);
        }
    }
}
// function s.stuff(c)
fn stuff(d: &mut Duel, c: CardId) -> bool {
    // local mt=c:GetMetatable()
    // return c:IsFaceup() and mt.has_malefic_unique and mt.has_malefic_unique[c]==true and not c:IsDisabled() and c:IsSetCard(SET_MALEFIC)
    card::is_faceup(d, c)
        && matches!(crate::card_var::get(d, c, crate::card_var::HAS_MALEFIC_UNIQUE), Value::Bool(true))
        && !card::is_disabled(d, c)
        && card::is_set_card(d, c, &[SET_MALEFIC as u16])
}
// function s.validitycheck()
fn validitycheck(d: &mut Duel) -> bool {
    // return Duel.IsExistingMatchingCard(s.stuff,0,LOCATION_MZONE,LOCATION_MZONE,1,nil)
    duel::is_existing_matching_card(d, Some(&|d, c| stuff(d, c)), 0, LOCATION_MZONE, LOCATION_MZONE, 1, NO_EX)
}
// function s.rmfilter(c,...)
fn rmfilter(d: &mut Duel, c: CardId, codes: &[u32]) -> bool {
    // return c:IsFaceup() and c:IsSetCard(SET_MALEFIC) and c:IsCode(...)
    card::is_faceup(d, c) && card::is_set_card(d, c, &[SET_MALEFIC as u16]) && card::is_code(d, c, codes)
}
// function s.rmfilter2(c,fieldid,...)
fn rmfilter2(d: &mut Duel, c: CardId, fieldid: u32, codes: &[u32]) -> bool {
    // return c:GetFieldID()<fieldid and c:IsCode(...)
    card::get_field_id(d, c) < fieldid && card::is_code(d, c, codes)
}
// function s.sumlimit(e,c,sump,sumtype,sumpos,targetp)
fn sumlimit(d: &mut Duel, a: Args) -> bool {
    let c = a.c(1);
    // if (not s.stuff(c) or not s.validitycheck()) then return false end
    if !stuff(d, c) || !validitycheck(d) {
        return false;
    }
    // local tp=sump
    // if targetp then tp=targetp end
    let tp = match a.v(5) {
        Value::Int(t) => *t as u8,
        _ => a.int(2) as u8,
    };
    // return Duel.IsExistingMatchingCard(s.rmfilter,tp,LOCATION_MZONE,LOCATION_MZONE,1,c,c:GetCode())
    let codes = card::get_code(d, c);
    duel::is_existing_matching_card(d, Some(&|d, x| rmfilter(d, x, &codes)), tp, LOCATION_MZONE, LOCATION_MZONE, 1, c.into())
}
// function s.checkok(c,group)
fn checkok(d: &mut Duel, c: CardId, group: &CardSet) -> bool {
    // return group:IsExists(s.rmfilter2,1,c,c:GetFieldID(),c:GetCode())
    let fid = card::get_field_id(d, c);
    let codes = card::get_code(d, c);
    group::is_exists(d, group, &|d, x| rmfilter2(d, x, fid, &codes), 1, c.into())
}
// function s.adjustop(e,tp,eg,ep,ev,re,r,rp)
fn adjustop(d: &mut Duel, _a: &EffArgs) {
    // local phase=Duel.GetCurrentPhase()
    let phase = duel::get_current_phase(d);
    // if (phase==PHASE_DAMAGE and not Duel.IsDamageCalculated()) or phase==PHASE_DAMAGE_CAL then return end
    if (phase == PHASE_DAMAGE && !duel::is_damage_calculated(d)) || phase == PHASE_DAMAGE_CAL {
        return;
    }
    // local g=Duel.GetMatchingGroup(aux.AND(Card.IsFaceup,Card.IsSetCard),0,LOCATION_MZONE,LOCATION_MZONE,nil,SET_MALEFIC)
    let g = duel::get_matching_group(d, Some(&|d, c| card::is_faceup(d, c) && card::is_set_card(d, c, &[SET_MALEFIC as u16])), 0, LOCATION_MZONE, LOCATION_MZONE, NO_EX);
    // local sg=g:Filter(s.checkok,nil,g)
    let sg = group::filter(d, &g, &|d, c| checkok(d, c, &g), NO_EX);
    // if #sg>0 then Duel.Destroy(sg,REASON_RULE) Duel.Readjust() end
    if !sg.is_empty() {
        duel::destroy(d, sg, REASON_RULE, None, None);
        duel::readjust(d);
    }
}
