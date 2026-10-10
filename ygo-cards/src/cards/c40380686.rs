//! Baby Mudragon
use super::prelude::*;

pub const ID: u32 = 40380686;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Can be treated as non-Tuner for a Synchro Summon
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
    // e1:SetProperty(EFFECT_FLAG_SINGLE_RANGE)
    effect::set_property(d, e1, EFFECT_FLAG_SINGLE_RANGE, 0);
    // e1:SetCode(EFFECT_NONTUNER)
    effect::set_code(d, e1, EFFECT_NONTUNER);
    // e1:SetRange(LOCATION_MZONE)
    effect::set_range(d, e1, LOCATION_MZONE);
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // Change Type or Attribute of a Synchro Monster
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e2, aux::stringid(ID, 0));
    // e2:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e2, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_O);
    // e2:SetProperty(EFFECT_FLAG_DELAY+EFFECT_FLAG_CARD_TARGET)
    effect::set_property(d, e2, EFFECT_FLAG_DELAY + EFFECT_FLAG_CARD_TARGET, 0);
    // e2:SetCode(EVENT_BE_MATERIAL)
    effect::set_code(d, e2, EVENT_BE_MATERIAL);
    // e2:SetCondition(s.chcon)
    effect::set_condition(d, e2, Some(condition(chcon)));
    // e2:SetTarget(s.chtg)
    effect::set_target(d, e2, Some(cost_target(chtg)));
    // e2:SetOperation(s.chop)
    effect::set_operation(d, e2, Some(operation(chop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
}
/// `s.chcon(e,tp,eg,ep,ev,re,r,rp)`
fn chcon(d: &mut Duel, a: &EffArgs) -> bool {
    // return e:GetHandler():IsLocation(LOCATION_GRAVE) and r==REASON_SYNCHRO
    let h = effect::get_handler(d, a.e).nil();
    card::is_location(d, h, LOCATION_GRAVE) && a.r == REASON_SYNCHRO
}
/// `s.chfilter(c,e)`
fn chfilter(d: &mut Duel, c: CardId, e: EffectId) -> bool {
    // return c:IsFaceup() and c:IsType(TYPE_SYNCHRO) and c:IsCanBeEffectTarget(e)
    card::is_faceup(d, c) && card::is_type(d, c, TYPE_SYNCHRO) && card::is_can_be_effect_target(d, c, Some(e))
}
/// `s.chtg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn chtg(d: &mut Duel, a: &EffArgs) -> bool {
    let (e, tp) = (a.e, a.tp);
    // local g=Duel.GetMatchingGroup(s.chfilter,tp,LOCATION_MZONE,0,nil,e)
    let g = duel::get_matching_group(d, Some(&|d, c| chfilter(d, c, e)), tp, LOCATION_MZONE, 0, NO_EX);
    // if chk==0 then return #g>0 end
    if a.chk == Some(0) {
        return !g.is_empty();
    }
    // local cd=e:GetChainData()
    // if Duel.SelectOption(tp,aux.Stringid(id,1),aux.Stringid(id,2))==0 then
    if duel::select_option(d, tp, true, &[aux::stringid(ID, 1), aux::stringid(ID, 2)]) == 0 {
        // cd.race=Duel.AnnounceAnotherRace(g,tp)
        let race = duel::announce_another_race(d, &g, tp);
        chain::set_effect_data(d, e, None, chain::field::CARD, Value::Int(race as i64));
        // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_TARGET)
        duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_TARGET as u64);
        // local sg=g:FilterSelect(tp,Card.IsRaceExcept,1,1,nil,cd.race)
        let sg = group::filter_select(d, &g, tp, &|d, c| card::is_race_except(d, c, race, None, 0, None), 1, 1, false, NO_EX).nil();
        // Duel.SetTargetCard(sg)
        duel::set_target_card(d, sg);
    } else {
        // cd.attribute=Duel.AnnounceAnotherAttribute(g,tp)
        let attribute = duel::announce_another_attribute(d, &g, tp);
        chain::set_effect_data(d, e, None, chain::field::CARD + 1, Value::Int(attribute as i64));
        // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_TARGET)
        duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_TARGET as u64);
        // local sg=g:FilterSelect(tp,Card.IsAttributeExcept,1,1,nil,cd.attribute)
        let sg = group::filter_select(d, &g, tp, &|d, c| card::is_attribute_except(d, c, attribute, None, 0, None), 1, 1, false, NO_EX).nil();
        // Duel.SetTargetCard(sg)
        duel::set_target_card(d, sg);
    }
    true
}
/// `s.chop(e,tp,eg,ep,ev,re,r,rp)`
fn chop(d: &mut Duel, a: &EffArgs) {
    let e = a.e;
    // local tc=Duel.GetFirstTarget()
    // if not tc or not tc:IsRelateToEffect(e) or tc:IsFacedown() then return end
    let Some(tc) = duel::first_target(d) else {
        return;
    };
    if !card::is_relate_to_effect(d, tc, e) || card::is_facedown(d, tc) {
        return;
    }
    // local cd=e:GetChainData()
    let race = chain::effect_data(d, e, None, chain::field::CARD);
    let attribute = chain::effect_data(d, e, None, chain::field::CARD + 1);
    // if cd.race and tc:IsRaceExcept(cd.race) then
    if !race.is_nil() && card::is_race_except(d, tc, race.int() as u64, None, 0, None) {
        // Change monster type
        // local e1=Effect.CreateEffect(e:GetHandler())
        let h = effect::get_handler(d, e).nil();
        let e1 = effect::create_effect(d, h);
        // e1:SetType(EFFECT_TYPE_SINGLE)
        effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
        // e1:SetCode(EFFECT_CHANGE_RACE)
        effect::set_code(d, e1, EFFECT_CHANGE_RACE);
        // e1:SetValue(cd.race)
        effect::set_value(d, e1, race.int());
        // e1:SetReset(RESETS_STANDARD_PHASE_END)
        effect::set_reset(d, e1, RESETS_STANDARD_PHASE_END, 1);
        // tc:RegisterEffect(e1)
        card::register_effect(d, tc, e1, false);
    } else if !attribute.is_nil() && card::is_attribute_except(d, tc, attribute.int() as u32, None, 0, None) {
        // Change attribute
        // local e1=Effect.CreateEffect(e:GetHandler())
        let h = effect::get_handler(d, e).nil();
        let e1 = effect::create_effect(d, h);
        // e1:SetType(EFFECT_TYPE_SINGLE)
        effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
        // e1:SetCode(EFFECT_CHANGE_ATTRIBUTE)
        effect::set_code(d, e1, EFFECT_CHANGE_ATTRIBUTE);
        // e1:SetValue(cd.attribute)
        effect::set_value(d, e1, attribute.int());
        // e1:SetReset(RESETS_STANDARD_PHASE_END)
        effect::set_reset(d, e1, RESETS_STANDARD_PHASE_END, 1);
        // tc:RegisterEffect(e1)
        card::register_effect(d, tc, e1, false);
    }
}
