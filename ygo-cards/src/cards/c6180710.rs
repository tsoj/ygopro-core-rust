//! Dream Shark
use super::prelude::*;

pub const ID: u32 = 6180710;

pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_HEROIC], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // The first time it would be destroyed by battle, it is not
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
    // e1:SetProperty(EFFECT_FLAG_SINGLE_RANGE)
    effect::set_property(d, e1, EFFECT_FLAG_SINGLE_RANGE, 0);
    // e1:SetRange(LOCATION_MZONE)
    effect::set_range(d, e1, LOCATION_MZONE);
    // e1:SetCode(EFFECT_INDESTRUCTABLE_COUNT)
    effect::set_code(d, e1, EFFECT_INDESTRUCTABLE_COUNT);
    // e1:SetCountLimit(1)
    effect::set_count_limit(d, e1, 1, 0, 0, 0);
    // e1:SetValue(s.valcon)
    effect::set_value_fn(d, e1, lua_fn(valcon));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // Special summon itself from the hand
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e2, aux::stringid(ID, 0));
    // e2:SetCategory(CATEGORY_SPECIAL_SUMMON)
    effect::set_category(d, e2, CATEGORY_SPECIAL_SUMMON);
    // e2:SetType(EFFECT_TYPE_IGNITION)
    effect::set_type(d, e2, EFFECT_TYPE_IGNITION);
    // e2:SetRange(LOCATION_HAND)
    effect::set_range(d, e2, LOCATION_HAND);
    // e2:SetCountLimit(1,{id,0})
    effect::set_count_limit(d, e2, 1, ID, 0, 0);
    // e2:SetCondition(s.spcon)
    effect::set_condition(d, e2, Some(condition(spcon)));
    // e2:SetTarget(s.sptg)
    effect::set_target(d, e2, Some(cost_target(sptg)));
    // e2:SetOperation(s.spop)
    effect::set_operation(d, e2, Some(operation(spop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
    // Special summon itself from the GY if an effect would inflict damage
    // local e4=Effect.CreateEffect(c)
    let e4 = effect::create_effect(d, c);
    // e4:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e4, aux::stringid(ID, 1));
    // e4:SetCategory(CATEGORY_SPECIAL_SUMMON)
    effect::set_category(d, e4, CATEGORY_SPECIAL_SUMMON);
    // e4:SetType(EFFECT_TYPE_QUICK_O)
    effect::set_type(d, e4, EFFECT_TYPE_QUICK_O);
    // e4:SetCode(EVENT_CHAINING)
    effect::set_code(d, e4, EVENT_CHAINING);
    // e4:SetRange(LOCATION_GRAVE)
    effect::set_range(d, e4, LOCATION_GRAVE);
    // e4:SetCountLimit(1,{id,1})
    effect::set_count_limit(d, e4, 1, ID, 1, 0);
    // e4:SetCondition(aux.damcon1)
    effect::set_condition(d, e4, Some(aux::damcon1()));
    // e4:SetTarget(s.damtg)
    effect::set_target(d, e4, Some(cost_target(damtg)));
    // e4:SetOperation(s.damop)
    effect::set_operation(d, e4, Some(operation(damop)));
    // c:RegisterEffect(e4)
    card::register_effect(d, c, e4, false);
}
/// `s.valcon(e,re,r,rp)`
fn valcon(_d: &mut Duel, a: Args) -> bool {
    // return (r&REASON_BATTLE)~=0
    (a.int(2) as u32 & REASON_BATTLE) != 0
}
/// `s.cfilter(c)`
fn cfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsFacedown() or not c:IsAttribute(ATTRIBUTE_WATER)
    card::is_facedown(d, c) || !card::is_attribute(d, c, ATTRIBUTE_WATER)
}
/// `s.spcon(e,tp,eg,ep,ev,re,r,rp)`
fn spcon(d: &mut Duel, a: &EffArgs) -> bool {
    // return not Duel.IsExistingMatchingCard(s.cfilter,tp,LOCATION_MZONE,0,1,nil)
    !duel::is_existing_matching_card(d, Some(&cfilter), a.tp, LOCATION_MZONE, 0, 1, NO_EX)
}
/// `s.sptg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn sptg(d: &mut Duel, a: &EffArgs) -> bool {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if chk==0 then return Duel.GetLocationCount(tp,LOCATION_MZONE)>0 and c:IsCanBeSpecialSummoned(e,0,tp,false,false) end
    if a.chk == Some(0) {
        return duel::get_location_count(d, a.tp, LOCATION_MZONE as u8, None, None, None).0 > 0
            && card::is_can_be_special_summoned(d, c, a.e, 0, a.tp, false, false, None, None, None);
    }
    // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,c,1,0,LOCATION_HAND)
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, Some(CardSet::from_card(c)), 1, 0, LOCATION_HAND as i32);
    true
}
/// `s.spop(e,tp,eg,ep,ev,re,r,rp)`
fn spop(d: &mut Duel, a: &EffArgs) {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if c:IsRelateToEffect(e) then
    if card::is_relate_to_effect(d, c, a.e) {
        // Duel.SpecialSummon(c,0,tp,tp,false,false,POS_FACEUP)
        duel::special_summon(d, c, 0, a.tp, a.tp, false, false, POS_FACEUP as u8, None);
    }
}
/// `s.damtg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn damtg(d: &mut Duel, a: &EffArgs) -> bool {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if chk==0 then return Duel.GetLocationCount(tp,LOCATION_MZONE)>0 and c:IsCanBeSpecialSummoned(e,0,tp,false,false) end
    if a.chk == Some(0) {
        return duel::get_location_count(d, a.tp, LOCATION_MZONE as u8, None, None, None).0 > 0
            && card::is_can_be_special_summoned(d, c, a.e, 0, a.tp, false, false, None, None, None);
    }
    // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,c,1,0,LOCATION_GRAVE)
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, Some(CardSet::from_card(c)), 1, 0, LOCATION_GRAVE as i32);
    true
}
/// `s.damop(e,tp,eg,ep,ev,re,r,rp)`
fn damop(d: &mut Duel, a: &EffArgs) {
    let e = a.e;
    let tp = a.tp;
    // local c=e:GetHandler()
    let c = effect::get_handler(d, e).nil();
    // if c:IsRelateToEffect(e) and Duel.SpecialSummonStep(c,0,tp,tp,false,false,POS_FACEUP) then
    if card::is_relate_to_effect(d, c, e) && duel::special_summon_step(d, c, 0, tp, tp, false, false, POS_FACEUP as u8, None) {
        // Decrease DEF by 1000
        // local e1=Effect.CreateEffect(c)
        let e1 = effect::create_effect(d, c);
        // e1:SetType(EFFECT_TYPE_SINGLE)
        effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
        // e1:SetProperty(EFFECT_FLAG_CANNOT_DISABLE)
        effect::set_property(d, e1, EFFECT_FLAG_CANNOT_DISABLE, 0);
        // e1:SetCode(EFFECT_UPDATE_DEFENSE)
        effect::set_code(d, e1, EFFECT_UPDATE_DEFENSE);
        // e1:SetReset(RESET_EVENT|RESETS_STANDARD)
        effect::set_reset(d, e1, RESET_EVENT | RESETS_STANDARD, 1);
        // e1:SetValue(-1000)
        effect::set_value(d, e1, -1000);
        // c:RegisterEffect(e1,true)
        card::register_effect(d, c, e1, true);
        // Banish it if it leaves the field
        // local e2=Effect.CreateEffect(c)
        let e2 = effect::create_effect(d, c);
        // e2:SetDescription(3300)
        effect::set_description(d, e2, 3300);
        // e2:SetType(EFFECT_TYPE_SINGLE)
        effect::set_type(d, e2, EFFECT_TYPE_SINGLE);
        // e2:SetProperty(EFFECT_FLAG_CANNOT_DISABLE+EFFECT_FLAG_CLIENT_HINT)
        effect::set_property(d, e2, EFFECT_FLAG_CANNOT_DISABLE + EFFECT_FLAG_CLIENT_HINT, 0);
        // e2:SetCode(EFFECT_LEAVE_FIELD_REDIRECT)
        effect::set_code(d, e2, EFFECT_LEAVE_FIELD_REDIRECT);
        // e2:SetValue(LOCATION_REMOVED)
        effect::set_value(d, e2, LOCATION_REMOVED as i64);
        // e2:SetReset(RESET_EVENT|RESETS_REDIRECT)
        effect::set_reset(d, e2, RESET_EVENT | RESETS_REDIRECT, 1);
        // c:RegisterEffect(e2,true)
        card::register_effect(d, c, e2, true);
        // Take no damage from the activated effect
        // local cid=Duel.GetChainInfo(ev,CHAININFO_CHAIN_ID)
        let cid = duel::get_chain_info(d, a.ev as u8, &[CHAININFO_CHAIN_ID])[0].int();
        // local e3=Effect.CreateEffect(c)
        let e3 = effect::create_effect(d, c);
        // e3:SetType(EFFECT_TYPE_FIELD)
        effect::set_type(d, e3, EFFECT_TYPE_FIELD);
        // e3:SetCode(EFFECT_CHANGE_DAMAGE)
        effect::set_code(d, e3, EFFECT_CHANGE_DAMAGE);
        // e3:SetProperty(EFFECT_FLAG_PLAYER_TARGET)
        effect::set_property(d, e3, EFFECT_FLAG_PLAYER_TARGET, 0);
        // e3:SetTargetRange(1,0)
        effect::set_target_range(d, e3, 1, 0);
        // e3:SetLabel(cid)
        effect::set_label(d, e3, &[cid]);
        // e3:SetValue(s.damval)
        effect::set_value_fn(d, e3, lua_fn(damval));
        // e3:SetReset(RESET_CHAIN)
        effect::set_reset(d, e3, RESET_CHAIN, 1);
        // Duel.RegisterEffect(e3,tp)
        duel::register_effect(d, e3, tp);
    }
    // Duel.SpecialSummonComplete()
    duel::special_summon_complete(d);
}
/// `s.damval(e,re,val,r,rp,rc)`
fn damval(d: &mut Duel, a: Args) -> i64 {
    let e = a.e(0);
    let val = a.int(2);
    let r = a.int(3) as u32;
    // local cc=Duel.GetCurrentChain()
    // if cc==0 or (r&REASON_EFFECT)==0 then return val end
    if duel::get_current_chain(d, false) == 0 || (r & REASON_EFFECT) == 0 {
        return val;
    }
    // local cid=Duel.GetChainInfo(0,CHAININFO_CHAIN_ID)
    let cid = duel::get_chain_info(d, 0, &[CHAININFO_CHAIN_ID])[0].int();
    // if cid~=e:GetLabel() then return val end
    if cid != effect::get_label(d, e) {
        return val;
    }
    // return 0
    0
}
