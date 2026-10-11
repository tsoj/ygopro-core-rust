//! Full-Speed Warrior
use super::prelude::*;

pub const ID: u32 = 17201951;

pub const INFO: ScriptInfo = ScriptInfo {
    listed_names: &[CARD_JUNK_SYNCHRON, CARD_JUNK_WARRIOR],
    listed_series: &[SET_WARRIOR],
    ..ScriptInfo::DEFAULT
};

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Add 1 "Junk Synchron" or 1 Spell/Trap that mentions "Junk Warrior" from your Deck to your hand
    // local e1a=Effect.CreateEffect(c)
    let e1a = effect::create_effect(d, c);
    // e1a:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1a, aux::stringid(ID, 0));
    // e1a:SetCategory(CATEGORY_TOHAND+CATEGORY_SEARCH)
    effect::set_category(d, e1a, CATEGORY_TOHAND + CATEGORY_SEARCH);
    // e1a:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e1a, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_O);
    // e1a:SetProperty(EFFECT_FLAG_DELAY)
    effect::set_property(d, e1a, EFFECT_FLAG_DELAY, 0);
    // e1a:SetCode(EVENT_SUMMON_SUCCESS)
    effect::set_code(d, e1a, EVENT_SUMMON_SUCCESS);
    // e1a:SetCountLimit(1,id)
    effect::set_count_limit(d, e1a, 1, ID, 0, 0);
    // e1a:SetTarget(s.thtg)
    effect::set_target(d, e1a, Some(cost_target(thtg)));
    // e1a:SetOperation(s.thop)
    effect::set_operation(d, e1a, Some(operation(thop)));
    // c:RegisterEffect(e1a)
    card::register_effect(d, c, e1a, false);
    // local e1b=e1a:Clone()
    let e1b = effect::clone(d, e1a);
    // e1b:SetCode(EVENT_SPSUMMON_SUCCESS)
    effect::set_code(d, e1b, EVENT_SPSUMMON_SUCCESS);
    // c:RegisterEffect(e1b)
    card::register_effect(d, c, e1b, false);
    // Monsters you control that mention "Junk Warrior", and Synchro Monsters you control with "Warrior" in their original names, gain 900 ATK during your Battle Phase only
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e2, EFFECT_TYPE_FIELD);
    // e2:SetCode(EFFECT_UPDATE_ATTACK)
    effect::set_code(d, e2, EFFECT_UPDATE_ATTACK);
    // e2:SetRange(LOCATION_MZONE)
    effect::set_range(d, e2, LOCATION_MZONE);
    // e2:SetTargetRange(LOCATION_MZONE,0)
    effect::set_target_range(d, e2, LOCATION_MZONE, 0);
    // e2:SetCondition(function(e) return Duel.IsBattlePhase() and Duel.IsTurnPlayer(e:GetHandlerPlayer()) end)
    effect::set_condition(d, e2, Some(condition(|d, a| {
        let hp = effect::get_handler_player(d, a.e);
        duel::is_battle_phase(d, None) && duel::is_turn_player(d, hp)
    })));
    // e2:SetTarget(function(e,c) return c:ListsCode(CARD_JUNK_WARRIOR) or (c:IsType(TYPE_SYNCHRO) and c:IsOriginalSetCard(SET_WARRIOR)) end)
    effect::set_target(d, e2, Some(effect_card_fn(|d, _e, c| {
        card::lists_code(d, c, &[CARD_JUNK_WARRIOR])
            || (card::is_type(d, c, TYPE_SYNCHRO) && card::is_original_set_card(d, c, &[SET_WARRIOR as u16]))
    })));
    // e2:SetValue(900)
    effect::set_value(d, e2, 900);
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
}
// s.thfilter(c)
fn thfilter(d: &mut Duel, c: CardId) -> bool {
    // return (c:IsCode(CARD_JUNK_SYNCHRON) or (c:IsSpellTrap() and c:ListsCode(CARD_JUNK_WARRIOR))) and c:IsAbleToHand()
    (card::is_code(d, c, &[CARD_JUNK_SYNCHRON]) || (card::is_spell_trap(d, c) && card::lists_code(d, c, &[CARD_JUNK_WARRIOR])))
        && card::is_able_to_hand(d, c, None)
}
// s.thtg(e,tp,eg,ep,ev,re,r,rp,chk)
fn thtg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return Duel.IsExistingMatchingCard(s.thfilter,tp,LOCATION_DECK,0,1,nil) end
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, Some(&thfilter), a.tp, LOCATION_DECK, 0, 1, NO_EX);
    }
    // Duel.SetOperationInfo(0,CATEGORY_TOHAND,nil,1,tp,LOCATION_DECK)
    duel::set_operation_info(d, 0, CATEGORY_TOHAND, None, 1, a.tp, LOCATION_DECK as i32);
    true
}
// s.thop(e,tp,eg,ep,ev,re,r,rp)
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
