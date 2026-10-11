//! Triamid Loading
use super::prelude::*;

pub const ID: u32 = 64576557;

pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_TRIAMID], listed_names: &[ID], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // local e1=Effect.CreateEffect(c)
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    // e1:SetCode(EVENT_FREE_CHAIN)
    // c:RegisterEffect(e1)
    let e1 = effect::create_effect(d, c);
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    card::register_effect(d, c, e1, false);
    // Add 1 "Triamid" Spell/Trap from your Deck to your hand
    // local e2=Effect.CreateEffect(c)
    // e2:SetDescription(aux.Stringid(id,0))
    // e2:SetCategory(CATEGORY_TOHAND+CATEGORY_SEARCH)
    // e2:SetProperty(EFFECT_FLAG_DELAY)
    // e2:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_TRIGGER_O)
    // e2:SetRange(LOCATION_SZONE)
    // e2:SetCode(EVENT_MOVE)
    // e2:SetCountLimit(1,id)
    // e2:SetCondition(s.thcon)
    // e2:SetTarget(s.thtg)
    // e2:SetOperation(s.thop)
    // c:RegisterEffect(e2)
    let e2 = effect::create_effect(d, c);
    effect::set_description(d, e2, aux::stringid(ID, 0));
    effect::set_category(d, e2, CATEGORY_TOHAND + CATEGORY_SEARCH);
    effect::set_property(d, e2, EFFECT_FLAG_DELAY, 0);
    effect::set_type(d, e2, EFFECT_TYPE_FIELD + EFFECT_TYPE_TRIGGER_O);
    effect::set_range(d, e2, LOCATION_SZONE);
    effect::set_code(d, e2, EVENT_MOVE);
    effect::set_count_limit(d, e2, 1, ID, 0, 0);
    effect::set_condition(d, e2, Some(condition(thcon)));
    effect::set_target(d, e2, Some(cost_target(thtg)));
    effect::set_operation(d, e2, Some(operation(thop)));
    card::register_effect(d, c, e2, false);
    // local e3=e2:Clone()
    // e3:SetCode(EVENT_CHAIN_SOLVED)
    // c:RegisterEffect(e3)
    let e3 = effect::clone(d, e2);
    effect::set_code(d, e3, EVENT_CHAIN_SOLVED);
    card::register_effect(d, c, e3, false);
    // Special Summon 1 "Triamid" monster from your Deck
    // local e4=Effect.CreateEffect(c)
    // e4:SetDescription(aux.Stringid(id,1))
    // e4:SetCategory(CATEGORY_SPECIAL_SUMMON)
    // e4:SetProperty(EFFECT_FLAG_DELAY+EFFECT_FLAG_DAMAGE_STEP)
    // e4:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_TRIGGER_O)
    // e4:SetRange(LOCATION_SZONE)
    // e4:SetCode(EVENT_DESTROYED)
    // e4:SetCountLimit(1,{id,1})
    // e4:SetCondition(s.spcon)
    // e4:SetTarget(s.sptg)
    // e4:SetOperation(s.spop)
    // c:RegisterEffect(e4)
    let e4 = effect::create_effect(d, c);
    effect::set_description(d, e4, aux::stringid(ID, 1));
    effect::set_category(d, e4, CATEGORY_SPECIAL_SUMMON);
    effect::set_property(d, e4, EFFECT_FLAG_DELAY + EFFECT_FLAG_DAMAGE_STEP, 0);
    effect::set_type(d, e4, EFFECT_TYPE_FIELD + EFFECT_TYPE_TRIGGER_O);
    effect::set_range(d, e4, LOCATION_SZONE);
    effect::set_code(d, e4, EVENT_DESTROYED);
    effect::set_count_limit(d, e4, 1, ID, 1, 0);
    effect::set_condition(d, e4, Some(condition(spcon)));
    effect::set_target(d, e4, Some(cost_target(sptg)));
    effect::set_operation(d, e4, Some(operation(spop)));
    card::register_effect(d, c, e4, false);
}
// function s.thconfilter(c)
fn thconfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsFaceup() and c:IsLocation(LOCATION_FZONE) and c:IsFieldSpell()
    card::is_faceup(d, c) && card::is_location(d, c, LOCATION_FZONE) && card::is_field_spell(d, c)
}
// function s.thcon(e,tp,eg,ep,ev,re,r,rp)
fn thcon(d: &mut Duel, a: &EffArgs) -> bool {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if ((eg and eg:IsExists(s.thconfilter,1,nil)) or (re and re:IsHasType(EFFECT_TYPE_ACTIVATE) and re:IsActiveType(TYPE_FIELD))) and c:GetFlagEffect(id)==0 then
    let b1 = match a.eg.clone() {
        Some(eg) => group::is_exists(d, &eg, &thconfilter, 1, NO_EX),
        None => false,
    };
    let b2 = match a.re {
        Some(re) => effect::is_has_type(d, re, EFFECT_TYPE_ACTIVATE) && effect::is_active_type(d, re, TYPE_FIELD),
        None => false,
    };
    if (b1 || b2) && card::get_flag_effect(d, c, ID) == 0 {
        // c:RegisterFlagEffect(id,RESET_CHAIN,0,1)
        // return true
        card::register_flag_effect(d, c, ID, RESET_CHAIN, 0, 1, 0, 0);
        return true;
    }
    false
}
// function s.thfilter(c)
fn thfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsSetCard(SET_TRIAMID) and c:IsSpellTrap() and c:IsAbleToHand() and not c:IsCode(id)
    card::is_set_card(d, c, &[SET_TRIAMID as u16]) && card::is_spell_trap(d, c) && card::is_able_to_hand(d, c, None) && !card::is_code(d, c, &[ID])
}
// function s.thtg(e,tp,eg,ep,ev,re,r,rp,chk)
fn thtg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // if chk==0 then return Duel.IsExistingMatchingCard(s.thfilter,tp,LOCATION_DECK,0,1,nil) end
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, Some(&thfilter), tp, LOCATION_DECK, 0, 1, NO_EX);
    }
    // Duel.SetOperationInfo(0,CATEGORY_TOHAND,nil,1,tp,LOCATION_DECK)
    duel::set_operation_info(d, 0, CATEGORY_TOHAND, None, 1, tp, LOCATION_DECK as i32);
    true
}
// function s.thop(e,tp,eg,ep,ev,re,r,rp)
fn thop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_ATOHAND)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_ATOHAND as u64);
    // local g=Duel.SelectMatchingCard(tp,s.thfilter,tp,LOCATION_DECK,0,1,1,nil)
    let g = duel::select_matching_card(d, tp, Some(&thfilter), tp, LOCATION_DECK, 0, 1, 1, false, NO_EX).nil();
    // if #g>0 then
    if !g.is_empty() {
        // Duel.SendtoHand(g,nil,REASON_EFFECT)
        duel::send_to_hand(d, g.clone(), None, REASON_EFFECT, None);
        // Duel.ConfirmCards(1-tp,g)
        duel::confirm_cards(d, 1 - tp, g);
    }
}
// function s.spconfilter(c,e,tp,eg)
fn spconfilter(d: &mut Duel, c: CardId, e: EffectId, tp: u8, eg: &CardSet) -> bool {
    // return (c:IsReason(REASON_BATTLE) or (c:IsReason(REASON_EFFECT) and c:GetReasonPlayer()==1-tp))
    (card::is_reason(d, c, REASON_BATTLE) || (card::is_reason(d, c, REASON_EFFECT) && card::get_reason_player(d, c) == 1 - tp))
        // and c:GetPreviousRaceOnField()&RACE_ROCK==RACE_ROCK and c:IsPreviousLocation(LOCATION_MZONE)
        && (card::get_previous_race_on_field(d, c) & RACE_ROCK) == RACE_ROCK && card::is_previous_location(d, c, LOCATION_MZONE)
        // and c:IsPreviousPosition(POS_FACEUP) and c:IsPreviousControler(tp)
        && card::is_previous_position(d, c, POS_FACEUP) && card::is_previous_controler(d, c, tp)
        // and Duel.IsExistingMatchingCard(s.spfilter,tp,LOCATION_DECK,0,1,nil,e,tp,eg)
        && duel::is_existing_matching_card(d, Some(&|d, cc| spfilter(d, cc, e, tp, eg)), tp, LOCATION_DECK, 0, 1, NO_EX)
}
// function s.spfilter(c,e,tp,eg)
fn spfilter(d: &mut Duel, c: CardId, e: EffectId, tp: u8, eg: &CardSet) -> bool {
    // return c:IsSetCard(SET_TRIAMID) and c:IsMonster() and c:IsCanBeSpecialSummoned(e,0,tp,false,false)
    card::is_set_card(d, c, &[SET_TRIAMID as u16])
        && card::is_monster(d, c)
        && card::is_can_be_special_summoned(d, c, e, 0, tp, false, false, None, None, None)
        // and not eg:IsExists(Card.IsCode,1,nil,c:GetCode())
        && {
            let codes = card::get_code(d, c);
            !group::is_exists(d, eg, &|d, cc| card::is_code(d, cc, &codes), 1, NO_EX)
        }
}
// function s.spcon(e,tp,eg,ep,ev,re,r,rp)
fn spcon(d: &mut Duel, a: &EffArgs) -> bool {
    let eg = a.eg.clone().nil();
    let (e, tp) = (a.e, a.tp);
    // return eg:IsExists(s.spconfilter,1,nil,e,tp,eg)
    group::is_exists(d, &eg, &|d, c| spconfilter(d, c, e, tp, &eg), 1, NO_EX)
}
// function s.sptg(e,tp,eg,ep,ev,re,r,rp,chk)
fn sptg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // if chk==0 then return Duel.GetLocationCount(tp,LOCATION_MZONE)>0 end
    if a.chk == Some(0) {
        return duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 > 0;
    }
    // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,nil,1,tp,LOCATION_DECK)
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, None, 1, tp, LOCATION_DECK as i32);
    true
}
// function s.spop(e,tp,eg,ep,ev,re,r,rp)
fn spop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    let eg = a.eg.clone().nil();
    let e = a.e;
    // if Duel.GetLocationCount(tp,LOCATION_MZONE)<=0 then return end
    if duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 <= 0 {
        return;
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_SPSUMMON)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_SPSUMMON as u64);
    // local g=Duel.SelectMatchingCard(tp,s.spfilter,tp,LOCATION_DECK,0,1,1,nil,e,tp,eg)
    let g = duel::select_matching_card(d, tp, Some(&|d, c| spfilter(d, c, e, tp, &eg)), tp, LOCATION_DECK, 0, 1, 1, false, NO_EX).nil();
    // if #g>0 then
    if !g.is_empty() {
        // Duel.SpecialSummon(g,0,tp,tp,false,false,POS_FACEUP)
        duel::special_summon(d, g, 0, tp, tp, false, false, POS_FACEUP as u8, None);
    }
}
