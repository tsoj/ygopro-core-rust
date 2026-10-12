//! The Sun God Leading Down into Darkness
//! scripted by Naim
use super::prelude::*;

pub const ID: u32 = 101403052;

pub const INFO: ScriptInfo = ScriptInfo { listed_names: &[CARD_RA], listed_series: &[SET_THE_WINGED_DRAGON_OF_RA, SET_SUN_GOD], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate this card by sending 1 "The Winged Dragon of Ra" monster from your hand, Deck, or face-up field to the GY
    // local e0=Effect.CreateEffect(c)
    let e0 = effect::create_effect(d, c);
    // e0:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e0, EFFECT_TYPE_ACTIVATE);
    // e0:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e0, EVENT_FREE_CHAIN);
    // e0:SetCost(s.cost)
    effect::set_cost(d, e0, Some(cost_target(cost_fn)));
    // c:RegisterEffect(e0)
    card::register_effect(d, c, e0, false);
    // During your Main Phase, if this card was activated this turn: You can add 1 "Sun God" monster from your Deck to your hand
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_TOHAND+CATEGORY_SEARCH)
    effect::set_category(d, e1, CATEGORY_TOHAND + CATEGORY_SEARCH);
    // e1:SetType(EFFECT_TYPE_IGNITION)
    effect::set_type(d, e1, EFFECT_TYPE_IGNITION);
    // e1:SetRange(LOCATION_SZONE)
    effect::set_range(d, e1, LOCATION_SZONE);
    // e1:SetCountLimit(1,{id,0})
    effect::set_count_limit(d, e1, 1, ID, 0, 0);
    // e1:SetCondition(function(e,tp,eg,ep,ev,re,r,rp) return e:GetHandler():HasFlagEffect(id) end)
    effect::set_condition(
        d,
        e1,
        Some(condition(|d, a| {
            let h = effect::get_handler(d, a.e).nil();
            card::has_flag_effect(d, h, ID, None)
        })),
    );
    // e1:SetTarget(s.thtg)
    effect::set_target(d, e1, Some(cost_target(thtg)));
    // e1:SetOperation(s.thop)
    effect::set_operation(d, e1, Some(operation(thop)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // If a monster(s) is destroyed by battle: Its controller takes damage equal to its original ATK, then if you have "The Winged Dragon of Ra" in your GY, you gain 1000 LP
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e2, aux::stringid(ID, 1));
    // e2:SetCategory(CATEGORY_DAMAGE+CATEGORY_RECOVER)
    effect::set_category(d, e2, CATEGORY_DAMAGE + CATEGORY_RECOVER);
    // e2:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_TRIGGER_F)
    effect::set_type(d, e2, EFFECT_TYPE_FIELD + EFFECT_TYPE_TRIGGER_F);
    // e2:SetCode(EVENT_BATTLE_DESTROYED)
    effect::set_code(d, e2, EVENT_BATTLE_DESTROYED);
    // e2:SetRange(LOCATION_SZONE)
    effect::set_range(d, e2, LOCATION_SZONE);
    // e2:SetCountLimit(1,{id,1})
    effect::set_count_limit(d, e2, 1, ID, 1, 0);
    // e2:SetTarget(s.damtg)
    effect::set_target(d, e2, Some(cost_target(damtg)));
    // e2:SetOperation(s.damop)
    effect::set_operation(d, e2, Some(operation(damop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
}
// function s.costfilter(c)
fn costfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsSetCard(SET_THE_WINGED_DRAGON_OF_RA) and c:IsMonster() and (c:IsFaceup() or not c:IsOnField()) and c:IsAbleToGraveAsCost()
    card::is_set_card(d, c, &[SET_THE_WINGED_DRAGON_OF_RA as u16])
        && card::is_monster(d, c)
        && (card::is_faceup(d, c) || !card::is_on_field(d, c))
        && card::is_able_to_grave_as_cost(d, c)
}
// function s.cost(e,tp,eg,ep,ev,re,r,rp,chk)
fn cost_fn(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return Duel.IsExistingMatchingCard(s.costfilter,tp,LOCATION_HAND|LOCATION_DECK|LOCATION_MZONE,0,1,nil) end
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, Some(&costfilter), a.tp, LOCATION_HAND | LOCATION_DECK | LOCATION_MZONE, 0, 1, NO_EX);
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_TOGRAVE)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_TOGRAVE as u64);
    // local g=Duel.SelectMatchingCard(tp,s.costfilter,tp,LOCATION_HAND|LOCATION_DECK|LOCATION_MZONE,0,1,1,nil)
    let g = duel::select_matching_card(d, a.tp, Some(&costfilter), a.tp, LOCATION_HAND | LOCATION_DECK | LOCATION_MZONE, 0, 1, 1, false, NO_EX).nil();
    // Duel.SendtoGrave(g,REASON_COST)
    duel::send_to_grave(d, g, REASON_COST, None, None);
    // e:GetHandler():RegisterFlagEffect(id,RESETS_STANDARD_PHASE_END,EFFECT_FLAG_OATH,1)
    let h = effect::get_handler(d, a.e).nil();
    card::register_flag_effect(d, h, ID, RESETS_STANDARD_PHASE_END, EFFECT_FLAG_OATH, 1, 0, 0);
    true
}
// function s.thfilter(c)
fn thfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsSetCard(SET_SUN_GOD) and c:IsMonster() and c:IsAbleToHand()
    card::is_set_card(d, c, &[SET_SUN_GOD as u16]) && card::is_monster(d, c) && card::is_able_to_hand(d, c, None)
}
// function s.thtg(e,tp,eg,ep,ev,re,r,rp,chk)
fn thtg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return Duel.IsExistingMatchingCard(s.thfilter,tp,LOCATION_DECK,0,1,nil) end
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, Some(&thfilter), a.tp, LOCATION_DECK, 0, 1, NO_EX);
    }
    // Duel.SetOperationInfo(0,CATEGORY_TOHAND,nil,1,tp,LOCATION_DECK)
    duel::set_operation_info(d, 0, CATEGORY_TOHAND, None, 1, a.tp, LOCATION_DECK as i32);
    true
}
// function s.thop(e,tp,eg,ep,ev,re,r,rp)
fn thop(d: &mut Duel, a: &EffArgs) {
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_ATOHAND)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_ATOHAND as u64);
    // local g=Duel.SelectMatchingCard(tp,s.thfilter,tp,LOCATION_DECK,0,1,1,nil)
    let g = duel::select_matching_card(d, a.tp, Some(&thfilter), a.tp, LOCATION_DECK, 0, 1, 1, false, NO_EX).nil();
    // if #g>0 then
    if !g.is_empty() {
        // Duel.SendtoHand(g,nil,REASON_EFFECT)
        duel::send_to_hand(d, g.clone(), None, REASON_EFFECT, None);
        // Duel.ConfirmCards(1-tp,g)
        duel::confirm_cards(d, 1 - a.tp, g);
    }
}
// function s.damtg(e,tp,eg,ep,ev,re,r,rp,chk)
fn damtg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return true end
    if a.chk == Some(0) {
        return true;
    }
    // local dg_self,dg_opp=eg:Split(Card.IsPreviousControler,nil,tp)
    let eg = a.eg.clone().nil();
    let (dg_self, dg_opp) = group::split(d, &eg, &|d, c| card::is_previous_controler(d, c, a.tp), NO_EX);
    // local your_total_atk,opp_total_atk,players=0,0,0
    // Split always returns two groups (possibly empty), so the `if dg_self` / `if dg_opp` tests
    // below are always true, and both players end up in `players`.
    // if dg_self then your_total_atk=dg_self:GetSum(Card.GetBaseAttack) players=players|tp end
    let your_total_atk = group::get_sum(d, &dg_self, &|d, c| card::get_base_attack(d, c) as i64);
    // if dg_opp then opp_total_atk=dg_opp:GetSum(Card.GetBaseAttack) players=players|(1-tp) end
    let opp_total_atk = group::get_sum(d, &dg_opp, &|d, c| card::get_base_attack(d, c) as i64);
    // if dg_opp and dg_self then players=PLAYER_ALL end
    let players = PLAYER_ALL;
    // local cd=e:GetChainData()
    // cd.your_damage=your_total_atk
    chain::set_effect_data(d, a.e, None, chain::field::CARD, Value::Int(your_total_atk));
    // cd.oppo_damage=opp_total_atk
    chain::set_effect_data(d, a.e, None, chain::field::CARD + 1, Value::Int(opp_total_atk));
    // Duel.SetOperationInfo(0,CATEGORY_DAMAGE,nil,1,players,your_total_atk+opp_total_atk)
    duel::set_operation_info(d, 0, CATEGORY_DAMAGE, None, 1, players as u8, (your_total_atk + opp_total_atk) as i32);
    // if Duel.IsExistingMatchingCard(Card.IsCode,tp,LOCATION_GRAVE,0,1,nil,CARD_RA) then
    if duel::is_existing_matching_card(d, Some(&|d, c| card::is_code(d, c, &[CARD_RA])), a.tp, LOCATION_GRAVE, 0, 1, NO_EX) {
        // Duel.SetOperationInfo(0,CATEGORY_RECOVER,nil,1,tp,1000)
        duel::set_operation_info(d, 0, CATEGORY_RECOVER, None, 1, a.tp, 1000);
    }
    // Duel.SetPossibleOperationInfo(0,CATEGORY_RECOVER,nil,1,tp,1000)
    duel::set_possible_operation_info(d, 0, CATEGORY_RECOVER, None, 1, a.tp, 1000);
    true
}
// function s.damop(e,tp,eg,ep,ev,re,r,rp)
fn damop(d: &mut Duel, a: &EffArgs) {
    // local cd=e:GetChainData()
    // local your_damage=cd.your_damage
    let your_damage = chain::effect_data(d, a.e, None, chain::field::CARD).int();
    // local oppo_damage=cd.oppo_damage
    let oppo_damage = chain::effect_data(d, a.e, None, chain::field::CARD + 1).int();
    // local total_damage=0
    let mut total_damage: i64 = 0;
    // if your_damage>0 then
    if your_damage > 0 {
        // total_damage=Duel.Damage(tp,your_damage,REASON_EFFECT,true)
        total_damage = duel::damage(d, a.tp, your_damage, REASON_EFFECT, true, None) as i64;
    }
    // if oppo_damage>0 then
    if oppo_damage > 0 {
        // total_damage=total_damage+Duel.Damage(1-tp,oppo_damage,REASON_EFFECT,true)
        total_damage += duel::damage(d, 1 - a.tp, oppo_damage, REASON_EFFECT, true, None) as i64;
    }
    // Duel.RDComplete()
    duel::rd_complete(d);
    // if total_damage>0 and Duel.IsExistingMatchingCard(Card.IsCode,tp,LOCATION_GRAVE,0,1,nil,CARD_RA) then
    if total_damage > 0 && duel::is_existing_matching_card(d, Some(&|d, c| card::is_code(d, c, &[CARD_RA])), a.tp, LOCATION_GRAVE, 0, 1, NO_EX) {
        // Duel.BreakEffect()
        duel::break_effect(d);
        // Duel.Recover(tp,1000,REASON_EFFECT)
        duel::recover(d, a.tp, 1000, REASON_EFFECT, false, None);
    }
}
