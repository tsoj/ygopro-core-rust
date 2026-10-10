//! Uria, Lord of Searing Flames
use super::prelude::*;

pub const ID: u32 = 6007213;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // c:EnableReviveLimit()
    card::enable_revive_limit(d, c);
    // Cannot be special summoned
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
    // e1:SetProperty(EFFECT_FLAG_CANNOT_DISABLE+EFFECT_FLAG_UNCOPYABLE)
    effect::set_property(d, e1, EFFECT_FLAG_CANNOT_DISABLE + EFFECT_FLAG_UNCOPYABLE, 0);
    // e1:SetCode(EFFECT_SPSUMMON_CONDITION)
    effect::set_code(d, e1, EFFECT_SPSUMMON_CONDITION);
    // e1:SetValue(aux.FALSE)
    effect::set_value_fn(d, e1, aux_false());
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // Special summon procedure
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e2, EFFECT_TYPE_FIELD);
    // e2:SetCode(EFFECT_SPSUMMON_PROC)
    effect::set_code(d, e2, EFFECT_SPSUMMON_PROC);
    // e2:SetProperty(EFFECT_FLAG_UNCOPYABLE)
    effect::set_property(d, e2, EFFECT_FLAG_UNCOPYABLE, 0);
    // e2:SetRange(LOCATION_HAND)
    effect::set_range(d, e2, LOCATION_HAND);
    // e2:SetCondition(s.spcon)
    effect::set_condition(d, e2, Some(lua_fn(spcon)));
    // e2:SetTarget(s.sptg)
    effect::set_target(d, e2, Some(lua_fn(sptg)));
    // e2:SetOperation(s.spop)
    effect::set_operation(d, e2, Some(lua_fn(spop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
    // Increase ATK
    // local e3=Effect.CreateEffect(c)
    let e3 = effect::create_effect(d, c);
    // e3:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e3, EFFECT_TYPE_SINGLE);
    // e3:SetCode(EFFECT_UPDATE_ATTACK)
    effect::set_code(d, e3, EFFECT_UPDATE_ATTACK);
    // e3:SetProperty(EFFECT_FLAG_SINGLE_RANGE)
    effect::set_property(d, e3, EFFECT_FLAG_SINGLE_RANGE, 0);
    // e3:SetRange(LOCATION_MZONE)
    effect::set_range(d, e3, LOCATION_MZONE);
    // e3:SetValue(s.atkval)
    effect::set_value_fn(d, e3, effect_card_value(atkval));
    // c:RegisterEffect(e3)
    card::register_effect(d, c, e3, false);
    // Destroy Spell/Trap
    // local e4=Effect.CreateEffect(c)
    let e4 = effect::create_effect(d, c);
    // e4:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e4, aux::stringid(ID, 0));
    // e4:SetCategory(CATEGORY_DESTROY)
    effect::set_category(d, e4, CATEGORY_DESTROY);
    // e4:SetType(EFFECT_TYPE_IGNITION)
    effect::set_type(d, e4, EFFECT_TYPE_IGNITION);
    // e4:SetRange(LOCATION_MZONE)
    effect::set_range(d, e4, LOCATION_MZONE);
    // e4:SetCountLimit(1)
    effect::set_count_limit(d, e4, 1, 0, 0, 0);
    // e4:SetProperty(EFFECT_FLAG_CARD_TARGET)
    effect::set_property(d, e4, EFFECT_FLAG_CARD_TARGET, 0);
    // e4:SetTarget(s.destg)
    effect::set_target(d, e4, Some(cost_target(destg)));
    // e4:SetOperation(s.desop)
    effect::set_operation(d, e4, Some(operation(desop)));
    // c:RegisterEffect(e4)
    card::register_effect(d, c, e4, false);
}
fn spfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsFaceup() and c:IsTrap() and c:IsAbleToGraveAsCost()
    card::is_faceup(d, c) && card::is_trap(d, c) && card::is_able_to_grave_as_cost(d, c)
}
fn exfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsTrap() and c:IsAbleToGraveAsCost()
    card::is_trap(d, c) && card::is_able_to_grave_as_cost(d, c)
}
/// The cards that can be sent to the GY for the Special Summon (`s.spfilter`, or `s.exfilter` under 16317140)
fn summon_group(d: &mut Duel, tp: u8) -> Group {
    // if Duel.IsPlayerAffectedByEffect(tp,16317140) then
    if duel::is_player_affected_by_effect(d, tp, 16317140).is_some() {
        // g=Duel.GetMatchingGroup(s.exfilter,tp,LOCATION_ONFIELD,0,nil)
        duel::get_matching_group(d, Some(&exfilter), tp, LOCATION_ONFIELD, 0, NO_EX)
    } else {
        // g=Duel.GetMatchingGroup(s.spfilter,tp,LOCATION_ONFIELD,0,nil)
        duel::get_matching_group(d, Some(&spfilter), tp, LOCATION_ONFIELD, 0, NO_EX)
    }
}
/// `s.spcon(e,c)`
fn spcon(d: &mut Duel, a: Args) -> bool {
    let e = a.e(0);
    // if c==nil then return true end
    let Some(c) = a.card(1) else { return true };
    // local tp=c:GetControler()
    let tp = card::get_controler(d, c);
    // local g=nil
    // (the group is picked below)
    let g = summon_group(d, tp);
    // return Duel.GetLocationCount(tp,LOCATION_MZONE)>-3 and #g>2 and aux.SelectUnselectGroup(g,e,tp,3,3,aux.ChkfMMZ(1),0)
    if duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 <= -3 {
        return false;
    }
    if g.len() <= 2 {
        return false;
    }
    let chkf = aux::chkf_mmz(1);
    matches!(aux::select_unselect_group(d, &g, Some(e), tp, Some(3), Some(3), Some(&chkf), 0, tp, None, None, None, false), Err(true))
}
/// `s.sptg(e,tp,eg,ep,ev,re,r,rp,c)`
fn sptg(d: &mut Duel, a: Args) -> bool {
    let (e, tp) = (a.e(0), a.p(1));
    // local g=nil (as in spcon)
    let g = summon_group(d, tp);
    // local sg=aux.SelectUnselectGroup(g,e,tp,3,3,aux.ChkfMMZ(1),1,tp,HINTMSG_TOGRAVE,nil,nil,true)
    let chkf = aux::chkf_mmz(1);
    let sg = aux::select_unselect_group(d, &g, Some(e), tp, Some(3), Some(3), Some(&chkf), 1, tp, Some(HINTMSG_TOGRAVE as u64), None, None, true).unwrap_or_default();
    // local dg=sg:Filter(Card.IsFacedown,nil)
    let dg = group::filter(d, &sg, &|d, x| card::is_facedown(d, x), NO_EX);
    // if #dg>0 then Duel.ConfirmCards(1-tp,dg) end
    if !dg.is_empty() {
        duel::confirm_cards(d, 1 - tp, dg);
    }
    // if #sg==3 then
    if sg.len() == 3 {
        // sg:KeepAlive()
        // e:SetLabelObject(sg)
        effect::set_label_object(d, e, Value::Group(sg));
        // return true
        return true;
    }
    // return false
    false
}
/// `s.spop(e,tp,eg,ep,ev,re,r,rp,c)`
fn spop(d: &mut Duel, a: Args) {
    let e = a.e(0);
    // local g=e:GetLabelObject()
    // if not g then return end
    let Some(g) = effect::get_label_object(d, e).as_group().cloned() else { return };
    // Duel.SendtoGrave(g,REASON_COST)
    duel::send_to_grave(d, g, REASON_COST, None, None);
    // g:DeleteGroup()
}
/// `s.atkfilter(c)`
fn atkfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsContinuousTrap()
    card::is_continuous_trap(d, c)
}
/// `s.atkval(e,c)`
fn atkval(d: &mut Duel, _e: EffectId, c: CardId) -> i64 {
    // return Duel.GetMatchingGroupCount(s.atkfilter,c:GetControler(),LOCATION_GRAVE,0,nil)*1000
    let p = card::get_controler(d, c);
    duel::get_matching_group_count(d, Some(&atkfilter), p, LOCATION_GRAVE, 0, NO_EX) as i64 * 1000
}
/// `s.destg(e,tp,eg,ep,ev,re,r,rp,chk,chkc)`
fn destg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // if chkc then return chkc:IsLocation(LOCATION_SZONE) and chkc:IsControler(1-tp) and chkc:IsFacedown() end
    if let Some(chkc) = a.chkc() {
        return card::is_location(d, chkc, LOCATION_SZONE) && card::is_controler(d, chkc, 1 - tp) && card::is_facedown(d, chkc);
    }
    // if chk==0 then return Duel.IsExistingTarget(Card.IsFacedown,tp,0,LOCATION_SZONE,1,nil) end
    if a.chk == Some(0) {
        return duel::is_existing_target(d, Some(&|d, x| card::is_facedown(d, x)), tp, 0, LOCATION_SZONE, 1, NO_EX);
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_DESTROY)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_DESTROY as u64);
    // local g=Duel.SelectTarget(tp,Card.IsFacedown,tp,0,LOCATION_SZONE,1,1,nil)
    let g = duel::select_target(d, tp, Some(&|d, x| card::is_facedown(d, x)), tp, 0, LOCATION_SZONE, 1, 1, false, NO_EX);
    // Duel.SetOperationInfo(0,CATEGORY_DESTROY,g,1,0,0)
    duel::set_operation_info(d, 0, CATEGORY_DESTROY, g, 1, 0, 0);
    // Duel.SetChainLimit(s.chainlimit)
    duel::set_chain_limit(d, lua_fn(chainlimit));
    true
}
/// `s.chainlimit(e,rp,tp)`
fn chainlimit(d: &mut Duel, a: Args) -> bool {
    // return not e:IsHasType(EFFECT_TYPE_ACTIVATE)
    !effect::is_has_type(d, a.e(0), EFFECT_TYPE_ACTIVATE)
}
/// `s.desop(e,tp,eg,ep,ev,re,r,rp)`
fn desop(d: &mut Duel, a: &EffArgs) {
    // local tc=Duel.GetFirstTarget()
    let tc = duel::first_target(d);
    // if tc and tc:IsFacedown() and tc:IsRelateToEffect(e) then
    if let Some(tc) = tc {
        if card::is_facedown(d, tc) && card::is_relate_to_effect(d, tc, a.e) {
            // Duel.Destroy(tc,REASON_EFFECT)
            duel::destroy(d, tc, REASON_EFFECT, None, None);
        }
    }
}
