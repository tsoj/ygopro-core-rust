//! Hideout in the Sky, Coulomb
//! 天空城塞クーロン
//! scripted by pyrQ
use super::prelude::*;

pub const ID: u32 = 37654623;
/// `local TOKEN_MECH_BEAST=id+1`
const TOKEN_MECH_BEAST: u32 = ID + 1;

pub const INFO: ScriptInfo = ScriptInfo {
    listed_names: &[TOKEN_MECH_BEAST],
    listed_series: &[SET_BLITZCLIQUE],
    ..ScriptInfo::DEFAULT
};

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate
    // local e0=Effect.CreateEffect(c)
    let e0 = effect::create_effect(d, c);
    // e0:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e0, EFFECT_TYPE_ACTIVATE);
    // e0:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e0, EVENT_FREE_CHAIN);
    // c:RegisterEffect(e0)
    card::register_effect(d, c, e0, false);
    // Monsters you control that were Normal or Special Summoned from the hand gain 500 ATK/DEF
    // local e1a=Effect.CreateEffect(c)
    let e1a = effect::create_effect(d, c);
    // e1a:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e1a, EFFECT_TYPE_FIELD);
    // e1a:SetCode(EFFECT_UPDATE_ATTACK)
    effect::set_code(d, e1a, EFFECT_UPDATE_ATTACK);
    // e1a:SetRange(LOCATION_FZONE)
    effect::set_range(d, e1a, LOCATION_FZONE);
    // e1a:SetTargetRange(LOCATION_MZONE,0)
    effect::set_target_range(d, e1a, LOCATION_MZONE, 0);
    // e1a:SetTarget(function(e,c) return (c:IsNormalSummoned() or c:IsSpecialSummoned()) and c:IsSummonLocation(LOCATION_HAND) end)
    effect::set_target(d, e1a, Some(effect_card_fn(|d, _e, c| (card::is_normal_summoned(d, c) || card::is_special_summoned(d, c)) && card::is_summon_location(d, c, LOCATION_HAND))));
    // e1a:SetValue(500)
    effect::set_value(d, e1a, 500);
    // c:RegisterEffect(e1a)
    card::register_effect(d, c, e1a, false);
    // local e1b=e1a:Clone()
    let e1b = effect::clone(d, e1a);
    // e1b:SetCode(EFFECT_UPDATE_DEFENSE)
    effect::set_code(d, e1b, EFFECT_UPDATE_DEFENSE);
    // c:RegisterEffect(e1b)
    card::register_effect(d, c, e1b, false);
    // During your Main Phase: You can Special Summon 1 "Mech Beast Token" (Machine/EARTH/Level 6/ATK 2000/DEF 2000) to your opponent's field, and if you do, add 1 "Blitzclique" monster from your Deck to your hand
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e2, aux::stringid(ID, 0));
    // e2:SetCategory(CATEGORY_SPECIAL_SUMMON+CATEGORY_TOKEN+CATEGORY_TOHAND+CATEGORY_SEARCH)
    effect::set_category(d, e2, CATEGORY_SPECIAL_SUMMON + CATEGORY_TOKEN + CATEGORY_TOHAND + CATEGORY_SEARCH);
    // e2:SetType(EFFECT_TYPE_IGNITION)
    effect::set_type(d, e2, EFFECT_TYPE_IGNITION);
    // e2:SetRange(LOCATION_FZONE)
    effect::set_range(d, e2, LOCATION_FZONE);
    // e2:SetCountLimit(1,{id,0})
    effect::set_count_limit(d, e2, 1, ID, 0, 0);
    // e2:SetTarget(s.tokentg)
    effect::set_target(d, e2, Some(cost_target(tokentg)));
    // e2:SetOperation(s.tokenop)
    effect::set_operation(d, e2, Some(operation(tokenop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
    // If a card(s) is destroyed by your "Blitzclique" card's effect, while this card is in your GY: You can add this card to your hand
    // local e3=Effect.CreateEffect(c)
    let e3 = effect::create_effect(d, c);
    // e3:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e3, aux::stringid(ID, 1));
    // e3:SetCategory(CATEGORY_TOHAND)
    effect::set_category(d, e3, CATEGORY_TOHAND);
    // e3:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e3, EFFECT_TYPE_FIELD + EFFECT_TYPE_TRIGGER_O);
    // e3:SetProperty(EFFECT_FLAG_DELAY,EFFECT_FLAG2_CHECK_SIMULTANEOUS)
    effect::set_property(d, e3, EFFECT_FLAG_DELAY, EFFECT_FLAG2_CHECK_SIMULTANEOUS);
    // e3:SetCode(EVENT_DESTROYED)
    effect::set_code(d, e3, EVENT_DESTROYED);
    // e3:SetRange(LOCATION_GRAVE)
    effect::set_range(d, e3, LOCATION_GRAVE);
    // e3:SetCountLimit(1,{id,1})
    effect::set_count_limit(d, e3, 1, ID, 1, 0);
    // e3:SetCondition(s.selfthcon)
    effect::set_condition(d, e3, Some(condition(selfthcon)));
    // e3:SetTarget(s.selfthtg)
    effect::set_target(d, e3, Some(cost_target(selfthtg)));
    // e3:SetOperation(s.selfthop)
    effect::set_operation(d, e3, Some(operation(selfthop)));
    // c:RegisterEffect(e3)
    card::register_effect(d, c, e3, false);
}
// function s.thfilter(c)
fn thfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsSetCard(SET_BLITZCLIQUE) and c:IsMonster() and c:IsAbleToHand()
    card::is_set_card(d, c, &[SET_BLITZCLIQUE as u16]) && card::is_monster(d, c) && card::is_able_to_hand(d, c, None)
}
// Mech Beast Token: (Machine/EARTH/Level 6/ATK 2000/DEF 2000), Duel.IsPlayerCanSpecialSummonMonster(...,TYPES_TOKEN,2000,2000,6,RACE_MACHINE,ATTRIBUTE_EARTH,...)
fn mech_beast_data() -> duel::MonsterData {
    duel::MonsterData {
        setcodes: Some(vec![0]),
        type_: Some(TYPES_TOKEN),
        attack: Some(2000),
        defense: Some(2000),
        level: Some(6),
        race: Some(RACE_MACHINE),
        attribute: Some(ATTRIBUTE_EARTH),
    }
}
// function s.tokentg(e,tp,eg,ep,ev,re,r,rp,chk)
fn tokentg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return Duel.GetLocationCount(1-tp,LOCATION_MZONE)>0
    //     and Duel.IsPlayerCanSpecialSummonMonster(tp,TOKEN_MECH_BEAST,0,TYPES_TOKEN,2000,2000,6,RACE_MACHINE,ATTRIBUTE_EARTH,POS_FACEUP,1-tp)
    //     and Duel.IsExistingMatchingCard(s.thfilter,tp,LOCATION_DECK,0,1,nil) end
    if a.chk == Some(0) {
        return duel::get_location_count(d, 1 - a.tp, LOCATION_MZONE as u8, None, None, None).0 > 0
            && duel::is_player_can_special_summon_monster(d, a.tp, TOKEN_MECH_BEAST, mech_beast_data(), Some(POS_FACEUP as u8), Some(1 - a.tp), 0)
            && duel::is_existing_matching_card(d, Some(&thfilter), a.tp, LOCATION_DECK, 0, 1, NO_EX);
    }
    // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,nil,1,tp,0)
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, None, 1, a.tp, 0);
    // Duel.SetOperationInfo(0,CATEGORY_TOKEN,nil,1,tp,0)
    duel::set_operation_info(d, 0, CATEGORY_TOKEN, None, 1, a.tp, 0);
    // Duel.SetOperationInfo(0,CATEGORY_TOHAND,nil,1,tp,LOCATION_DECK)
    duel::set_operation_info(d, 0, CATEGORY_TOHAND, None, 1, a.tp, LOCATION_DECK as i32);
    true
}
// function s.tokenop(e,tp,eg,ep,ev,re,r,rp)
fn tokenop(d: &mut Duel, a: &EffArgs) {
    // local opp=1-tp
    let opp = 1 - a.tp;
    // if Duel.GetLocationCount(opp,LOCATION_MZONE)<=0
    //     or not Duel.IsPlayerCanSpecialSummonMonster(tp,TOKEN_MECH_BEAST,0,TYPES_TOKEN,2000,2000,6,RACE_MACHINE,ATTRIBUTE_EARTH,POS_FACEUP,opp) then return end
    if duel::get_location_count(d, opp, LOCATION_MZONE as u8, None, None, None).0 <= 0
        || !duel::is_player_can_special_summon_monster(d, a.tp, TOKEN_MECH_BEAST, mech_beast_data(), Some(POS_FACEUP as u8), Some(opp), 0)
    {
        return;
    }
    // local token=Duel.CreateToken(tp,TOKEN_MECH_BEAST)
    let token = duel::create_token(d, a.tp, TOKEN_MECH_BEAST).nil();
    // if Duel.SpecialSummon(token,0,tp,opp,false,false,POS_FACEUP)>0 then
    if duel::special_summon(d, token, 0, a.tp, opp, false, false, POS_FACEUP as u8, None) > 0 {
        // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_ATOHAND)
        duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_ATOHAND as u64);
        // local g=Duel.SelectMatchingCard(tp,s.thfilter,tp,LOCATION_DECK,0,1,1,nil)
        let g = duel::select_matching_card(d, a.tp, Some(&thfilter), a.tp, LOCATION_DECK, 0, 1, 1, false, NO_EX).nil();
        // if #g>0 then
        if !g.is_empty() {
            // Duel.SendtoHand(g,nil,REASON_EFFECT)
            duel::send_to_hand(d, g.clone(), None, REASON_EFFECT, None);
            // Duel.ConfirmCards(opp,g)
            duel::confirm_cards(d, opp, g);
        }
    }
}
// function s.selfthcon(e,tp,eg,ep,ev,re,r,rp)
fn selfthcon(d: &mut Duel, a: &EffArgs) -> bool {
    // return rp==tp and re and not eg:IsContains(e:GetHandler()) and eg:IsExists(Card.IsReason,1,nil,REASON_EFFECT)
    //     and re:GetHandler() and re:GetHandler():IsSetCard(SET_BLITZCLIQUE)
    if a.rp != a.tp {
        return false;
    }
    let Some(re) = a.re else {
        return false;
    };
    let eg = a.eg.clone().nil();
    let handler = effect::get_handler(d, a.e).nil();
    if group::is_contains(&eg, handler) {
        return false;
    }
    if !group::is_exists(d, &eg, &|d, c| card::is_reason(d, c, REASON_EFFECT), 1, NO_EX) {
        return false;
    }
    match effect::get_handler(d, re) {
        Some(rc) => card::is_set_card(d, rc, &[SET_BLITZCLIQUE as u16]),
        None => false,
    }
}
// function s.selfthtg(e,tp,eg,ep,ev,re,r,rp,chk)
fn selfthtg(d: &mut Duel, a: &EffArgs) -> bool {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if chk==0 then return c:IsAbleToHand() end
    if a.chk == Some(0) {
        return card::is_able_to_hand(d, c, None);
    }
    // Duel.SetOperationInfo(0,CATEGORY_TOHAND,c,1,tp,0)
    duel::set_operation_info(d, 0, CATEGORY_TOHAND, Some(CardSet::from_card(c)), 1, a.tp, 0);
    true
}
// function s.selfthop(e,tp,eg,ep,ev,re,r,rp)
fn selfthop(d: &mut Duel, a: &EffArgs) {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if c:IsRelateToEffect(e) then
    if card::is_relate_to_effect(d, c, a.e) {
        // Duel.SendtoHand(c,nil,REASON_EFFECT)
        duel::send_to_hand(d, c, None, REASON_EFFECT, None);
    }
}
