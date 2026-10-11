//! Linkbelt Wall Dragon
use super::prelude::*;

pub const ID: u32 = 63092423;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // c:EnableCounterPermit(0x44)
    card::enable_counter_permit(d, c, 0x44, None, None);
    // c:EnableReviveLimit()
    card::enable_revive_limit(d, c);
    // cannot special summon
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
    // e1:SetProperty(EFFECT_FLAG_CANNOT_DISABLE+EFFECT_FLAG_UNCOPYABLE)
    effect::set_property(d, e1, EFFECT_FLAG_CANNOT_DISABLE + EFFECT_FLAG_UNCOPYABLE, 0);
    // e1:SetCode(EFFECT_SPSUMMON_CONDITION)
    effect::set_code(d, e1, EFFECT_SPSUMMON_CONDITION);
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // special summon
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e2, aux::stringid(ID, 0));
    // e2:SetCategory(CATEGORY_SPECIAL_SUMMON+CATEGORY_COUNTER)
    effect::set_category(d, e2, CATEGORY_SPECIAL_SUMMON + CATEGORY_COUNTER);
    // e2:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e2, EFFECT_TYPE_FIELD + EFFECT_TYPE_TRIGGER_O);
    // e2:SetCode(EVENT_SPSUMMON_SUCCESS)
    effect::set_code(d, e2, EVENT_SPSUMMON_SUCCESS);
    // e2:SetRange(LOCATION_HAND)
    effect::set_range(d, e2, LOCATION_HAND);
    // e2:SetCondition(s.spcon)
    effect::set_condition(d, e2, Some(condition(spcon)));
    // e2:SetTarget(s.sptg)
    effect::set_target(d, e2, Some(cost_target(sptg)));
    // e2:SetOperation(s.spop)
    effect::set_operation(d, e2, Some(operation(spop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
    // cannot link summon
    // local e3=Effect.CreateEffect(c)
    let e3 = effect::create_effect(d, c);
    // e3:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e3, EFFECT_TYPE_FIELD);
    // e3:SetProperty(EFFECT_FLAG_PLAYER_TARGET)
    effect::set_property(d, e3, EFFECT_FLAG_PLAYER_TARGET, 0);
    // e3:SetCode(EFFECT_CANNOT_SPECIAL_SUMMON)
    effect::set_code(d, e3, EFFECT_CANNOT_SPECIAL_SUMMON);
    // e3:SetRange(LOCATION_MZONE)
    effect::set_range(d, e3, LOCATION_MZONE);
    // e3:SetTargetRange(1,1)
    effect::set_target_range(d, e3, 1, 1);
    // e3:SetTarget(s.splimit)
    effect::set_target(d, e3, Some(lua_fn(splimit)));
    // c:RegisterEffect(e3)
    card::register_effect(d, c, e3, false);
    // cannot attack
    // local e4=Effect.CreateEffect(c)
    let e4 = effect::create_effect(d, c);
    // e4:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e4, EFFECT_TYPE_FIELD);
    // e4:SetCode(EFFECT_CANNOT_ATTACK)
    effect::set_code(d, e4, EFFECT_CANNOT_ATTACK);
    // e4:SetRange(LOCATION_MZONE)
    effect::set_range(d, e4, LOCATION_MZONE);
    // e4:SetTargetRange(LOCATION_MZONE,LOCATION_MZONE)
    effect::set_target_range(d, e4, LOCATION_MZONE, LOCATION_MZONE);
    // e4:SetTarget(s.atktg)
    effect::set_target(d, e4, Some(effect_card_fn(|d, _e, c| !card::is_link_monster(d, c))));
    // c:RegisterEffect(e4)
    card::register_effect(d, c, e4, false);
    // counter
    // local e5=Effect.CreateEffect(c)
    let e5 = effect::create_effect(d, c);
    // e5:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e5, aux::stringid(ID, 1));
    // e5:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_TRIGGER_F)
    effect::set_type(d, e5, EFFECT_TYPE_FIELD + EFFECT_TYPE_TRIGGER_F);
    // e5:SetCode(EVENT_SPSUMMON_SUCCESS)
    effect::set_code(d, e5, EVENT_SPSUMMON_SUCCESS);
    // e5:SetRange(LOCATION_MZONE)
    effect::set_range(d, e5, LOCATION_MZONE);
    // e5:SetCondition(s.ctcon1)
    effect::set_condition(d, e5, Some(condition(ctcon1)));
    // e5:SetOperation(s.ctop1)
    effect::set_operation(d, e5, Some(operation(ctop1)));
    // c:RegisterEffect(e5)
    card::register_effect(d, c, e5, false);
    // counter
    // local e6=Effect.CreateEffect(c)
    let e6 = effect::create_effect(d, c);
    // e6:SetDescription(aux.Stringid(id,2))
    effect::set_description(d, e6, aux::stringid(ID, 2));
    // e6:SetCategory(CATEGORY_COUNTER)
    effect::set_category(d, e6, CATEGORY_COUNTER);
    // e6:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_TRIGGER_F)
    effect::set_type(d, e6, EFFECT_TYPE_FIELD + EFFECT_TYPE_TRIGGER_F);
    // e6:SetCode(EVENT_PHASE|PHASE_STANDBY)
    effect::set_code(d, e6, EVENT_PHASE | PHASE_STANDBY);
    // e6:SetCountLimit(1)
    effect::set_count_limit(d, e6, 1, 0, 0, 0);
    // e6:SetRange(LOCATION_MZONE)
    effect::set_range(d, e6, LOCATION_MZONE);
    // e6:SetOperation(s.ctop2)
    effect::set_operation(d, e6, Some(operation(ctop2)));
    // c:RegisterEffect(e6)
    card::register_effect(d, c, e6, false);
}
/// `s.cfilter(c,tp)`
fn cfilter(d: &mut Duel, c: CardId, tp: u8) -> bool {
    // return c:IsFaceup() and c:IsSummonPlayer(tp) and c:IsLinkSummoned()
    card::is_faceup(d, c) && card::is_summon_player(d, c, tp) && card::is_link_summoned(d, c)
}
/// `s.spcon(e,tp,eg,ep,ev,re,r,rp)`
fn spcon(d: &mut Duel, a: &EffArgs) -> bool {
    // return eg:IsExists(s.cfilter,1,nil,tp)
    group::is_exists(d, &a.eg.clone().nil(), &|d, c| cfilter(d, c, a.tp), 1, NO_EX)
}
/// `s.sptg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn sptg(d: &mut Duel, a: &EffArgs) -> bool {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if chk==0 then return Duel.GetLocationCount(tp,LOCATION_MZONE)>0
    //     and e:GetHandler():IsCanBeSpecialSummoned(e,0,tp,true,false)
    //     and Duel.IsCanAddCounter(tp,0x44,1,e:GetHandler()) end
    if a.chk == Some(0) {
        return duel::get_location_count(d, a.tp, LOCATION_MZONE as u8, None, None, None).0 > 0
            && card::is_can_be_special_summoned(d, c, a.e, 0, a.tp, true, false, None, None, None)
            && duel::is_can_add_counter(d, a.tp, Some((0x44, 1, c)));
    }
    // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,e:GetHandler(),1,0,0)
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, Some(CardSet::from_card(c)), 1, 0, 0);
    true
}
/// `s.spop(e,tp,eg,ep,ev,re,r,rp)`
fn spop(d: &mut Duel, a: &EffArgs) {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if c:IsRelateToEffect(e) and Duel.SpecialSummon(c,0,tp,tp,true,false,POS_FACEUP)~=0 then
    if card::is_relate_to_effect(d, c, a.e) && duel::special_summon(d, c, 0, a.tp, a.tp, true, false, POS_FACEUP as u8, None) != 0 {
        // c:AddCounter(0x44,2)
        card::add_counter(d, c, 0x44, 2, false);
        // c:CompleteProcedure()
        card::complete_procedure(d, c);
    }
}
/// `s.splimit(e,c,tp,sumtp,sumpos)`
fn splimit(d: &mut Duel, a: Args) -> bool {
    // return c:GetLink()>e:GetHandler():GetCounter(0x44) and (sumtp&SUMMON_TYPE_LINK)==SUMMON_TYPE_LINK
    let c = a.c(1);
    let h = effect::get_handler(d, a.e(0)).nil();
    let sumtp = a.int(3);
    let link = card::get_link(d, c) as i64;
    let counter = card::get_counter(d, h, 0x44) as i64;
    link > counter && (sumtp & SUMMON_TYPE_LINK as i64) == SUMMON_TYPE_LINK as i64
}
/// `s.ctcon1(e,tp,eg,ep,ev,re,r,rp)`
fn ctcon1(d: &mut Duel, a: &EffArgs) -> bool {
    // return eg:IsExists(Card.IsSummonType,1,nil,SUMMON_TYPE_LINK)
    group::is_exists(d, &a.eg.clone().nil(), &|d, c| card::is_summon_type(d, c, &[SUMMON_TYPE_LINK]), 1, NO_EX)
}
/// `s.ctop1(e,tp,eg,ep,ev,re,r,rp)`
fn ctop1(d: &mut Duel, a: &EffArgs) {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if c:IsRelateToEffect(e) then
    if card::is_relate_to_effect(d, c, a.e) {
        // local ct=math.min(2,c:GetCounter(0x44))
        let ct = std::cmp::min(2, card::get_counter(d, c, 0x44));
        // c:RemoveCounter(tp,0x44,ct,REASON_EFFECT)
        card::remove_counter(d, c, a.tp, 0x44, ct as u16, REASON_EFFECT);
    }
}
/// `s.ctop2(e,tp,eg,ep,ev,re,r,rp)`
fn ctop2(d: &mut Duel, a: &EffArgs) {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if c:IsRelateToEffect(e) then
    if card::is_relate_to_effect(d, c, a.e) {
        // c:AddCounter(0x44,1)
        card::add_counter(d, c, 0x44, 1, false);
    }
}
