//! Replicard LAD
use super::prelude::*;

pub const ID: u32 = 1665819;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Special Summon this card from your hand
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
    // e1:SetCost(s.selfspcost)
    effect::set_cost(d, e1, Some(cost_target(selfspcost)));
    // e1:SetTarget(s.selfsptg)
    effect::set_target(d, e1, Some(cost_target(selfsptg)));
    // e1:SetOperation(s.selfspop)
    effect::set_operation(d, e1, Some(operation(selfspop)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // Special Summon 1 monster from your Deck with a different name that has 2 or more of the same Level, ATK, and/or DEF as the targeted monster
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e2, aux::stringid(ID, 1));
    // e2:SetCategory(CATEGORY_SPECIAL_SUMMON)
    effect::set_category(d, e2, CATEGORY_SPECIAL_SUMMON);
    // e2:SetType(EFFECT_TYPE_IGNITION)
    effect::set_type(d, e2, EFFECT_TYPE_IGNITION);
    // e2:SetProperty(EFFECT_FLAG_CARD_TARGET)
    effect::set_property(d, e2, EFFECT_FLAG_CARD_TARGET, 0);
    // e2:SetRange(LOCATION_MZONE)
    effect::set_range(d, e2, LOCATION_MZONE);
    // e2:SetCountLimit(1,{id,1})
    effect::set_count_limit(d, e2, 1, ID, 1, 0);
    // e2:SetTarget(s.decksptg)
    effect::set_target(d, e2, Some(cost_target(decksptg)));
    // e2:SetOperation(s.deckspop)
    effect::set_operation(d, e2, Some(operation(deckspop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
}
/// `s.selfspcostfilter(c,tp)`
fn selfspcostfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsLevelAbove(7) and c:IsAbleToRemoveAsCost()
    card::is_level_above(d, c, 7) && card::is_able_to_remove_as_cost(d, c, None)
}
/// `s.selfspcost(e,tp,eg,ep,ev,re,r,rp,chk)`
fn selfspcost(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if chk==0 then return Duel.IsExistingMatchingCard(s.selfspcostfilter,tp,LOCATION_HAND,0,1,c) end
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, Some(&selfspcostfilter), tp, LOCATION_HAND, 0, 1, c.into());
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_REMOVE)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_REMOVE as u64);
    // local g=Duel.SelectMatchingCard(tp,s.selfspcostfilter,tp,LOCATION_HAND,0,1,1,c)
    let g = duel::select_matching_card(d, tp, Some(&selfspcostfilter), tp, LOCATION_HAND, 0, 1, 1, false, c.into()).nil();
    // Duel.Remove(g,POS_FACEUP,REASON_COST)
    duel::remove(d, g, POS_FACEUP as u8, REASON_COST, None, None);
    true
}
/// `s.selfsptg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn selfsptg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if chk==0 then return Duel.GetLocationCount(tp,LOCATION_MZONE)>0
    //     and c:IsCanBeSpecialSummoned(e,0,tp,false,false) end
    if a.chk == Some(0) {
        return duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 > 0
            && card::is_can_be_special_summoned(d, c, a.e, 0, tp, false, false, None, None, None);
    }
    // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,c,1,tp,0)
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, Some(CardSet::from_card(c)), 1, tp, 0);
    true
}
/// `s.selfspop(e,tp,eg,ep,ev,re,r,rp)`
fn selfspop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if c:IsRelateToEffect(e) then
    if card::is_relate_to_effect(d, c, a.e) {
        // Duel.SpecialSummon(c,0,tp,tp,false,false,POS_FACEUP)
        duel::special_summon(d, c, 0, tp, tp, false, false, POS_FACEUP as u8, None);
    }
}
/// `s.tgfilter(c,e,tp)`
fn tgfilter(d: &mut Duel, c: CardId, e: EffectId, tp: u8) -> bool {
    // if c:IsFacedown() then return false end
    if card::is_facedown(d, c) {
        return false;
    }
    // local code=c:GetCode()
    let code = card::get_code(d, c)[0];
    // local lv=c:HasLevel() and c:GetLevel() or -1
    let lv: i64 = if card::has_level(d, c) { card::get_level(d, c) as i64 } else { -1 };
    // local atk=c:GetAttack()
    let atk = card::get_attack(d, c) as i64;
    // local def=c:HasDefense() and c:GetDefense() or -1
    let def: i64 = if card::has_defense(d, c) { card::get_defense(d, c) as i64 } else { -1 };
    // return Duel.IsExistingMatchingCard(s.deckspfilter,tp,LOCATION_DECK,0,1,nil,e,tp,code,lv,atk,def)
    duel::is_existing_matching_card(d, Some(&|d, x| deckspfilter(d, x, e, tp, code, lv, atk, def)), tp, LOCATION_DECK, 0, 1, NO_EX)
}
/// `s.deckspfilter(c,e,tp,code,lv,atk,def)`
#[allow(clippy::too_many_arguments)]
fn deckspfilter(d: &mut Duel, c: CardId, e: EffectId, tp: u8, code: u32, lv: i64, atk: i64, def: i64) -> bool {
    // if c:IsCode(code) or not c:IsCanBeSpecialSummoned(e,0,tp,false,false) then return false end
    if card::is_code(d, c, &[code]) || !card::is_can_be_special_summoned(d, c, e, 0, tp, false, false, None, None, None) {
        return false;
    }
    // local ct=0
    let mut ct = 0;
    // if c:IsLevel(lv) then ct=ct+1 end
    if card::is_level(d, c, &[lv as u32]) {
        ct += 1;
    }
    // if c:IsAttack(atk) then ct=ct+1 end
    if card::is_attack(d, c, &[atk as u32]) {
        ct += 1;
    }
    // if c:IsDefense(def) then ct=ct+1 end
    if card::is_defense(d, c, &[def as u32]) {
        ct += 1;
    }
    // return ct>=2
    ct >= 2
}
/// `s.decksptg(e,tp,eg,ep,ev,re,r,rp,chk,chkc)`
fn decksptg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    let e = a.e;
    // if chkc then return chkc:IsLocation(LOCATION_MZONE) and chkc:IsControler(tp) and s.tgfilter(chkc,e,tp) end
    if let Some(chkc) = a.chkc() {
        return card::is_location(d, chkc, LOCATION_MZONE) && card::is_controler(d, chkc, tp) && tgfilter(d, chkc, e, tp);
    }
    // if chk==0 then return Duel.GetLocationCount(tp,LOCATION_MZONE)>0
    //     and Duel.IsExistingTarget(s.tgfilter,tp,LOCATION_MZONE,0,1,nil,e,tp) end
    if a.chk == Some(0) {
        return duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 > 0
            && duel::is_existing_target(d, Some(&|d, c| tgfilter(d, c, e, tp)), tp, LOCATION_MZONE, 0, 1, NO_EX);
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_TARGET)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_TARGET as u64);
    // local g=Duel.SelectTarget(tp,s.tgfilter,tp,LOCATION_MZONE,0,1,1,nil,e,tp)
    let _g = duel::select_target(d, tp, Some(&|d, c| tgfilter(d, c, e, tp)), tp, LOCATION_MZONE, 0, 1, 1, false, NO_EX);
    // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,nil,1,tp,LOCATION_DECK)
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, None, 1, tp, LOCATION_DECK as i32);
    true
}
/// `s.deckspop(e,tp,eg,ep,ev,re,r,rp)`
fn deckspop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    let e = a.e;
    // local tc=Duel.GetFirstTarget()
    let tc = duel::first_target(d).nil();
    // if not (tc:IsFaceup() and tc:IsRelateToEffect(e) and Duel.GetLocationCount(tp,LOCATION_MZONE)>0) then return end
    if !(card::is_faceup(d, tc) && card::is_relate_to_effect(d, tc, e) && duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 > 0) {
        return;
    }
    // local code=tc:GetCode()
    let code = card::get_code(d, tc)[0];
    // local lv=tc:HasLevel() and tc:GetLevel() or -1
    let lv: i64 = if card::has_level(d, tc) { card::get_level(d, tc) as i64 } else { -1 };
    // local atk=tc:GetAttack()
    let atk = card::get_attack(d, tc) as i64;
    // local def=tc:HasDefense() and tc:GetDefense() or -1
    let def: i64 = if card::has_defense(d, tc) { card::get_defense(d, tc) as i64 } else { -1 };
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_SPSUMMON)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_SPSUMMON as u64);
    // local sc=Duel.SelectMatchingCard(tp,s.deckspfilter,tp,LOCATION_DECK,0,1,1,nil,e,tp,code,lv,atk,def):GetFirst()
    let g = duel::select_matching_card(
        d,
        tp,
        Some(&|d, x| deckspfilter(d, x, e, tp, code, lv, atk, def)),
        tp,
        LOCATION_DECK,
        0,
        1,
        1,
        false,
        NO_EX,
    )
    .nil();
    let sc = group::get_first(d, &g);
    // if sc and Duel.SpecialSummonStep(sc,0,tp,tp,false,false,POS_FACEUP) then
    if let Some(sc) = sc {
        if duel::special_summon_step(d, sc, 0, tp, tp, false, false, POS_FACEUP as u8, None) {
            // It cannot activate its effects this turn
            // local e1=Effect.CreateEffect(e:GetHandler())
            let h = effect::get_handler(d, e).nil();
            let e1 = effect::create_effect(d, h);
            // e1:SetDescription(3302)
            effect::set_description(d, e1, 3302);
            // e1:SetType(EFFECT_TYPE_SINGLE)
            effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
            // e1:SetProperty(EFFECT_FLAG_CLIENT_HINT)
            effect::set_property(d, e1, EFFECT_FLAG_CLIENT_HINT, 0);
            // e1:SetCode(EFFECT_CANNOT_TRIGGER)
            effect::set_code(d, e1, EFFECT_CANNOT_TRIGGER);
            // e1:SetReset(RESETS_STANDARD_PHASE_END)
            effect::set_reset(d, e1, RESETS_STANDARD_PHASE_END, 1);
            // sc:RegisterEffect(e1)
            card::register_effect(d, sc, e1, false);
        }
    }
    // Duel.SpecialSummonComplete()
    duel::special_summon_complete(d);
}
