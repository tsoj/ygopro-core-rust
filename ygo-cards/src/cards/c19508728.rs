//! Moon Mirror Shield
use super::prelude::*;

pub const ID: u32 = 19508728;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // aux.AddEquipProcedure(c)
    aux::add_equip_procedure(d, c, None, None, None, None, None, None, None, None);
    // atk
    // local e3=Effect.CreateEffect(c)
    let e3 = effect::create_effect(d, c);
    // e3:SetCategory(CATEGORY_ATKCHANGE+CATEGORY_DEFCHANGE)
    effect::set_category(d, e3, CATEGORY_ATKCHANGE + CATEGORY_DEFCHANGE);
    // e3:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_TRIGGER_F)
    effect::set_type(d, e3, EFFECT_TYPE_FIELD + EFFECT_TYPE_TRIGGER_F);
    // e3:SetCode(EVENT_PRE_DAMAGE_CALCULATE)
    effect::set_code(d, e3, EVENT_PRE_DAMAGE_CALCULATE);
    // e3:SetRange(LOCATION_SZONE)
    effect::set_range(d, e3, LOCATION_SZONE);
    // e3:SetCondition(s.atkcon)
    effect::set_condition(d, e3, Some(condition(atkcon)));
    // e3:SetOperation(s.atkop)
    effect::set_operation(d, e3, Some(operation(atkop)));
    // c:RegisterEffect(e3)
    card::register_effect(d, c, e3, false);
    // to deck
    // local e4=Effect.CreateEffect(c)
    let e4 = effect::create_effect(d, c);
    // e4:SetCategory(CATEGORY_TODECK)
    effect::set_category(d, e4, CATEGORY_TODECK);
    // e4:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_TRIGGER_F)
    effect::set_type(d, e4, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_F);
    // e4:SetCode(EVENT_TO_GRAVE)
    effect::set_code(d, e4, EVENT_TO_GRAVE);
    // e4:SetCondition(s.tdcon)
    effect::set_condition(d, e4, Some(condition(tdcon)));
    // e4:SetCost(Cost.PayLP(500))
    effect::set_cost(d, e4, Some(cost::pay_lp(cost::PayLp::Amount(500))));
    // e4:SetTarget(s.tdtg)
    effect::set_target(d, e4, Some(cost_target(tdtg)));
    // e4:SetOperation(s.tdop)
    effect::set_operation(d, e4, Some(operation(tdop)));
    // c:RegisterEffect(e4)
    card::register_effect(d, c, e4, false);
}
/// `s.atkcon(e,tp,eg,ep,ev,re,r,rp)`
fn atkcon(d: &mut Duel, a: &EffArgs) -> bool {
    // local ec=e:GetHandler():GetEquipTarget()
    let h = effect::get_handler(d, a.e).nil();
    let ec = card::get_equip_target(d, h);
    // local tc=ec:GetBattleTarget()
    let ec = ec.nil();
    let tc = card::get_battle_target(d, ec);
    // return ec and tc and tc:IsFaceup() and tc:IsControler(1-tp)
    match tc {
        Some(tc) => card::is_faceup(d, tc) && card::is_controler(d, tc, 1 - a.tp),
        None => false,
    }
}
/// `s.atkop(e,tp,eg,ep,ev,re,r,rp)`
fn atkop(d: &mut Duel, a: &EffArgs) {
    // if not e:GetHandler():IsRelateToEffect(e) then return end
    let h = effect::get_handler(d, a.e).nil();
    if !card::is_relate_to_effect(d, h, a.e) {
        return;
    }
    // local ec=e:GetHandler():GetEquipTarget()
    let ec = card::get_equip_target(d, h);
    // local tc=ec:GetBattleTarget()
    let ec = ec.nil();
    let tc = card::get_battle_target(d, ec);
    // if ec and tc and ec:IsFaceup() and tc:IsFaceup() then
    if let Some(tc) = tc {
        if card::is_faceup(d, ec) && card::is_faceup(d, tc) {
            // local val=math.max(tc:GetAttack(),tc:GetDefense())
            let val = card::get_attack(d, tc).max(card::get_defense(d, tc));
            // local e1=Effect.CreateEffect(e:GetHandler())
            let e1 = effect::create_effect(d, h);
            // e1:SetType(EFFECT_TYPE_SINGLE)
            effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
            // e1:SetCode(EFFECT_SET_ATTACK_FINAL)
            effect::set_code(d, e1, EFFECT_SET_ATTACK_FINAL);
            // e1:SetValue(val+100)
            effect::set_value(d, e1, (val + 100) as i64);
            // e1:SetReset(RESET_EVENT|RESETS_STANDARD|RESET_PHASE|PHASE_DAMAGE_CAL)
            effect::set_reset(d, e1, RESET_EVENT | RESETS_STANDARD | RESET_PHASE | PHASE_DAMAGE_CAL, 1);
            // ec:RegisterEffect(e1)
            card::register_effect(d, ec, e1, false);
            // local e2=e1:Clone()
            let e2 = effect::clone(d, e1);
            // e2:SetCode(EFFECT_SET_DEFENSE_FINAL)
            effect::set_code(d, e2, EFFECT_SET_DEFENSE_FINAL);
            // ec:RegisterEffect(e2)
            card::register_effect(d, ec, e2, false);
        }
    }
}
/// `s.tdcon(e,tp,eg,ep,ev,re,r,rp)`
fn tdcon(d: &mut Duel, a: &EffArgs) -> bool {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // return c:IsPreviousPosition(POS_FACEUP) and c:IsPreviousLocation(LOCATION_ONFIELD)
    card::is_previous_position(d, c, POS_FACEUP) && card::is_previous_location(d, c, LOCATION_ONFIELD)
}
/// `s.tdtg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn tdtg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return true end
    if a.chk == Some(0) {
        return true;
    }
    // local opt=Duel.SelectOption(tp,aux.Stringid(id,0),aux.Stringid(id,1))
    let opt = duel::select_option(d, a.tp, true, &[aux::stringid(ID, 0), aux::stringid(ID, 1)]);
    // e:SetLabel(opt)
    effect::set_label(d, a.e, &[opt as i64]);
    // Duel.SetOperationInfo(0,CATEGORY_TODECK,e:GetHandler(),1,0,0)
    let h = effect::get_handler(d, a.e).nil();
    duel::set_operation_info(d, 0, CATEGORY_TODECK, Some(CardSet::from_card(h)), 1, 0, 0);
    true
}
/// `s.tdop(e,tp,eg,ep,ev,re,r,rp)`
fn tdop(d: &mut Duel, a: &EffArgs) {
    // if e:GetHandler():IsRelateToEffect(e) then
    let h = effect::get_handler(d, a.e).nil();
    if card::is_relate_to_effect(d, h, a.e) {
        // Duel.SendtoDeck(e:GetHandler(),nil,e:GetLabel(),REASON_EFFECT)
        let label = effect::get_label(d, a.e);
        duel::send_to_deck(d, h, None, label as i32, REASON_EFFECT, None);
    }
}
