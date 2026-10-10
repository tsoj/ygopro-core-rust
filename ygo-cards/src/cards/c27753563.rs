//! Uradora of Fate
use super::prelude::*;

pub const ID: u32 = 27753563;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_ATKCHANGE)
    effect::set_category(d, e1, CATEGORY_ATKCHANGE);
    // e1:SetProperty(EFFECT_FLAG_CARD_TARGET)
    effect::set_property(d, e1, EFFECT_FLAG_CARD_TARGET, 0);
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    // e1:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    // e1:SetCost(Cost.PayLP(1000))
    effect::set_cost(d, e1, Some(cost::pay_lp(cost::PayLp::Amount(1000))));
    // e1:SetTarget(s.target)
    effect::set_target(d, e1, Some(cost_target(target)));
    // e1:SetOperation(s.operation)
    effect::set_operation(d, e1, Some(operation(op)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
// function s.target(e,tp,eg,ep,ev,re,r,rp,chk,chkc)
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // if chkc then return chkc:IsLocation(LOCATION_MZONE) and chkc:IsControler(tp) and chkc:IsFaceup() end
    if let Some(chkc) = a.chkc() {
        return card::is_location(d, chkc, LOCATION_MZONE) && card::is_controler(d, chkc, tp) && card::is_faceup(d, chkc);
    }
    // if chk==0 then return Duel.IsExistingTarget(Card.IsFaceup,tp,LOCATION_MZONE,0,1,nil) end
    if a.chk == Some(0) {
        return duel::is_existing_target(d, Some(&|d, c| card::is_faceup(d, c)), tp, LOCATION_MZONE, 0, 1, NO_EX);
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_FACEUP)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_FACEUP as u64);
    // Duel.SelectTarget(tp,Card.IsFaceup,tp,LOCATION_MZONE,0,1,1,nil)
    duel::select_target(d, tp, Some(&|d, c| card::is_faceup(d, c)), tp, LOCATION_MZONE, 0, 1, 1, false, NO_EX);
    true
}
// function s.operation(e,tp,eg,ep,ev,re,r,rp)
fn op(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // local tc=Duel.GetFirstTarget()
    let tc = duel::first_target(d);
    // if tc and tc:IsRelateToEffect(e) and tc:IsFaceup() then
    if let Some(tc) = tc {
        if card::is_relate_to_effect(d, tc, a.e) && card::is_faceup(d, tc) {
            // local c=e:GetHandler()
            let c = effect::get_handler(d, a.e).nil();
            // Increase ATK
            // local e1=Effect.CreateEffect(c)
            let e1 = effect::create_effect(d, c);
            // e1:SetType(EFFECT_TYPE_SINGLE)
            effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
            // e1:SetCode(EFFECT_UPDATE_ATTACK)
            effect::set_code(d, e1, EFFECT_UPDATE_ATTACK);
            // e1:SetReset(RESETS_STANDARD_PHASE_END|RESET_OPPO_TURN)
            effect::set_reset(d, e1, RESETS_STANDARD_PHASE_END | RESET_OPPO_TURN, 1);
            // e1:SetValue(1000)
            effect::set_value(d, e1, 1000);
            // tc:RegisterEffect(e1)
            card::register_effect(d, tc, e1, false);
            // Reveal the bottom card of your Deck
            // local e2=Effect.CreateEffect(c)
            let e2 = effect::create_effect(d, c);
            // e2:SetDescription(aux.Stringid(id,1))
            effect::set_description(d, e2, aux::stringid(ID, 1));
            // e2:SetCategory(CATEGORY_RECOVER+CATEGORY_DRAW)
            effect::set_category(d, e2, CATEGORY_RECOVER + CATEGORY_DRAW);
            // e2:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_TRIGGER_O)
            effect::set_type(d, e2, EFFECT_TYPE_FIELD + EFFECT_TYPE_TRIGGER_O);
            // e2:SetCode(EVENT_BATTLE_DESTROYING)
            effect::set_code(d, e2, EVENT_BATTLE_DESTROYING);
            // e2:SetLabelObject(tc)
            effect::set_label_object(d, e2, Value::Card(tc));
            // e2:SetCondition(s.drcon)
            effect::set_condition(d, e2, Some(condition(drcon)));
            // e2:SetTarget(s.drtg)
            effect::set_target(d, e2, Some(cost_target(drtg)));
            // e2:SetOperation(s.drop)
            effect::set_operation(d, e2, Some(operation(drop_op)));
            // e2:SetReset(RESET_PHASE|PHASE_END|RESET_OPPO_TURN)
            effect::set_reset(d, e2, RESET_PHASE | PHASE_END | RESET_OPPO_TURN, 1);
            // Duel.RegisterEffect(e2,tp)
            duel::register_effect(d, e2, tp);
            // tc:RegisterFlagEffect(id,RESETS_STANDARD_PHASE_END|RESET_OPPO_TURN&~(RESET_LEAVE|RESET_TODECK|RESET_TEMP_REMOVE|RESET_REMOVE|RESET_TOGRAVE),0,1)
            card::register_flag_effect(
                d,
                tc,
                ID,
                RESETS_STANDARD_PHASE_END | (RESET_OPPO_TURN & !(RESET_LEAVE | RESET_TODECK | RESET_TEMP_REMOVE | RESET_REMOVE | RESET_TOGRAVE)),
                0,
                1,
                0,
                0,
            );
        }
    }
}
// function s.drcon(e,tp,eg,ep,ev,re,r,rp)
fn drcon(d: &mut Duel, a: &EffArgs) -> bool {
    // local tc=e:GetLabelObject()
    let tc = effect::get_label_object(d, a.e).as_card().nil();
    // return tc:IsRelateToBattle() and tc:IsStatus(STATUS_OPPO_BATTLE) and tc:GetFlagEffect(id)>0
    card::is_relate_to_battle(d, tc) && card::is_status(d, tc, STATUS_OPPO_BATTLE) && card::get_flag_effect(d, tc, ID) > 0
}
// function s.drtg(e,tp,eg,ep,ev,re,r,rp,chk)
fn drtg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return Duel.GetFieldGroupCount(tp,LOCATION_DECK,0)>0 and Duel.IsPlayerCanDraw(tp) end
    if a.chk == Some(0) {
        return duel::get_field_group_count(d, a.tp, LOCATION_DECK, 0) > 0 && duel::is_player_can_draw(d, a.tp, 0);
    }
    true
}
// function s.drop(e,tp,eg,ep,ev,re,r,rp)
fn drop_op(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // local g=Duel.GetFieldGroup(tp,LOCATION_DECK,0)
    let g = duel::get_field_group(d, tp, LOCATION_DECK, 0);
    // if #g==0 then return end
    if g.is_empty() {
        return;
    }
    // local tc=g:GetMinGroup(Card.GetSequence):GetFirst()
    let mins = group::get_min_group(d, &g, &|d, c| card::get_sequence(d, c) as i64).map(|x| x.0).nil();
    let tc = group::get_first(d, &mins).nil();
    // Duel.ConfirmCards(tp,tc)
    duel::confirm_cards(d, tp, tc);
    // Duel.ConfirmCards(1-tp,tc)
    duel::confirm_cards(d, 1 - tp, tc);
    // local op=Duel.SelectOption(tp,aux.Stringid(id,2),aux.Stringid(id,3))
    let sel = duel::select_option(d, tp, true, &[aux::stringid(ID, 2), aux::stringid(ID, 3)]);
    // if op==0 then Duel.MoveSequence(tc,0) end
    if sel == 0 {
        duel::move_sequence(d, tc, 0, None);
    }
    // if not (tc:IsRace(RACE_DRAGON|RACE_DINOSAUR|RACE_SEASERPENT|RACE_WYRM) and tc:IsAttackAbove(1000)) then return end
    if !(card::is_race(d, tc, RACE_DRAGON | RACE_DINOSAUR | RACE_SEASERPENT | RACE_WYRM) && card::is_attack_above(d, tc, 1000)) {
        return;
    }
    // local ct=Duel.Draw(tp,math.floor(tc:GetAttack()/1000),REASON_EFFECT)
    let n = (card::get_attack(d, tc) as i64).div_euclid(1000);
    let ct = duel::draw(d, tp, n as u32, REASON_EFFECT);
    // if ct>0 then
    if ct > 0 {
        // Duel.BreakEffect()
        duel::break_effect(d);
        // Duel.Recover(tp,ct*1000,REASON_EFFECT)
        duel::recover(d, tp, ct as i64 * 1000, REASON_EFFECT, false, None);
    }
}
