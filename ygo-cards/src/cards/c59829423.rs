//! Lord of the Missing Barrows
//! Scripted by Eerie Code
use super::prelude::*;

pub const ID: u32 = 59829423;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Special Summon this card (from your hand)
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e1, EFFECT_TYPE_FIELD);
    // e1:SetProperty(EFFECT_FLAG_UNCOPYABLE)
    effect::set_property(d, e1, EFFECT_FLAG_UNCOPYABLE, 0);
    // e1:SetCode(EFFECT_SPSUMMON_PROC)
    effect::set_code(d, e1, EFFECT_SPSUMMON_PROC);
    // e1:SetRange(LOCATION_HAND)
    effect::set_range(d, e1, LOCATION_HAND);
    // e1:SetCondition(s.spcon)
    effect::set_condition(d, e1, Some(lua_fn(|d, a| spcon(d, a))));
    // e1:SetTarget(s.sptg)
    effect::set_target(d, e1, Some(cost_target(sptg)));
    // e1:SetOperation(s.spop)
    effect::set_operation(d, e1, Some(operation(spop)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // Banish 1 monster and 2 Spells/Traps from your opponent's field and/or GY
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e2, aux::stringid(ID, 1));
    // e2:SetCategory(CATEGORY_REMOVE)
    effect::set_category(d, e2, CATEGORY_REMOVE);
    // e2:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e2, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_O);
    // e2:SetProperty(EFFECT_FLAG_DELAY)
    effect::set_property(d, e2, EFFECT_FLAG_DELAY, 0);
    // e2:SetCode(EVENT_SPSUMMON_SUCCESS)
    effect::set_code(d, e2, EVENT_SPSUMMON_SUCCESS);
    // e2:SetCountLimit(1,id)
    effect::set_count_limit(d, e2, 1, ID, 0, 0);
    // e2:SetCondition(function(e) return e:GetHandler():IsSummonLocation(LOCATION_HAND) end)
    effect::set_condition(d, e2, Some(condition(sum_loc_cond)));
    // e2:SetTarget(s.rmtg)
    effect::set_target(d, e2, Some(cost_target(rmtg)));
    // e2:SetOperation(s.rmop)
    effect::set_operation(d, e2, Some(operation(rmop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
}
// function(e) return e:GetHandler():IsSummonLocation(LOCATION_HAND) end
fn sum_loc_cond(d: &mut Duel, a: &EffArgs) -> bool {
    let h = effect::get_handler(d, a.e).nil();
    card::is_summon_location(d, h, LOCATION_HAND)
}
// s.spcheck(sg,e,tp,mg)
fn spcheck(d: &mut Duel, sg: &CardSet, _e: Option<EffectId>, _tp: u8, _mg: &CardSet, _c: Option<CardId>) -> (bool, bool) {
    // return sg:CheckDifferentPropertyBinary(function(c) return c:GetType()&(TYPE_MONSTER|TYPE_SPELL|TYPE_TRAP) end)
    let res = group::check_different_property_binary(d, sg, &|d, c| (card::get_type(d, c) & (TYPE_MONSTER | TYPE_SPELL | TYPE_TRAP)) as u64);
    (res, false)
}
// s.spcon(e,c)
fn spcon(d: &mut Duel, a: Args) -> bool {
    // if c==nil then return true end
    let Some(c) = a.card(1) else {
        return true;
    };
    let e = a.e(0);
    // local tp=c:GetControler()
    let tp = card::get_controler(d, c);
    // local g=Duel.GetMatchingGroup(Card.IsAbleToRemoveAsCost,tp,LOCATION_GRAVE,0,nil)
    let g = duel::get_matching_group(d, Some(&|d, cc| card::is_able_to_remove_as_cost(d, cc, None)), tp, LOCATION_GRAVE, 0, NO_EX);
    // return Duel.GetLocationCount(tp,LOCATION_MZONE)>0 and aux.SelectUnselectGroup(g,e,tp,3,3,s.spcheck,0)
    duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 > 0
        && matches!(
            aux::select_unselect_group(d, &g, Some(e), tp, Some(3), Some(3), Some(&spcheck), 0, tp, None, None, None, false),
            Err(true)
        )
}
// s.sptg(e,tp,eg,ep,ev,re,r,rp,chk,c)
fn sptg(d: &mut Duel, a: &EffArgs) -> bool {
    // local g=Duel.GetMatchingGroup(Card.IsAbleToRemoveAsCost,tp,LOCATION_GRAVE,0,nil)
    let g = duel::get_matching_group(d, Some(&|d, cc| card::is_able_to_remove_as_cost(d, cc, None)), a.tp, LOCATION_GRAVE, 0, NO_EX);
    // local rg=aux.SelectUnselectGroup(g,e,tp,3,3,s.spcheck,1,tp,HINTMSG_REMOVE,nil,nil,true)
    let rg = aux::select_unselect_group(
        d,
        &g,
        Some(a.e),
        a.tp,
        Some(3),
        Some(3),
        Some(&spcheck),
        1,
        a.tp,
        Some(HINTMSG_REMOVE as u64),
        None,
        None,
        true,
    )
    .unwrap_or_default();
    // if #rg>0 then
    if !rg.is_empty() {
        // rg:KeepAlive()
        // e:SetLabelObject(rg)
        effect::set_label_object(d, a.e, Value::Group(rg));
        // return true
        return true;
    }
    // return false
    false
}
// s.spop(e,tp,eg,ep,ev,re,r,rp,c)
fn spop(d: &mut Duel, a: &EffArgs) {
    // local rg=e:GetLabelObject()
    // if not rg then return end
    let Some(rg) = effect::get_label_object(d, a.e).as_group().cloned() else {
        return;
    };
    // Duel.Remove(rg,POS_FACEUP,REASON_COST)
    duel::remove(d, rg, POS_FACEUP as u8, REASON_COST, None, None);
    // rg:DeleteGroup()
}
// s.rmcheck(sg,e,tp,mg)
fn rmcheck(d: &mut Duel, sg: &CardSet, _e: Option<EffectId>, _tp: u8, _mg: &CardSet, _c: Option<CardId>) -> (bool, bool) {
    // return sg:IsExists(Card.IsMonster,1,nil) and sg:IsExists(Card.IsSpellTrap,2,nil)
    let res = group::is_exists(d, sg, &|d, c| card::is_monster(d, c), 1, NO_EX) && group::is_exists(d, sg, &|d, c| card::is_spell_trap(d, c), 2, NO_EX);
    (res, false)
}
// s.rmtg(e,tp,eg,ep,ev,re,r,rp,chk)
fn rmtg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then
    if a.chk == Some(0) {
        // local g=Duel.GetMatchingGroup(Card.IsAbleToRemove,tp,0,LOCATION_ONFIELD|LOCATION_GRAVE,nil)
        let g = duel::get_matching_group(d, Some(&|d, cc| card::is_able_to_remove(d, cc, None, None, None)), a.tp, 0, LOCATION_ONFIELD | LOCATION_GRAVE, NO_EX);
        // return aux.SelectUnselectGroup(g,e,tp,3,3,s.rmcheck,0)
        return matches!(
            aux::select_unselect_group(d, &g, Some(a.e), a.tp, Some(3), Some(3), Some(&rmcheck), 0, a.tp, None, None, None, false),
            Err(true)
        );
    }
    // Duel.SetOperationInfo(0,CATEGORY_REMOVE,nil,3,1-tp,LOCATION_ONFIELD|LOCATION_GRAVE)
    duel::set_operation_info(d, 0, CATEGORY_REMOVE, None, 3, 1 - a.tp, (LOCATION_ONFIELD | LOCATION_GRAVE) as i32);
    true
}
// s.rmop(e,tp,eg,ep,ev,re,r,rp)
fn rmop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // local g=Duel.GetMatchingGroup(Card.IsAbleToRemove,tp,0,LOCATION_ONFIELD|LOCATION_GRAVE,nil)
    let g = duel::get_matching_group(d, Some(&|d, cc| card::is_able_to_remove(d, cc, None, None, None)), tp, 0, LOCATION_ONFIELD | LOCATION_GRAVE, NO_EX);
    // if #g<3 then return end
    if g.len() < 3 {
        return;
    }
    // local rg=aux.SelectUnselectGroup(g,e,tp,3,3,s.rmcheck,1,tp,HINTMSG_REMOVE)
    let rg = aux::select_unselect_group(d, &g, Some(a.e), tp, Some(3), Some(3), Some(&rmcheck), 1, tp, Some(HINTMSG_REMOVE as u64), None, None, false)
        .unwrap_or_default();
    // if #rg==3 then
    if rg.len() == 3 {
        // Duel.Remove(rg,POS_FACEUP,REASON_EFFECT)
        duel::remove(d, rg, POS_FACEUP as u8, REASON_EFFECT, None, None);
    }
}
