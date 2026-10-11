//! Archlord Zerato
use super::prelude::*;

pub const ID: u32 = 18378582;
pub const INFO: ScriptInfo = ScriptInfo { listed_names: &[66073051, CARD_SANCTUARY_SKY], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // c:EnableReviveLimit()
    card::enable_revive_limit(d, c);
    // Cannot Special Summon
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
    // Special Summon procedure
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e2, EFFECT_TYPE_FIELD);
    // e2:SetCode(EFFECT_SPSUMMON_PROC)
    effect::set_code(d, e2, EFFECT_SPSUMMON_PROC);
    // e2:SetProperty(EFFECT_FLAG_UNCOPYABLE+EFFECT_FLAG_CANNOT_DISABLE)
    effect::set_property(d, e2, EFFECT_FLAG_UNCOPYABLE + EFFECT_FLAG_CANNOT_DISABLE, 0);
    // e2:SetRange(LOCATION_HAND)
    effect::set_range(d, e2, LOCATION_HAND);
    // e2:SetCondition(s.hspcon)
    effect::set_condition(d, e2, Some(lua_fn(hspcon)));
    // e2:SetTarget(s.hsptg)
    effect::set_target(d, e2, Some(lua_fn(hsptg)));
    // e2:SetOperation(s.hspop)
    effect::set_operation(d, e2, Some(lua_fn(hspop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
    // Destroy all the opponent's monsters
    // local e3=Effect.CreateEffect(c)
    let e3 = effect::create_effect(d, c);
    // e3:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e3, aux::stringid(ID, 0));
    // e3:SetCategory(CATEGORY_DESTROY)
    effect::set_category(d, e3, CATEGORY_DESTROY);
    // e3:SetType(EFFECT_TYPE_IGNITION)
    effect::set_type(d, e3, EFFECT_TYPE_IGNITION);
    // e3:SetRange(LOCATION_MZONE)
    effect::set_range(d, e3, LOCATION_MZONE);
    // e3:SetCondition(s.descon)
    effect::set_condition(d, e3, Some(condition(descon)));
    // e3:SetCost(s.descost)
    effect::set_cost(d, e3, Some(cost_target(descost)));
    // e3:SetTarget(s.destg)
    effect::set_target(d, e3, Some(cost_target(destg)));
    // e3:SetOperation(s.desop)
    effect::set_operation(d, e3, Some(operation(desop)));
    // c:RegisterEffect(e3)
    card::register_effect(d, c, e3, false);
}
/// `aux.FaceupFilter(Card.IsCode,CARD_SANCTUARY_SKY)`
fn faceup_sanctuary(d: &mut Duel, c: CardId) -> bool {
    card::is_faceup(d, c) && card::is_code(d, c, &[CARD_SANCTUARY_SKY])
}
/// `s.hspcon(e,c)`: the core passes no card when it only checks the effect
fn hspcon(d: &mut Duel, a: Args) -> bool {
    let c = a.card(1);
    let Some(c) = c else {
        // if c==nil then return Duel.IsExistingMatchingCard(aux.FaceupFilter(Card.IsCode,CARD_SANCTUARY_SKY),0,LOCATION_ONFIELD,LOCATION_ONFIELD,1,nil)
        //   or Duel.IsEnvironment(CARD_SANCTUARY_SKY) end
        return duel::is_existing_matching_card(d, Some(&faceup_sanctuary), 0, LOCATION_ONFIELD, LOCATION_ONFIELD, 1, NO_EX)
            || duel::is_environment(d, CARD_SANCTUARY_SKY, None, None);
    };
    // return Duel.CheckReleaseGroup(c:GetControler(),Card.IsCode,1,false,1,true,c,c:GetControler(),nil,false,nil,66073051)
    let ctrl = card::get_controler(d, c);
    let ra = duel::ReleaseArgs {
        use_hand: Some(false),
        max: Some(1),
        check_field: true,
        to_check: Some(c),
        toplayer: Some(ctrl),
        ..Default::default()
    };
    duel::check_release_group(d, ctrl, Some(&|d, x| card::is_code(d, x, &[66073051])), 1, ra, NO_EX, false)
}
/// `s.hsptg(e,tp,eg,ep,ev,re,r,rp,c)`
/// The Lua names this argument `c` but has no `chk`: the core passes the number 1 there, so the
/// release's `to_check` card is nil.
fn hsptg(d: &mut Duel, a: Args) -> bool {
    let e = a.e(0);
    let tp = a.p(1);
    // local g=Duel.SelectReleaseGroup(tp,Card.IsCode,1,1,false,true,true,c,nil,nil,false,nil,66073051)
    let ra = duel::ReleaseArgs {
        use_hand: Some(false),
        cancelable: true,
        check_field: true,
        to_check: a.card(8),
        ..Default::default()
    };
    let g = duel::select_release_group(d, tp, Some(&|d, x| card::is_code(d, x, &[66073051])), 1, 1, ra, NO_EX, false);
    // if g then
    if let Some(g) = g {
        // g:KeepAlive()
        // e:SetLabelObject(g)
        effect::set_label_object(d, e, Value::Group(g));
        // return true
        true
    } else {
        // return false
        false
    }
}
/// `s.hspop(e,tp,eg,ep,ev,re,r,rp,c)`
fn hspop(d: &mut Duel, a: Args) {
    let e = a.e(0);
    // local g=e:GetLabelObject()
    let g = effect::get_label_object(d, e).as_group().cloned();
    // if not g then return end
    let Some(g) = g else {
        return;
    };
    // Duel.Release(g,REASON_COST)
    duel::release(d, g, REASON_COST, None);
    // g:DeleteGroup()
}
/// `s.descon(e,tp,eg,ep,ev,re,r,rp)`
fn descon(d: &mut Duel, a: &EffArgs) -> bool {
    // return Duel.IsExistingMatchingCard(aux.FaceupFilter(Card.IsCode,CARD_SANCTUARY_SKY),tp,LOCATION_ONFIELD,0,1,nil)
    //   or Duel.IsEnvironment(CARD_SANCTUARY_SKY,tp)
    duel::is_existing_matching_card(d, Some(&faceup_sanctuary), a.tp, LOCATION_ONFIELD, 0, 1, NO_EX) || duel::is_environment(d, CARD_SANCTUARY_SKY, Some(a.tp), None)
}
/// `s.cfilter(c)`
fn cfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsAttribute(ATTRIBUTE_LIGHT) and c:IsDiscardable() and c:IsAbleToGraveAsCost()
    card::is_attribute(d, c, ATTRIBUTE_LIGHT) && card::is_discardable(d, c, None, None) && card::is_able_to_grave_as_cost(d, c)
}
/// `s.descost(e,tp,eg,ep,ev,re,r,rp,chk)`
fn descost(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return Duel.IsExistingMatchingCard(s.cfilter,tp,LOCATION_HAND,0,1,nil) end
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, Some(&cfilter), a.tp, LOCATION_HAND, 0, 1, NO_EX);
    }
    // Duel.DiscardHand(tp,s.cfilter,1,1,REASON_COST|REASON_DISCARD)
    duel::discard_hand(d, a.tp, Some(&cfilter), 1, 1, REASON_COST | REASON_DISCARD, NO_EX);
    true
}
/// `s.destg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn destg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return Duel.IsExistingMatchingCard(aux.TRUE,tp,0,LOCATION_MZONE,1,nil) end
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, None, a.tp, 0, LOCATION_MZONE, 1, NO_EX);
    }
    // local g=Duel.GetMatchingGroup(aux.TRUE,tp,0,LOCATION_MZONE,nil)
    let g = duel::get_matching_group(d, None, a.tp, 0, LOCATION_MZONE, NO_EX);
    // Duel.SetOperationInfo(0,CATEGORY_DESTROY,g,#g,0,0)
    let n = g.len() as u8;
    duel::set_operation_info(d, 0, CATEGORY_DESTROY, Some(g), n, 0, 0);
    true
}
/// `s.desop(e,tp,eg,ep,ev,re,r,rp)`
fn desop(d: &mut Duel, a: &EffArgs) {
    // if Duel.IsExistingMatchingCard(aux.FaceupFilter(Card.IsCode,CARD_SANCTUARY_SKY),tp,LOCATION_ONFIELD,0,1,nil)
    //   or Duel.IsEnvironment(CARD_SANCTUARY_SKY,tp) then
    if duel::is_existing_matching_card(d, Some(&faceup_sanctuary), a.tp, LOCATION_ONFIELD, 0, 1, NO_EX) || duel::is_environment(d, CARD_SANCTUARY_SKY, Some(a.tp), None) {
        // local g=Duel.GetMatchingGroup(aux.TRUE,tp,0,LOCATION_MZONE,nil)
        let g = duel::get_matching_group(d, None, a.tp, 0, LOCATION_MZONE, NO_EX);
        // Duel.Destroy(g,REASON_EFFECT)
        duel::destroy(d, g, REASON_EFFECT, None, None);
    }
}
