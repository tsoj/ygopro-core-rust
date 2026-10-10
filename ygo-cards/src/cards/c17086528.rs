//! Xiangsheng Magician
use super::prelude::*;

pub const ID: u32 = 17086528;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // pendulum summon
    // Pendulum.AddProcedure(c)
    pendulum::add_procedure(d, c, None, None);
    // rank
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetType(EFFECT_TYPE_IGNITION)
    effect::set_type(d, e2, EFFECT_TYPE_IGNITION);
    // e2:SetRange(LOCATION_PZONE)
    effect::set_range(d, e2, LOCATION_PZONE);
    // e2:SetProperty(EFFECT_FLAG_CARD_TARGET)
    effect::set_property(d, e2, EFFECT_FLAG_CARD_TARGET, 0);
    // e2:SetCountLimit(1)
    effect::set_count_limit(d, e2, 1, 0, 0, 0);
    // e2:SetTarget(s.rktg)
    effect::set_target(d, e2, Some(cost_target(rktg)));
    // e2:SetOperation(s.rkop)
    effect::set_operation(d, e2, Some(operation(rkop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
    // scale
    // local e3=Effect.CreateEffect(c)
    let e3 = effect::create_effect(d, c);
    // e3:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e3, EFFECT_TYPE_SINGLE);
    // e3:SetCode(EFFECT_CHANGE_LSCALE)
    effect::set_code(d, e3, EFFECT_CHANGE_LSCALE);
    // e3:SetProperty(EFFECT_FLAG_SINGLE_RANGE)
    effect::set_property(d, e3, EFFECT_FLAG_SINGLE_RANGE, 0);
    // e3:SetRange(LOCATION_PZONE)
    effect::set_range(d, e3, LOCATION_PZONE);
    // e3:SetCondition(s.slcon)
    effect::set_condition(d, e3, Some(condition(slcon)));
    // e3:SetValue(4)
    effect::set_value(d, e3, 4);
    // c:RegisterEffect(e3)
    card::register_effect(d, c, e3, false);
    // local e4=e3:Clone()
    let e4 = effect::clone(d, e3);
    // e4:SetCode(EFFECT_CHANGE_RSCALE)
    effect::set_code(d, e4, EFFECT_CHANGE_RSCALE);
    // c:RegisterEffect(e4)
    card::register_effect(d, c, e4, false);
    // damage 0
    // local e5=Effect.CreateEffect(c)
    let e5 = effect::create_effect(d, c);
    // e5:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e5, EFFECT_TYPE_SINGLE);
    // e5:SetCode(EFFECT_NO_BATTLE_DAMAGE)
    effect::set_code(d, e5, EFFECT_NO_BATTLE_DAMAGE);
    // c:RegisterEffect(e5)
    card::register_effect(d, c, e5, false);
    // atk
    // local e6=Effect.CreateEffect(c)
    let e6 = effect::create_effect(d, c);
    // e6:SetCategory(CATEGORY_ATKCHANGE)
    effect::set_category(d, e6, CATEGORY_ATKCHANGE);
    // e6:SetType(EFFECT_TYPE_IGNITION)
    effect::set_type(d, e6, EFFECT_TYPE_IGNITION);
    // e6:SetRange(LOCATION_MZONE)
    effect::set_range(d, e6, LOCATION_MZONE);
    // e6:SetProperty(EFFECT_FLAG_CARD_TARGET)
    effect::set_property(d, e6, EFFECT_FLAG_CARD_TARGET, 0);
    // e6:SetCountLimit(1)
    effect::set_count_limit(d, e6, 1, 0, 0, 0);
    // e6:SetTarget(s.atktg)
    effect::set_target(d, e6, Some(cost_target(atktg)));
    // e6:SetOperation(s.atkop)
    effect::set_operation(d, e6, Some(operation(atkop)));
    // c:RegisterEffect(e6)
    card::register_effect(d, c, e6, false);
}
/// `s.rkfilter(c,tp)`
fn rkfilter(d: &mut Duel, c: CardId, tp: u8) -> bool {
    // return c:IsFaceup() and c:IsType(TYPE_XYZ)
    //     and Duel.IsExistingTarget(s.lvfilter,tp,LOCATION_MZONE,0,1,c,c:GetRank())
    if !(card::is_faceup(d, c) && card::is_type(d, c, TYPE_XYZ)) {
        return false;
    }
    let rk = card::get_rank(d, c);
    duel::is_existing_target(d, Some(&|d, x| lvfilter(d, x, rk)), tp, LOCATION_MZONE, 0, 1, c.into())
}
/// `s.lvfilter(c,rk)`
fn lvfilter(d: &mut Duel, c: CardId, rk: u32) -> bool {
    // return c:IsFaceup() and c:IsLevelAbove(5) and c:GetLevel()~=rk
    card::is_faceup(d, c) && card::is_level_above(d, c, 5) && card::get_level(d, c) != rk
}
/// `s.rktg(e,tp,eg,ep,ev,re,r,rp,chk,chkc)`
fn rktg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chkc then return false end
    if a.chkc().is_some() {
        return false;
    }
    let tp = a.tp;
    // if chk==0 then return Duel.IsExistingTarget(s.rkfilter,tp,LOCATION_MZONE,0,1,nil,tp) end
    if a.chk == Some(0) {
        return duel::is_existing_target(d, Some(&|d, c| rkfilter(d, c, tp)), tp, LOCATION_MZONE, 0, 1, NO_EX);
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_TARGET)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_TARGET as u64);
    // local g=Duel.SelectTarget(tp,s.rkfilter,tp,LOCATION_MZONE,0,1,1,nil,tp)
    let g = duel::select_target(d, tp, Some(&|d, c| rkfilter(d, c, tp)), tp, LOCATION_MZONE, 0, 1, 1, false, NO_EX).nil();
    // e:SetLabelObject(g:GetFirst())
    let first = group::get_first(d, &g);
    effect::set_label_object(d, a.e, first.map_or(Value::Nil, Value::Card));
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_TARGET)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_TARGET as u64);
    // Duel.SelectTarget(tp,s.lvfilter,tp,LOCATION_MZONE,0,1,1,g:GetFirst(),g:GetFirst():GetRank())
    let first = group::get_first(d, &g).nil();
    let rk = card::get_rank(d, first);
    let _ = duel::select_target(d, tp, Some(&|d, x| lvfilter(d, x, rk)), tp, LOCATION_MZONE, 0, 1, 1, false, first.into());
    true
}
/// `s.rkop(e,tp,eg,ep,ev,re,r,rp)`
fn rkop(d: &mut Duel, a: &EffArgs) {
    // if not e:GetHandler():IsRelateToEffect(e) then return end
    let h = effect::get_handler(d, a.e).nil();
    if !card::is_relate_to_effect(d, h, a.e) {
        return;
    }
    // local tc=e:GetLabelObject()
    let tc = effect::get_label_object(d, a.e).as_card();
    // local tg=Duel.GetChainInfo(0,CHAININFO_TARGET_CARDS)
    let tg = duel::get_chain_info(d, 0, &[CHAININFO_TARGET_CARDS]).first().and_then(|v| v.as_group().cloned()).nil();
    // local lc=tg:GetFirst()
    let order = group::iter(d, &tg);
    let mut lc = order.first().copied();
    // if lc==tc then lc=tg:GetNext() end
    if lc == tc {
        lc = order.get(1).copied();
    }
    // if tc:IsRelateToEffect(e) and tc:IsFaceup() and lc:IsRelateToEffect(e) and lc:IsFaceup() and lc:IsLevelAbove(5) then
    if card::is_relate_to_effect(d, tc.nil(), a.e)
        && card::is_faceup(d, tc.nil())
        && card::is_relate_to_effect(d, lc.nil(), a.e)
        && card::is_faceup(d, lc.nil())
        && card::is_level_above(d, lc.nil(), 5)
    {
        // local e1=Effect.CreateEffect(e:GetHandler())
        let e1 = effect::create_effect(d, h);
        // e1:SetType(EFFECT_TYPE_SINGLE)
        effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
        // e1:SetCode(EFFECT_CHANGE_RANK)
        effect::set_code(d, e1, EFFECT_CHANGE_RANK);
        // e1:SetValue(lc:GetLevel())
        let lv = card::get_level(d, lc.nil());
        effect::set_value(d, e1, lv as i64);
        // e1:SetReset(RESETS_STANDARD_PHASE_END)
        effect::set_reset(d, e1, RESETS_STANDARD_PHASE_END, 1);
        // tc:RegisterEffect(e1)
        card::register_effect(d, tc.nil(), e1, false);
    }
}
/// `s.slcon(e)`
fn slcon(d: &mut Duel, a: &EffArgs) -> bool {
    // local tp=e:GetHandlerPlayer()
    let tp = effect::get_handler_player(d, a.e);
    // return Duel.GetFieldGroupCount(tp,LOCATION_ONFIELD,0)>Duel.GetFieldGroupCount(tp,0,LOCATION_ONFIELD)
    duel::get_field_group_count(d, tp, LOCATION_ONFIELD, 0) > duel::get_field_group_count(d, tp, 0, LOCATION_ONFIELD)
}
/// `s.atkfilter(c,atk)`
fn atkfilter(d: &mut Duel, c: CardId, atk: i32) -> bool {
    // return c:IsFaceup() and c:GetAttack()~=atk
    card::is_faceup(d, c) && card::get_attack(d, c) != atk
}
/// `s.atktg(e,tp,eg,ep,ev,re,r,rp,chk,chkc)`
fn atktg(d: &mut Duel, a: &EffArgs) -> bool {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // local atk=c:GetAttack()
    let atk = card::get_attack(d, c);
    let tp = a.tp;
    // if chkc then return chkc:IsLocation(LOCATION_MZONE) and chkc:IsControler(tp) and chkc~=c and s.atkfilter(chkc,atk) end
    if let Some(chkc) = a.chkc() {
        return card::is_location(d, chkc, LOCATION_MZONE) && card::is_controler(d, chkc, tp) && chkc != c && atkfilter(d, chkc, atk);
    }
    // if chk==0 then return Duel.IsExistingTarget(s.atkfilter,tp,LOCATION_MZONE,0,1,c,atk) end
    if a.chk == Some(0) {
        return duel::is_existing_target(d, Some(&|d, x| atkfilter(d, x, atk)), tp, LOCATION_MZONE, 0, 1, c.into());
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_TARGET)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_TARGET as u64);
    // Duel.SelectTarget(tp,s.atkfilter,tp,LOCATION_MZONE,0,1,1,c,atk)
    let _ = duel::select_target(d, tp, Some(&|d, x| atkfilter(d, x, atk)), tp, LOCATION_MZONE, 0, 1, 1, false, c.into());
    true
}
/// `s.atkop(e,tp,eg,ep,ev,re,r,rp)`
fn atkop(d: &mut Duel, a: &EffArgs) {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // local tc=Duel.GetFirstTarget()
    let tc = duel::first_target(d).nil();
    // local atk=tc:GetAttack()
    let atk = card::get_attack(d, tc);
    // if tc:IsRelateToEffect(e) and tc:IsFaceup() and c:IsRelateToEffect(e) and c:IsFaceup() then
    if card::is_relate_to_effect(d, tc, a.e) && card::is_faceup(d, tc) && card::is_relate_to_effect(d, c, a.e) && card::is_faceup(d, c) {
        // local e1=Effect.CreateEffect(c)
        let e1 = effect::create_effect(d, c);
        // e1:SetType(EFFECT_TYPE_SINGLE)
        effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
        // e1:SetCode(EFFECT_SET_ATTACK_FINAL)
        effect::set_code(d, e1, EFFECT_SET_ATTACK_FINAL);
        // e1:SetValue(atk)
        effect::set_value(d, e1, atk as i64);
        // e1:SetReset(RESETS_STANDARD_PHASE_END)
        effect::set_reset(d, e1, RESETS_STANDARD_PHASE_END, 1);
        // c:RegisterEffect(e1)
        card::register_effect(d, c, e1, false);
    }
}
