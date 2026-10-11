//! Kuebiko
//! 久延毘古
//! Scripted by Hatter
use super::prelude::*;

pub const ID: u32 = 20087414;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Special Summon this card as an Effect Monster
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_SPECIAL_SUMMON)
    effect::set_category(d, e1, CATEGORY_SPECIAL_SUMMON);
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    // e1:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    // e1:SetCountLimit(1,id,EFFECT_COUNT_CODE_OATH)
    effect::set_count_limit(d, e1, 1, ID, 0, EFFECT_COUNT_CODE_OATH as u8);
    // e1:SetHintTiming(0,TIMING_STANDBY_PHASE|TIMING_MAIN_END|TIMINGS_CHECK_MONSTER_E)
    effect::set_hint_timing(d, e1, 0, Some(TIMING_STANDBY_PHASE | TIMING_MAIN_END | TIMINGS_CHECK_MONSTER_E));
    // e1:SetTarget(s.sptg)
    effect::set_target(d, e1, Some(cost_target(sptg)));
    // e1:SetOperation(s.spop)
    effect::set_operation(d, e1, Some(operation(spop)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
/// The check of `s.sptg(...,0)`: `Duel.GetLocationCount(tp,LOCATION_MZONE)>0 and Duel.IsPlayerCanSpecialSummonMonster(tp,id,0,
/// TYPE_MONSTER|TYPE_EFFECT,0,0,2,RACE_FAIRY,ATTRIBUTE_EARTH,POS_FACEUP,tp,1)`
fn sp_check(d: &mut Duel, tp: u8) -> bool {
    duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 > 0
        && duel::is_player_can_special_summon_monster(
            d,
            tp,
            ID,
            duel::MonsterData {
                setcodes: Some(vec![0]),
                type_: Some(TYPE_MONSTER | TYPE_EFFECT),
                attack: Some(0),
                defense: Some(0),
                level: Some(2),
                race: Some(RACE_FAIRY),
                attribute: Some(ATTRIBUTE_EARTH),
            },
            Some(POS_FACEUP as u8),
            Some(tp),
            1,
        )
}
/// `s.sptg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn sptg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return ... end
    if a.chk == Some(0) {
        return sp_check(d, a.tp);
    }
    // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,e:GetHandler(),1,tp,0)
    let h = effect::get_handler(d, a.e).nil();
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, Some(CardSet::from_card(h)), 1, a.tp, 0);
    true
}
/// `s.spop(e,tp,eg,ep,ev,re,r,rp)`
fn spop(d: &mut Duel, a: &EffArgs) {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if c:IsRelateToEffect(e) and s.sptg(e,tp,eg,ep,ev,re,r,rp,0) then
    if card::is_relate_to_effect(d, c, a.e) && sp_check(d, a.tp) {
        // c:AddMonsterAttribute(TYPE_EFFECT|TYPE_TRAP)
        card::add_monster_attribute(d, c, TYPE_EFFECT | TYPE_TRAP, 0, 0, 0, 0, 0);
        // Duel.SpecialSummonStep(c,1,tp,tp,true,false,POS_FACEUP)
        duel::special_summon_step(d, c, 1, a.tp, a.tp, true, false, POS_FACEUP as u8, None);
        // Set this card in the Spell/Trap Zone and negate activated monster effect
        // local e1=Effect.CreateEffect(c)
        let e1 = effect::create_effect(d, c);
        // e1:SetDescription(aux.Stringid(id,1))
        effect::set_description(d, e1, aux::stringid(ID, 1));
        // e1:SetCategory(CATEGORY_DISABLE+CATEGORY_TOHAND+CATEGORY_SET)
        effect::set_category(d, e1, CATEGORY_DISABLE + CATEGORY_TOHAND + CATEGORY_SET);
        // e1:SetType(EFFECT_TYPE_QUICK_O)
        effect::set_type(d, e1, EFFECT_TYPE_QUICK_O);
        // e1:SetCode(EVENT_CHAINING)
        effect::set_code(d, e1, EVENT_CHAINING);
        // e1:SetRange(LOCATION_MZONE)
        effect::set_range(d, e1, LOCATION_MZONE);
        // e1:SetCountLimit(1,0,EFFECT_COUNT_CODE_CHAIN)
        effect::set_count_limit(d, e1, 1, 0, 0, EFFECT_COUNT_CODE_CHAIN as u8);
        // e1:SetCondition(s.setcon)
        effect::set_condition(d, e1, Some(condition(setcon)));
        // e1:SetTarget(s.settg)
        effect::set_target(d, e1, Some(cost_target(settg)));
        // e1:SetOperation(s.setop)
        effect::set_operation(d, e1, Some(operation(setop)));
        // e1:SetReset(RESET_EVENT|RESETS_STANDARD)
        effect::set_reset(d, e1, RESET_EVENT | RESETS_STANDARD, 1);
        // c:RegisterEffect(e1,true)
        card::register_effect(d, c, e1, true);
        // c:AddMonsterAttributeComplete()
        card::add_monster_attribute_complete(d, c);
    }
    // Duel.SpecialSummonComplete()
    duel::special_summon_complete(d);
}
/// `s.setconfilter(c,tp)`
fn setconfilter(d: &mut Duel, c: CardId, tp: u8) -> bool {
    // return c:IsOnField() and c:IsControler(tp)
    card::is_on_field(d, c) && card::is_controler(d, c, tp)
}
/// `s.setcon(e,tp,eg,ep,ev,re,r,rp)`
fn setcon(d: &mut Duel, a: &EffArgs) -> bool {
    // if not (rp==1-tp and re:IsMonsterEffect() and re:IsHasProperty(EFFECT_FLAG_CARD_TARGET) and Duel.IsChainDisablable(ev)) then return false end
    let re = a.re.nil();
    if !(a.rp == 1 - a.tp
        && effect::is_monster_effect(d, re)
        && effect::is_has_property(d, re, EFFECT_FLAG_CARD_TARGET, 0)
        && duel::is_chain_disablable(d, a.ev as u8))
    {
        return false;
    }
    // local tg,trig_loc=Duel.GetChainInfo(ev,CHAININFO_TARGET_CARDS,CHAININFO_TRIGGERING_LOCATION)
    let info = duel::get_chain_info(d, a.ev as u8, &[CHAININFO_TARGET_CARDS, CHAININFO_TRIGGERING_LOCATION]);
    // return trig_loc==LOCATION_MZONE and tg and tg:IsExists(s.setconfilter,1,nil,tp)
    let trig_loc = info[1].int();
    if trig_loc != LOCATION_MZONE as i64 {
        return false;
    }
    match info[0].as_group().cloned() {
        Some(tg) => group::is_exists(d, &tg, &|d, c| setconfilter(d, c, a.tp), 1, NO_EX),
        None => false,
    }
}
/// `s.settg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn settg(d: &mut Duel, a: &EffArgs) -> bool {
    // local c=e:GetHandler()  (the handler is needed by the check)
    let c = effect::get_handler(d, a.e).nil();
    // if chk==0 then return e:GetHandler():IsSSetable() end
    if a.chk == Some(0) {
        return card::is_s_setable(d, c, false, None);
    }
    // Duel.SetOperationInfo(0,CATEGORY_DISABLE,eg,1,tp,0)
    duel::set_operation_info(d, 0, CATEGORY_DISABLE, a.eg.clone(), 1, a.tp, 0);
    // Duel.SetPossibleOperationInfo(0,CATEGORY_TOHAND,re:GetHandler(),1,tp,0)
    let rc = effect::get_handler(d, a.re.nil()).nil();
    duel::set_possible_operation_info(d, 0, CATEGORY_TOHAND, Some(CardSet::from_card(rc)), 1, a.tp, 0);
    true
}
/// `s.setop(e,tp,eg,ep,ev,re,r,rp)`
fn setop(d: &mut Duel, a: &EffArgs) {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // local rc=re:GetHandler()
    let re = a.re.nil();
    let rc = effect::get_handler(d, re).nil();
    // if c:IsRelateToEffect(e) and c:IsLocation(LOCATION_MZONE) and c:IsSSetable() and Duel.SSet(tp,c)>0
    //   and Duel.NegateEffect(ev) and rc:IsRelateToEffect(re) and rc:IsControler(1-tp) and rc:IsFaceup()
    //   and not Duel.IsExistingMatchingCard(aux.FaceupFilter(Card.IsAttackAbove,rc:GetAttack()+1),tp,0,LOCATION_MZONE,1,nil) then
    if !(card::is_relate_to_effect(d, c, a.e) && card::is_location(d, c, LOCATION_MZONE) && card::is_s_setable(d, c, false, None)) {
        return;
    }
    if duel::s_set(d, a.tp, c, None, true) <= 0 || !duel::negate_effect(d, a.ev as u8) {
        return;
    }
    if !(card::is_relate_to_effect(d, rc, re) && card::is_controler(d, rc, 1 - a.tp) && card::is_faceup(d, rc)) {
        return;
    }
    let atk = card::get_attack(d, rc) + 1;
    if !duel::is_existing_matching_card(d, Some(&|d, x| card::is_faceup(d, x) && card::is_attack_above(d, x, atk)), a.tp, 0, LOCATION_MZONE, 1, NO_EX) {
        // Duel.BreakEffect()
        duel::break_effect(d);
        // Duel.SendtoHand(rc,nil,REASON_EFFECT)
        duel::send_to_hand(d, rc, None, REASON_EFFECT, None);
    }
}
