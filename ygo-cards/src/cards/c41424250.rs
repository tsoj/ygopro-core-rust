//! Chaos End Ruler - Ruler of the Beginning and the End
use super::prelude::*;

pub const ID: u32 = 41424250;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // c:EnableReviveLimit()
    card::enable_revive_limit(d, c);
    // c:AddMustBeSpecialSummoned()
    card::add_must_be_special_summoned(d, c);
    // Must be Special Summoned (from your hand) by banishing 1 LIGHT Warrior and 1 DARK Fiend monster from your GY
    // local e0=Effect.CreateEffect(c)
    let e0 = effect::create_effect(d, c);
    // e0:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e0, aux::stringid(ID, 0));
    // e0:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e0, EFFECT_TYPE_FIELD);
    // e0:SetProperty(EFFECT_FLAG_CANNOT_DISABLE+EFFECT_FLAG_UNCOPYABLE)
    effect::set_property(d, e0, EFFECT_FLAG_CANNOT_DISABLE + EFFECT_FLAG_UNCOPYABLE, 0);
    // e0:SetCode(EFFECT_SPSUMMON_PROC)
    effect::set_code(d, e0, EFFECT_SPSUMMON_PROC);
    // e0:SetRange(LOCATION_HAND)
    effect::set_range(d, e0, LOCATION_HAND);
    // e0:SetCondition(s.spcon)
    effect::set_condition(d, e0, Some(lua_fn(spcon)));
    // e0:SetTarget(s.sptg)
    effect::set_target(d, e0, Some(cost_target(sptg)));
    // e0:SetOperation(s.spop)
    effect::set_operation(d, e0, Some(operation(spop)));
    // e0:SetValue(9300)
    effect::set_value(d, e0, 9300);
    // c:RegisterEffect(e0)
    card::register_effect(d, c, e0, false);
    // This card's Special Summon cannot be negated
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
    // e1:SetProperty(EFFECT_FLAG_CANNOT_DISABLE+EFFECT_FLAG_UNCOPYABLE)
    effect::set_property(d, e1, EFFECT_FLAG_CANNOT_DISABLE + EFFECT_FLAG_UNCOPYABLE, 0);
    // e1:SetCode(EFFECT_CANNOT_DISABLE_SPSUMMON)
    effect::set_code(d, e1, EFFECT_CANNOT_DISABLE_SPSUMMON);
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // Neither player can activate cards or effects when this card is Special Summoned
    // local e2a=Effect.CreateEffect(c)
    let e2a = effect::create_effect(d, c);
    // e2a:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_CONTINUOUS)
    effect::set_type(d, e2a, EFFECT_TYPE_SINGLE + EFFECT_TYPE_CONTINUOUS);
    // e2a:SetCode(EVENT_SPSUMMON_SUCCESS)
    effect::set_code(d, e2a, EVENT_SPSUMMON_SUCCESS);
    // e2a:SetOperation(s.limop)
    effect::set_operation(d, e2a, Some(operation(limop)));
    // c:RegisterEffect(e2a)
    card::register_effect(d, c, e2a, false);
    // local e2b=Effect.CreateEffect(c)
    let e2b = effect::create_effect(d, c);
    // e2b:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_CONTINUOUS)
    effect::set_type(d, e2b, EFFECT_TYPE_FIELD + EFFECT_TYPE_CONTINUOUS);
    // e2b:SetCode(EVENT_CHAIN_END)
    effect::set_code(d, e2b, EVENT_CHAIN_END);
    // e2b:SetRange(LOCATION_MZONE)
    effect::set_range(d, e2b, LOCATION_MZONE);
    // e2b:SetCondition(function(e) return e:GetHandler():HasFlagEffect(id) end)
    effect::set_condition(d, e2b, Some(condition(|d, a| {
        let h = effect::get_handler(d, a.e).nil();
        card::has_flag_effect(d, h, ID, None)
    })));
    // e2b:SetOperation(function(e) e:GetHandler():ResetFlagEffect(id) Duel.SetChainLimitTillChainEnd(aux.FALSE) end)
    effect::set_operation(d, e2b, Some(operation(e2b_op)));
    // c:RegisterEffect(e2b)
    card::register_effect(d, c, e2b, false);
    // If you control this card that was Summoned its own way: You can pay 1000 LP; banish as many cards from your opponent's field and GY as possible, then inflict 500 damage to your opponent for each card banished by this effect
    // local e3=Effect.CreateEffect(c)
    let e3 = effect::create_effect(d, c);
    // e3:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e3, aux::stringid(ID, 1));
    // e3:SetCategory(CATEGORY_REMOVE+CATEGORY_DAMAGE)
    effect::set_category(d, e3, CATEGORY_REMOVE + CATEGORY_DAMAGE);
    // e3:SetType(EFFECT_TYPE_IGNITION)
    effect::set_type(d, e3, EFFECT_TYPE_IGNITION);
    // e3:SetRange(LOCATION_MZONE)
    effect::set_range(d, e3, LOCATION_MZONE);
    // e3:SetCondition(function(e) return e:GetHandler():IsSummonType(SUMMON_TYPE_SPECIAL+9300) end)
    effect::set_condition(d, e3, Some(condition(|d, a| {
        let h = effect::get_handler(d, a.e).nil();
        card::is_summon_type(d, h, &[SUMMON_TYPE_SPECIAL + 9300])
    })));
    // e3:SetCost(Cost.PayLP(1000))
    effect::set_cost(d, e3, Some(cost::pay_lp(cost::PayLp::Amount(1000))));
    // e3:SetTarget(s.bandamtg)
    effect::set_target(d, e3, Some(cost_target(bandamtg)));
    // e3:SetOperation(s.bandamop)
    effect::set_operation(d, e3, Some(operation(bandamop)));
    // e3:SetCountLimit(1,id)
    effect::set_count_limit(d, e3, 1, ID, 0, 0);
    // c:RegisterEffect(e3)
    card::register_effect(d, c, e3, false);
}
/// `s.spcostfilter(c)`
fn spcostfilter(d: &mut Duel, c: CardId) -> bool {
    // return ((c:IsAttribute(ATTRIBUTE_LIGHT) and c:IsRace(RACE_WARRIOR)) or (c:IsAttribute(ATTRIBUTE_DARK) and c:IsRace(RACE_FIEND)))
    //     and c:IsAbleToRemoveAsCost() and aux.SpElimFilter(c,true)
    ((card::is_attribute(d, c, ATTRIBUTE_LIGHT) && card::is_race(d, c, RACE_WARRIOR as u64))
        || (card::is_attribute(d, c, ATTRIBUTE_DARK) && card::is_race(d, c, RACE_FIEND as u64)))
        && card::is_able_to_remove_as_cost(d, c, None)
        && aux::sp_elim_filter(d, c, true, false)
}
/// `s.propertyfilter(c,attribute,race)`
fn propertyfilter(d: &mut Duel, c: CardId, attribute: u32, race: u64) -> bool {
    // return c:IsAttribute(attribute) and c:IsRace(race)
    card::is_attribute(d, c, attribute) && card::is_race(d, c, race)
}
/// `s.rescon(sg,e,tp,mg)`
fn rescon(d: &mut Duel, sg: &CardSet, _e: Option<EffectId>, tp: u8, _mg: &CardSet, _c: Option<CardId>) -> (bool, bool) {
    // return Duel.GetMZoneCount(tp,sg)>0 and sg:IsExists(s.propertyfilter,1,nil,ATTRIBUTE_LIGHT,RACE_WARRIOR)
    //     and sg:IsExists(s.propertyfilter,1,nil,ATTRIBUTE_DARK,RACE_FIEND)
    let ok = duel::get_m_zone_count(d, tp, sg.into(), None, None, None).0 > 0
        && group::is_exists(d, sg, &|d, c| propertyfilter(d, c, ATTRIBUTE_LIGHT, RACE_WARRIOR as u64), 1, NO_EX)
        && group::is_exists(d, sg, &|d, c| propertyfilter(d, c, ATTRIBUTE_DARK, RACE_FIEND as u64), 1, NO_EX);
    (ok, false)
}
/// `s.spcon(e,c)`
fn spcon(d: &mut Duel, a: Args) -> bool {
    let e = a.e(0);
    // if c==nil then return true end
    if a.card(1).is_none() {
        return true;
    }
    // local tp=e:GetHandlerPlayer()
    let tp = effect::get_handler_player(d, e);
    // local g=Duel.GetMatchingGroup(s.spcostfilter,tp,LOCATION_MZONE|LOCATION_GRAVE,0,nil)
    let g = duel::get_matching_group(d, Some(&spcostfilter), tp, LOCATION_MZONE | LOCATION_GRAVE, 0, NO_EX);
    // return #g>=2 and Duel.GetMZoneCount(tp,g)>0 and aux.SelectUnselectGroup(g,e,tp,2,2,s.rescon,0)
    let rs: aux::Rescon = &rescon;
    g.len() >= 2
        && duel::get_m_zone_count(d, tp, (&g).into(), None, None, None).0 > 0
        && matches!(aux::select_unselect_group(d, &g, Some(e), tp, Some(2), Some(2), Some(rs), 0, tp, None, None, None, false), Err(true))
}
/// `s.sptg(e,tp,eg,ep,ev,re,r,rp,chk,c)`
fn sptg(d: &mut Duel, a: &EffArgs) -> bool {
    // local g=Duel.GetMatchingGroup(s.spcostfilter,tp,LOCATION_MZONE|LOCATION_GRAVE,0,nil)
    let g = duel::get_matching_group(d, Some(&spcostfilter), a.tp, LOCATION_MZONE | LOCATION_GRAVE, 0, NO_EX);
    // local sg=aux.SelectUnselectGroup(g,e,tp,2,2,s.rescon,1,tp,HINTMSG_REMOVE,nil,nil,true)
    let rs: aux::Rescon = &rescon;
    let sg = aux::select_unselect_group(d, &g, Some(a.e), a.tp, Some(2), Some(2), Some(rs), 1, a.tp, Some(HINTMSG_REMOVE as u64), None, None, true).unwrap_or_default();
    // if #sg>0 then
    if !sg.is_empty() {
        // e:SetLabelObject(sg)
        effect::set_label_object(d, a.e, Value::Group(sg));
        // return true
        return true;
    }
    // return false
    false
}
/// `s.spop(e,tp,eg,ep,ev,re,r,rp,c)`
fn spop(d: &mut Duel, a: &EffArgs) {
    // local sg=e:GetLabelObject()
    let sg = effect::get_label_object(d, a.e).as_group().cloned();
    // if sg then
    if let Some(sg) = sg {
        // Duel.Remove(sg,POS_FACEUP,REASON_COST)
        duel::remove(d, sg, POS_FACEUP as u8, REASON_COST, None, None);
    }
}
/// `s.limop(e,tp,eg,ep,ev,re,r,rp)`
fn limop(d: &mut Duel, a: &EffArgs) {
    // local current_chain=Duel.GetCurrentChain()
    let current_chain = duel::get_current_chain(d, false);
    // if current_chain==0 then
    if current_chain == 0 {
        // Duel.SetChainLimitTillChainEnd(aux.FALSE)
        duel::set_chain_limit_till_chain_end(d, aux_false());
    // elseif current_chain==1 then
    } else if current_chain == 1 {
        // e:GetHandler():RegisterFlagEffect(id,RESETS_STANDARD_PHASE_END,0,1)
        let h = effect::get_handler(d, a.e).nil();
        card::register_flag_effect(d, h, ID, RESETS_STANDARD_PHASE_END, 0, 1, 0, 0);
    }
}
/// `function(e) e:GetHandler():ResetFlagEffect(id) Duel.SetChainLimitTillChainEnd(aux.FALSE) end` (e2b)
fn e2b_op(d: &mut Duel, a: &EffArgs) {
    let h = effect::get_handler(d, a.e).nil();
    card::reset_flag_effect(d, h, ID);
    duel::set_chain_limit_till_chain_end(d, aux_false());
}
/// `s.bandamtg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn bandamtg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return Duel.IsExistingMatchingCard(Card.IsAbleToRemove,tp,0,LOCATION_ONFIELD|LOCATION_GRAVE,1,nil) end
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(
            d,
            Some(&|d, c| card::is_able_to_remove(d, c, None, None, None)),
            a.tp,
            0,
            LOCATION_ONFIELD | LOCATION_GRAVE,
            1,
            NO_EX,
        );
    }
    // local g=Duel.GetMatchingGroup(Card.IsAbleToRemove,tp,0,LOCATION_ONFIELD|LOCATION_GRAVE,nil)
    let g = duel::get_matching_group(d, Some(&|d, c| card::is_able_to_remove(d, c, None, None, None)), a.tp, 0, LOCATION_ONFIELD | LOCATION_GRAVE, NO_EX);
    // Duel.SetOperationInfo(0,CATEGORY_REMOVE,g,#g,tp,LOCATION_ONFIELD|LOCATION_GRAVE)
    let n = g.len() as u8;
    duel::set_operation_info(d, 0, CATEGORY_REMOVE, Some(g.clone()), n, a.tp, (LOCATION_ONFIELD | LOCATION_GRAVE) as i32);
    // Duel.SetOperationInfo(0,CATEGORY_DAMAGE,nil,0,1-tp,#g*500)
    duel::set_operation_info(d, 0, CATEGORY_DAMAGE, None, 0, 1 - a.tp, (g.len() * 500) as i32);
    true
}
/// `s.damfilter(c)`
fn damfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsLocation(LOCATION_REMOVED) and not c:IsReason(REASON_REDIRECT)
    card::is_location(d, c, LOCATION_REMOVED) && !card::is_reason(d, c, REASON_REDIRECT)
}
/// `s.bandamop(e,tp,eg,ep,ev,re,r,rp)`
fn bandamop(d: &mut Duel, a: &EffArgs) {
    // local g=Duel.GetMatchingGroup(Card.IsAbleToRemove,tp,0,LOCATION_ONFIELD|LOCATION_GRAVE,nil)
    let g = duel::get_matching_group(d, Some(&|d, c| card::is_able_to_remove(d, c, None, None, None)), a.tp, 0, LOCATION_ONFIELD | LOCATION_GRAVE, NO_EX);
    // if Duel.Remove(g,POS_FACEUP,REASON_EFFECT)>0 then
    if duel::remove(d, g, POS_FACEUP as u8, REASON_EFFECT, None, None) > 0 {
        // local damage=500*Duel.GetOperatedGroup():FilterCount(s.damfilter,nil)
        let operated = duel::get_operated_group(d);
        let damage = 500 * group::filter_count(d, &operated, &damfilter, NO_EX) as i64;
        // if damage>0 then
        if damage > 0 {
            // Duel.Damage(1-tp,damage,REASON_EFFECT)
            duel::damage(d, 1 - a.tp, damage, REASON_EFFECT, false, None);
        }
    }
}
