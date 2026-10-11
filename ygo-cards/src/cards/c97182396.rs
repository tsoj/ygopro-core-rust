//! Amaze Attraction Viking Vortex
use super::prelude::*;

pub const ID: u32 = 97182396;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // From cards_specific_functions.lua
    // aux.AddAttractionEquipProc(c)
    aux::add_attraction_equip_proc(d, c);
    // You
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_TRIGGER_O+EFFECT_TYPE_FIELD)
    effect::set_type(d, e1, EFFECT_TYPE_TRIGGER_O + EFFECT_TYPE_FIELD);
    // e1:SetCode(EVENT_ATTACK_ANNOUNCE)
    effect::set_code(d, e1, EVENT_ATTACK_ANNOUNCE);
    // e1:SetRange(LOCATION_SZONE)
    effect::set_range(d, e1, LOCATION_SZONE);
    // e1:SetCategory(CATEGORY_CONTROL)
    effect::set_category(d, e1, CATEGORY_CONTROL);
    // e1:SetCountLimit(1)
    effect::set_count_limit(d, e1, 1, 0, 0, 0);
    // e1:SetCondition(aux.AttractionEquipCon(true))
    effect::set_condition(d, e1, Some(aux::attraction_equip_con(true)));
    // e1:SetTarget(s.negtg)
    effect::set_target(d, e1, Some(cost_target(negtg)));
    // e1:SetOperation(s.negop)
    effect::set_operation(d, e1, Some(operation(negop)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // Your opponent
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetCategory(CATEGORY_TOHAND)
    effect::set_category(d, e2, CATEGORY_TOHAND);
    // e2:SetType(EFFECT_TYPE_QUICK_O)
    effect::set_type(d, e2, EFFECT_TYPE_QUICK_O);
    // e2:SetCode(EVENT_CHAINING)
    effect::set_code(d, e2, EVENT_CHAINING);
    // e2:SetRange(LOCATION_SZONE)
    effect::set_range(d, e2, LOCATION_SZONE);
    // e2:SetCondition(aux.AttractionEquipCon(false))
    effect::set_condition(d, e2, Some(aux::attraction_equip_con(false)));
    // e2:SetTarget(s.thtg)
    effect::set_target(d, e2, Some(cost_target(thtg)));
    // e2:SetOperation(s.thop)
    effect::set_operation(d, e2, Some(operation(thop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
}
fn negtg(d: &mut Duel, a: &EffArgs) -> bool {
    // local ec=e:GetHandler():GetEquipTarget()
    let h = effect::get_handler(d, a.e).nil();
    let ec = card::get_equip_target(d, h);
    // local tg=Duel.GetAttacker()
    let tg = duel::get_attacker(d);
    // if chk==0 then return tp==1-Duel.GetTurnPlayer() and tg and tg:IsOnField() and ec:IsControlerCanBeChanged() end
    if a.chk == Some(0) {
        let turn = duel::get_turn_player(d);
        if a.tp != 1 - turn {
            return false;
        }
        let tg = match tg {
            Some(t) => t,
            None => return false,
        };
        if !card::is_on_field(d, tg) {
            return false;
        }
        return card::is_controler_can_be_changed(d, ec.nil(), false, None);
    }
    // Duel.SetOperationInfo(0,CATEGORY_CONTROL,ec,1,0,0)
    duel::set_operation_info(d, 0, CATEGORY_CONTROL, ec.map(CardSet::from_card), 1, 0, 0);
    true
}
fn negop(d: &mut Duel, a: &EffArgs) {
    // if not e:GetHandler():IsRelateToEffect(e) then return end
    let h = effect::get_handler(d, a.e).nil();
    if !card::is_relate_to_effect(d, h, a.e) {
        return;
    }
    // local ec=e:GetHandler():GetEquipTarget()
    let ec = card::get_equip_target(d, h);
    // if Duel.NegateAttack() and ec:IsControlerCanBeChanged() then
    if duel::negate_attack(d) && card::is_controler_can_be_changed(d, ec.nil(), false, None) {
        let ec = ec.nil();
        // Duel.GetControl(ec,1-ec:GetControler(),PHASE_BATTLE,1)
        let p = 1 - card::get_controler(d, ec);
        duel::get_control(d, ec, p, PHASE_BATTLE as u16, 1, None, None);
    }
}
fn thtg(d: &mut Duel, a: &EffArgs) -> bool {
    // local ec=e:GetHandler():GetEquipTarget()
    let h = effect::get_handler(d, a.e).nil();
    let ec = card::get_equip_target(d, h);
    // if chk==0 then return re:GetHandler()==ec and ec:IsAbleToHand() end
    if a.chk == Some(0) {
        let re = a.re.nil();
        let reh = effect::get_handler(d, re);
        if reh != ec {
            return false;
        }
        return card::is_able_to_hand(d, ec.nil(), None);
    }
    // Duel.SetOperationInfo(0,CATEGORY_TOHAND,ec,1,0,0)
    duel::set_operation_info(d, 0, CATEGORY_TOHAND, ec.map(CardSet::from_card), 1, 0, 0);
    true
}
fn thop(d: &mut Duel, a: &EffArgs) {
    // if not e:GetHandler():IsRelateToEffect(e) then return end
    let h = effect::get_handler(d, a.e).nil();
    if !card::is_relate_to_effect(d, h, a.e) {
        return;
    }
    // local ec=e:GetHandler():GetEquipTarget()
    let ec = card::get_equip_target(d, h);
    // if ec then Duel.SendtoHand(ec,nil,REASON_EFFECT) end
    if let Some(ec) = ec {
        duel::send_to_hand(d, ec, None, REASON_EFFECT, None);
    }
}
