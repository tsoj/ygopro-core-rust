//! Toy Box
//! scripted by Naim
use super::prelude::*;

pub const ID: u32 = 24878656;

pub const INFO: ScriptInfo = ScriptInfo {
    // s.listed_series={SET_TOY}
    listed_series: &[SET_TOY],
    ..ScriptInfo::DEFAULT
};

/// `LOCATIONS_HAND_DECK_MZONE_GRAVE`
const LOCATIONS_HAND_DECK_MZONE_GRAVE: u32 = LOCATION_HAND | LOCATION_DECK | LOCATION_MZONE | LOCATION_GRAVE;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    // e1:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // Destroy an opponent's attacking monster
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e2, aux::stringid(ID, 0));
    // e2:SetCategory(CATEGORY_DESTROY)
    effect::set_category(d, e2, CATEGORY_DESTROY);
    // e2:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e2, EFFECT_TYPE_FIELD + EFFECT_TYPE_TRIGGER_O);
    // e2:SetCode(EVENT_ATTACK_ANNOUNCE)
    effect::set_code(d, e2, EVENT_ATTACK_ANNOUNCE);
    // e2:SetRange(LOCATION_SZONE)
    effect::set_range(d, e2, LOCATION_SZONE);
    // e2:SetCountLimit(1)
    effect::set_count_limit(d, e2, 1, 0, 0, 0);
    // e2:SetCondition(function(e,tp) return Duel.GetAttacker():IsControler(1-tp) end)
    effect::set_condition(d, e2, Some(condition(|d, a| {
        let at = duel::get_attacker(d).nil();
        card::is_controler(d, at, 1 - a.tp)
    })));
    // e2:SetCost(s.mondescost)
    effect::set_cost(d, e2, Some(cost_target(mondescost)));
    // e2:SetTarget(s.mondestg)
    effect::set_target(d, e2, Some(cost_target(mondestg)));
    // e2:SetOperation(s.mondesop)
    effect::set_operation(d, e2, Some(operation(mondesop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
    // Activate 1 of these effects
    // local e3=Effect.CreateEffect(c)
    let e3 = effect::create_effect(d, c);
    // e3:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e3, aux::stringid(ID, 1));
    // e3:SetType(EFFECT_TYPE_IGNITION)
    effect::set_type(d, e3, EFFECT_TYPE_IGNITION);
    // e3:SetRange(LOCATION_SZONE)
    effect::set_range(d, e3, LOCATION_SZONE);
    // e3:SetCountLimit(1)
    effect::set_count_limit(d, e3, 1, 0, 0, 0);
    // e3:SetTarget(s.efftg)
    effect::set_target(d, e3, Some(cost_target(efftg)));
    // e3:SetOperation(s.effop)
    effect::set_operation(d, e3, Some(operation(effop)));
    // c:RegisterEffect(e3)
    card::register_effect(d, c, e3, false);
}
/// `s.cstfilter(c)`
fn cstfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsFacedown() and c:IsAbleToGraveAsCost()
    card::is_facedown(d, c) && card::is_able_to_grave_as_cost(d, c)
}
/// `s.mondescost(e,tp,eg,ep,ev,re,r,rp,chk)`
fn mondescost(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // if chk==0 then return Duel.IsExistingMatchingCard(s.cstfilter,tp,LOCATION_ONFIELD,0,1,nil) end
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, Some(&cstfilter), tp, LOCATION_ONFIELD, 0, 1, NO_EX);
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_TOGRAVE)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_TOGRAVE as u64);
    // local g=Duel.SelectMatchingCard(tp,s.cstfilter,tp,LOCATION_ONFIELD,0,1,1,nil)
    let g = duel::select_matching_card(d, tp, Some(&cstfilter), tp, LOCATION_ONFIELD, 0, 1, 1, false, NO_EX).nil();
    // Duel.SendtoGrave(g,REASON_COST)
    duel::send_to_grave(d, g, REASON_COST, None, None);
    true
}
/// `s.mondestg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn mondestg(d: &mut Duel, a: &EffArgs) -> bool {
    // local at=Duel.GetAttacker()
    let at = duel::get_attacker(d).nil();
    // if chk==0 then return at:IsRelateToBattle() end
    if a.chk == Some(0) {
        return card::is_relate_to_battle(d, at);
    }
    // Duel.SetOperationInfo(0,CATEGORY_DESTROY,at,1,0,0)
    duel::set_operation_info(d, 0, CATEGORY_DESTROY, Some(CardSet::from_card(at)), 1, 0, 0);
    true
}
/// `s.mondesop(e,tp,eg,ep,ev,re,r,rp)`
fn mondesop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // local at=Duel.GetAttacker()
    let at = duel::get_attacker(d).nil();
    // if at:IsRelateToBattle() and at:IsControler(1-tp) then
    if card::is_relate_to_battle(d, at) && card::is_controler(d, at, 1 - tp) {
        // Duel.Destroy(at,REASON_EFFECT)
        duel::destroy(d, at, REASON_EFFECT, None, None);
    }
}
/// `s.setfilter(c)`
fn setfilter(d: &mut Duel, c: CardId) -> bool {
    // local set_eff=c:IsHasEffect(EFFECT_MONSTER_SSET)
    let set_eff = card::is_has_effect(d, c, EFFECT_MONSTER_SSET, None).first().copied();
    // return c:IsOriginalSetCard(SET_TOY) and c:IsMonster() and set_eff and set_eff:GetValue()&TYPE_SPELL>0
    //     and c:IsSSetable() and (c:IsFaceup() or not c:IsLocation(LOCATION_MZONE))
    if !card::is_original_set_card(d, c, &[SET_TOY as u16]) || !card::is_monster(d, c) {
        return false;
    }
    let Some(set_eff) = set_eff else { return false };
    if effect::get_value(d, set_eff).int() & TYPE_SPELL as i64 <= 0 {
        return false;
    }
    card::is_s_setable(d, c, false, None) && (card::is_faceup(d, c) || !card::is_location(d, c, LOCATION_MZONE))
}
/// `s.efftg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn efftg(d: &mut Duel, a: &EffArgs) -> bool {
    let (e, tp) = (a.e, a.tp);
    // local g=Duel.GetMatchingGroup(nil,tp,LOCATION_STZONE,0,nil)
    let g = duel::get_matching_group(d, None, tp, LOCATION_STZONE, 0, NO_EX);
    // local b1=Duel.IsExistingMatchingCard(s.setfilter,tp,LOCATIONS_HAND_DECK_MZONE_GRAVE,0,1,nil)
    let b1 = duel::is_existing_matching_card(d, Some(&setfilter), tp, LOCATIONS_HAND_DECK_MZONE_GRAVE, 0, 1, NO_EX);
    // local b2=#g>0
    let b2 = !g.is_empty();
    // if chk==0 then return b1 or b2 end
    if a.chk == Some(0) {
        return b1 || b2;
    }
    // local op=Duel.SelectEffect(tp,{b1,aux.Stringid(id,2)},{b2,aux.Stringid(id,3)})
    let op = duel::select_effect(d, tp, &[(b1, aux::stringid(ID, 2)), (b2, aux::stringid(ID, 3))]);
    // e:SetLabel(op)
    effect::set_label(d, e, &[op.unwrap_or(0) as i64]);
    // if op==1 then
    if op == Some(1) {
        // e:SetCategory(CATEGORY_SET)
        effect::set_category(d, e, CATEGORY_SET);
    // elseif op==2 then
    } else if op == Some(2) {
        // e:SetCategory(CATEGORY_DESTROY)
        effect::set_category(d, e, CATEGORY_DESTROY);
        // Duel.SetOperationInfo(0,CATEGORY_DESTROY,g,1,tp,0)
        duel::set_operation_info(d, 0, CATEGORY_DESTROY, Some(g), 1, tp, 0);
    }
    true
}
/// `s.effop(e,tp,eg,ep,ev,re,r,rp)`
fn effop(d: &mut Duel, a: &EffArgs) {
    let (e, tp) = (a.e, a.tp);
    // local op=e:GetLabel()
    let op = effect::get_label(d, e);
    // if op==1 then
    if op == 1 {
        // local ft=math.min(Duel.GetLocationCount(tp,LOCATION_SZONE),2)
        let ft = duel::get_location_count(d, tp, LOCATION_SZONE as u8, None, None, None).0.min(2);
        // if ft==0 then return end
        if ft == 0 {
            return;
        }
        // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_SET)
        duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_SET as u64);
        // local g=Duel.SelectMatchingCard(tp,aux.NecroValleyFilter(s.setfilter),tp,LOCATIONS_HAND_DECK_MZONE_GRAVE,0,1,ft,nil)
        let f = aux::necro_valley_filter(setfilter);
        let g = duel::select_matching_card(d, tp, Some(&f), tp, LOCATIONS_HAND_DECK_MZONE_GRAVE, 0, 1, ft as u16, false, NO_EX).nil();
        // if #g>0 then
        if !g.is_empty() {
            // Duel.SSet(tp,g)
            duel::s_set(d, tp, g, None, true);
        }
    // elseif op==2 then
    } else if op == 2 {
        // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_DESTROY)
        duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_DESTROY as u64);
        // local g=Duel.SelectMatchingCard(tp,nil,tp,LOCATION_STZONE,0,1,2,nil)
        let g = duel::select_matching_card(d, tp, None, tp, LOCATION_STZONE, 0, 1, 2, false, NO_EX).nil();
        // if #g>0 then
        if !g.is_empty() {
            // Duel.HintSelection(g,true)
            duel::hint_selection(d, g.clone(), true);
            // Duel.Destroy(g,REASON_EFFECT)
            duel::destroy(d, g, REASON_EFFECT, None, None);
        }
    }
}
