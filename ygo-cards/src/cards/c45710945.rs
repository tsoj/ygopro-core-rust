//! Galaxy-Eyes Tachyon Primal
use super::prelude::*;

pub const ID: u32 = 45710945;

pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_TACHYON], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Attach this card on the field to a Dragon Xyz Monster as material
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_TRIGGER_F)
    effect::set_type(d, e1, EFFECT_TYPE_FIELD + EFFECT_TYPE_TRIGGER_F);
    // e1:SetCode(EVENT_ATTACK_ANNOUNCE)
    effect::set_code(d, e1, EVENT_ATTACK_ANNOUNCE);
    // e1:SetRange(LOCATION_MZONE)
    effect::set_range(d, e1, LOCATION_MZONE);
    // e1:SetCondition(s.attachcon)
    effect::set_condition(d, e1, Some(condition(attachcon)));
    // e1:SetTarget(s.attachtg)
    effect::set_target(d, e1, Some(cost_target(attachtg)));
    // e1:SetOperation(s.attachop)
    effect::set_operation(d, e1, Some(operation(attachop)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // Special Summon this card from your hand to either field
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e2, aux::stringid(ID, 1));
    // e2:SetCategory(CATEGORY_SPECIAL_SUMMON)
    effect::set_category(d, e2, CATEGORY_SPECIAL_SUMMON);
    // e2:SetType(EFFECT_TYPE_QUICK_O)
    effect::set_type(d, e2, EFFECT_TYPE_QUICK_O);
    // e2:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e2, EVENT_FREE_CHAIN);
    // e2:SetRange(LOCATION_HAND)
    effect::set_range(d, e2, LOCATION_HAND);
    // e2:SetHintTiming(0,TIMING_MAIN_END|TIMING_BATTLE_START|TIMING_BATTLE_END|TIMINGS_CHECK_MONSTER_E)
    effect::set_hint_timing(d, e2, 0, Some(TIMING_MAIN_END | TIMING_BATTLE_START | TIMING_BATTLE_END | TIMINGS_CHECK_MONSTER_E));
    // e2:SetCountLimit(1,{id,0})
    effect::set_count_limit(d, e2, 1, ID, 0, 0);
    // e2:SetCondition(s.spcon)
    effect::set_condition(d, e2, Some(condition(spcon)));
    // e2:SetTarget(s.sptg)
    effect::set_target(d, e2, Some(cost_target(sptg)));
    // e2:SetOperation(s.spop)
    effect::set_operation(d, e2, Some(operation(spop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
    // This card's owner adds 1 "Tachyon" Spell/Trap from their Deck to their hand
    // local e3=Effect.CreateEffect(c)
    let e3 = effect::create_effect(d, c);
    // e3:SetDescription(aux.Stringid(id,2))
    effect::set_description(d, e3, aux::stringid(ID, 2));
    // e3:SetCategory(CATEGORY_TOHAND+CATEGORY_SEARCH)
    effect::set_category(d, e3, CATEGORY_TOHAND + CATEGORY_SEARCH);
    // e3:SetType(EFFECT_TYPE_SINGLE|EFFECT_TYPE_TRIGGER_F)
    effect::set_type(d, e3, EFFECT_TYPE_SINGLE | EFFECT_TYPE_TRIGGER_F);
    // e3:SetCode(EVENT_SUMMON_SUCCESS)
    effect::set_code(d, e3, EVENT_SUMMON_SUCCESS);
    // e3:SetCountLimit(1,{id,1})
    effect::set_count_limit(d, e3, 1, ID, 1, 0);
    // e3:SetTarget(s.thtg)
    effect::set_target(d, e3, Some(cost_target(thtg)));
    // e3:SetOperation(s.thop)
    effect::set_operation(d, e3, Some(operation(thop)));
    // c:RegisterEffect(e3)
    card::register_effect(d, c, e3, false);
    // local e4=e3:Clone()
    let e4 = effect::clone(d, e3);
    // e4:SetCode(EVENT_SPSUMMON_SUCCESS)
    effect::set_code(d, e4, EVENT_SPSUMMON_SUCCESS);
    // c:RegisterEffect(e4)
    card::register_effect(d, c, e4, false);
}
/// `s.attachcon(e,tp,eg,ep,ev,re,r,rp)`
fn attachcon(d: &mut Duel, a: &EffArgs) -> bool {
    // local ac=Duel.GetAttacker()
    let ac = duel::get_attacker(d).nil();
    // return ac:IsRace(RACE_DRAGON) and ac:IsType(TYPE_XYZ) and e:GetHandler():IsCanBeXyzMaterial(ac,tp,REASON_EFFECT)
    let h = effect::get_handler(d, a.e).nil();
    card::is_race(d, ac, RACE_DRAGON)
        && card::is_type(d, ac, TYPE_XYZ)
        && card::is_can_be_xyz_material(d, h, Some(ac), Some(a.tp), Some(REASON_EFFECT))
}
/// `s.attachtg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn attachtg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return true end
    if a.chk == Some(0) {
        return true;
    }
    // Duel.SetTargetCard(Duel.GetAttacker())
    match duel::get_attacker(d) {
        Some(ac) => duel::set_target_card(d, ac),
        None => duel::set_target_card(d, CardSet::new()),
    }
    true
}
/// `s.attachop(e,tp,eg,ep,ev,re,r,rp)`
fn attachop(d: &mut Duel, a: &EffArgs) {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // local ac=Duel.GetAttacker()
    let ac = duel::get_attacker(d);
    // if c:IsRelateToEffect(e) and ac:IsRelateToEffect(e) then
    if card::is_relate_to_effect(d, c, a.e) && card::is_relate_to_effect(d, ac.nil(), a.e) {
        // Duel.Overlay(ac,c,true)
        duel::overlay(d, ac.nil(), c, true);
    }
}
/// `s.spconfilter(c)`
fn spconfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsRace(RACE_DRAGON) and c:IsType(TYPE_XYZ) and c:IsFaceup()
    card::is_race(d, c, RACE_DRAGON) && card::is_type(d, c, TYPE_XYZ) && card::is_faceup(d, c)
}
/// `s.spcon(e,tp,eg,ep,ev,re,r,rp)`
fn spcon(d: &mut Duel, a: &EffArgs) -> bool {
    // return Duel.IsExistingMatchingCard(s.spconfilter,tp,LOCATION_MZONE,LOCATION_MZONE,1,nil)
    duel::is_existing_matching_card(d, Some(&spconfilter), a.tp, LOCATION_MZONE, LOCATION_MZONE, 1, NO_EX)
}
/// `s.sptg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn sptg(d: &mut Duel, a: &EffArgs) -> bool {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if chk==0 then return (Duel.GetLocationCount(tp,LOCATION_MZONE)>0 and c:IsCanBeSpecialSummoned(e,0,tp,false,false))
    //     or (Duel.GetLocationCount(1-tp,LOCATION_MZONE)>0 and c:IsCanBeSpecialSummoned(e,0,tp,false,false,POS_FACEUP,1-tp)) end
    if a.chk == Some(0) {
        return (duel::get_location_count(d, a.tp, LOCATION_MZONE as u8, None, None, None).0 > 0
            && card::is_can_be_special_summoned(d, c, a.e, 0, a.tp, false, false, None, None, None))
            || (duel::get_location_count(d, 1 - a.tp, LOCATION_MZONE as u8, None, None, None).0 > 0
                && card::is_can_be_special_summoned(d, c, a.e, 0, a.tp, false, false, Some(POS_FACEUP as u8), Some(1 - a.tp), None));
    }
    // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,c,1,tp,0)
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, Some(CardSet::from_card(c)), 1, a.tp, 0);
    true
}
/// `s.spop(e,tp,eg,ep,ev,re,r,rp)`
fn spop(d: &mut Duel, a: &EffArgs) {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if not c:IsRelateToEffect(e) then return end
    if !card::is_relate_to_effect(d, c, a.e) {
        return;
    }
    // local b1=Duel.GetLocationCount(tp,LOCATION_MZONE)>0 and c:IsCanBeSpecialSummoned(e,0,tp,false,false)
    let b1 = duel::get_location_count(d, a.tp, LOCATION_MZONE as u8, None, None, None).0 > 0
        && card::is_can_be_special_summoned(d, c, a.e, 0, a.tp, false, false, None, None, None);
    // local b2=Duel.GetLocationCount(1-tp,LOCATION_MZONE)>0 and c:IsCanBeSpecialSummoned(e,0,tp,false,false,POS_FACEUP,1-tp)
    let b2 = duel::get_location_count(d, 1 - a.tp, LOCATION_MZONE as u8, None, None, None).0 > 0
        && card::is_can_be_special_summoned(d, c, a.e, 0, a.tp, false, false, Some(POS_FACEUP as u8), Some(1 - a.tp), None);
    // if not (b1 or b2) then return end
    if !(b1 || b2) {
        return;
    }
    // local op=Duel.SelectEffect(tp,{b1,aux.Stringid(id,3)},{b2,aux.Stringid(id,4)})
    let op = duel::select_effect(d, a.tp, &[(b1, aux::stringid(ID, 3)), (b2, aux::stringid(ID, 4))]).nil();
    // local target_player=op==1 and tp or 1-tp
    let target_player = if op == 1 { a.tp } else { 1 - a.tp };
    // Duel.SpecialSummon(c,0,tp,target_player,false,false,POS_FACEUP)
    duel::special_summon(d, c, 0, a.tp, target_player, false, false, POS_FACEUP as u8, None);
}
/// `s.thtg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn thtg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return true end
    if a.chk == Some(0) {
        return true;
    }
    // Duel.SetOperationInfo(0,CATEGORY_TOHAND,nil,1,e:GetHandler():GetOwner(),LOCATION_DECK)
    let h = effect::get_handler(d, a.e).nil();
    let owner = card::get_owner(d, h);
    duel::set_operation_info(d, 0, CATEGORY_TOHAND, None, 1, owner, LOCATION_DECK as i32);
    true
}
/// `s.thfilter(c)`
fn thfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsSetCard(SET_TACHYON) and c:IsSpellTrap() and c:IsAbleToHand()
    card::is_set_card(d, c, &[SET_TACHYON as u16]) && card::is_spell_trap(d, c) && card::is_able_to_hand(d, c, None)
}
/// `s.thop(e,tp,eg,ep,ev,re,r,rp)`
fn thop(d: &mut Duel, a: &EffArgs) {
    // local p=e:GetHandler():GetOwner()
    let h = effect::get_handler(d, a.e).nil();
    let p = card::get_owner(d, h);
    // Duel.Hint(HINT_SELECTMSG,p,HINTMSG_ATOHAND)
    duel::hint(d, HINT_SELECTMSG as u8, p, HINTMSG_ATOHAND as u64);
    // local g=Duel.SelectMatchingCard(p,s.thfilter,p,LOCATION_DECK,0,1,1,nil)
    let g = duel::select_matching_card(d, p, Some(&thfilter), p, LOCATION_DECK, 0, 1, 1, false, NO_EX).nil();
    // if #g>0 then
    if !g.is_empty() {
        // Duel.SendtoHand(g,p,REASON_EFFECT)
        duel::send_to_hand(d, g.clone(), Some(p), REASON_EFFECT, None);
        // Duel.ConfirmCards(1-p,g)
        duel::confirm_cards(d, 1 - p, g);
    }
}
