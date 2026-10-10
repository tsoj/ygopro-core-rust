//! Crusadia Testament
//! Scripted by AlphaKretin
use super::prelude::*;

pub const ID: u32 = 87497553;

pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_CRUSADIA], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // cannot activate
    let e1 = effect::create_effect(d, c);
    effect::set_description(d, e1, aux::stringid(ID, 0));
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    effect::set_count_limit(d, e1, 1, ID, 0, EFFECT_COUNT_CODE_OATH as u8);
    effect::set_operation(d, e1, Some(operation(acop)));
    card::register_effect(d, c, e1, false);
    // local e2=e1:Clone()
    let e2 = effect::clone(d, e1);
    effect::set_description(d, e2, aux::stringid(ID, 1));
    effect::set_category(d, e2, CATEGORY_DRAW);
    effect::set_code(d, e2, EVENT_BATTLED);
    effect::set_condition(d, e2, Some(condition(drcon)));
    effect::set_target(d, e2, Some(cost_target(drtg)));
    effect::set_operation(d, e2, Some(operation(drop_op)));
    card::register_effect(d, c, e2, false);
}
/// `s.acop(e,tp,eg,ep,ev,re,r,rp)`
fn acop(d: &mut Duel, a: &EffArgs) {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if Duel.GetFlagEffect(tp,id)~=0 then return end
    if duel::get_flag_effect(d, a.tp, ID) != 0 {
        return;
    }
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,2))
    effect::set_description(d, e1, aux::stringid(ID, 2));
    // e1:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_CONTINUOUS)
    effect::set_type(d, e1, EFFECT_TYPE_FIELD + EFFECT_TYPE_CONTINUOUS);
    // e1:SetCode(EVENT_CHAINING)
    effect::set_code(d, e1, EVENT_CHAINING);
    // e1:SetProperty(EFFECT_FLAG_CLIENT_HINT)
    effect::set_property(d, e1, EFFECT_FLAG_CLIENT_HINT, 0);
    // e1:SetOperation(s.actop)
    effect::set_operation(d, e1, Some(operation(actop)));
    // e1:SetReset(RESET_PHASE|PHASE_END)
    effect::set_reset(d, e1, RESET_PHASE | PHASE_END, 1);
    // Duel.RegisterEffect(e1,tp)
    duel::register_effect(d, e1, a.tp);
    // Duel.RegisterFlagEffect(tp,id,RESET_PHASE|PHASE_END,0,1)
    duel::register_flag_effect(d, a.tp, ID, RESET_PHASE | PHASE_END, 0, 1, 0);
}
/// `s.actop(e,tp,eg,ep,ev,re,r,rp)`
fn actop(d: &mut Duel, a: &EffArgs) {
    // local rc=re:GetHandler()
    let re = a.re.nil();
    let rc = effect::get_handler(d, re);
    // if re:IsMonsterEffect() and rc:IsSetCard(SET_CRUSADIA) and ep==tp then
    if effect::is_monster_effect(d, re) && card::is_set_card(d, rc.nil(), &[SET_CRUSADIA as u16]) && a.ep == a.tp {
        // Duel.SetChainLimit(s.chainlm)
        duel::set_chain_limit(d, lua_fn(chainlm));
    }
}
/// `s.chainlm(e,rp,tp)`
fn chainlm(_d: &mut Duel, a: Args) -> bool {
    // return tp==rp
    a.int(2) == a.int(1)
}
/// `s.drcon(e,tp,eg,ep,ev,re,r,rp)`
fn drcon(d: &mut Duel, a: &EffArgs) -> bool {
    // local a=Duel.GetAttacker()
    // local d=Duel.GetAttackTarget()
    let mut att = duel::get_attacker(d);
    let mut dfn = duel::get_attack_target(d);
    // if not d then return false end
    let Some(_) = dfn else { return false };
    // if a:IsStatus(STATUS_OPPO_BATTLE) and d:IsControler(tp) then a,d=d,a end
    if card::is_status(d, att.nil(), STATUS_OPPO_BATTLE) && card::is_controler(d, dfn.nil(), a.tp) {
        std::mem::swap(&mut att, &mut dfn);
    }
    // if a:IsLinkMonster() and a:IsSetCard(SET_CRUSADIA) and d:IsStatus(STATUS_BATTLE_DESTROYED) then
    let att_c = att.nil();
    if card::is_link_monster(d, att_c) && card::is_set_card(d, att_c, &[SET_CRUSADIA as u16]) && card::is_status(d, dfn.nil(), STATUS_BATTLE_DESTROYED) {
        // e:SetLabelObject(a)
        effect::set_label_object(d, a.e, Value::Card(att_c));
        // return true
        true
    } else {
        // return false
        false
    }
}
/// `s.drtg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn drtg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return true end
    if a.chk == Some(0) {
        return true;
    }
    // local ct=e:GetLabelObject():GetLink()
    let rc = effect::get_label_object(d, a.e).as_card().nil();
    let ct = card::get_link(d, rc);
    // Duel.SetTargetPlayer(tp)
    duel::set_target_player(d, a.tp);
    // Duel.SetTargetParam(ct)
    duel::set_target_param(d, ct);
    // Duel.SetOperationInfo(0,CATEGORY_DRAW,0,0,tp,ct)
    duel::set_operation_info(d, 0, CATEGORY_DRAW, None, 0, a.tp, ct as i32);
    true
}
/// `s.drop(e,tp,eg,ep,ev,re,r,rp)`
fn drop_op(d: &mut Duel, a: &EffArgs) {
    // local rc=e:GetLabelObject()
    let rc = effect::get_label_object(d, a.e).as_card().nil();
    // local p=Duel.GetChainInfo(0,CHAININFO_TARGET_PLAYER)
    let p = duel::get_chain_info(d, 0, &[CHAININFO_TARGET_PLAYER])[0].int() as u8;
    // local ct=rc:GetLink()
    let ct = card::get_link(d, rc);
    // if rc:IsFaceup() and rc:IsRelateToBattle() and rc:IsControler(tp) and ct>0 then
    if card::is_faceup(d, rc) && card::is_relate_to_battle(d, rc) && card::is_controler(d, rc, a.tp) && ct > 0 {
        // Duel.Draw(p,ct,REASON_EFFECT)
        duel::draw(d, p, ct, REASON_EFFECT);
    }
}
