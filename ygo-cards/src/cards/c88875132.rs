//! Flying Pegasus Railroad Stampede
//! Scripted by AlphaKretin
use super::prelude::*;

pub const ID: u32 = 88875132;

pub const INFO: ScriptInfo = ScriptInfo { listed_names: &[ID], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // --special summon
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_SPECIAL_SUMMON)
    effect::set_category(d, e1, CATEGORY_SPECIAL_SUMMON);
    // e1:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_O);
    // e1:SetCode(EVENT_SUMMON_SUCCESS)
    effect::set_code(d, e1, EVENT_SUMMON_SUCCESS);
    // e1:SetProperty(EFFECT_FLAG_CARD_TARGET+EFFECT_FLAG_DELAY)
    effect::set_property(d, e1, EFFECT_FLAG_CARD_TARGET + EFFECT_FLAG_DELAY, 0);
    // e1:SetCountLimit(1,id)
    effect::set_count_limit(d, e1, 1, ID, 0, 0);
    // e1:SetTarget(s.sptg)
    effect::set_target(d, e1, Some(cost_target(sptg)));
    // e1:SetOperation(s.spop)
    effect::set_operation(d, e1, Some(operation(spop)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // local e2=e1:Clone()
    let e2 = effect::clone(d, e1);
    // e2:SetCode(EVENT_SPSUMMON_SUCCESS)
    effect::set_code(d, e2, EVENT_SPSUMMON_SUCCESS);
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
    // --lv change
    // local e3=Effect.CreateEffect(c)
    let e3 = effect::create_effect(d, c);
    // e3:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e3, aux::stringid(ID, 1));
    // e3:SetProperty(EFFECT_FLAG_CARD_TARGET)
    effect::set_property(d, e3, EFFECT_FLAG_CARD_TARGET, 0);
    // e3:SetType(EFFECT_TYPE_IGNITION)
    effect::set_type(d, e3, EFFECT_TYPE_IGNITION);
    // e3:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e3, EVENT_FREE_CHAIN);
    // e3:SetRange(LOCATION_MZONE)
    effect::set_range(d, e3, LOCATION_MZONE);
    // e3:SetCountLimit(1,{id,1})
    effect::set_count_limit(d, e3, 1, ID, 1, 0);
    // e3:SetCost(s.lvcost)
    effect::set_cost(d, e3, Some(cost_target(lvcost)));
    // e3:SetTarget(s.lvtg)
    effect::set_target(d, e3, Some(cost_target(lvtg)));
    // e3:SetOperation(s.lvop)
    effect::set_operation(d, e3, Some(operation(lvop)));
    // c:RegisterEffect(e3)
    card::register_effect(d, c, e3, false);
    // Duel.AddCustomActivityCounter(id,ACTIVITY_ATTACK,s.counterfilter)
    duel::add_custom_activity_counter(d, ID, ACTIVITY_ATTACK, lua_fn(counterfilter));
}
// s.spfilter(c,e,tp)
fn spfilter(d: &mut Duel, c: CardId, e: EffectId, tp: u8) -> bool {
    // return c:IsAttribute(ATTRIBUTE_EARTH) and c:IsRace(RACE_MACHINE) and not c:IsCode(id)
    //     and c:IsCanBeSpecialSummoned(e,0,tp,false,false,POS_FACEUP_DEFENSE)
    card::is_attribute(d, c, ATTRIBUTE_EARTH)
        && card::is_race(d, c, RACE_MACHINE)
        && !card::is_code(d, c, &[ID])
        && card::is_can_be_special_summoned(d, c, e, 0, tp, false, false, Some(POS_FACEUP_DEFENSE as u8), None, None)
}
// s.sptg(e,tp,eg,ep,ev,re,r,rp,chk,chkc)
fn sptg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chkc then return chkc:IsLocation(LOCATION_GRAVE) and chkc:IsControler(tp) and s.spfilter(chkc,e,tp) end
    if let Some(chkc) = a.chkc() {
        return card::is_location(d, chkc, LOCATION_GRAVE) && card::is_controler(d, chkc, a.tp) && spfilter(d, chkc, a.e, a.tp);
    }
    // if chk==0 then return Duel.IsExistingTarget(s.spfilter,tp,LOCATION_GRAVE,0,1,nil,e,tp)
    //     and Duel.GetLocationCount(tp,LOCATION_MZONE)>0 end
    if a.chk == Some(0) {
        if !duel::is_existing_target(d, Some(&|d, x| spfilter(d, x, a.e, a.tp)), a.tp, LOCATION_GRAVE, 0, 1, NO_EX) {
            return false;
        }
        return duel::get_location_count(d, a.tp, LOCATION_MZONE as u8, None, None, None).0 > 0;
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_TARGET)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_TARGET as u64);
    // local g=Duel.SelectTarget(tp,s.spfilter,tp,LOCATION_GRAVE,0,1,1,nil,e,tp)
    let g = duel::select_target(d, a.tp, Some(&|d, x| spfilter(d, x, a.e, a.tp)), a.tp, LOCATION_GRAVE, 0, 1, 1, false, NO_EX);
    // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,g,1,0,0)
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, g, 1, 0, 0);
    true
}
// s.spop(e,tp,eg,ep,ev,re,r,rp)
fn spop(d: &mut Duel, a: &EffArgs) {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // local tc=Duel.GetFirstTarget()
    let tc = duel::first_target(d);
    // if tc and tc:IsRelateToEffect(e) and Duel.SpecialSummonStep(tc,0,tp,tp,false,false,POS_FACEUP_DEFENSE) then
    if let Some(tc) = tc {
        if card::is_relate_to_effect(d, tc, a.e) && duel::special_summon_step(d, tc, 0, a.tp, a.tp, false, false, POS_FACEUP_DEFENSE as u8, None) {
            // local e1=Effect.CreateEffect(c)
            let e1 = effect::create_effect(d, c);
            // e1:SetType(EFFECT_TYPE_SINGLE)
            effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
            // e1:SetCode(EFFECT_DISABLE)
            effect::set_code(d, e1, EFFECT_DISABLE);
            // e1:SetReset(RESET_EVENT|RESETS_STANDARD)
            effect::set_reset(d, e1, RESET_EVENT | RESETS_STANDARD, 1);
            // tc:RegisterEffect(e1)
            card::register_effect(d, tc, e1, false);
            // local e2=Effect.CreateEffect(c)
            let e2 = effect::create_effect(d, c);
            // e2:SetType(EFFECT_TYPE_SINGLE)
            effect::set_type(d, e2, EFFECT_TYPE_SINGLE);
            // e2:SetCode(EFFECT_DISABLE_EFFECT)
            effect::set_code(d, e2, EFFECT_DISABLE_EFFECT);
            // e2:SetReset(RESET_EVENT|RESETS_STANDARD)
            effect::set_reset(d, e2, RESET_EVENT | RESETS_STANDARD, 1);
            // tc:RegisterEffect(e2)
            card::register_effect(d, tc, e2, false);
        }
    }
    // Duel.SpecialSummonComplete()
    duel::special_summon_complete(d);
}
// s.counterfilter(c)
fn counterfilter(d: &mut Duel, a: Args) -> bool {
    // return c:IsType(TYPE_XYZ)
    card::is_type(d, a.c(0), TYPE_XYZ)
}
// s.lvcost(e,tp,eg,ep,ev,re,r,rp,chk)
fn lvcost(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return Duel.GetCustomActivityCount(id,tp,ACTIVITY_ATTACK)==0 end
    if a.chk == Some(0) {
        return duel::get_custom_activity_count(d, ID, a.tp, ACTIVITY_ATTACK) == 0;
    }
    let h = effect::get_handler(d, a.e).nil();
    // local e1=Effect.CreateEffect(e:GetHandler())
    let e1 = effect::create_effect(d, h);
    // e1:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e1, EFFECT_TYPE_FIELD);
    // e1:SetCode(EFFECT_CANNOT_ATTACK_ANNOUNCE)
    effect::set_code(d, e1, EFFECT_CANNOT_ATTACK_ANNOUNCE);
    // e1:SetProperty(EFFECT_FLAG_IGNORE_IMMUNE+EFFECT_FLAG_OATH)
    effect::set_property(d, e1, EFFECT_FLAG_IGNORE_IMMUNE + EFFECT_FLAG_OATH, 0);
    // e1:SetTargetRange(LOCATION_MZONE,0)
    effect::set_target_range(d, e1, LOCATION_MZONE, 0);
    // e1:SetTarget(s.atktg)
    effect::set_target(d, e1, Some(effect_card_fn(atktg)));
    // e1:SetReset(RESET_PHASE|PHASE_END)
    effect::set_reset(d, e1, RESET_PHASE | PHASE_END, 1);
    // Duel.RegisterEffect(e1,tp)
    duel::register_effect(d, e1, a.tp);
    // local e3=Effect.CreateEffect(e:GetHandler())
    let e3 = effect::create_effect(d, h);
    // e3:SetProperty(EFFECT_FLAG_PLAYER_TARGET+EFFECT_FLAG_CLIENT_HINT)
    effect::set_property(d, e3, EFFECT_FLAG_PLAYER_TARGET + EFFECT_FLAG_CLIENT_HINT, 0);
    // e3:SetDescription(aux.Stringid(id,3))
    effect::set_description(d, e3, aux::stringid(ID, 3));
    // e3:SetReset(RESET_PHASE|PHASE_END)
    effect::set_reset(d, e3, RESET_PHASE | PHASE_END, 1);
    // e3:SetTargetRange(1,0)
    effect::set_target_range(d, e3, 1, 0);
    // Duel.RegisterEffect(e3,tp)
    duel::register_effect(d, e3, a.tp);
    true
}
// s.atktg(e,c)
fn atktg(d: &mut Duel, _e: EffectId, c: CardId) -> bool {
    // return not c:IsType(TYPE_XYZ)
    !card::is_type(d, c, TYPE_XYZ)
}
// s.lvfilter(c,lv)
fn lvfilter(d: &mut Duel, c: CardId, lv: u32) -> bool {
    // return c:IsFaceup() and c:IsLevelAbove(1) and not c:IsLevel(lv)
    card::is_faceup(d, c) && card::is_level_above(d, c, 1) && !card::is_level(d, c, &[lv])
}
// s.lvtg(e,tp,eg,ep,ev,re,r,rp,chk,chkc)
fn lvtg(d: &mut Duel, a: &EffArgs) -> bool {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // local lv=c:GetLevel()
    let lv = card::get_level(d, c);
    // if chkc then return chkc:IsLocation(LOCATION_MZONE) and s.lvfilter(chkc,lv) end
    if let Some(chkc) = a.chkc() {
        return card::is_location(d, chkc, LOCATION_MZONE) && lvfilter(d, chkc, lv);
    }
    // if chk==0 then return Duel.IsExistingTarget(s.lvfilter,tp,LOCATION_MZONE,0,1,c,lv) end
    if a.chk == Some(0) {
        return duel::is_existing_target(d, Some(&|d, x| lvfilter(d, x, lv)), a.tp, LOCATION_MZONE, 0, 1, c.into());
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_TARGET)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_TARGET as u64);
    // Duel.SelectTarget(tp,s.lvfilter,tp,LOCATION_MZONE,0,1,1,c,lv)
    let _ = duel::select_target(d, a.tp, Some(&|d, x| lvfilter(d, x, lv)), a.tp, LOCATION_MZONE, 0, 1, 1, false, c.into());
    true
}
// s.lvop(e,tp,eg,ep,ev,re,r,rp)
fn lvop(d: &mut Duel, a: &EffArgs) {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // local tc=Duel.GetFirstTarget()
    let tc = duel::first_target(d);
    // if c:IsFaceup() and c:IsRelateToEffect(e) and tc and tc:IsFaceup() and tc:IsRelateToEffect(e) and not tc:IsLevel(c:GetLevel()) then
    if !(card::is_faceup(d, c) && card::is_relate_to_effect(d, c, a.e)) {
        return;
    }
    let tc = match tc {
        Some(tc) => tc,
        None => return,
    };
    if !(card::is_faceup(d, tc) && card::is_relate_to_effect(d, tc, a.e)) {
        return;
    }
    let lvc = card::get_level(d, c);
    if card::is_level(d, tc, &[lvc]) {
        return;
    }
    // local g=Group.FromCards(c,tc)
    let g = CardSet::from_iter([c, tc]);
    // Duel.Hint(HINT_SELECTMSG,tp,aux.Stringid(id,2)) --Select the monster with the level you want
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, aux::stringid(ID, 2));
    // local sg=g:Select(tp,1,1,nil)
    let sg = group::select(d, &g, a.tp, 1, 1, false, NO_EX).nil();
    // local oc=(g-sg):GetFirst()
    let mut rest = g.clone();
    group::remove_card(&mut rest, &sg);
    let oc = group::get_first(d, &rest).nil();
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetProperty(EFFECT_FLAG_CANNOT_DISABLE)
    effect::set_property(d, e1, EFFECT_FLAG_CANNOT_DISABLE, 0);
    // e1:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
    // e1:SetCode(EFFECT_CHANGE_LEVEL)
    effect::set_code(d, e1, EFFECT_CHANGE_LEVEL);
    // e1:SetReset(RESET_EVENT|RESETS_STANDARD)
    effect::set_reset(d, e1, RESET_EVENT | RESETS_STANDARD, 1);
    // e1:SetValue(sg:GetFirst():GetLevel())
    let sg_first = group::get_first(d, &sg).nil();
    let lv = card::get_level(d, sg_first);
    effect::set_value(d, e1, lv as i64);
    // oc:RegisterEffect(e1)
    card::register_effect(d, oc, e1, false);
}
