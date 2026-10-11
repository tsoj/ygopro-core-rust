//! Spawn Alligator
use super::prelude::*;

pub const ID: u32 = 39984786;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // tribute check
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
    // e1:SetCode(EFFECT_MATERIAL_CHECK)
    effect::set_code(d, e1, EFFECT_MATERIAL_CHECK);
    // e1:SetValue(s.valcheck)
    effect::set_value_fn(d, e1, lua_fn(valcheck));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
/// `s.valcheck(e,c)`
fn valcheck(d: &mut Duel, a: Args) -> Value {
    // local c = the summoned card (the second argument)
    let c = a.c(1);
    // local g=c:GetMaterial()
    let g = card::get_material(d, c);
    // local tc=g:GetFirst()
    let _tc = group::get_first(d, &g);
    // local sp=false
    let mut sp = false;
    // for tc in aux.Next(g) do
    for tc in group::iter(d, &g) {
        // if tc:IsRace(RACE_REPTILE) and not tc:IsType(TYPE_TOKEN) then
        if card::is_race(d, tc, RACE_REPTILE as u64) && !card::is_type(d, tc, TYPE_TOKEN as u32) {
            // tc:RegisterFlagEffect(id,RESET_EVENT|RESETS_STANDARD_EXC_GRAVE|RESET_PHASE|PHASE_END,0,1)
            card::register_flag_effect(d, tc, ID, RESET_EVENT | RESETS_STANDARD_EXC_GRAVE | RESET_PHASE | PHASE_END, 0, 1, 0, 0);
            // sp=true
            sp = true;
        }
    }
    // if sp then
    if sp {
        // spsummon
        // local e1=Effect.CreateEffect(c)
        let e1 = effect::create_effect(d, c);
        // e1:SetDescription(aux.Stringid(id,0))
        effect::set_description(d, e1, aux::stringid(ID, 0));
        // e1:SetCategory(CATEGORY_SPECIAL_SUMMON)
        effect::set_category(d, e1, CATEGORY_SPECIAL_SUMMON);
        // e1:SetProperty(EFFECT_FLAG_CARD_TARGET)
        effect::set_property(d, e1, EFFECT_FLAG_CARD_TARGET, 0);
        // e1:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_TRIGGER_F)
        effect::set_type(d, e1, EFFECT_TYPE_FIELD + EFFECT_TYPE_TRIGGER_F);
        // e1:SetCode(EVENT_PHASE+PHASE_END)
        effect::set_code(d, e1, EVENT_PHASE + PHASE_END);
        // e1:SetRange(LOCATION_MZONE)
        effect::set_range(d, e1, LOCATION_MZONE);
        // e1:SetCountLimit(1)
        effect::set_count_limit(d, e1, 1, 0, 0, 0);
        // e1:SetTarget(s.sptg)
        effect::set_target(d, e1, Some(cost_target(sptg)));
        // e1:SetOperation(s.spop)
        effect::set_operation(d, e1, Some(operation(spop)));
        // e1:SetReset(RESET_EVENT|RESETS_STANDARD_PHASE_END&~(RESET_TOFIELD|RESET_TURN_SET))
        effect::set_reset(d, e1, RESET_EVENT | (RESETS_STANDARD_PHASE_END & !(RESET_TOFIELD | RESET_TURN_SET)), 1);
        // c:RegisterEffect(e1)
        card::register_effect(d, c, e1, false);
    }
    Value::Nil
}
/// `s.filter(c,e,tp)`
fn filter(d: &mut Duel, c: CardId, e: EffectId, tp: u8) -> bool {
    // return c:GetFlagEffect(id)~=0 and c:IsCanBeEffectTarget(e) and c:IsCanBeSpecialSummoned(e,0,tp,false,false)
    card::get_flag_effect(d, c, ID) != 0
        && card::is_can_be_effect_target(d, c, Some(e))
        && card::is_can_be_special_summoned(d, c, e, 0, tp, false, false, None, None, None)
}
/// `s.sptg(e,tp,eg,ep,ev,re,r,rp,chk,chkc)`
fn sptg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chkc then return e:GetHandler():GetMaterial():IsContains(chkc) and s.filter(chkc,e,tp) end
    if let Some(chkc) = a.chkc() {
        let h = effect::get_handler(d, a.e).nil();
        let mat = card::get_material(d, h);
        return group::is_contains(&mat, chkc) && filter(d, chkc, a.e, a.tp);
    }
    // if chk==0 then return true end
    if a.chk == Some(0) {
        return true;
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_SPSUMMON)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_SPSUMMON as u64);
    // local mg=e:GetHandler():GetMaterial()
    let h = effect::get_handler(d, a.e).nil();
    let mg = card::get_material(d, h);
    // local g=mg:FilterSelect(tp,s.filter,1,1,nil,e,tp)
    let g = group::filter_select(d, &mg, a.tp, &|d, c| filter(d, c, a.e, a.tp), 1, 1, false, NO_EX).nil();
    // Duel.SetTargetCard(g)
    duel::set_target_card(d, g.clone());
    // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,g,1,0,0)
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, Some(g), 1, 0, 0);
    true
}
/// `s.spop(e,tp,eg,ep,ev,re,r,rp)`
fn spop(d: &mut Duel, a: &EffArgs) {
    // local tc=Duel.GetFirstTarget()
    // if tc and tc:IsRelateToEffect(e) then
    if let Some(tc) = duel::first_target(d) {
        if card::is_relate_to_effect(d, tc, a.e) {
            // Duel.SpecialSummon(tc,0,tp,tp,false,false,POS_FACEUP)
            duel::special_summon(d, CardSet::from_card(tc), 0, a.tp, a.tp, false, false, POS_FACEUP as u8, None);
        }
    }
}
