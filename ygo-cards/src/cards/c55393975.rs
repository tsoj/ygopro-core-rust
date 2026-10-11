//! Angelechy Destrier
use super::prelude::*;

pub const ID: u32 = 55393975;

pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_ANGELECHY], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // c:EnableReviveLimit()
    card::enable_revive_limit(d, c);
    // Synchro Summon procedure: 1 Tuner + 1+ non-Tuners
    // Synchro.AddProcedure(c,nil,1,1,Synchro.NonTuner(nil),1,99)
    synchro::add_procedure(d, c, None, 1, 1, Some(synchro::non_tuner(None)), 1, 99, None, None, None, None, None);
    // You can target 1 monster in another column; banish it
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_REMOVE)
    effect::set_category(d, e1, CATEGORY_REMOVE);
    // e1:SetType(EFFECT_TYPE_IGNITION)
    effect::set_type(d, e1, EFFECT_TYPE_IGNITION);
    // e1:SetProperty(EFFECT_FLAG_CARD_TARGET)
    effect::set_property(d, e1, EFFECT_FLAG_CARD_TARGET, 0);
    // e1:SetRange(LOCATION_MZONE)
    effect::set_range(d, e1, LOCATION_MZONE);
    // e1:SetCountLimit(1,{id,0})
    effect::set_count_limit(d, e1, 1, ID, 0, 0);
    // e1:SetTarget(s.bantg)
    effect::set_target(d, e1, Some(cost_target(bantg)));
    // e1:SetOperation(s.banop)
    effect::set_operation(d, e1, Some(operation(banop)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // If this card is placed in the Spell & Trap Zone as a Continuous Spell: You can add 1 "Angelechy" Spell from your Deck to your hand
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e2, aux::stringid(ID, 1));
    // e2:SetCategory(CATEGORY_TOHAND+CATEGORY_SEARCH)
    effect::set_category(d, e2, CATEGORY_TOHAND + CATEGORY_SEARCH);
    // e2:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e2, EFFECT_TYPE_FIELD + EFFECT_TYPE_TRIGGER_O);
    // e2:SetProperty(EFFECT_FLAG_DELAY)
    effect::set_property(d, e2, EFFECT_FLAG_DELAY, 0);
    // e2:SetCode(EVENT_MOVE)
    effect::set_code(d, e2, EVENT_MOVE);
    // e2:SetRange(LOCATION_STZONE)
    effect::set_range(d, e2, LOCATION_STZONE);
    // e2:SetCountLimit(1,{id,1})
    effect::set_count_limit(d, e2, 1, ID, 1, 0);
    // e2:SetCondition(s.thcon)
    effect::set_condition(d, e2, Some(condition(thcon)));
    // e2:SetTarget(s.thtg)
    effect::set_target(d, e2, Some(cost_target(thtg)));
    // e2:SetOperation(s.thop)
    effect::set_operation(d, e2, Some(operation(thop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
    // While this card is treated as a Continuous Spell, each time your opponent activates a card or effect, inflict 500 damage to your opponent when it resolves
    // local e3a=Effect.CreateEffect(c)
    let e3a = effect::create_effect(d, c);
    // e3a:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_CONTINUOUS)
    effect::set_type(d, e3a, EFFECT_TYPE_FIELD + EFFECT_TYPE_CONTINUOUS);
    // e3a:SetProperty(EFFECT_FLAG_CANNOT_DISABLE)
    effect::set_property(d, e3a, EFFECT_FLAG_CANNOT_DISABLE, 0);
    // e3a:SetCode(EVENT_CHAINING)
    effect::set_code(d, e3a, EVENT_CHAINING);
    // e3a:SetRange(LOCATION_STZONE)
    effect::set_range(d, e3a, LOCATION_STZONE);
    // e3a:SetCondition(function(e,tp,eg,ep)
    //     return ep==1-tp and e:GetHandler():IsContinuousSpell()
    // end)
    effect::set_condition(d, e3a, Some(condition(e3a_cond)));
    // e3a:SetOperation(function(e)
    //     e:GetHandler():RegisterFlagEffect(id,RESET_EVENT|RESETS_STANDARD|RESET_CHAIN,0,1)
    // end)
    effect::set_operation(d, e3a, Some(operation(e3a_op)));
    // c:RegisterEffect(e3a)
    card::register_effect(d, c, e3a, false);
    // local e3b=Effect.CreateEffect(c)
    let e3b = effect::create_effect(d, c);
    // e3b:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_CONTINUOUS)
    effect::set_type(d, e3b, EFFECT_TYPE_FIELD + EFFECT_TYPE_CONTINUOUS);
    // e3b:SetCode(EVENT_CHAIN_SOLVED)
    effect::set_code(d, e3b, EVENT_CHAIN_SOLVED);
    // e3b:SetRange(LOCATION_STZONE)
    effect::set_range(d, e3b, LOCATION_STZONE);
    // e3b:SetCondition(function(e,tp,eg,ep)
    //     return ep==1-tp and e:GetHandler():HasFlagEffect(id)
    // end)
    effect::set_condition(d, e3b, Some(condition(e3b_cond)));
    // e3b:SetOperation(function(e,tp,eg,ep,ev,re,r,rp)
    //     Duel.Hint(HINT_CARD,0,id)
    //     Duel.Damage(1-tp,500,REASON_EFFECT)
    // end)
    effect::set_operation(d, e3b, Some(operation(e3b_op)));
    // c:RegisterEffect(e3b)
    card::register_effect(d, c, e3b, false);
}
fn bantg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    let e = a.e;
    // local c=e:GetHandler()
    let c = effect::get_handler(d, e).nil();
    // local colg=c:GetColumnGroup():AddCard(c)
    let mut colg = card::get_column_group(d, c, 0, 0);
    group::add_card(&mut colg, &CardSet::from_card(c));
    // if chkc then return chkc:IsLocation(LOCATION_MZONE) and not colg:IsContains(chkc) and chkc:IsAbleToRemove() end
    if let Some(chkc) = a.chkc() {
        return card::is_location(d, chkc, LOCATION_MZONE) && !group::is_contains(&colg, chkc) && card::is_able_to_remove(d, chkc, None, None, None);
    }
    // if chk==0 then return Duel.IsExistingTarget(Card.IsAbleToRemove,tp,LOCATION_MZONE,LOCATION_MZONE,1,colg) end
    if a.chk == Some(0) {
        return duel::is_existing_target(
            d,
            Some(&|d, c| card::is_able_to_remove(d, c, None, None, None)),
            tp,
            LOCATION_MZONE,
            LOCATION_MZONE,
            1,
            (&colg).into(),
        );
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_REMOVE)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_REMOVE as u64);
    // local g=Duel.SelectTarget(tp,Card.IsAbleToRemove,tp,LOCATION_MZONE,LOCATION_MZONE,1,1,colg)
    let g = duel::select_target(
        d,
        tp,
        Some(&|d, c| card::is_able_to_remove(d, c, None, None, None)),
        tp,
        LOCATION_MZONE,
        LOCATION_MZONE,
        1,
        1,
        false,
        (&colg).into(),
    );
    // Duel.SetOperationInfo(0,CATEGORY_REMOVE,g,1,tp,0)
    duel::set_operation_info(d, 0, CATEGORY_REMOVE, g, 1, tp, 0);
    true
}
fn banop(d: &mut Duel, a: &EffArgs) {
    // local tc=Duel.GetFirstTarget()
    let tc = duel::first_target(d).nil();
    // if tc:IsRelateToEffect(e) then
    if card::is_relate_to_effect(d, tc, a.e) {
        // Duel.Remove(tc,POS_FACEUP,REASON_EFFECT)
        duel::remove(d, tc, POS_FACEUP as u8, REASON_EFFECT, None, None);
    }
}
fn thcon(d: &mut Duel, a: &EffArgs) -> bool {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // return eg:IsContains(c) and not c:IsPreviousLocation(LOCATION_SZONE)
    let eg = a.eg.clone().nil();
    group::is_contains(&eg, c) && !card::is_previous_location(d, c, LOCATION_SZONE)
}
fn thfilter(d: &mut Duel, c: CardId) -> bool {
    // c:IsSetCard(SET_ANGELECHY) and c:IsSpell() and c:IsAbleToHand()
    card::is_set_card(d, c, &[SET_ANGELECHY as u16]) && card::is_spell(d, c) && card::is_able_to_hand(d, c, None)
}
fn thtg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // if chk==0 then return e:GetHandler():IsContinuousSpell()
    //     and Duel.IsExistingMatchingCard(s.thfilter,tp,LOCATION_DECK,0,1,nil) end
    if a.chk == Some(0) {
        let h = effect::get_handler(d, a.e).nil();
        return card::is_continuous_spell(d, h) && duel::is_existing_matching_card(d, Some(&thfilter), tp, LOCATION_DECK, 0, 1, NO_EX);
    }
    // Duel.SetOperationInfo(0,CATEGORY_TOHAND,nil,1,tp,LOCATION_DECK)
    duel::set_operation_info(d, 0, CATEGORY_TOHAND, None, 1, tp, LOCATION_DECK as i32);
    true
}
fn thop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_ATOHAND)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_ATOHAND as u64);
    // local g=Duel.SelectMatchingCard(tp,s.thfilter,tp,LOCATION_DECK,0,1,1,nil)
    let g = duel::select_matching_card(d, tp, Some(&thfilter), tp, LOCATION_DECK, 0, 1, 1, false, NO_EX).nil();
    // if #g>0 then
    if !g.is_empty() {
        // Duel.SendtoHand(g,nil,REASON_EFFECT)
        duel::send_to_hand(d, &g, None, REASON_EFFECT, None);
        // Duel.ConfirmCards(1-tp,g)
        duel::confirm_cards(d, 1 - tp, &g);
    }
}
fn e3a_cond(d: &mut Duel, a: &EffArgs) -> bool {
    // return ep==1-tp and e:GetHandler():IsContinuousSpell()
    let h = effect::get_handler(d, a.e).nil();
    a.ep as u8 == 1 - a.tp && card::is_continuous_spell(d, h)
}
fn e3a_op(d: &mut Duel, a: &EffArgs) {
    // e:GetHandler():RegisterFlagEffect(id,RESET_EVENT|RESETS_STANDARD|RESET_CHAIN,0,1)
    let h = effect::get_handler(d, a.e).nil();
    let _ = card::register_flag_effect(d, h, ID, RESET_EVENT | RESETS_STANDARD | RESET_CHAIN, 0, 1, 0, 0);
}
fn e3b_cond(d: &mut Duel, a: &EffArgs) -> bool {
    // return ep==1-tp and e:GetHandler():HasFlagEffect(id)
    let h = effect::get_handler(d, a.e).nil();
    a.ep as u8 == 1 - a.tp && card::has_flag_effect(d, h, ID, None)
}
fn e3b_op(d: &mut Duel, a: &EffArgs) {
    // Duel.Hint(HINT_CARD,0,id)
    duel::hint(d, HINT_CARD as u8, 0, ID as u64);
    // Duel.Damage(1-tp,500,REASON_EFFECT)
    duel::damage(d, 1 - a.tp, 500, REASON_EFFECT, false, None);
}
