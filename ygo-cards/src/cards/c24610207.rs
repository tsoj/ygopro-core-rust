//! Star Drawing
use super::prelude::*;

pub const ID: u32 = 24610207;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // xyzlv
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
    // e1:SetCode(EFFECT_XYZ_LEVEL)
    effect::set_code(d, e1, EFFECT_XYZ_LEVEL);
    // e1:SetProperty(EFFECT_FLAG_SINGLE_RANGE)
    effect::set_property(d, e1, EFFECT_FLAG_SINGLE_RANGE, 0);
    // e1:SetRange(LOCATION_MZONE)
    effect::set_range(d, e1, LOCATION_MZONE);
    // e1:SetValue(s.xyzlv)
    effect::set_value_fn(d, e1, effect_value(xyzlv));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // effect gain
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_CONTINUOUS)
    effect::set_type(d, e2, EFFECT_TYPE_SINGLE + EFFECT_TYPE_CONTINUOUS);
    // e2:SetCode(EVENT_BE_MATERIAL)
    effect::set_code(d, e2, EVENT_BE_MATERIAL);
    // e2:SetCondition(s.efcon)
    effect::set_condition(d, e2, Some(condition(efcon)));
    // e2:SetOperation(s.efop)
    effect::set_operation(d, e2, Some(operation(efop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
}
/// `s.xyzlv(e,c,rc)`
fn xyzlv(d: &mut Duel, e: EffectId) -> i64 {
    // return 0x50000+e:GetHandler():GetLevel()
    let h = effect::get_handler(d, e).nil();
    0x50000 + card::get_level(d, h) as i64
}
/// `s.efcon(e,tp,eg,ep,ev,re,r,rp)`
fn efcon(_d: &mut Duel, a: &EffArgs) -> bool {
    // return r==REASON_XYZ
    a.r == REASON_XYZ
}
/// `s.efop(e,tp,eg,ep,ev,re,r,rp)`
fn efop(d: &mut Duel, a: &EffArgs) {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // local rc=c:GetReasonCard()
    let rc = card::get_reason_card(d, c).nil();
    // local e1=Effect.CreateEffect(rc)
    let e1 = effect::create_effect(d, rc);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_DRAW)
    effect::set_category(d, e1, CATEGORY_DRAW);
    // e1:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_TRIGGER_F)
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_F);
    // e1:SetProperty(EFFECT_FLAG_PLAYER_TARGET)
    effect::set_property(d, e1, EFFECT_FLAG_PLAYER_TARGET, 0);
    // e1:SetCode(EVENT_SPSUMMON_SUCCESS)
    effect::set_code(d, e1, EVENT_SPSUMMON_SUCCESS);
    // e1:SetCondition(s.drcon)
    effect::set_condition(d, e1, Some(condition(drcon)));
    // e1:SetTarget(s.drtg)
    effect::set_target(d, e1, Some(cost_target(drtg)));
    // e1:SetOperation(s.drop)
    effect::set_operation(d, e1, Some(operation(drop_op)));
    // e1:SetReset(RESET_EVENT|RESETS_STANDARD)
    effect::set_reset(d, e1, RESET_EVENT | RESETS_STANDARD, 1);
    // rc:RegisterEffect(e1,true)
    card::register_effect(d, rc, e1, true);
    // if not rc:IsType(TYPE_EFFECT) then
    if !card::is_type(d, rc, TYPE_EFFECT) {
        // local e2=Effect.CreateEffect(c)
        let e2 = effect::create_effect(d, c);
        // e2:SetType(EFFECT_TYPE_SINGLE)
        effect::set_type(d, e2, EFFECT_TYPE_SINGLE);
        // e2:SetCode(EFFECT_ADD_TYPE)
        effect::set_code(d, e2, EFFECT_ADD_TYPE);
        // e2:SetValue(TYPE_EFFECT)
        effect::set_value(d, e2, TYPE_EFFECT as i64);
        // e2:SetReset(RESET_EVENT|RESETS_STANDARD)
        effect::set_reset(d, e2, RESET_EVENT | RESETS_STANDARD, 1);
        // rc:RegisterEffect(e2,true)
        card::register_effect(d, rc, e2, true);
    }
}
/// `s.drcon(e,tp,eg,ep,ev,re,r,rp)`
fn drcon(d: &mut Duel, a: &EffArgs) -> bool {
    // return e:GetHandler():IsXyzSummoned()
    let h = effect::get_handler(d, a.e).nil();
    card::is_xyz_summoned(d, h)
}
/// `s.drtg(e,tp,eg,ep,ev,re,r,rp,chk)`
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
/// `s.drop(e,tp,eg,ep,ev,re,r,rp)`
fn drop_op(d: &mut Duel, _a: &EffArgs) {
    // local p,d=Duel.GetChainInfo(0,CHAININFO_TARGET_PLAYER,CHAININFO_TARGET_PARAM)
    let info = duel::get_chain_info(d, 0, &[CHAININFO_TARGET_PLAYER, CHAININFO_TARGET_PARAM]);
    // Duel.Draw(p,d,REASON_EFFECT)
    duel::draw(d, info[0].int() as u8, info[1].int() as u32, REASON_EFFECT);
}
