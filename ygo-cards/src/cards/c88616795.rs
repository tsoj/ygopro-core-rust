//! Spellbook of Wisdom
use super::prelude::*;

pub const ID: u32 = 88616795;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Targeted spellcaster monster becomes unaffected by spells or traps
    let e1 = effect::create_effect(d, c);
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    effect::set_property(d, e1, EFFECT_FLAG_CARD_TARGET, 0);
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    effect::set_target(d, e1, Some(cost_target(target)));
    effect::set_operation(d, e1, Some(operation(activate)));
    effect::set_hint_timing(d, e1, 0, Some(TIMINGS_CHECK_MONSTER | TIMING_END_PHASE));
    card::register_effect(d, c, e1, false);
}
/// `s.filter(c)`
fn filter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsFaceup() and c:IsRace(RACE_SPELLCASTER)
    card::is_faceup(d, c) && card::is_race(d, c, RACE_SPELLCASTER)
}
/// `s.target(e,...,chk,chkc)`
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    // if chkc then return chkc:IsLocation(LOCATION_MZONE) and s.filter(chkc) end
    if let Some(chkc) = a.chkc() {
        return card::is_location(d, chkc, LOCATION_MZONE) && filter(d, chkc);
    }
    // if chk==0 then return Duel.IsExistingTarget(s.filter,tp,LOCATION_MZONE,LOCATION_MZONE,1,nil) end
    if a.chk == Some(0) {
        return duel::is_existing_target(d, Some(&filter), a.tp, LOCATION_MZONE, LOCATION_MZONE, 1, NO_EX);
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_FACEUP)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_FACEUP as u64);
    // Duel.SelectTarget(tp,s.filter,tp,LOCATION_MZONE,LOCATION_MZONE,1,1,nil)
    duel::select_target(d, a.tp, Some(&filter), a.tp, LOCATION_MZONE, LOCATION_MZONE, 1, 1, false, NO_EX);
    // local opt=Duel.SelectOption(tp,aux.Stringid(id,0),aux.Stringid(id,1))
    let opt = duel::select_option(d, a.tp, true, &[aux::stringid(ID, 0), aux::stringid(ID, 1)]);
    // e:SetLabel(opt)
    effect::set_label(d, a.e, &[opt as i64]);
    true
}
/// `s.activate(e,...)`
fn activate(d: &mut Duel, a: &EffArgs) {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // local tc=Duel.GetFirstTarget()
    let tc = duel::first_target(d).nil();
    // if tc:IsRelateToEffect(e) and tc:IsFaceup() then
    if card::is_relate_to_effect(d, tc, a.e) && card::is_faceup(d, tc) {
        // local e1=Effect.CreateEffect(c)
        let e1 = effect::create_effect(d, c);
        effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
        effect::set_property(d, e1, EFFECT_FLAG_SINGLE_RANGE + EFFECT_FLAG_CLIENT_HINT, 0);
        effect::set_range(d, e1, LOCATION_MZONE);
        effect::set_code(d, e1, EFFECT_IMMUNE_EFFECT);
        // if e:GetLabel()==0 then
        if effect::get_label(d, a.e) == 0 {
            // Unaffected by spells
            effect::set_description(d, e1, 3102);
            effect::set_value_fn(d, e1, lua_fn(efilter1));
        } else {
            // Unaffected by traps
            effect::set_description(d, e1, 3103);
            effect::set_value_fn(d, e1, lua_fn(efilter2));
        }
        effect::set_reset(d, e1, RESETS_STANDARD_PHASE_END, 1);
        card::register_effect(d, tc, e1, false);
    }
}
/// `s.efilter1(e,te)`
fn efilter1(d: &mut Duel, a: Args) -> bool {
    // return te:IsSpellEffect() and te:GetOwner()~=e:GetOwner()
    effect::is_spell_effect(d, a.e(1)) && effect::get_owner(d, a.e(1)) != effect::get_owner(d, a.e(0))
}
/// `s.efilter2(e,te)`
fn efilter2(d: &mut Duel, a: Args) -> bool {
    // return te:IsTrapEffect()
    effect::is_trap_effect(d, a.e(1))
}
