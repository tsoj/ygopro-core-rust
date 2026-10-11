//! Performapal Call
use super::prelude::*;

pub const ID: u32 = 22765132;

pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_PERFORMAPAL], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetCategory(CATEGORY_TOHAND+CATEGORY_SEARCH)
    effect::set_category(d, e1, CATEGORY_TOHAND + CATEGORY_SEARCH);
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    // e1:SetCode(EVENT_ATTACK_ANNOUNCE)
    effect::set_code(d, e1, EVENT_ATTACK_ANNOUNCE);
    // e1:SetProperty(EFFECT_FLAG_CARD_TARGET)
    effect::set_property(d, e1, EFFECT_FLAG_CARD_TARGET, 0);
    // e1:SetCondition(s.condition)
    effect::set_condition(d, e1, Some(condition(condition_fn)));
    // e1:SetTarget(s.target)
    effect::set_target(d, e1, Some(cost_target(target)));
    // e1:SetOperation(s.activate)
    effect::set_operation(d, e1, Some(operation(activate)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
/// `s.condition(e,tp,eg,ep,ev,re,r,rp)`
fn condition_fn(d: &mut Duel, a: &EffArgs) -> bool {
    // return Duel.GetAttacker():IsControler(1-tp) and Duel.GetAttackTarget()==nil
    let at = duel::get_attacker(d).nil();
    card::is_controler(d, at, 1 - a.tp) && duel::get_attack_target(d).is_none()
}
/// `s.filter(c,def)`
fn filter(d: &mut Duel, c: CardId, def: i32) -> bool {
    // return c:IsSetCard(SET_PERFORMAPAL) and c:IsDefenseBelow(def) and c:IsAbleToHand()
    card::is_set_card(d, c, &[SET_PERFORMAPAL as u16]) && card::is_defense_below(d, c, def) && card::is_able_to_hand(d, c, None)
}
/// `s.target(e,tp,eg,ep,ev,re,r,rp,chk,chkc)`
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    // local at=Duel.GetAttacker()
    let at = duel::get_attacker(d);
    // if chkc then return chkc==at end
    if let Some(chkc) = a.chkc() {
        return at == Some(chkc);
    }
    // if chk==0 then return at:IsOnField() and at:IsCanBeEffectTarget(e)
    //     and Duel.IsExistingMatchingCard(s.filter,tp,LOCATION_DECK,0,1,nil,at:GetAttack()) end
    if a.chk == Some(0) {
        let at = at.nil();
        let def = card::get_attack(d, at);
        return card::is_on_field(d, at)
            && card::is_can_be_effect_target(d, at, Some(a.e))
            && duel::is_existing_matching_card(d, Some(&|d, c| filter(d, c, def)), a.tp, LOCATION_DECK, 0, 1, NO_EX);
    }
    // Duel.SetTargetCard(at)
    duel::set_target_card(d, at.nil());
    // Duel.SetOperationInfo(0,CATEGORY_TOHAND,nil,1,tp,LOCATION_DECK)
    duel::set_operation_info(d, 0, CATEGORY_TOHAND, None, 1, a.tp, LOCATION_DECK as i32);
    true
}
/// `s.activate(e,tp,eg,ep,ev,re,r,rp)`
fn activate(d: &mut Duel, a: &EffArgs) {
    // local tc=Duel.GetFirstTarget()
    let tc = duel::first_target(d);
    // if Duel.NegateAttack() then
    if duel::negate_attack(d) {
        // local val=tc:GetAttack()
        let mut val = card::get_attack(d, tc.nil());
        // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_ATOHAND)
        duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_ATOHAND as u64);
        // local g1=Duel.SelectMatchingCard(tp,s.filter,tp,LOCATION_DECK,0,1,1,nil,val)
        let mut g1 = duel::select_matching_card(d, a.tp, Some(&|d, c| filter(d, c, val)), a.tp, LOCATION_DECK, 0, 1, 1, false, NO_EX).nil();
        // local sc=g1:GetFirst()
        let sc = group::get_first(d, &g1);
        // if sc then
        if let Some(sc) = sc {
            // val=val-sc:GetDefense()
            val -= card::get_defense(d, sc);
            // if Duel.IsExistingMatchingCard(s.filter,tp,LOCATION_DECK,0,1,sc,val)
            //     and Duel.SelectYesNo(tp,aux.Stringid(id,0)) then
            if duel::is_existing_matching_card(d, Some(&|d, c| filter(d, c, val)), a.tp, LOCATION_DECK, 0, 1, sc.into())
                && duel::select_yes_no(d, a.tp, aux::stringid(ID, 0))
            {
                // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_ATOHAND)
                duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_ATOHAND as u64);
                // local g2=Duel.SelectMatchingCard(tp,s.filter,tp,LOCATION_DECK,0,1,1,sc,val)
                let g2 = duel::select_matching_card(d, a.tp, Some(&|d, c| filter(d, c, val)), a.tp, LOCATION_DECK, 0, 1, 1, false, sc.into()).nil();
                // g1:Merge(g2)
                group::add_card(&mut g1, &g2);
            }
            // Duel.SendtoHand(g1,nil,REASON_EFFECT)
            duel::send_to_hand(d, g1.clone(), None, REASON_EFFECT, None);
            // Duel.ConfirmCards(1-tp,g1)
            duel::confirm_cards(d, 1 - a.tp, g1);
        }
    }
    // if e:IsHasType(EFFECT_TYPE_ACTIVATE) then
    if effect::is_has_type(d, a.e, EFFECT_TYPE_ACTIVATE) {
        let h = effect::get_handler(d, a.e).nil();
        // local e1=Effect.CreateEffect(e:GetHandler())
        let e1 = effect::create_effect(d, h);
        // e1:SetType(EFFECT_TYPE_FIELD)
        effect::set_type(d, e1, EFFECT_TYPE_FIELD);
        // e1:SetProperty(EFFECT_FLAG_PLAYER_TARGET)
        effect::set_property(d, e1, EFFECT_FLAG_PLAYER_TARGET, 0);
        // e1:SetCode(EFFECT_CANNOT_SPECIAL_SUMMON)
        effect::set_code(d, e1, EFFECT_CANNOT_SPECIAL_SUMMON);
        // e1:SetTargetRange(1,0)
        effect::set_target_range(d, e1, 1, 0);
        // e1:SetTarget(s.sumlimit)
        effect::set_target(d, e1, Some(effect_card_fn(|d, _e, c| card::is_location(d, c, LOCATION_EXTRA))));
        // if Duel.IsTurnPlayer(tp) then
        if duel::is_turn_player(d, a.tp) {
            // e1:SetReset(RESET_PHASE|PHASE_END|RESET_SELF_TURN,2)
            effect::set_reset(d, e1, RESET_PHASE | PHASE_END | RESET_SELF_TURN, 2);
        } else {
            // e1:SetReset(RESET_PHASE|PHASE_END|RESET_SELF_TURN)
            effect::set_reset(d, e1, RESET_PHASE | PHASE_END | RESET_SELF_TURN, 1);
        }
        // Duel.RegisterEffect(e1,tp)
        duel::register_effect(d, e1, a.tp);
        // aux.addTempLizardCheck(e:GetHandler(),tp,aux.TRUE,RESET_PHASE|PHASE_END|RESET_SELF_TURN,0xff,0,Duel.IsTurnPlayer(tp) and 2 or 1)
        let count: u16 = if duel::is_turn_player(d, a.tp) { 2 } else { 1 };
        aux::add_temp_lizard_check(d, h, a.tp, Some(aux_true()), Some(RESET_PHASE | PHASE_END | RESET_SELF_TURN), Some(0xff), Some(0), Some(count));
    }
}
