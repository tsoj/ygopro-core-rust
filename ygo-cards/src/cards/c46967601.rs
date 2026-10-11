//! Cursed Bill
use super::prelude::*;

pub const ID: u32 = 46967601;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // aux.AddEquipProcedure(c)
    aux::add_equip_procedure(d, c, None, None, None, None, None, None, None, None);
    // --damage
    // local e3=Effect.CreateEffect(c)
    let e3 = effect::create_effect(d, c);
    // e3:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e3, aux::stringid(ID, 0));
    // e3:SetCategory(CATEGORY_DAMAGE)
    effect::set_category(d, e3, CATEGORY_DAMAGE);
    // e3:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_TRIGGER_F)
    effect::set_type(d, e3, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_F);
    // e3:SetCode(EVENT_TO_GRAVE)
    effect::set_code(d, e3, EVENT_TO_GRAVE);
    // e3:SetCondition(s.damcon)
    effect::set_condition(d, e3, Some(condition(damcon)));
    // e3:SetTarget(s.damtg)
    effect::set_target(d, e3, Some(cost_target(damtg)));
    // e3:SetOperation(s.damop)
    effect::set_operation(d, e3, Some(operation(damop)));
    // c:RegisterEffect(e3)
    card::register_effect(d, c, e3, false);
}
/// `s.damcon(e,tp,eg,ep,ev,re,r,rp)`
fn damcon(d: &mut Duel, a: &EffArgs) -> bool {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // local ec=c:GetPreviousEquipTarget()
    let ec = card::get_previous_equip_target(d, c);
    // if not ec then return end
    let Some(ec) = ec else {
        return false;
    };
    // e:SetLabelObject(ec)
    effect::set_label_object(d, a.e, Value::Card(ec));
    // e:SetLabel(ec:GetPreviousControler())
    let prev_controler = card::get_previous_controler(d, ec);
    effect::set_label(d, a.e, &[prev_controler as i64]);
    // return c:IsReason(REASON_LOST_TARGET) and ec:IsLocation(LOCATION_GRAVE) and ec:IsReason(REASON_DESTROY)
    card::is_reason(d, c, REASON_LOST_TARGET) && card::is_location(d, ec, LOCATION_GRAVE) && card::is_reason(d, ec, REASON_DESTROY)
}
/// `s.damtg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn damtg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return true end
    if a.chk == Some(0) {
        return true;
    }
    // local dam=e:GetLabelObject():GetTextDefense()
    let tc = effect::get_label_object(d, a.e).as_card().nil();
    let mut dam = card::get_text_defense(d, tc);
    // if dam<0 then dam=0 end
    if dam < 0 {
        dam = 0;
    }
    let label = effect::get_label(d, a.e);
    // Duel.SetTargetPlayer(e:GetLabel())
    duel::set_target_player(d, label as u8);
    // Duel.SetTargetParam(dam)
    duel::set_target_param(d, dam as u32);
    // Duel.SetTargetCard(e:GetLabelObject())
    duel::set_target_card(d, tc);
    // Duel.SetOperationInfo(0,CATEGORY_DAMAGE,nil,0,e:GetLabel(),dam)
    duel::set_operation_info(d, 0, CATEGORY_DAMAGE, None, 0, label as u8, dam);
    true
}
/// `s.damop(e,tp,eg,ep,ev,re,r,rp)`
fn damop(d: &mut Duel, a: &EffArgs) {
    // local g=Duel.GetTargetCards(e)
    let g = duel::get_target_cards(d, a.e).nil();
    // if #g>0 then
    if !g.is_empty() {
        // local p=Duel.GetChainInfo(0,CHAININFO_TARGET_PLAYER)
        let p = duel::get_chain_info(d, 0, &[CHAININFO_TARGET_PLAYER])[0].int() as u8;
        // Duel.Damage(p,g:GetFirst():GetTextDefense(),REASON_EFFECT)
        let tc = group::get_first(d, &g).nil();
        let amount = card::get_text_defense(d, tc);
        duel::damage(d, p, amount as i64, REASON_EFFECT, false, None);
    }
}
