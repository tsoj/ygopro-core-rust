//! Kaiza the Hidden Star
//! 地葬星カイザ
//! Scripted by Hatter
use super::prelude::*;

pub const ID: u32 = 25449584;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Special Summon
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e1, EFFECT_TYPE_FIELD);
    // e1:SetCode(EFFECT_SPSUMMON_PROC)
    effect::set_code(d, e1, EFFECT_SPSUMMON_PROC);
    // e1:SetProperty(EFFECT_FLAG_UNCOPYABLE)
    effect::set_property(d, e1, EFFECT_FLAG_UNCOPYABLE, 0);
    // e1:SetRange(LOCATION_HAND)
    effect::set_range(d, e1, LOCATION_HAND);
    // e1:SetCountLimit(1,id,EFFECT_COUNT_CODE_OATH)
    effect::set_count_limit(d, e1, 1, ID, 0, EFFECT_COUNT_CODE_OATH as u8);
    // e1:SetCondition(s.spcon)
    effect::set_condition(d, e1, Some(lua_fn(spcon)));
    // e1:SetTarget(s.sptg)
    effect::set_target(d, e1, Some(lua_fn(sptg)));
    // e1:SetOperation(s.spop)
    effect::set_operation(d, e1, Some(lua_fn(spop)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);

    // Send to GY
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e2, aux::stringid(ID, 0));
    // e2:SetCategory(CATEGORY_TOGRAVE)
    effect::set_category(d, e2, CATEGORY_TOGRAVE);
    // e2:SetType(EFFECT_TYPE_IGNITION)
    effect::set_type(d, e2, EFFECT_TYPE_IGNITION);
    // e2:SetRange(LOCATION_MZONE)
    effect::set_range(d, e2, LOCATION_MZONE);
    // e2:SetCountLimit(1,{id,1})
    effect::set_count_limit(d, e2, 1, ID, 1, 0);
    // e2:SetCost(s.tgcost)
    effect::set_cost(d, e2, Some(cost_target(tgcost)));
    // e2:SetTarget(s.tgtg)
    effect::set_target(d, e2, Some(cost_target(tgtg)));
    // e2:SetOperation(s.tgop)
    effect::set_operation(d, e2, Some(operation(tgop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
}
/// `s.attfilter(c)`
fn attfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsRace(RACE_WARRIOR) and (c:IsAttribute(ATTRIBUTE_EARTH) or c:IsAttribute(ATTRIBUTE_LIGHT))
    card::is_race(d, c, RACE_WARRIOR) && (card::is_attribute(d, c, ATTRIBUTE_EARTH) || card::is_attribute(d, c, ATTRIBUTE_LIGHT))
}
/// `s.spcon(e,c)`
fn spcon(d: &mut Duel, a: Args) -> bool {
    // return not c or Duel.CheckReleaseGroup(c:GetControler(),s.attfilter,1,false,1,true,c,c:GetControler(),nil,false,nil)
    let Some(c) = a.card(1) else { return true };
    let ctrl = card::get_controler(d, c);
    let ra = duel::ReleaseArgs {
        use_hand: Some(false),
        max: Some(1),
        check_field: true,
        to_check: Some(c),
        toplayer: Some(ctrl),
        ..Default::default()
    };
    duel::check_release_group(d, ctrl, Some(&attfilter), 1, ra, NO_EX, false)
}
/// `s.sptg(e,tp,eg,ep,ev,re,r,rp,c)`
fn sptg(d: &mut Duel, a: Args) -> bool {
    let e = a.e(0);
    let tp = a.p(1);
    let c = a.card(8);
    // local g=Duel.SelectReleaseGroup(tp,s.attfilter,1,1,false,true,true,c,nil,nil,false,nil)
    let ra = duel::ReleaseArgs {
        use_hand: Some(false),
        cancelable: true,
        check_field: true,
        to_check: c,
        ..Default::default()
    };
    // if g then
    if let Some(g) = duel::select_release_group(d, tp, Some(&attfilter), 1, 1, ra, NO_EX, false) {
        // g:KeepAlive()
        // e:SetLabelObject(g)
        effect::set_label_object(d, e, Value::Group(g));
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
    // if not g or #g<1 then return end
    let Value::Group(g) = effect::get_label_object(d, e) else { return };
    if g.is_empty() {
        return;
    }
    // Duel.Release(g,REASON_COST)
    duel::release(d, g.clone(), REASON_COST, None);
    // local atk=g:GetFirst():GetBaseAttack()
    let first = group::get_first(d, &g).nil();
    let atk = card::get_base_attack(d, first);
    // if atk>0 then
    if atk > 0 {
        // local c=e:GetHandler()
        let c = effect::get_handler(d, e).nil();
        // Gain ATK
        // local e1=Effect.CreateEffect(c)
        let e1 = effect::create_effect(d, c);
        // e1:SetType(EFFECT_TYPE_SINGLE)
        effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
        // e1:SetCode(EFFECT_UPDATE_ATTACK)
        effect::set_code(d, e1, EFFECT_UPDATE_ATTACK);
        // e1:SetValue(atk)
        effect::set_value(d, e1, atk as i64);
        // e1:SetReset(RESET_EVENT|(RESETS_STANDARD_DISABLE&~RESET_TOFIELD))
        effect::set_reset(d, e1, RESET_EVENT | (RESETS_STANDARD_DISABLE & !RESET_TOFIELD), 1);
        // c:RegisterEffect(e1)
        card::register_effect(d, c, e1, false);
    }
    // g:DeleteGroup()
}
/// `s.tgcostfilter(c)`
fn tgcostfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsRace(RACE_WARRIOR) and c:IsAbleToRemoveAsCost() and aux.SpElimFilter(c,true)
    card::is_race(d, c, RACE_WARRIOR) && card::is_able_to_remove_as_cost(d, c, None) && aux::sp_elim_filter(d, c, true, false)
}
/// `s.tgcost(e,tp,eg,ep,ev,re,r,rp,chk)`
fn tgcost(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return Duel.IsExistingMatchingCard(s.tgcostfilter,tp,LOCATION_GRAVE|LOCATION_MZONE,0,1,nil) end
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, Some(&tgcostfilter), a.tp, LOCATION_GRAVE | LOCATION_MZONE, 0, 1, NO_EX);
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_REMOVE)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_REMOVE as u64);
    // local sg=Duel.SelectMatchingCard(tp,s.tgcostfilter,tp,LOCATION_GRAVE|LOCATION_MZONE,0,1,1,nil)
    let sg = duel::select_matching_card(d, a.tp, Some(&tgcostfilter), a.tp, LOCATION_GRAVE | LOCATION_MZONE, 0, 1, 1, false, NO_EX).nil();
    // Duel.Remove(sg,POS_FACEUP,REASON_COST)
    duel::remove(d, sg, POS_FACEUP as u8, REASON_COST, None, None);
    true
}
/// `s.tgtg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn tgtg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return Duel.IsExistingMatchingCard(aux.AND(s.attfilter,Card.IsAbleToGrave),tp,LOCATION_DECK,0,1,nil) end
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, Some(&|d, c| attfilter(d, c) && card::is_able_to_grave(d, c, None)), a.tp, LOCATION_DECK, 0, 1, NO_EX);
    }
    // Duel.SetOperationInfo(0,CATEGORY_TOGRAVE,nil,1,tp,LOCATION_DECK)
    duel::set_operation_info(d, 0, CATEGORY_TOGRAVE, None, 1, a.tp, LOCATION_DECK as i32);
    true
}
/// `s.tgop(e,tp,eg,ep,ev,re,r,rp)`
fn tgop(d: &mut Duel, a: &EffArgs) {
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_TOGRAVE)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_TOGRAVE as u64);
    // local g=Duel.SelectMatchingCard(tp,aux.AND(s.attfilter,Card.IsAbleToGrave),tp,LOCATION_DECK,0,1,1,nil)
    let g = duel::select_matching_card(d, a.tp, Some(&|d, c| attfilter(d, c) && card::is_able_to_grave(d, c, None)), a.tp, LOCATION_DECK, 0, 1, 1, false, NO_EX).nil();
    // if #g>0 then
    if !g.is_empty() {
        // Duel.SendtoGrave(g,REASON_EFFECT)
        duel::send_to_grave(d, g, REASON_EFFECT, None, None);
    }
}
