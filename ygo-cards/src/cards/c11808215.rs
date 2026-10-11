//! Dice Dungeon
//! scripted by Naim
use super::prelude::*;

pub const ID: u32 = 11808215;

pub const INFO: ScriptInfo = ScriptInfo { roll_dice: true, listed_names: &[47292920], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetCategory(CATEGORY_TOHAND+CATEGORY_SEARCH)
    effect::set_category(d, e1, CATEGORY_TOHAND + CATEGORY_SEARCH);
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    // e1:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    // e1:SetOperation(s.activate)
    effect::set_operation(d, e1, Some(operation(activate)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // At the start of the Battle Phase, roll die and apply effects
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e2, aux::stringid(ID, 1));
    // e2:SetCategory(CATEGORY_DICE+CATEGORY_ATKCHANGE)
    effect::set_category(d, e2, CATEGORY_DICE + CATEGORY_ATKCHANGE);
    // e2:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e2, EFFECT_TYPE_FIELD + EFFECT_TYPE_TRIGGER_O);
    // e2:SetCode(EVENT_PHASE|PHASE_BATTLE_START)
    effect::set_code(d, e2, EVENT_PHASE | PHASE_BATTLE_START);
    // e2:SetRange(LOCATION_FZONE)
    effect::set_range(d, e2, LOCATION_FZONE);
    // e2:SetCountLimit(1)
    effect::set_count_limit(d, e2, 1, 0, 0, 0);
    // e2:SetTarget(s.dicetg)
    effect::set_target(d, e2, Some(cost_target(dicetg)));
    // e2:SetOperation(s.diceop)
    effect::set_operation(d, e2, Some(operation(diceop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
}
/// `s.thfilter(c)`: `return c:IsCode(47292920) and c:IsAbleToHand()`
fn thfilter(d: &mut Duel, c: CardId) -> bool {
    card::is_code(d, c, &[47292920]) && card::is_able_to_hand(d, c, None)
}
/// `s.activate(e,tp,eg,ep,ev,re,r,rp)`
fn activate(d: &mut Duel, a: &EffArgs) {
    let (e, tp) = (a.e, a.tp);
    // if not e:GetHandler():IsRelateToEffect(e) then return end
    let h = effect::get_handler(d, e).nil();
    if !card::is_relate_to_effect(d, h, e) {
        return;
    }
    // local g=Duel.GetMatchingGroup(s.thfilter,tp,LOCATION_DECK,0,nil)
    let g = duel::get_matching_group(d, Some(&thfilter), tp, LOCATION_DECK, 0, NO_EX);
    // if #g>0 and Duel.SelectYesNo(tp,aux.Stringid(id,0)) then
    if !g.is_empty() && duel::select_yes_no(d, tp, aux::stringid(ID, 0)) {
        // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_ATOHAND)
        duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_ATOHAND as u64);
        // local sg=g:Select(tp,1,1,nil)
        let sg = group::select(d, &g, tp, 1, 1, false, NO_EX).nil();
        // Duel.SendtoHand(sg,nil,REASON_EFFECT)
        duel::send_to_hand(d, sg.clone(), None, REASON_EFFECT, None);
        // Duel.ConfirmCards(1-tp,sg)
        duel::confirm_cards(d, 1 - tp, sg);
    }
}
/// `s.dicetg(e,tp,eg,ep,ev,re,r,rp,chk)`
/// Note: scripted as if the effect applies only to monsters the players current control
fn dicetg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // local b1=Duel.IsExistingMatchingCard(Card.IsFaceup,tp,LOCATION_MZONE,0,1,nil)
    let b1 = duel::is_existing_matching_card(d, Some(&|d, c| card::is_faceup(d, c)), tp, LOCATION_MZONE, 0, 1, NO_EX);
    // local b2=Duel.IsExistingMatchingCard(Card.IsFaceup,tp,0,LOCATION_MZONE,1,nil)
    let b2 = duel::is_existing_matching_card(d, Some(&|d, c| card::is_faceup(d, c)), tp, 0, LOCATION_MZONE, 1, NO_EX);
    // if chk==0 then return b1 or b2 end
    if a.chk == Some(0) {
        return b1 || b2;
    }
    // Duel.SetOperationInfo(0,CATEGORY_DICE,nil,1,PLAYER_ALL,0)
    duel::set_operation_info(d, 0, CATEGORY_DICE, None, 1, PLAYER_ALL as u8, 0);
    true
}
/// `s.diceop(e,tp,eg,ep,ev,re,r,rp)`
fn diceop(d: &mut Duel, a: &EffArgs) {
    let (e, tp) = (a.e, a.tp);
    // local turn_p=Duel.GetTurnPlayer()
    let turn_p = duel::get_turn_player(d);
    // local res1=Duel.TossDice(turn_p,1)
    let res1 = duel::toss_dice(d, turn_p, 1, 0)[0] as i64;
    // local res2=Duel.TossDice(1-turn_p,1)
    let res2 = duel::toss_dice(d, 1 - turn_p, 1, 0)[0] as i64;
    // local g1=Duel.GetMatchingGroup(Card.IsFaceup,tp,LOCATION_MZONE,0,nil)
    let g1 = duel::get_matching_group(d, Some(&|d, c| card::is_faceup(d, c)), tp, LOCATION_MZONE, 0, NO_EX);
    // local g2=Duel.GetMatchingGroup(Card.IsFaceup,tp,0,LOCATION_MZONE,nil)
    let g2 = duel::get_matching_group(d, Some(&|d, c| card::is_faceup(d, c)), tp, 0, LOCATION_MZONE, NO_EX);
    // if #g1==0 and #g2==0 then return end
    if g1.is_empty() && g2.is_empty() {
        return;
    }
    // local c=e:GetHandler()
    let c = effect::get_handler(d, e).nil();
    // if #g1>0 then g1:ForEach(s.atkchange,res1,c) end
    if !g1.is_empty() {
        for tc in group::iter(d, &g1) {
            atkchange(d, tc, res1, c);
        }
    }
    // if #g2>0 then g2:ForEach(s.atkchange,res2,c) end
    if !g2.is_empty() {
        for tc in group::iter(d, &g2) {
            atkchange(d, tc, res2, c);
        }
    }
}
/// `s.atkchange(tc,opt,c)`
fn atkchange(d: &mut Duel, tc: CardId, opt: i64, c: CardId) {
    // if opt==5 or opt==6 then --for double or half ATK
    if opt == 5 || opt == 6 {
        // local e1=Effect.CreateEffect(c)
        let e1 = effect::create_effect(d, c);
        // e1:SetType(EFFECT_TYPE_SINGLE)
        effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
        // e1:SetCode(EFFECT_SET_ATTACK_FINAL)
        effect::set_code(d, e1, EFFECT_SET_ATTACK_FINAL);
        if opt == 5 {
            // e1:SetValue(tc:GetAttack()/2) --5: halve the current aTK
            // Lua `/` is float division; the core rounds half away from zero
            let atk = card::get_attack(d, tc);
            effect::set_value(d, e1, lua_int(atk as f64 / 2.0));
        } else {
            // e1:SetValue(tc:GetAttack()*2) --6: double the current aTK
            let atk = card::get_attack(d, tc);
            effect::set_value(d, e1, atk as i64 * 2);
        }
        // e1:SetReset(RESETS_STANDARD_PHASE_END)
        effect::set_reset(d, e1, RESETS_STANDARD_PHASE_END, 1);
        // tc:RegisterEffect(e1)
        card::register_effect(d, tc, e1, false);
    } else {
        // else --for other atk increase/decrease
        // local e1=Effect.CreateEffect(c)
        let e1 = effect::create_effect(d, c);
        // e1:SetType(EFFECT_TYPE_SINGLE)
        effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
        // e1:SetCode(EFFECT_UPDATE_ATTACK)
        effect::set_code(d, e1, EFFECT_UPDATE_ATTACK);
        // if opt==1 then e1:SetValue(-1000) --1: lose 1000 ATK
        // elseif opt==2 then e1:SetValue(1000) --2: gain 1000 ATK
        // elseif opt==3 then e1:SetValue(-500) --3: lose 500 ATK
        // elseif opt==4 then e1:SetValue(500) --4: gain 500 ATK
        // end (other results set no value, as in the Lua)
        match opt {
            1 => effect::set_value(d, e1, -1000),
            2 => effect::set_value(d, e1, 1000),
            3 => effect::set_value(d, e1, -500),
            4 => effect::set_value(d, e1, 500),
            _ => {}
        }
        // e1:SetReset(RESETS_STANDARD_PHASE_END)
        effect::set_reset(d, e1, RESETS_STANDARD_PHASE_END, 1);
        // tc:RegisterEffect(e1)
        card::register_effect(d, tc, e1, false);
    }
}
