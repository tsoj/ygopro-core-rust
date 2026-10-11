//! Machina Armored Unit
use super::prelude::*;

pub const ID: u32 = 31828916;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    // e1:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // special summon
    // local e3=Effect.CreateEffect(c)
    let e3 = effect::create_effect(d, c);
    // e3:SetCategory(CATEGORY_SPECIAL_SUMMON)
    effect::set_category(d, e3, CATEGORY_SPECIAL_SUMMON);
    // e3:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e3, aux::stringid(ID, 0));
    // e3:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e3, EFFECT_TYPE_FIELD + EFFECT_TYPE_TRIGGER_O);
    // e3:SetRange(LOCATION_SZONE)
    effect::set_range(d, e3, LOCATION_SZONE);
    // e3:SetCode(EVENT_BATTLE_DESTROYED)
    effect::set_code(d, e3, EVENT_BATTLE_DESTROYED);
    // e3:SetCountLimit(1)
    effect::set_count_limit(d, e3, 1, 0, 0, 0);
    // e3:SetTarget(s.target)
    effect::set_target(d, e3, Some(cost_target(target)));
    // e3:SetOperation(s.operation)
    effect::set_operation(d, e3, Some(operation(op)));
    // c:RegisterEffect(e3)
    card::register_effect(d, c, e3, false);
}
/// `s.cfilter(c,e,tp)`
fn cfilter(d: &mut Duel, c: CardId, e: EffectId, tp: u8) -> bool {
    // return c:IsRace(RACE_MACHINE) and c:IsControler(tp) and c:IsLocation(LOCATION_GRAVE) and c:IsReason(REASON_BATTLE)
    //   and (c:GetPreviousRaceOnField()&RACE_MACHINE)~=0
    //   and Duel.IsExistingMatchingCard(s.filter,tp,LOCATION_DECK,0,1,nil,c:GetAttack(),c:GetAttribute(),e,tp)
    if !(card::is_race(d, c, RACE_MACHINE as u64)
        && card::is_controler(d, c, tp)
        && card::is_location(d, c, LOCATION_GRAVE)
        && card::is_reason(d, c, REASON_BATTLE))
    {
        return false;
    }
    if card::get_previous_race_on_field(d, c) & (RACE_MACHINE as u64) == 0 {
        return false;
    }
    let atk = card::get_attack(d, c);
    let att = card::get_attribute(d, c);
    duel::is_existing_matching_card(d, Some(&|d, x| filter(d, x, atk, att, e, tp)), tp, LOCATION_DECK, 0, 1, NO_EX)
}
/// `s.filter(c,atk,att,e,tp)`
fn filter(d: &mut Duel, c: CardId, atk: i32, att: u32, e: EffectId, tp: u8) -> bool {
    // local a=c:GetAttack()
    // return a>=0 and a<atk and c:IsRace(RACE_MACHINE) and c:IsAttribute(att) and c:IsCanBeSpecialSummoned(e,0,tp,false,false)
    let a = card::get_attack(d, c);
    a >= 0
        && a < atk
        && card::is_race(d, c, RACE_MACHINE as u64)
        && card::is_attribute(d, c, att)
        && card::is_can_be_special_summoned(d, c, e, 0, tp, false, false, None, None, None)
}
/// `s.target(e,tp,eg,ep,ev,re,r,rp,chk)`
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    let eg = a.eg.clone().nil();
    // if chk==0 then return Duel.GetLocationCount(tp,LOCATION_MZONE)>0 and eg:IsExists(s.cfilter,1,nil,e,tp) end
    if a.chk == Some(0) {
        return duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 > 0
            && group::is_exists(d, &eg, &|d, c| cfilter(d, c, a.e, tp), 1, NO_EX);
    }
    // Duel.SetTargetCard(eg)
    duel::set_target_card(d, eg.clone());
    // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,nil,1,tp,LOCATION_DECK)
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, None, 1, tp, LOCATION_DECK as i32);
    true
}
/// `s.cfilter2(c,e,tp)`
fn cfilter2(d: &mut Duel, c: CardId, e: EffectId, tp: u8) -> bool {
    // return c:IsRace(RACE_MACHINE) and c:IsControler(tp) and c:IsRelateToEffect(e)
    //   and Duel.IsExistingMatchingCard(s.filter,tp,LOCATION_DECK,0,1,nil,c:GetAttack(),c:GetAttribute(),e,tp)
    if !(card::is_race(d, c, RACE_MACHINE as u64) && card::is_controler(d, c, tp) && card::is_relate_to_effect(d, c, e)) {
        return false;
    }
    let atk = card::get_attack(d, c);
    let att = card::get_attribute(d, c);
    duel::is_existing_matching_card(d, Some(&|d, x| filter(d, x, atk, att, e, tp)), tp, LOCATION_DECK, 0, 1, NO_EX)
}
/// `s.operation(e,tp,eg,ep,ev,re,r,rp)`
fn op(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    let h = effect::get_handler(d, a.e).nil();
    // if not e:GetHandler():IsRelateToEffect(e) then return end
    if !card::is_relate_to_effect(d, h, a.e) {
        return;
    }
    // if Duel.GetLocationCount(tp,LOCATION_MZONE)<=0 then return end
    if duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 <= 0 {
        return;
    }
    // local sg=eg:Filter(s.cfilter2,nil,e,tp)
    let eg = a.eg.clone().nil();
    let sg = group::filter(d, &eg, &|d, c| cfilter2(d, c, a.e, tp), NO_EX);
    // if #sg==1 then
    if sg.len() == 1 {
        // local tc=sg:GetFirst()
        let tc = group::get_first(d, &sg).nil();
        // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_SPSUMMON)
        duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_SPSUMMON as u64);
        let atk = card::get_attack(d, tc);
        let att = card::get_attribute(d, tc);
        // local g=Duel.SelectMatchingCard(tp,s.filter,tp,LOCATION_DECK,0,1,1,nil,tc:GetAttack(),tc:GetAttribute(),e,tp)
        let g = duel::select_matching_card(d, tp, Some(&|d, x| filter(d, x, atk, att, a.e, tp)), tp, LOCATION_DECK, 0, 1, 1, false, NO_EX);
        // if #g>0 then Duel.SpecialSummon(g,0,tp,tp,false,false,POS_FACEUP) end
        if let Some(g) = g {
            if !g.is_empty() {
                duel::special_summon(d, g, 0, tp, tp, false, false, POS_FACEUP as u8, None);
            }
        }
    } else {
        // local tc=sg:GetFirst()
        // if not tc then return end
        let Some(first) = group::get_first(d, &sg) else { return };
        // local atk=tc:GetAttack()
        let mut atk = card::get_attack(d, first);
        // local att=tc:GetAttribute()
        let mut att = card::get_attribute(d, first);
        // tc=sg:GetNext()
        let second = group::iter(d, &sg).get(1).copied();
        // if tc then
        if let Some(tc) = second {
            // if tc:GetAttack()>atk then atk=tc:GetAttack() end
            let tc_atk = card::get_attack(d, tc);
            if tc_atk > atk {
                atk = tc_atk;
            }
            // att=(att|tc:GetAttribute())
            att |= card::get_attribute(d, tc);
        }
        // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_SPSUMMON)
        duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_SPSUMMON as u64);
        // local g=Duel.SelectMatchingCard(tp,s.filter,tp,LOCATION_DECK,0,1,1,nil,atk,att,e,tp)
        let g = duel::select_matching_card(d, tp, Some(&|d, x| filter(d, x, atk, att, a.e, tp)), tp, LOCATION_DECK, 0, 1, 1, false, NO_EX);
        // if #g>0 then Duel.SpecialSummon(g,0,tp,tp,false,false,POS_FACEUP) end
        if let Some(g) = g {
            if !g.is_empty() {
                duel::special_summon(d, g, 0, tp, tp, false, false, POS_FACEUP as u8, None);
            }
        }
    }
}
