//! Loud Cloud the Storm Serpent
//! 擾乱騒蛇ラウドクラウド
use super::prelude::*;

pub const ID: u32 = 61465001;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // c:EnableReviveLimit()
    card::enable_revive_limit(d, c);
    // special summon
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e1, EFFECT_TYPE_FIELD);
    // e1:SetCode(EFFECT_SPSUMMON_PROC)
    effect::set_code(d, e1, EFFECT_SPSUMMON_PROC);
    // e1:SetProperty(EFFECT_FLAG_UNCOPYABLE)
    effect::set_property(d, e1, EFFECT_FLAG_UNCOPYABLE, 0);
    // e1:SetRange(LOCATION_HAND)
    effect::set_range(d, e1, LOCATION_HAND);
    // e1:SetCondition(s.spcon1)
    effect::set_condition(d, e1, Some(lua_fn(spcon1)));
    // e1:SetTarget(s.sptg1)
    effect::set_target(d, e1, Some(lua_fn(sptg1)));
    // e1:SetOperation(s.spop1)
    effect::set_operation(d, e1, Some(lua_fn(spop1)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // destroy monster
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e2, aux::stringid(ID, 0));
    // e2:SetCategory(CATEGORY_DESTROY)
    effect::set_category(d, e2, CATEGORY_DESTROY);
    // e2:SetType(EFFECT_TYPE_IGNITION)
    effect::set_type(d, e2, EFFECT_TYPE_IGNITION);
    // e2:SetProperty(EFFECT_FLAG_CARD_TARGET)
    effect::set_property(d, e2, EFFECT_FLAG_CARD_TARGET, 0);
    // e2:SetRange(LOCATION_MZONE)
    effect::set_range(d, e2, LOCATION_MZONE);
    // e2:SetCountLimit(1,id)
    effect::set_count_limit(d, e2, 1, ID, 0, 0);
    // e2:SetCost(s.descost1)
    effect::set_cost(d, e2, Some(cost_target(descost1)));
    // e2:SetTarget(s.destg1)
    effect::set_target(d, e2, Some(cost_target(destg1)));
    // e2:SetOperation(s.desop1)
    effect::set_operation(d, e2, Some(operation(desop1)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
    // destroy s/t
    // local e3=e2:Clone()
    let e3 = effect::clone(d, e2);
    // e3:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e3, aux::stringid(ID, 1));
    // e3:SetCountLimit(1,{id,1})
    effect::set_count_limit(d, e3, 1, ID, 1, 0);
    // e3:SetCost(s.descost2)
    effect::set_cost(d, e3, Some(cost_target(descost2)));
    // e3:SetTarget(s.destg2)
    effect::set_target(d, e3, Some(cost_target(destg2)));
    // e3:SetOperation(s.desop2)
    effect::set_operation(d, e3, Some(operation(desop2)));
    // c:RegisterEffect(e3)
    card::register_effect(d, c, e3, false);
}
/// `s.rescon(sg,e,tp,mg)`
fn rescon(d: &mut Duel, sg: &CardSet, e: Option<EffectId>, tp: u8, mg: &CardSet, c: Option<CardId>) -> (bool, bool) {
    // return aux.ChkfMMZ(1)(sg,e,tp,mg) and sg:IsExists(s.atchk1,1,nil,sg)
    let (ok, _) = aux::chkf_mmz(1)(d, sg, e, tp, mg, c);
    (ok && group::is_exists(d, sg, &|d, cc| atchk1(d, cc, sg), 1, NO_EX), false)
}
/// `s.atchk1(c,sg)`
fn atchk1(d: &mut Duel, c: CardId, sg: &CardSet) -> bool {
    // return c:IsAttribute(ATTRIBUTE_FIRE) and sg:FilterCount(Card.IsAttribute,c,ATTRIBUTE_WIND)==1
    card::is_attribute(d, c, ATTRIBUTE_FIRE)
        && group::filter_count(d, sg, &|d, cc| card::is_attribute(d, cc, ATTRIBUTE_WIND), c.into()) == 1
}
/// `s.spfilter1(c,att)`
fn spfilter1(d: &mut Duel, c: CardId, att: u32) -> bool {
    // return c:IsAttribute(att) and c:IsAbleToRemoveAsCost() and aux.SpElimFilter(c,true)
    card::is_attribute(d, c, att) && card::is_able_to_remove_as_cost(d, c, None) && aux::sp_elim_filter(d, c, true, false)
}
/// `s.spcon1(e,c)`
fn spcon1(d: &mut Duel, a: Args) -> bool {
    let e = a.e(0);
    // if c==nil then return true end
    if a.card(1).is_none() {
        return true;
    }
    // local tp=e:GetHandlerPlayer()
    let tp = effect::get_handler_player(d, e);
    // local rg1=Duel.GetMatchingGroup(s.spfilter1,tp,LOCATION_MZONE|LOCATION_GRAVE,0,nil,ATTRIBUTE_FIRE)
    let rg1 = duel::get_matching_group(d, Some(&|d, c| spfilter1(d, c, ATTRIBUTE_FIRE)), tp, LOCATION_MZONE | LOCATION_GRAVE, 0, NO_EX);
    // local rg2=Duel.GetMatchingGroup(s.spfilter1,tp,LOCATION_MZONE|LOCATION_GRAVE,0,nil,ATTRIBUTE_WIND)
    let rg2 = duel::get_matching_group(d, Some(&|d, c| spfilter1(d, c, ATTRIBUTE_WIND)), tp, LOCATION_MZONE | LOCATION_GRAVE, 0, NO_EX);
    // local rg=rg1:Clone()
    // rg:Merge(rg2)
    let mut rg = rg1.clone();
    group::add_card(&mut rg, &rg2);
    // local ft=Duel.GetLocationCount(tp,LOCATION_MZONE)
    let ft = duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0;
    // return ft>-2 and #rg1>0 and #rg2>0 and aux.SelectUnselectGroup(rg,e,tp,2,2,s.rescon,0)
    ft > -2
        && !rg1.is_empty()
        && !rg2.is_empty()
        && matches!(
            aux::select_unselect_group(d, &rg, Some(e), tp, Some(2), Some(2), Some(&rescon), 0, tp, None, None, None, false),
            Err(true)
        )
}
/// `s.sptg1(e,tp,eg,ep,ev,re,r,rp,c)`
fn sptg1(d: &mut Duel, a: Args) -> bool {
    let e = a.e(0);
    let tp = a.p(1);
    // local c=e:GetHandler()
    let _c = effect::get_handler(d, e);
    // local g=nil
    // local rg=Duel.GetMatchingGroup(s.spfilter1,tp,LOCATION_MZONE|LOCATION_GRAVE,0,nil,ATTRIBUTE_FIRE+ATTRIBUTE_WIND)
    let rg = duel::get_matching_group(d, Some(&|d, c| spfilter1(d, c, ATTRIBUTE_FIRE | ATTRIBUTE_WIND)), tp, LOCATION_MZONE | LOCATION_GRAVE, 0, NO_EX);
    // local g=aux.SelectUnselectGroup(rg,e,tp,2,2,s.rescon,1,tp,HINTMSG_REMOVE,nil,nil,true)
    let g = aux::select_unselect_group(d, &rg, Some(e), tp, Some(2), Some(2), Some(&rescon), 1, tp, Some(HINTMSG_REMOVE as u64), None, None, true).unwrap_or_default();
    // if #g>0 then
    if !g.is_empty() {
        // g:KeepAlive()
        // e:SetLabelObject(g)
        effect::set_label_object(d, e, Value::Group(g));
        // return true
        return true;
    }
    // return false
    false
}
/// `s.spop1(e,tp,eg,ep,ev,re,r,rp,c)`
fn spop1(d: &mut Duel, a: Args) {
    let e = a.e(0);
    // local g=e:GetLabelObject()
    let g = effect::get_label_object(d, e).as_group().cloned();
    // if not g then return end
    let Some(g) = g else {
        return;
    };
    // Duel.Remove(g,POS_FACEUP,REASON_COST)
    duel::remove(d, g, POS_FACEUP as u8, REASON_COST, None, None);
    // g:DeleteGroup()
}
/// `s.costfilter1(c)`
fn costfilter1(d: &mut Duel, c: CardId) -> bool {
    // return c:IsAttribute(ATTRIBUTE_FIRE) and c:IsAbleToRemoveAsCost()
    card::is_attribute(d, c, ATTRIBUTE_FIRE) && card::is_able_to_remove_as_cost(d, c, None)
}
/// `s.descost1(e,tp,eg,ep,ev,re,r,rp,chk)`
fn descost1(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // if chk==0 then return Duel.IsExistingMatchingCard(s.costfilter1,tp,LOCATION_GRAVE,0,1,nil) end
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, Some(&costfilter1), tp, LOCATION_GRAVE, 0, 1, NO_EX);
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_REMOVE)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_REMOVE as u64);
    // local g=Duel.SelectMatchingCard(tp,s.costfilter1,tp,LOCATION_GRAVE,0,1,1,nil)
    let g = duel::select_matching_card(d, tp, Some(&costfilter1), tp, LOCATION_GRAVE, 0, 1, 1, false, NO_EX).nil();
    // Duel.Remove(g,POS_FACEUP,REASON_COST)
    duel::remove(d, g, POS_FACEUP as u8, REASON_COST, None, None);
    true
}
/// `s.destg1(e,tp,eg,ep,ev,re,r,rp,chk,chkc)`
fn destg1(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // if chkc then return chkc:IsLocation(LOCATION_MZONE) and chkc:IsControler(1-tp) end
    if let Some(chkc) = a.chkc() {
        return card::is_location(d, chkc, LOCATION_MZONE) && card::is_controler(d, chkc, 1 - tp);
    }
    // if chk==0 then return Duel.IsExistingTarget(aux.TRUE,tp,0,LOCATION_MZONE,1,nil) end
    if a.chk == Some(0) {
        return duel::is_existing_target(d, None, tp, 0, LOCATION_MZONE, 1, NO_EX);
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_DESTROY)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_DESTROY as u64);
    // local g=Duel.SelectTarget(tp,aux.TRUE,tp,0,LOCATION_MZONE,1,1,nil)
    let g = duel::select_target(d, tp, None, tp, 0, LOCATION_MZONE, 1, 1, false, NO_EX);
    // Duel.SetOperationInfo(0,CATEGORY_DESTROY,g,1,0,0)
    duel::set_operation_info(d, 0, CATEGORY_DESTROY, g, 1, 0, 0);
    true
}
/// `s.desop1(e,tp,eg,ep,ev,re,r,rp)`
fn desop1(d: &mut Duel, a: &EffArgs) {
    let e = a.e;
    // local c=e:GetHandler()
    let c = effect::get_handler(d, e).nil();
    // local tc=Duel.GetFirstTarget()
    let tc = duel::first_target(d);
    // if tc and tc:IsRelateToEffect(e) and Duel.Destroy(tc,REASON_EFFECT)~=0 and tc:GetBaseAttack()>=0
    //   and c:IsRelateToEffect(e) and c:IsFaceup() then
    let Some(tc) = tc else {
        return;
    };
    if !card::is_relate_to_effect(d, tc, e) {
        return;
    }
    if duel::destroy(d, CardSet::from_card(tc), REASON_EFFECT, None, None) == 0 {
        return;
    }
    let base = card::get_base_attack(d, tc);
    if base < 0 || !card::is_relate_to_effect(d, c, e) || !card::is_faceup(d, c) {
        return;
    }
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
    // e1:SetCode(EFFECT_UPDATE_ATTACK)
    effect::set_code(d, e1, EFFECT_UPDATE_ATTACK);
    // e1:SetReset(RESET_EVENT|RESETS_STANDARD_DISABLE)
    effect::set_reset(d, e1, RESET_EVENT | RESETS_STANDARD_DISABLE, 1);
    // e1:SetValue(tc:GetBaseAttack())
    effect::set_value(d, e1, base as i64);
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
/// `s.costfilter2(c)`
fn costfilter2(d: &mut Duel, c: CardId) -> bool {
    // return c:IsAttribute(ATTRIBUTE_WIND) and c:IsAbleToRemoveAsCost()
    card::is_attribute(d, c, ATTRIBUTE_WIND) && card::is_able_to_remove_as_cost(d, c, None)
}
/// `s.desfilter(c)`
fn desfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:GetSequence()<5
    card::get_sequence(d, c) < 5
}
/// `s.descost2(e,tp,eg,ep,ev,re,r,rp,chk)`
fn descost2(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // if chk==0 then return Duel.IsExistingMatchingCard(s.costfilter2,tp,LOCATION_GRAVE,0,1,nil) end
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, Some(&costfilter2), tp, LOCATION_GRAVE, 0, 1, NO_EX);
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_REMOVE)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_REMOVE as u64);
    // local g=Duel.SelectMatchingCard(tp,s.costfilter2,tp,LOCATION_GRAVE,0,1,1,nil)
    let g = duel::select_matching_card(d, tp, Some(&costfilter2), tp, LOCATION_GRAVE, 0, 1, 1, false, NO_EX).nil();
    // Duel.Remove(g,POS_FACEUP,REASON_COST)
    duel::remove(d, g, POS_FACEUP as u8, REASON_COST, None, None);
    true
}
/// `s.destg2(e,tp,eg,ep,ev,re,r,rp,chk,chkc)`
fn destg2(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // if chkc then return chkc:IsLocation(LOCATION_SZONE) and s.desfilter(chkc) and chkc:IsControler(1-tp) end
    if let Some(chkc) = a.chkc() {
        return card::is_location(d, chkc, LOCATION_SZONE) && desfilter(d, chkc) && card::is_controler(d, chkc, 1 - tp);
    }
    // if chk==0 then return Duel.IsExistingTarget(s.desfilter,tp,0,LOCATION_SZONE,1,nil) end
    if a.chk == Some(0) {
        return duel::is_existing_target(d, Some(&desfilter), tp, 0, LOCATION_SZONE, 1, NO_EX);
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_DESTROY)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_DESTROY as u64);
    // local g=Duel.SelectTarget(tp,s.desfilter,tp,0,LOCATION_SZONE,1,1,nil)
    let g = duel::select_target(d, tp, Some(&desfilter), tp, 0, LOCATION_SZONE, 1, 1, false, NO_EX);
    // Duel.SetOperationInfo(0,CATEGORY_DESTROY,g,1,0,0)
    duel::set_operation_info(d, 0, CATEGORY_DESTROY, g, 1, 0, 0);
    true
}
/// `s.desop2(e,tp,eg,ep,ev,re,r,rp)`
fn desop2(d: &mut Duel, a: &EffArgs) {
    // local tc=Duel.GetFirstTarget()
    let tc = duel::first_target(d).nil();
    // if tc:IsRelateToEffect(e) then Duel.Destroy(tc,REASON_EFFECT) end
    if card::is_relate_to_effect(d, tc, a.e) {
        duel::destroy(d, CardSet::from_card(tc), REASON_EFFECT, None, None);
    }
}
