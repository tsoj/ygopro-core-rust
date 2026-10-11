//! No.19: Freezadon
use super::prelude::*;

pub const ID: u32 = 55067058;

pub const INFO: ScriptInfo = ScriptInfo { xyz_number: Some(19), ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Xyz Summon
    // Xyz.AddProcedure(c,nil,5,2)
    xyz::add_procedure(d, c, None, Some(5), 2, None, None, None, None, false, None);
    // c:EnableReviveLimit()
    card::enable_revive_limit(d, c);
    // Detach cost replacement
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetType(EFFECT_TYPE_CONTINUOUS+EFFECT_TYPE_FIELD)
    effect::set_type(d, e1, EFFECT_TYPE_CONTINUOUS + EFFECT_TYPE_FIELD);
    // e1:SetCode(EFFECT_OVERLAY_REMOVE_REPLACE)
    effect::set_code(d, e1, EFFECT_OVERLAY_REMOVE_REPLACE);
    // e1:SetRange(LOCATION_MZONE)
    effect::set_range(d, e1, LOCATION_MZONE);
    // e1:SetCountLimit(1)
    effect::set_count_limit(d, e1, 1, 0, 0, 0);
    // e1:SetCondition(s.rcon)
    effect::set_condition(d, e1, Some(condition(rcon)));
    // e1:SetOperation(s.rop)
    effect::set_operation(d, e1, Some(condition(rop)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
// function s.rcon(e,tp,eg,ep,ev,re,r,rp)
fn rcon(d: &mut Duel, a: &EffArgs) -> bool {
    // return (r&REASON_COST)~=0 and re:IsActivated() and re:IsActiveType(TYPE_XYZ)
    //     and e:GetHandler():CheckRemoveOverlayCard(tp,1,REASON_EFFECT)
    //     and ep==e:GetOwnerPlayer() and re:GetHandler():GetOverlayCount()>=ev-1
    let re = a.re.nil();
    if (a.r & REASON_COST) == 0 || !effect::is_activated(d, re) || !effect::is_active_type(d, re, TYPE_XYZ) {
        return false;
    }
    let c = effect::get_handler(d, a.e).nil();
    if !card::check_remove_overlay_card(d, c, a.tp, 1, REASON_EFFECT) {
        return false;
    }
    let owner = effect::get_owner_player(d, a.e);
    if a.ep != owner {
        return false;
    }
    let rc = effect::get_handler(d, re).nil();
    card::get_overlay_count(d, rc) as i64 >= a.ev as i64 - 1
}
// function s.rop(e,tp,eg,ep,ev,re,r,rp)
fn rop(d: &mut Duel, a: &EffArgs) -> bool {
    // return e:GetHandler():RemoveOverlayCard(tp,1,1,REASON_EFFECT)
    // (a Lua number is truthy, so the replacement always happens)
    let c = effect::get_handler(d, a.e).nil();
    card::remove_overlay_card(d, c, a.tp, 1, 1, REASON_EFFECT);
    true
}
