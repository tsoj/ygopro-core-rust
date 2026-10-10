//! Giant Kozaky
use super::prelude::*;

pub const ID: u32 = 58185394;
pub const INFO: ScriptInfo = ScriptInfo { listed_names: &[99171160], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // self destroy
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
    // e1:SetProperty(EFFECT_FLAG_SINGLE_RANGE)
    effect::set_property(d, e1, EFFECT_FLAG_SINGLE_RANGE, 0);
    // e1:SetRange(LOCATION_MZONE)
    effect::set_range(d, e1, LOCATION_MZONE);
    // e1:SetCode(EFFECT_SELF_DESTROY)
    effect::set_code(d, e1, EFFECT_SELF_DESTROY);
    // e1:SetCondition(s.sdcon)
    effect::set_condition(d, e1, Some(lua_fn(sdcon)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // damage
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e2, aux::stringid(ID, 0));
    // e2:SetCategory(CATEGORY_DAMAGE)
    effect::set_category(d, e2, CATEGORY_DAMAGE);
    // e2:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_TRIGGER_F)
    effect::set_type(d, e2, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_F);
    // e2:SetCode(EVENT_LEAVE_FIELD)
    effect::set_code(d, e2, EVENT_LEAVE_FIELD);
    // e2:SetCondition(s.dmcon)
    effect::set_condition(d, e2, Some(condition(dmcon)));
    // e2:SetTarget(s.dmtg)
    effect::set_target(d, e2, Some(cost_target(dmtg)));
    // e2:SetOperation(s.dmop)
    effect::set_operation(d, e2, Some(operation(dmop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
}
/// `s.sdfilter(c)`
fn sdfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsFaceup() and c:IsCode(99171160)
    card::is_faceup(d, c) && card::is_code(d, c, &[99171160])
}
/// `s.sdcon(e)`
fn sdcon(d: &mut Duel, a: Args) -> bool {
    let e = a.e(0);
    // return not e:GetHandler():IsStatus(STATUS_BATTLE_DESTROYED)
    let h = effect::get_handler(d, e).nil();
    // and not Duel.IsExistingMatchingCard(s.sdfilter,0,LOCATION_ONFIELD,LOCATION_ONFIELD,1,nil)
    !card::is_status(d, h, STATUS_BATTLE_DESTROYED) && !duel::is_existing_matching_card(d, Some(&sdfilter), 0, LOCATION_ONFIELD, LOCATION_ONFIELD, 1, NO_EX)
}
/// `s.dmcon(e,tp,...)`
fn dmcon(d: &mut Duel, a: &EffArgs) -> bool {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // return c:IsReason(REASON_DESTROY) and c:IsPreviousPosition(POS_FACEUP)
    card::is_reason(d, c, REASON_DESTROY) && card::is_previous_position(d, c, POS_FACEUP as u32)
}
/// `s.dmtg(e,tp,...,chk)`
fn dmtg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return true end
    if a.chk == Some(0) {
        return true;
    }
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // Duel.SetTargetPlayer(c:GetPreviousControler())
    duel::set_target_player(d, card::get_previous_controler(d, c));
    // Duel.SetTargetParam(2500)
    duel::set_target_param(d, 2500);
    // Duel.SetOperationInfo(0,CATEGORY_DAMAGE,nil,0,c:GetPreviousControler(),2500)
    let prev = card::get_previous_controler(d, c);
    duel::set_operation_info(d, 0, CATEGORY_DAMAGE, None, 0, prev, 2500);
    true
}
/// `s.dmop(e,tp,...)`
fn dmop(d: &mut Duel, _a: &EffArgs) {
    // local p,d=Duel.GetChainInfo(0,CHAININFO_TARGET_PLAYER,CHAININFO_TARGET_PARAM)
    let info = duel::get_chain_info(d, 0, &[CHAININFO_TARGET_PLAYER, CHAININFO_TARGET_PARAM]);
    // Duel.Damage(p,d,REASON_EFFECT)
    duel::damage(d, info[0].int() as u8, info[1].int(), REASON_EFFECT, false, None);
}
