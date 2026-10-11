//! Pride of the Plunder Patroll
use super::prelude::*;

pub const ID: u32 = 17016131;

pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_PLUNDER_PATROLL], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    // e1:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    // e1:SetHintTiming(0,TIMING_END_PHASE)
    effect::set_hint_timing(d, e1, 0, Some(TIMING_END_PHASE));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // If your "Plunder Patroll" monster destroys a monster by battle, draw 1
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e2, aux::stringid(ID, 0));
    // e2:SetCategory(CATEGORY_DRAW)
    effect::set_category(d, e2, CATEGORY_DRAW);
    // e2:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_TRIGGER_F)
    effect::set_type(d, e2, EFFECT_TYPE_FIELD + EFFECT_TYPE_TRIGGER_F);
    // e2:SetProperty(EFFECT_FLAG_PLAYER_TARGET)
    effect::set_property(d, e2, EFFECT_FLAG_PLAYER_TARGET, 0);
    // e2:SetCode(EVENT_BATTLE_DESTROYING)
    effect::set_code(d, e2, EVENT_BATTLE_DESTROYING);
    // e2:SetCountLimit(1,id)
    effect::set_count_limit(d, e2, 1, ID, 0, 0);
    // e2:SetRange(LOCATION_SZONE)
    effect::set_range(d, e2, LOCATION_SZONE);
    // e2:SetCondition(s.drcon)
    effect::set_condition(d, e2, Some(condition(drcon)));
    // e2:SetTarget(s.drtg)
    effect::set_target(d, e2, Some(cost_target(drtg)));
    // e2:SetOperation(s.drop)
    effect::set_operation(d, e2, Some(operation(drop_op)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
    // Send this face-up card to GY to activate 1 of these effects
    // Opponent draws 1, then you send 1 monster from their hand to GY
    // local e3=Effect.CreateEffect(c)
    let e3 = effect::create_effect(d, c);
    // e3:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e3, aux::stringid(ID, 1));
    // e3:SetCategory(CATEGORY_HANDES+CATEGORY_DRAW)
    effect::set_category(d, e3, CATEGORY_HANDES + CATEGORY_DRAW);
    // e3:SetType(EFFECT_TYPE_QUICK_O)
    effect::set_type(d, e3, EFFECT_TYPE_QUICK_O);
    // e3:SetProperty(EFFECT_FLAG_PLAYER_TARGET)
    effect::set_property(d, e3, EFFECT_FLAG_PLAYER_TARGET, 0);
    // e3:SetRange(LOCATION_SZONE)
    effect::set_range(d, e3, LOCATION_SZONE);
    // e3:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e3, EVENT_FREE_CHAIN);
    // e3:SetCountLimit(1,{id,1})
    effect::set_count_limit(d, e3, 1, ID, 1, 0);
    // e3:SetCondition(s.condition)
    effect::set_condition(d, e3, Some(condition(condition_fn)));
    // e3:SetCost(s.handcost)
    effect::set_cost(d, e3, Some(cost_target(handcost)));
    // e3:SetTarget(s.handtg)
    effect::set_target(d, e3, Some(cost_target(handtg)));
    // e3:SetOperation(s.handop)
    effect::set_operation(d, e3, Some(operation(handop)));
    // e3:SetHintTiming(TIMING_STANDBY_PHASE|TIMING_END_PHASE)
    effect::set_hint_timing(d, e3, TIMING_STANDBY_PHASE | TIMING_END_PHASE, None);
    // c:RegisterEffect(e3)
    card::register_effect(d, c, e3, false);
    // Look at opponent's extra deck and send 1 monster from it to GY
    // local e4=e3:Clone()
    let e4 = effect::clone(d, e3);
    // e4:SetDescription(aux.Stringid(id,2))
    effect::set_description(d, e4, aux::stringid(ID, 2));
    // e4:SetCategory(CATEGORY_TOGRAVE)
    effect::set_category(d, e4, CATEGORY_TOGRAVE);
    // e4:SetTarget(s.extdtg)
    effect::set_target(d, e4, Some(cost_target(extdtg)));
    // e4:SetOperation(s.extdop)
    effect::set_operation(d, e4, Some(operation(extdop)));
    // c:RegisterEffect(e4)
    card::register_effect(d, c, e4, false);
}
/// `s.drcon(e,tp,eg,ep,ev,re,r,rp)`
fn drcon(d: &mut Duel, a: &EffArgs) -> bool {
    // local c=e:GetHandler()
    // if not eg then return end
    let Some(eg) = a.eg.clone() else {
        return false;
    };
    // for rc in aux.Next(eg) do
    for rc in group::iter(d, &eg) {
        // if rc:IsStatus(STATUS_OPPO_BATTLE) then
        if card::is_status(d, rc, STATUS_OPPO_BATTLE) {
            // if rc:IsRelateToBattle() then
            if card::is_relate_to_battle(d, rc) {
                // if rc:IsControler(tp) and rc:IsSetCard(SET_PLUNDER_PATROLL) then return true end
                if card::is_controler(d, rc, a.tp) && card::is_set_card(d, rc, &[SET_PLUNDER_PATROLL as u16]) {
                    return true;
                }
            } else {
                // if rc:IsPreviousControler(tp) and rc:IsPreviousSetCard(SET_PLUNDER_PATROLL) then return true end
                if card::is_previous_controler(d, rc, a.tp) && card::is_previous_set_card(d, rc, &[SET_PLUNDER_PATROLL as u16]) {
                    return true;
                }
            }
        }
    }
    // return false
    false
}
/// `s.drtg(e,tp,...,chk)`
fn drtg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return true end
    if a.chk == Some(0) {
        return true;
    }
    // Duel.SetTargetPlayer(tp)
    duel::set_target_player(d, a.tp);
    // Duel.SetTargetParam(1)
    duel::set_target_param(d, 1);
    // Duel.SetOperationInfo(0,CATEGORY_DRAW,nil,0,tp,1)
    duel::set_operation_info(d, 0, CATEGORY_DRAW, None, 0, a.tp, 1);
    true
}
/// `s.drop(e,tp,...)`
fn drop_op(d: &mut Duel, a: &EffArgs) {
    // if not e:GetHandler():IsRelateToEffect(e) then return end
    let h = effect::get_handler(d, a.e).nil();
    if !card::is_relate_to_effect(d, h, a.e) {
        return;
    }
    // local p,d=Duel.GetChainInfo(0,CHAININFO_TARGET_PLAYER,CHAININFO_TARGET_PARAM)
    let info = duel::get_chain_info(d, 0, &[CHAININFO_TARGET_PLAYER, CHAININFO_TARGET_PARAM]);
    // Duel.Draw(p,d,REASON_EFFECT)
    duel::draw(d, info[0].int() as u8, info[1].int() as u32, REASON_EFFECT);
}
/// `s.condition(e,tp,...)`
fn condition_fn(d: &mut Duel, a: &EffArgs) -> bool {
    // return e:GetHandler():IsStatus(STATUS_EFFECT_ENABLED)
    //   and Duel.IsExistingMatchingCard(aux.FaceupFilter(Card.IsSetCard,SET_PLUNDER_PATROLL),tp,LOCATION_MZONE,0,1,nil)
    let h = effect::get_handler(d, a.e).nil();
    card::is_status(d, h, STATUS_EFFECT_ENABLED)
        && duel::is_existing_matching_card(d, Some(&|d, c| card::is_faceup(d, c) && card::is_set_card(d, c, &[SET_PLUNDER_PATROLL as u16])), a.tp, LOCATION_MZONE, 0, 1, NO_EX)
}
/// `s.handcost(e,tp,...,chk)`
fn handcost(d: &mut Duel, a: &EffArgs) -> bool {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if chk==0 then return c:IsAbleToGraveAsCost() and c:IsStatus(STATUS_EFFECT_ENABLED) end
    if a.chk == Some(0) {
        return card::is_able_to_grave_as_cost(d, c) && card::is_status(d, c, STATUS_EFFECT_ENABLED);
    }
    // Duel.SendtoGrave(c,REASON_COST)
    duel::send_to_grave(d, c, REASON_COST, None, None);
    true
}
/// `s.handtg(e,tp,...,chk)`
fn handtg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return Duel.IsPlayerCanDraw(1-tp,1) end
    if a.chk == Some(0) {
        return duel::is_player_can_draw(d, 1 - a.tp, 1);
    }
    // Duel.Hint(HINT_OPSELECTED,1-tp,e:GetDescription())
    let desc = effect::get_description(d, a.e);
    duel::hint(d, HINT_OPSELECTED as u8, 1 - a.tp, desc);
    // Duel.SetTargetPlayer(1-tp)
    duel::set_target_player(d, 1 - a.tp);
    // Duel.SetTargetParam(1)
    duel::set_target_param(d, 1);
    // Duel.SetOperationInfo(0,CATEGORY_DRAW,nil,0,1-tp,1)
    duel::set_operation_info(d, 0, CATEGORY_DRAW, None, 0, 1 - a.tp, 1);
    // Duel.SetOperationInfo(0,CATEGORY_HANDES,nil,0,1-tp,1)
    duel::set_operation_info(d, 0, CATEGORY_HANDES, None, 0, 1 - a.tp, 1);
    true
}
/// `s.handop(e,tp,...)`
fn handop(d: &mut Duel, _a: &EffArgs) {
    // local p,d=Duel.GetChainInfo(0,CHAININFO_TARGET_PLAYER,CHAININFO_TARGET_PARAM)
    let info = duel::get_chain_info(d, 0, &[CHAININFO_TARGET_PLAYER, CHAININFO_TARGET_PARAM]);
    let p = info[0].int() as u8;
    // if Duel.Draw(p,d,REASON_EFFECT)>0 then
    if duel::draw(d, p, info[1].int() as u32, REASON_EFFECT) > 0 {
        // local g=Duel.GetFieldGroup(p,LOCATION_HAND,0)
        let g = duel::get_field_group(d, p, LOCATION_HAND, 0);
        // if #g>0 then
        if !g.is_empty() {
            // Duel.BreakEffect()
            duel::break_effect(d);
            // Duel.ConfirmCards(1-p,g)
            duel::confirm_cards(d, 1 - p, g.clone());
            // Duel.Hint(HINT_SELECTMSG,p,HINTMSG_DISCARD)
            duel::hint(d, HINT_SELECTMSG as u8, p, HINTMSG_DISCARD as u64);
            // local sg=g:FilterSelect(1-p,Card.IsMonster,1,1,nil)
            let sg = group::filter_select(d, &g, 1 - p, &|d, c| card::is_monster(d, c), 1, 1, false, NO_EX).nil();
            // Duel.SendtoGrave(sg,REASON_EFFECT|REASON_DISCARD)
            duel::send_to_grave(d, sg, REASON_EFFECT | REASON_DISCARD, None, None);
            // Duel.ShuffleHand(p)
            duel::shuffle_hand(d, p);
        }
    }
}
/// `s.extdtg(e,tp,...,chk)`
fn extdtg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return Duel.GetFieldGroupCount(tp,0,LOCATION_EXTRA)>0 end
    if a.chk == Some(0) {
        return duel::get_field_group_count(d, a.tp, 0, LOCATION_EXTRA) > 0;
    }
    // Duel.Hint(HINT_OPSELECTED,1-tp,e:GetDescription())
    let desc = effect::get_description(d, a.e);
    duel::hint(d, HINT_OPSELECTED as u8, 1 - a.tp, desc);
    // Duel.SetOperationInfo(0,CATEGORY_TOGRAVE,nil,0,1-tp,LOCATION_EXTRA)
    duel::set_operation_info(d, 0, CATEGORY_TOGRAVE, None, 0, 1 - a.tp, LOCATION_EXTRA as i32);
    true
}
/// `s.extdop(e,tp,...)`
fn extdop(d: &mut Duel, a: &EffArgs) {
    // local g=Duel.GetFieldGroup(tp,0,LOCATION_EXTRA)
    let g = duel::get_field_group(d, a.tp, 0, LOCATION_EXTRA);
    // if #g>0 then
    if !g.is_empty() {
        // Duel.ConfirmCards(tp,g)
        duel::confirm_cards(d, a.tp, g.clone());
        // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_TOGRAVE)
        duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_TOGRAVE as u64);
        // local sg=g:FilterSelect(tp,Card.IsMonster,1,1,nil)
        let sg = group::filter_select(d, &g, a.tp, &|d, c| card::is_monster(d, c), 1, 1, false, NO_EX).nil();
        // Duel.SendtoGrave(sg,REASON_EFFECT)
        duel::send_to_grave(d, sg, REASON_EFFECT, None, None);
        // Duel.ShuffleExtra(1-tp)
        duel::shuffle_extra(d, 1 - a.tp);
    }
}
