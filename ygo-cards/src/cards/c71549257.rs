//! Heroic Challenger - Knuckle Sword
use super::prelude::*;

pub const ID: u32 = 71549257;

pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_HEROIC], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Special Summon itself from hand
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_SPECIAL_SUMMON)
    effect::set_category(d, e1, CATEGORY_SPECIAL_SUMMON);
    // e1:SetType(EFFECT_TYPE_IGNITION)
    effect::set_type(d, e1, EFFECT_TYPE_IGNITION);
    // e1:SetRange(LOCATION_HAND)
    effect::set_range(d, e1, LOCATION_HAND);
    // e1:SetCountLimit(1,id)
    effect::set_count_limit(d, e1, 1, ID, 0, 0);
    // e1:SetCondition(s.spcon)
    effect::set_condition(d, e1, Some(condition(spcon)));
    // e1:SetTarget(s.sptg)
    effect::set_target(d, e1, Some(cost_target(sptg)));
    // e1:SetOperation(s.spop)
    effect::set_operation(d, e1, Some(operation(spop)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // If Normal or Special Summoned, change levels
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e2, aux::stringid(ID, 1));
    // e2:SetCategory(CATEGORY_LVCHANGE)
    effect::set_category(d, e2, CATEGORY_LVCHANGE);
    // e2:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e2, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_O);
    // e2:SetProperty(EFFECT_FLAG_CARD_TARGET+EFFECT_FLAG_DELAY)
    effect::set_property(d, e2, EFFECT_FLAG_CARD_TARGET + EFFECT_FLAG_DELAY, 0);
    // e2:SetCode(EVENT_SUMMON_SUCCESS)
    effect::set_code(d, e2, EVENT_SUMMON_SUCCESS);
    // e2:SetCountLimit(1,{id,1})
    effect::set_count_limit(d, e2, 1, ID, 1, 0);
    // e2:SetTarget(s.lvtg)
    effect::set_target(d, e2, Some(cost_target(lvtg)));
    // e2:SetOperation(s.lvop)
    effect::set_operation(d, e2, Some(operation(lvop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
    // local e3=e2:Clone()
    let e3 = effect::clone(d, e2);
    // e3:SetCode(EVENT_SPSUMMON_SUCCESS)
    effect::set_code(d, e3, EVENT_SPSUMMON_SUCCESS);
    // c:RegisterEffect(e3)
    card::register_effect(d, c, e3, false);
}
/// `s.cfilter(c)`
fn cfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsFaceup() and c:IsSetCard(SET_HEROIC) and not c:IsLevel(1)
    card::is_faceup(d, c) && card::is_set_card(d, c, &[SET_HEROIC as u16]) && !card::is_level(d, c, &[1])
}
/// `s.spcon(e,tp,...)`
fn spcon(d: &mut Duel, a: &EffArgs) -> bool {
    // return Duel.IsExistingMatchingCard(s.cfilter,tp,LOCATION_MZONE,0,1,nil)
    duel::is_existing_matching_card(d, Some(&cfilter), a.tp, LOCATION_MZONE, 0, 1, NO_EX)
}
/// `s.sptg(e,tp,...,chk)`
fn sptg(d: &mut Duel, a: &EffArgs) -> bool {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if chk==0 then return Duel.GetLocationCount(tp,LOCATION_MZONE)>0
    //     and c:IsCanBeSpecialSummoned(e,0,tp,false,false) end
    if a.chk == Some(0) {
        return duel::get_location_count(d, a.tp, LOCATION_MZONE as u8, None, None, None).0 > 0
            && card::is_can_be_special_summoned(d, c, a.e, 0, a.tp, false, false, None, None, None);
    }
    // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,c,1,0,0)
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, Some(CardSet::from_card(c)), 1, 0, 0);
    true
}
/// `s.spop(e,tp,...)`
fn spop(d: &mut Duel, a: &EffArgs) {
    // if Duel.GetLocationCount(tp,LOCATION_MZONE)<=0 then return end
    if duel::get_location_count(d, a.tp, LOCATION_MZONE as u8, None, None, None).0 <= 0 {
        return;
    }
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if not c:IsRelateToEffect(e) then return end
    if !card::is_relate_to_effect(d, c, a.e) {
        return;
    }
    // Duel.SpecialSummon(c,0,tp,tp,false,false,POS_FACEUP)
    duel::special_summon(d, c, 0, a.tp, a.tp, false, false, POS_FACEUP as u8, None);
}
/// `s.lvfilter(c,lv)`
fn lvfilter(d: &mut Duel, c: CardId, lv: u32) -> bool {
    // return c:IsFaceup() and c:IsRace(RACE_WARRIOR) and c:IsLevelAbove(1) and not c:IsLevel(lv)
    card::is_faceup(d, c) && card::is_race(d, c, RACE_WARRIOR) && card::is_level_above(d, c, 1) && !card::is_level(d, c, &[lv])
}
/// `s.lvtg(e,tp,...,chk,chkc)`
fn lvtg(d: &mut Duel, a: &EffArgs) -> bool {
    let (e, tp) = (a.e, a.tp);
    // local c=e:GetHandler()
    let c = effect::get_handler(d, e).nil();
    // local lv=c:GetLevel()
    let lv = card::get_level(d, c);
    // if chkc then return chkc:IsControler(tp) and chkc:IsLocation(LOCATION_MZONE) and s.lvfilter(chkc,lv) end
    if let Some(chkc) = a.chkc() {
        return card::is_controler(d, chkc, tp) && card::is_location(d, chkc, LOCATION_MZONE) && lvfilter(d, chkc, lv);
    }
    // if chk==0 then return lv>0 and Duel.IsExistingTarget(s.lvfilter,tp,LOCATION_MZONE,0,1,c,lv) end
    if a.chk == Some(0) {
        return lv > 0 && duel::is_existing_target(d, Some(&|d, x| lvfilter(d, x, lv)), tp, LOCATION_MZONE, 0, 1, c.into());
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_TARGET)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_TARGET as u64);
    // Duel.SelectTarget(tp,s.lvfilter,tp,LOCATION_MZONE,0,1,1,c,lv)
    duel::select_target(d, tp, Some(&|d, x| lvfilter(d, x, lv)), tp, LOCATION_MZONE, 0, 1, 1, false, c.into());
    true
}
/// `s.lvop(e,tp,...)`
fn lvop(d: &mut Duel, a: &EffArgs) {
    let (e, tp) = (a.e, a.tp);
    // local c=e:GetHandler()
    let c = effect::get_handler(d, e).nil();
    // local tc=Duel.GetFirstTarget()
    let tc = duel::first_target(d);
    // if c:IsFaceup() and c:IsRelateToEffect(e) and tc and tc:IsFaceup() and tc:IsRelateToEffect(e)
    //     and not tc:IsLevel(c:GetLevel()) then
    let clv = card::get_level(d, c);
    let ok = card::is_faceup(d, c)
        && card::is_relate_to_effect(d, c, e)
        && match tc {
            Some(tc) => card::is_faceup(d, tc) && card::is_relate_to_effect(d, tc, e) && !card::is_level(d, tc, &[clv]),
            None => false,
        };
    if ok {
        let tc = tc.nil();
        // local g=Group.FromCards(c,tc)
        let mut g = CardSet::from_card(c);
        group::add_card(&mut g, &CardSet::from_card(tc));
        // Duel.Hint(HINT_SELECTMSG,tp,aux.Stringid(id,2)) --Select the monster with the level you want
        duel::hint(d, HINT_SELECTMSG as u8, tp, aux::stringid(ID, 2));
        // local sg=g:Select(tp,1,1,nil)
        let sg = group::select(d, &g, tp, 1, 1, false, NO_EX).nil();
        // local oc=(g-sg):GetFirst()
        let mut rest = g.clone();
        group::remove_card(&mut rest, &sg);
        let oc = group::get_first(d, &rest).nil();
        // Change Level
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
        let lv = card::get_level(d, sg_first) as i64;
        effect::set_value(d, e1, lv);
        // oc:RegisterEffect(e1)
        card::register_effect(d, oc, e1, false);
    }
    // Cannot declare attacks, except with Xyz Monsters
    // local ge1=Effect.CreateEffect(c)
    let ge1 = effect::create_effect(d, c);
    // ge1:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, ge1, EFFECT_TYPE_FIELD);
    // ge1:SetCode(EFFECT_CANNOT_ATTACK_ANNOUNCE)
    effect::set_code(d, ge1, EFFECT_CANNOT_ATTACK_ANNOUNCE);
    // ge1:SetProperty(EFFECT_FLAG_IGNORE_IMMUNE)
    effect::set_property(d, ge1, EFFECT_FLAG_IGNORE_IMMUNE, 0);
    // ge1:SetTargetRange(LOCATION_MZONE,0)
    effect::set_target_range(d, ge1, LOCATION_MZONE, 0);
    // ge1:SetTarget(function(e,c) return not c:IsType(TYPE_XYZ) end)
    effect::set_target(d, ge1, Some(effect_card_fn(|d, _e, c| !card::is_type(d, c, TYPE_XYZ))));
    // ge1:SetReset(RESET_PHASE|PHASE_END)
    effect::set_reset(d, ge1, RESET_PHASE | PHASE_END, 1);
    // Duel.RegisterEffect(ge1,tp)
    duel::register_effect(d, ge1, tp);
    // aux.RegisterClientHint(c,nil,tp,1,0,aux.Stringid(id,3),nil)
    aux::register_client_hint(d, c, 0, tp, 1, 0, Some(aux::stringid(ID, 3)), 0, None);
}
