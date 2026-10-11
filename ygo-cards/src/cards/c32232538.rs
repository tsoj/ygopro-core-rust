//! D/D/D Wise King Solomon
//! scripted by Naim
use super::prelude::*;

pub const ID: u32 = 32232538;

pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_DD], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // c:EnableReviveLimit()
    card::enable_revive_limit(d, c);
    // Xyz Summon procedure: 2 Level 4 "D/D" monsters
    // Xyz.AddProcedure(c,aux.FilterBoolFunctionEx(Card.IsSetCard,SET_DD),4,2)
    xyz::add_procedure(
        d,
        c,
        Some(mat_filter(|d, x, sc, st, tp| card::is_set_card_ctx(d, x, &[SET_DD as u16], sc, st, Some(tp)))),
        Some(4),
        2,
        None,
        None,
        None,
        None,
        false,
        None,
    );
    // Add 1 "D/D" card from your Deck to your hand
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_TOHAND+CATEGORY_SEARCH)
    effect::set_category(d, e1, CATEGORY_TOHAND + CATEGORY_SEARCH);
    // e1:SetType(EFFECT_TYPE_IGNITION)
    effect::set_type(d, e1, EFFECT_TYPE_IGNITION);
    // e1:SetRange(LOCATION_MZONE)
    effect::set_range(d, e1, LOCATION_MZONE);
    // e1:SetCountLimit(1,id)
    effect::set_count_limit(d, e1, 1, ID, 0, 0);
    // e1:SetCost(Cost.DetachFromSelf(1))
    effect::set_cost(d, e1, Some(cost::detach_from_self(cost::Count::N(1), None, None)));
    // e1:SetTarget(s.thtg)
    effect::set_target(d, e1, Some(cost_target(thtg)));
    // e1:SetOperation(s.thop)
    effect::set_operation(d, e1, Some(operation(thop)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // A targeted "D/D" Effect Monster gains an effect until the end of this turn
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e2, aux::stringid(ID, 1));
    // e2:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e2, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_O);
    // e2:SetProperty(EFFECT_FLAG_CARD_TARGET+EFFECT_FLAG_DELAY)
    effect::set_property(d, e2, EFFECT_FLAG_CARD_TARGET + EFFECT_FLAG_DELAY, 0);
    // e2:SetCode(EVENT_REMOVE)
    effect::set_code(d, e2, EVENT_REMOVE);
    // e2:SetCountLimit(1,{id,1})
    effect::set_count_limit(d, e2, 1, ID, 1, 0);
    // e2:SetTarget(s.efftg)
    effect::set_target(d, e2, Some(cost_target(efftg)));
    // e2:SetOperation(s.effop)
    effect::set_operation(d, e2, Some(operation(effop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
}
/// `s.thfilter(c)`
fn thfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsSetCard(SET_DD) and c:IsAbleToHand()
    card::is_set_card(d, c, &[SET_DD as u16]) && card::is_able_to_hand(d, c, None)
}
/// `s.thtg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn thtg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return Duel.IsExistingMatchingCard(s.thfilter,tp,LOCATION_DECK,0,1,nil) end
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, Some(&thfilter), a.tp, LOCATION_DECK, 0, 1, NO_EX);
    }
    // Duel.SetOperationInfo(0,CATEGORY_TOHAND,nil,1,tp,LOCATION_DECK)
    duel::set_operation_info(d, 0, CATEGORY_TOHAND, None, 1, a.tp, LOCATION_DECK as i32);
    true
}
/// `s.thop(e,tp,eg,ep,ev,re,r,rp)`
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
/// `s.tgfilter(c)`
fn tgfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsSetCard(SET_DD) and c:IsType(TYPE_EFFECT) and c:IsFaceup()
    card::is_set_card(d, c, &[SET_DD as u16]) && card::is_type(d, c, TYPE_EFFECT) && card::is_faceup(d, c)
}
/// `s.efftg(e,tp,eg,ep,ev,re,r,rp,chk,chkc)`
fn efftg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // if chkc then return chkc:IsControler(tp) and chkc:IsLocation(LOCATION_MZONE) and s.tgfilter(chkc) end
    if let Some(chkc) = a.chkc() {
        return card::is_controler(d, chkc, tp) && card::is_location(d, chkc, LOCATION_MZONE) && tgfilter(d, chkc);
    }
    // if chk==0 then return Duel.IsExistingTarget(s.tgfilter,tp,LOCATION_MZONE,0,1,nil) end
    if a.chk == Some(0) {
        return duel::is_existing_target(d, Some(&tgfilter), tp, LOCATION_MZONE, 0, 1, NO_EX);
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_APPLYTO)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_APPLYTO as u64);
    // Duel.SelectTarget(tp,s.tgfilter,tp,LOCATION_MZONE,0,1,1,nil)
    duel::select_target(d, tp, Some(&tgfilter), tp, LOCATION_MZONE, 0, 1, 1, false, NO_EX);
    true
}
/// `s.damtg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn damtg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // if chk==0 then return true end
    if a.chk == Some(0) {
        return true;
    }
    // local dam=e:GetHandler():GetBattleTarget():GetBaseAttack()
    let h = effect::get_handler(d, a.e).nil();
    let bt = card::get_battle_target(d, h).nil();
    let mut dam = card::get_base_attack(d, bt);
    // if dam<0 then dam=0 end
    if dam < 0 {
        dam = 0;
    }
    // Duel.SetTargetPlayer(1-tp)
    duel::set_target_player(d, 1 - tp);
    // Duel.SetTargetParam(dam)
    duel::set_target_param(d, dam as u32);
    // Duel.SetOperationInfo(0,CATEGORY_DAMAGE,nil,0,1-tp,dam)
    duel::set_operation_info(d, 0, CATEGORY_DAMAGE, None, 0, 1 - tp, dam);
    true
}
/// `s.damop(e,tp,eg,ep,ev,re,r,rp)`
fn damop(d: &mut Duel, _a: &EffArgs) {
    // local p,d=Duel.GetChainInfo(0,CHAININFO_TARGET_PLAYER,CHAININFO_TARGET_PARAM)
    let info = duel::get_chain_info(d, 0, &[CHAININFO_TARGET_PLAYER, CHAININFO_TARGET_PARAM]);
    let p = info[0].int() as u8;
    let dam = info[1].int();
    // Duel.Damage(p,d,REASON_EFFECT)
    duel::damage(d, p, dam, REASON_EFFECT, false, None);
}
/// `s.effop(e,tp,eg,ep,ev,re,r,rp)`
fn effop(d: &mut Duel, a: &EffArgs) {
    // local tc=Duel.GetFirstTarget()
    let tc = duel::first_target(d).nil();
    // if tc:IsRelateToEffect(e) and tc:IsFaceup() and tc:IsType(TYPE_EFFECT) then
    if card::is_relate_to_effect(d, tc, a.e) && card::is_faceup(d, tc) && card::is_type(d, tc, TYPE_EFFECT) {
        // tc:RegisterFlagEffect(id,RESETS_STANDARD_PHASE_END,EFFECT_FLAG_CLIENT_HINT,1,0,aux.Stringid(id,2))
        card::register_flag_effect(d, tc, ID, RESETS_STANDARD_PHASE_END, EFFECT_FLAG_CLIENT_HINT, 1, 0, aux::stringid(ID, 2));
        // local e1=Effect.CreateEffect(tc)
        let e1 = effect::create_effect(d, tc);
        // e1:SetDescription(aux.Stringid(id,3))
        effect::set_description(d, e1, aux::stringid(ID, 3));
        // e1:SetCategory(CATEGORY_DAMAGE)
        effect::set_category(d, e1, CATEGORY_DAMAGE);
        // e1:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_TRIGGER_F)
        effect::set_type(d, e1, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_F);
        // e1:SetProperty(EFFECT_FLAG_PLAYER_TARGET)
        effect::set_property(d, e1, EFFECT_FLAG_PLAYER_TARGET, 0);
        // e1:SetCode(EVENT_BATTLE_DESTROYING)
        effect::set_code(d, e1, EVENT_BATTLE_DESTROYING);
        // e1:SetCondition(aux.bdcon)
        effect::set_condition(d, e1, Some(aux::bdcon()));
        // e1:SetTarget(s.damtg)
        effect::set_target(d, e1, Some(cost_target(damtg)));
        // e1:SetOperation(s.damop)
        effect::set_operation(d, e1, Some(operation(damop)));
        // e1:SetReset(RESETS_STANDARD_PHASE_END)
        effect::set_reset(d, e1, RESETS_STANDARD_PHASE_END, 1);
        // tc:RegisterEffect(e1)
        card::register_effect(d, tc, e1, false);
    }
}
