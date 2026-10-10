//! Smoke Grenade of the Thief
use super::prelude::*;

pub const ID: u32 = 63789924;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // aux.AddEquipProcedure(c)
    aux::add_equip_procedure(d, c, None, None, None, None, None, None, None, None);
    // Check & Discard
    // local e3=Effect.CreateEffect(c)
    let e3 = effect::create_effect(d, c);
    // e3:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e3, aux::stringid(ID, 0));
    // e3:SetCategory(CATEGORY_HANDES)
    effect::set_category(d, e3, CATEGORY_HANDES);
    // e3:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_TRIGGER_F)
    effect::set_type(d, e3, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_F);
    // e3:SetProperty(EFFECT_FLAG_PLAYER_TARGET)
    effect::set_property(d, e3, EFFECT_FLAG_PLAYER_TARGET, 0);
    // e3:SetCode(EVENT_LEAVE_FIELD)
    effect::set_code(d, e3, EVENT_LEAVE_FIELD);
    // e3:SetCondition(s.discon)
    effect::set_condition(d, e3, Some(condition(discon)));
    // e3:SetTarget(s.distg)
    effect::set_target(d, e3, Some(cost_target(distg)));
    // e3:SetOperation(s.disop)
    effect::set_operation(d, e3, Some(operation(disop)));
    // c:RegisterEffect(e3)
    card::register_effect(d, c, e3, false);
}
/// `s.discon(e,tp,eg,ep,ev,re,r,rp)`
fn discon(d: &mut Duel, a: &EffArgs) -> bool {
    // return (r&(REASON_DESTROY|REASON_EFFECT))==(REASON_DESTROY|REASON_EFFECT) and e:GetHandler():GetEquipTarget()~=nil
    //   and e:GetHandler():IsLocation(LOCATION_GRAVE)
    let h = effect::get_handler(d, a.e).nil();
    (a.r & (REASON_DESTROY | REASON_EFFECT)) == (REASON_DESTROY | REASON_EFFECT)
        && card::get_equip_target(d, h).is_some()
        && card::is_location(d, h, LOCATION_GRAVE)
}
/// `s.distg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn distg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return true end
    if a.chk == Some(0) {
        return true;
    }
    // Duel.SetTargetPlayer(tp)
    duel::set_target_player(d, a.tp);
    // Duel.SetOperationInfo(0,CATEGORY_HANDES,nil,0,1-tp,1)
    duel::set_operation_info(d, 0, CATEGORY_HANDES, None, 0, 1 - a.tp, 1);
    true
}
/// `s.disop(e,tp,eg,ep,ev,re,r,rp)`
fn disop(d: &mut Duel, _a: &EffArgs) {
    // local p=Duel.GetChainInfo(0,CHAININFO_TARGET_PLAYER)
    let p = duel::get_chain_info(d, 0, &[CHAININFO_TARGET_PLAYER])[0].int() as u8;
    // local g=Duel.GetFieldGroup(p,0,LOCATION_HAND)
    let g = duel::get_field_group(d, p, 0, LOCATION_HAND);
    // if #g>0 then
    if !g.is_empty() {
        // Duel.ConfirmCards(p,g)
        duel::confirm_cards(d, p, g.clone());
        // Duel.Hint(HINT_SELECTMSG,p,HINTMSG_DISCARD)
        duel::hint(d, HINT_SELECTMSG as u8, p, HINTMSG_DISCARD as u64);
        // local sg=g:Select(p,1,1,nil)
        let sg = group::select(d, &g, p, 1, 1, false, NO_EX).nil();
        // Duel.SendtoGrave(sg,REASON_EFFECT|REASON_DISCARD)
        duel::send_to_grave(d, sg, REASON_EFFECT | REASON_DISCARD, None, None);
        // Duel.ShuffleHand(1-p)
        duel::shuffle_hand(d, 1 - p);
    }
}
