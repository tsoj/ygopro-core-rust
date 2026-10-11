//! Spirit of the Pot of Greed
//! 強欲な壺の精霊
use super::prelude::*;

pub const ID: u32 = 4896788;

pub const INFO: ScriptInfo = ScriptInfo { listed_names: &[55144522], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // The player that activates "Pot of Greed" can draw 1 card
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_DRAW)
    effect::set_category(d, e1, CATEGORY_DRAW);
    // e1:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_TRIGGER_F)
    effect::set_type(d, e1, EFFECT_TYPE_FIELD + EFFECT_TYPE_TRIGGER_F);
    // e1:SetCode(EVENT_CHAINING)
    effect::set_code(d, e1, EVENT_CHAINING);
    // e1:SetRange(LOCATION_MZONE)
    effect::set_range(d, e1, LOCATION_MZONE);
    // e1:SetCondition(s.drcon)
    effect::set_condition(d, e1, Some(condition(drcon)));
    // e1:SetTarget(s.drtg)
    effect::set_target(d, e1, Some(cost_target(drtg)));
    // e1:SetOperation(s.drop)
    effect::set_operation(d, e1, Some(operation(drop_op)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
/// `s.drcon(e,tp,eg,ep,ev,re,r,rp)`
fn drcon(d: &mut Duel, a: &EffArgs) -> bool {
    // return e:GetHandler():IsAttackPos() and re:IsHasType(EFFECT_TYPE_ACTIVATE) and re:GetHandler():IsCode(55144522)
    let h = effect::get_handler(d, a.e).nil();
    if !card::is_attack_pos(d, h) {
        return false;
    }
    if !effect::is_has_type(d, a.re.nil(), EFFECT_TYPE_ACTIVATE) {
        return false;
    }
    let rh = effect::get_handler(d, a.re.nil()).nil();
    card::is_code(d, rh, &[55144522])
}
/// `s.drtg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn drtg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return true end
    if a.chk == Some(0) {
        return true;
    }
    // Duel.SetTargetPlayer(rp)
    duel::set_target_player(d, a.rp);
    // Duel.SetTargetParam(1)
    duel::set_target_param(d, 1);
    // Duel.SetPossibleOperationInfo(0,CATEGORY_DRAW,nil,0,rp,1)
    duel::set_possible_operation_info(d, 0, CATEGORY_DRAW, None, 0, a.rp, 1);
    true
}
/// `s.drop(e,tp,eg,ep,ev,re,r,rp)`
fn drop_op(d: &mut Duel, a: &EffArgs) {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if not (c:IsRelateToEffect(e) and c:IsAttackPos() and c:IsFaceup()) then return end
    if !(card::is_relate_to_effect(d, c, a.e) && card::is_attack_pos(d, c) && card::is_faceup(d, c)) {
        return;
    }
    // local p,d=Duel.GetChainInfo(0,CHAININFO_TARGET_PLAYER,CHAININFO_TARGET_PARAM)
    let info = duel::get_chain_info(d, 0, &[CHAININFO_TARGET_PLAYER, CHAININFO_TARGET_PARAM]);
    let p = info[0].int() as u8;
    let dn = info[1].int() as u32;
    // if Duel.IsPlayerCanDraw(p,d) and Duel.SelectYesNo(p,aux.Stringid(id,1)) then
    if duel::is_player_can_draw(d, p, dn) && duel::select_yes_no(d, p, aux::stringid(ID, 1)) {
        // Duel.Draw(p,d,REASON_EFFECT)
        duel::draw(d, p, dn, REASON_EFFECT);
    }
}
