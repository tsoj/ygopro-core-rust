//! Necklace of Command
use super::prelude::*;

pub const ID: u32 = 48576971;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // aux.AddEquipProcedure(c)
    aux::add_equip_procedure(d, c, None, None, None, None, None, None, None, None);
    // effect
    // local e3=Effect.CreateEffect(c)
    let e3 = effect::create_effect(d, c);
    // e3:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e3, aux::stringid(ID, 0));
    // e3:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_TRIGGER_F)
    effect::set_type(d, e3, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_F);
    // e3:SetCode(EVENT_TO_GRAVE)
    effect::set_code(d, e3, EVENT_TO_GRAVE);
    // e3:SetCondition(s.effcon)
    effect::set_condition(d, e3, Some(condition(effcon)));
    // e3:SetTarget(s.efftg)
    effect::set_target(d, e3, Some(cost_target(efftg)));
    // e3:SetOperation(s.effop)
    effect::set_operation(d, e3, Some(operation(effop)));
    // c:RegisterEffect(e3)
    card::register_effect(d, c, e3, false);
}
/// `s.effcon(e,tp,eg,ep,ev,re,r,rp)`
fn effcon(d: &mut Duel, a: &EffArgs) -> bool {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // local ec=c:GetPreviousEquipTarget()
    let ec = card::get_previous_equip_target(d, c);
    // return c:IsReason(REASON_LOST_TARGET) and ec:IsReason(REASON_BATTLE) and ec:IsPreviousControler(tp)
    card::is_reason(d, c, REASON_LOST_TARGET) && card::is_reason(d, ec.nil(), REASON_BATTLE) && card::is_previous_controler(d, ec.nil(), a.tp)
}
/// `s.efftg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn efftg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // if chk==0 then return true end
    if a.chk == Some(0) {
        return true;
    }
    // local b1=Duel.IsPlayerCanDraw(tp,1)
    let b1 = duel::is_player_can_draw(d, tp, 1);
    // local b2=Duel.GetFieldGroupCount(tp,0,LOCATION_HAND)>0
    let b2 = duel::get_field_group_count(d, tp, 0, LOCATION_HAND) > 0;
    // local opt=0
    // if b1 and b2 then opt=Duel.SelectOption(tp,aux.Stringid(id,1),aux.Stringid(id,2))
    let opt: i32 = if b1 && b2 {
        duel::select_option(d, tp, true, &[aux::stringid(ID, 1), aux::stringid(ID, 2)])
    }
    // elseif b1 then opt=Duel.SelectOption(tp,aux.Stringid(id,1))
    else if b1 {
        duel::select_option(d, tp, true, &[aux::stringid(ID, 1)])
    }
    // elseif b2 then opt=Duel.SelectOption(tp,aux.Stringid(id,2))+1
    else if b2 {
        duel::select_option(d, tp, true, &[aux::stringid(ID, 2)]) + 1
    }
    // else opt=2 end
    else {
        2
    };
    // e:SetLabel(opt)
    effect::set_label(d, a.e, &[opt as i64]);
    // if opt==0 then
    if opt == 0 {
        // e:SetCategory(CATEGORY_DRAW)
        effect::set_category(d, a.e, CATEGORY_DRAW);
        // e:SetProperty(EFFECT_FLAG_PLAYER_TARGET)
        effect::set_property(d, a.e, EFFECT_FLAG_PLAYER_TARGET, 0);
        // Duel.SetTargetPlayer(tp)
        duel::set_target_player(d, tp);
        // Duel.SetTargetParam(1)
        duel::set_target_param(d, 1);
        // Duel.SetOperationInfo(0,CATEGORY_DRAW,nil,0,tp,1)
        duel::set_operation_info(d, 0, CATEGORY_DRAW, None, 0, tp, 1);
    }
    // elseif opt==1 then
    else if opt == 1 {
        // e:SetCategory(CATEGORY_HANDES)
        effect::set_category(d, a.e, CATEGORY_HANDES);
        // e:SetProperty(EFFECT_FLAG_PLAYER_TARGET)
        effect::set_property(d, a.e, EFFECT_FLAG_PLAYER_TARGET, 0);
        // Duel.SetTargetPlayer(1-tp)
        duel::set_target_player(d, 1 - tp);
        // Duel.SetTargetParam(1)
        duel::set_target_param(d, 1);
        // Duel.SetOperationInfo(0,CATEGORY_HANDES,nil,0,1-tp,1)
        duel::set_operation_info(d, 0, CATEGORY_HANDES, None, 0, 1 - tp, 1);
    }
    // else
    else {
        // e:SetCategory(0)
        effect::set_category(d, a.e, 0);
        // e:SetProperty(0)
        effect::set_property(d, a.e, 0, 0);
    }
    true
}
/// `s.effop(e,tp,eg,ep,ev,re,r,rp)`
fn effop(d: &mut Duel, a: &EffArgs) {
    // if e:GetLabel()==0 then
    if effect::get_label(d, a.e) == 0 {
        // local p,d=Duel.GetChainInfo(0,CHAININFO_TARGET_PLAYER,CHAININFO_TARGET_PARAM)
        let info = duel::get_chain_info(d, 0, &[CHAININFO_TARGET_PLAYER, CHAININFO_TARGET_PARAM]);
        // Duel.Draw(p,d,REASON_EFFECT)
        duel::draw(d, info[0].int() as u8, info[1].int() as u32, REASON_EFFECT);
    }
    // elseif e:GetLabel()==1 then
    else if effect::get_label(d, a.e) == 1 {
        // local p,d=Duel.GetChainInfo(0,CHAININFO_TARGET_PLAYER,CHAININFO_TARGET_PARAM)
        let info = duel::get_chain_info(d, 0, &[CHAININFO_TARGET_PLAYER, CHAININFO_TARGET_PARAM]);
        let p = info[0].int() as u8;
        let dn = info[1].int() as usize;
        // local g=Duel.GetFieldGroup(p,LOCATION_HAND,0):RandomSelect(p,d)
        let hand = duel::get_field_group(d, p, LOCATION_HAND, 0);
        let g = group::random_select(d, &hand, p, dn);
        // Duel.SendtoGrave(g,REASON_EFFECT|REASON_DISCARD)
        duel::send_to_grave(d, g, REASON_EFFECT | REASON_DISCARD, None, None);
    }
}
