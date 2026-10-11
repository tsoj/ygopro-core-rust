//! Snowdust Giant
use super::prelude::*;

pub const ID: u32 = 73659078;
pub const INFO: ScriptInfo = ScriptInfo { counter_place_list: &[0x1015], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // c:EnableReviveLimit()
    card::enable_revive_limit(d, c);
    // Xyz.AddProcedure(c,aux.FilterBoolFunctionEx(Card.IsAttribute,ATTRIBUTE_WATER),4,2)
    xyz::add_procedure(
        d,
        c,
        Some(mat_filter(|d, c, sc, st, tp| card::is_attribute_ctx(d, c, ATTRIBUTE_WATER, sc, st, Some(tp)))),
        Some(4),
        2,
        None,
        None,
        None,
        None,
        false,
        None,
    );
    // Place Ice Counters on face-up monsters on the field
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_COUNTER)
    effect::set_category(d, e1, CATEGORY_COUNTER);
    // e1:SetType(EFFECT_TYPE_IGNITION)
    effect::set_type(d, e1, EFFECT_TYPE_IGNITION);
    // e1:SetRange(LOCATION_MZONE)
    effect::set_range(d, e1, LOCATION_MZONE);
    // e1:SetCountLimit(1)
    effect::set_count_limit(d, e1, 1, 0, 0, 0);
    // e1:SetCost(Cost.DetachFromSelf(1,1,nil))
    effect::set_cost(d, e1, Some(cost::detach_from_self(cost::Count::N(1), Some(cost::Count::N(1)), None)));
    // e1:SetTarget(s.countertg)
    effect::set_target(d, e1, Some(cost_target(countertg)));
    // e1:SetOperation(s.counterop)
    effect::set_operation(d, e1, Some(operation(counterop)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // Non-WATER monsters lose 200 ATK for each Ice Counter on the field
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e2, EFFECT_TYPE_FIELD);
    // e2:SetCode(EFFECT_UPDATE_ATTACK)
    effect::set_code(d, e2, EFFECT_UPDATE_ATTACK);
    // e2:SetRange(LOCATION_MZONE)
    effect::set_range(d, e2, LOCATION_MZONE);
    // e2:SetTargetRange(LOCATION_MZONE,LOCATION_MZONE)
    effect::set_target_range(d, e2, LOCATION_MZONE, LOCATION_MZONE);
    // e2:SetTarget(function(_,c) return c:IsAttributeExcept(ATTRIBUTE_WATER) end)
    effect::set_target(d, e2, Some(effect_card_fn(|d, _e, c| !card::is_attribute_except(d, c, ATTRIBUTE_WATER, None, 0, None))));
    // e2:SetValue(function() return Duel.GetCounter(0,1,1,0x1015)*-200 end)
    effect::set_value_fn(d, e2, lua_fn(counter_value));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
}
/// `function() return Duel.GetCounter(0,1,1,0x1015)*-200 end`
fn counter_value(d: &mut Duel, _a: Args) -> i64 {
    duel::get_counter(d, 0, 1, 1, 0x1015) as i64 * -200
}
/// `s.cfilter(c)`
fn cfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsAttribute(ATTRIBUTE_WATER) and not c:IsPublic()
    card::is_attribute(d, c, ATTRIBUTE_WATER) && !card::is_public(d, c)
}
/// `s.countertg(e,tp,...,chk)`
fn countertg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return Duel.IsExistingMatchingCard(s.cfilter,tp,LOCATION_HAND,0,1,nil)
    //     and Duel.IsExistingMatchingCard(Card.IsFaceup,tp,LOCATION_MZONE,LOCATION_MZONE,1,nil) end
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, Some(&cfilter), a.tp, LOCATION_HAND, 0, 1, NO_EX)
            && duel::is_existing_matching_card(d, Some(&|d, c| card::is_faceup(d, c)), a.tp, LOCATION_MZONE, LOCATION_MZONE, 1, NO_EX);
    }
    true
}
/// `s.counterop(e,tp,...)`
fn counterop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // local g=Duel.GetMatchingGroup(Card.IsFaceup,tp,LOCATION_MZONE,LOCATION_MZONE,nil)
    let g = duel::get_matching_group(d, Some(&|d, c| card::is_faceup(d, c)), tp, LOCATION_MZONE, LOCATION_MZONE, NO_EX);
    // if #g==0 then return end
    if g.is_empty() {
        return;
    }
    // local hg=Duel.GetMatchingGroup(s.cfilter,tp,LOCATION_HAND,0,nil)
    let hg = duel::get_matching_group(d, Some(&cfilter), tp, LOCATION_HAND, 0, NO_EX);
    // if #hg==0 then return end
    if hg.is_empty() {
        return;
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_CONFIRM)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_CONFIRM as u64);
    // local rg=hg:Select(tp,1,#hg,nil)
    let rg = group::select(d, &hg, tp, 1, hg.len() as u16, false, NO_EX).nil();
    // Duel.ConfirmCards(1-tp,rg)
    duel::confirm_cards(d, 1 - tp, rg.clone());
    // Duel.ShuffleHand(tp)
    duel::shuffle_hand(d, tp);
    // local ct=#rg
    let ct = rg.len();
    // local tc=nil
    // for i=1,ct do
    for _ in 0..ct {
        // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_COUNTER)
        duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_COUNTER as u64);
        // tc=g:Select(tp,1,1,nil):GetFirst()
        let sel = group::select(d, &g, tp, 1, 1, false, NO_EX).nil();
        let tc = group::get_first(d, &sel).nil();
        // tc:AddCounter(0x1015,1)
        card::add_counter(d, tc, 0x1015, 1, false);
    }
}
