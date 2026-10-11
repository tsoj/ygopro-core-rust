//! Gouki Dragon Ogre
//! scripted by Naim
use super::prelude::*;

pub const ID: u32 = 69453825;

pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_GOUKI], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // c:EnableReviveLimit()
    card::enable_revive_limit(d, c);
    // --Synchro Summon procedure: 1 Tuner + 1+ non-Tuner monsters
    // Synchro.AddProcedure(c,nil,1,1,Synchro.NonTuner(nil),1,99)
    synchro::add_procedure(d, c, None, 1, 1, Some(synchro::non_tuner(None)), 1, 99, None, None, None, None, None);
    // --Add 1 "Gouki" Spell from your Deck to your hand
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_TOHAND+CATEGORY_SEARCH)
    effect::set_category(d, e1, CATEGORY_TOHAND + CATEGORY_SEARCH);
    // e1:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_O);
    // e1:SetProperty(EFFECT_FLAG_DELAY)
    effect::set_property(d, e1, EFFECT_FLAG_DELAY, 0);
    // e1:SetCode(EVENT_SPSUMMON_SUCCESS)
    effect::set_code(d, e1, EVENT_SPSUMMON_SUCCESS);
    // e1:SetCountLimit(1,id)
    effect::set_count_limit(d, e1, 1, ID, 0, 0);
    // e1:SetCondition(function(e) return e:GetHandler():IsSynchroSummoned() end)
    effect::set_condition(d, e1, Some(condition(|d, a| {
        let c = effect::get_handler(d, a.e).nil();
        card::is_synchro_summoned(d, c)
    })));
    // e1:SetTarget(s.deckthtg)
    effect::set_target(d, e1, Some(cost_target(deckthtg)));
    // e1:SetOperation(s.deckthop)
    effect::set_operation(d, e1, Some(operation(deckthop)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // --Add 1 "Gouki" monster with lower ATK than a targeted EARTH monster you control from your GY to your hand
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e2, aux::stringid(ID, 1));
    // e2:SetCategory(CATEGORY_TOHAND)
    effect::set_category(d, e2, CATEGORY_TOHAND);
    // e2:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e2, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_O);
    // e2:SetProperty(EFFECT_FLAG_CARD_TARGET+EFFECT_FLAG_DELAY)
    effect::set_property(d, e2, EFFECT_FLAG_CARD_TARGET + EFFECT_FLAG_DELAY, 0);
    // e2:SetCode(EVENT_TO_GRAVE)
    effect::set_code(d, e2, EVENT_TO_GRAVE);
    // e2:SetCountLimit(1,{id,1})
    effect::set_count_limit(d, e2, 1, ID, 1, 0);
    // e2:SetTarget(s.gythtg)
    effect::set_target(d, e2, Some(cost_target(gythtg)));
    // e2:SetOperation(s.gythop)
    effect::set_operation(d, e2, Some(operation(gythop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
}
/// `s.deckthfilter(c)`
fn deckthfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsSetCard(SET_GOUKI) and c:IsSpell() and c:IsAbleToHand()
    card::is_set_card(d, c, &[SET_GOUKI as u16]) && card::is_spell(d, c) && card::is_able_to_hand(d, c, None)
}
/// `s.deckthtg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn deckthtg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // if chk==0 then return Duel.IsExistingMatchingCard(s.deckthfilter,tp,LOCATION_DECK,0,1,nil) end
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, Some(&deckthfilter), tp, LOCATION_DECK, 0, 1, NO_EX);
    }
    // Duel.SetOperationInfo(0,CATEGORY_TOHAND,nil,1,tp,LOCATION_DECK)
    duel::set_operation_info(d, 0, CATEGORY_TOHAND, None, 1, tp, LOCATION_DECK as i32);
    true
}
/// `s.deckthop(e,tp,eg,ep,ev,re,r,rp)`
fn deckthop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_ATOHAND)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_ATOHAND as u64);
    // local g=Duel.SelectMatchingCard(tp,s.deckthfilter,tp,LOCATION_DECK,0,1,1,nil)
    let g = duel::select_matching_card(d, tp, Some(&deckthfilter), tp, LOCATION_DECK, 0, 1, 1, false, NO_EX).nil();
    // if #g>0 then
    if !g.is_empty() {
        // Duel.SendtoHand(g,nil,REASON_EFFECT)
        duel::send_to_hand(d, g.clone(), None, REASON_EFFECT, None);
        // Duel.ConfirmCards(1-tp,g)
        duel::confirm_cards(d, 1 - tp, g);
    }
}
/// `s.earthfilter(c,tp)`
fn earthfilter(d: &mut Duel, c: CardId, tp: u8) -> bool {
    // return c:IsAttribute(ATTRIBUTE_EARTH) and c:IsFaceup()
    //   and Duel.IsExistingMatchingCard(s.gythfilter,tp,LOCATION_GRAVE,0,1,nil,c:GetAttack())
    card::is_attribute(d, c, ATTRIBUTE_EARTH)
        && card::is_faceup(d, c)
        && {
            let atk = card::get_attack(d, c);
            duel::is_existing_matching_card(d, Some(&|d, x| gythfilter(d, x, atk)), tp, LOCATION_GRAVE, 0, 1, NO_EX)
        }
}
/// `s.gythfilter(c,atk)`
fn gythfilter(d: &mut Duel, c: CardId, atk: i32) -> bool {
    // return c:IsSetCard(SET_GOUKI) and c:IsMonster() and c:GetAttack()<atk and c:IsAbleToHand()
    card::is_set_card(d, c, &[SET_GOUKI as u16]) && card::is_monster(d, c) && card::get_attack(d, c) < atk && card::is_able_to_hand(d, c, None)
}
/// `s.gythtg(e,tp,eg,ep,ev,re,r,rp,chk,chkc)`
fn gythtg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // if chkc then return chkc:IsControler(tp) and chkc:IsLocation(LOCATION_MZONE) and s.earthfilter(chkc,tp) end
    if let Some(chkc) = a.chkc() {
        return card::is_controler(d, chkc, tp) && card::is_location(d, chkc, LOCATION_MZONE) && earthfilter(d, chkc, tp);
    }
    // if chk==0 then return Duel.IsExistingTarget(s.earthfilter,tp,LOCATION_MZONE,0,1,nil,tp) end
    if a.chk == Some(0) {
        return duel::is_existing_target(d, Some(&|d, x| earthfilter(d, x, tp)), tp, LOCATION_MZONE, 0, 1, NO_EX);
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_TARGET)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_TARGET as u64);
    // Duel.SelectTarget(tp,s.earthfilter,tp,LOCATION_MZONE,0,1,1,nil,tp)
    duel::select_target(d, tp, Some(&|d, x| earthfilter(d, x, tp)), tp, LOCATION_MZONE, 0, 1, 1, false, NO_EX);
    // Duel.SetOperationInfo(0,CATEGORY_TOHAND,nil,1,tp,LOCATION_GRAVE)
    duel::set_operation_info(d, 0, CATEGORY_TOHAND, None, 1, tp, LOCATION_GRAVE as i32);
    true
}
/// `s.gythop(e,tp,eg,ep,ev,re,r,rp)`
fn gythop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    let e = a.e;
    // local tc=Duel.GetFirstTarget()
    let tc = duel::first_target(d).nil();
    // if tc:IsFaceup() and tc:IsRelateToEffect(e) then
    if card::is_faceup(d, tc) && card::is_relate_to_effect(d, tc, e) {
        // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_ATOHAND)
        duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_ATOHAND as u64);
        // local g=Duel.SelectMatchingCard(tp,s.gythfilter,tp,LOCATION_GRAVE,0,1,1,nil,tc:GetAttack())
        let atk = card::get_attack(d, tc);
        let g = duel::select_matching_card(d, tp, Some(&|d, x| gythfilter(d, x, atk)), tp, LOCATION_GRAVE, 0, 1, 1, false, NO_EX).nil();
        // if #g>0 then
        if !g.is_empty() {
            // Duel.HintSelection(g)
            duel::hint_selection(d, g.clone(), true);
            // Duel.SendtoHand(g,nil,REASON_EFFECT)
            duel::send_to_hand(d, g, None, REASON_EFFECT, None);
        }
        // tc:RegisterFlagEffect(id,RESET_EVENT|RESETS_STANDARD,EFFECT_FLAG_CLIENT_HINT,1,0,aux.Stringid(id,2))
        card::register_flag_effect(d, tc, ID, RESET_EVENT | RESETS_STANDARD, EFFECT_FLAG_CLIENT_HINT, 1, 0, aux::stringid(ID, 2));
        // local e1=Effect.CreateEffect(e:GetHandler())
        let c = effect::get_handler(d, e).nil();
        let e1 = effect::create_effect(d, c);
        // e1:SetType(EFFECT_TYPE_FIELD)
        effect::set_type(d, e1, EFFECT_TYPE_FIELD);
        // e1:SetCode(EFFECT_ONLY_ATTACK_MONSTER)
        effect::set_code(d, e1, EFFECT_ONLY_ATTACK_MONSTER);
        // e1:SetRange(LOCATION_MZONE)
        effect::set_range(d, e1, LOCATION_MZONE);
        // e1:SetTargetRange(0,LOCATION_MZONE)
        effect::set_target_range(d, e1, 0, LOCATION_MZONE);
        // e1:SetCondition(function(e) return e:GetHandler():IsControler(e:GetOwnerPlayer()) end)
        effect::set_condition(d, e1, Some(condition(|d, a| {
            let h = effect::get_handler(d, a.e).nil();
            let owner = effect::get_owner_player(d, a.e);
            card::is_controler(d, h, owner)
        })));
        // e1:SetValue(function(e,c) return c==tc end)
        effect::set_value_fn(d, e1, effect_card_fn(move |_d, _e, x| x == tc));
        // e1:SetReset(RESET_EVENT|RESETS_STANDARD)
        effect::set_reset(d, e1, RESET_EVENT | RESETS_STANDARD, 1);
        // tc:RegisterEffect(e1,true)
        card::register_effect(d, tc, e1, true);
    }
}
