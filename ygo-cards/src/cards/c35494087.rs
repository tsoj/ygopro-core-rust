//! Speedroid Skull Marbles
//! SRビードロ・ドクロ
use super::prelude::*;

pub const ID: u32 = 35494087;
pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_SPEEDROID], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // special summon
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_SPECIAL_SUMMON)
    effect::set_category(d, e1, CATEGORY_SPECIAL_SUMMON);
    // e1:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e1, EFFECT_TYPE_FIELD + EFFECT_TYPE_TRIGGER_O);
    // e1:SetCode(EVENT_PHASE|PHASE_STANDBY)
    effect::set_code(d, e1, EVENT_PHASE | PHASE_STANDBY);
    // e1:SetRange(LOCATION_HAND)
    effect::set_range(d, e1, LOCATION_HAND);
    // e1:SetCountLimit(1)
    effect::set_count_limit(d, e1, 1, 0, 0, 0);
    // e1:SetCondition(s.spcon)
    effect::set_condition(d, e1, Some(condition(spcon)));
    // e1:SetTarget(s.sptg)
    effect::set_target(d, e1, Some(cost_target(sptg)));
    // e1:SetOperation(s.spop)
    effect::set_operation(d, e1, Some(operation(spop)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // indes
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e2, EFFECT_TYPE_SINGLE);
    // e2:SetCode(EFFECT_INDESTRUCTABLE_BATTLE)
    effect::set_code(d, e2, EFFECT_INDESTRUCTABLE_BATTLE);
    // e2:SetValue(s.indval)
    effect::set_value_fn(d, e2, effect_card_fn(indval));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
    // battle damage
    // local e3=Effect.CreateEffect(c)
    let e3 = effect::create_effect(d, c);
    // e3:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e3, EFFECT_TYPE_SINGLE);
    // e3:SetCode(EFFECT_REFLECT_BATTLE_DAMAGE)
    effect::set_code(d, e3, EFFECT_REFLECT_BATTLE_DAMAGE);
    // e3:SetValue(s.refval)
    effect::set_value_fn(d, e3, effect_card_fn(refval));
    // c:RegisterEffect(e3)
    card::register_effect(d, c, e3, false);
    // self destroy
    // local e4=Effect.CreateEffect(c)
    let e4 = effect::create_effect(d, c);
    // e4:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e4, EFFECT_TYPE_SINGLE);
    // e4:SetProperty(EFFECT_FLAG_SINGLE_RANGE)
    effect::set_property(d, e4, EFFECT_FLAG_SINGLE_RANGE, 0);
    // e4:SetCode(EFFECT_SELF_DESTROY)
    effect::set_code(d, e4, EFFECT_SELF_DESTROY);
    // e4:SetRange(LOCATION_MZONE)
    effect::set_range(d, e4, LOCATION_MZONE);
    // e4:SetCondition(s.sdcon)
    effect::set_condition(d, e4, Some(lua_fn(sdcon)));
    // c:RegisterEffect(e4)
    card::register_effect(d, c, e4, false);
}
/// `s.spcon(e,tp,eg,ep,ev,re,r,rp)`
fn spcon(d: &mut Duel, a: &EffArgs) -> bool {
    // return Duel.IsExistingMatchingCard(aux.FilterEqualFunction(Card.GetSummonLocation,LOCATION_EXTRA),tp,0,LOCATION_MZONE,1,nil)
    duel::is_existing_matching_card(d, Some(&|d, c| card::get_summon_location(d, c) == LOCATION_EXTRA), a.tp, 0, LOCATION_MZONE, 1, NO_EX)
}
/// `s.sptg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn sptg(d: &mut Duel, a: &EffArgs) -> bool {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if chk==0 then return Duel.GetLocationCount(tp,LOCATION_MZONE)>0 and e:GetHandler():IsCanBeSpecialSummoned(e,0,tp,false,false) end
    if a.chk == Some(0) {
        return duel::get_location_count(d, a.tp, LOCATION_MZONE as u8, None, None, None).0 > 0
            && card::is_can_be_special_summoned(d, c, a.e, 0, a.tp, false, false, None, None, None);
    }
    // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,e:GetHandler(),1,0,0)
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, Some(CardSet::from_card(c)), 1, 0, 0);
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
/// `s.indval(e,c)`
fn indval(d: &mut Duel, _e: EffectId, c: CardId) -> bool {
    // return c:IsNormalSummoned()
    card::is_normal_summoned(d, c)
}
/// `s.refval(e,c)`
fn refval(d: &mut Duel, e: EffectId, _c: CardId) -> bool {
    // local h = e:GetHandler() -- (the Lua calls e:GetHandler() each time)
    let h = effect::get_handler(d, e).nil();
    // if e:GetHandler():GetFlagEffect(id)~=0 then
    if card::get_flag_effect(d, h, ID) != 0 {
        // Duel.RegisterFlagEffect(e:GetHandlerPlayer(),id,RESET_PHASE|PHASE_END,0,1)
        let hp = effect::get_handler_player(d, e);
        duel::register_flag_effect(d, hp, ID, RESET_PHASE | PHASE_END, 0, 1, 0);
        // e:GetHandler():ResetFlagEffect(id)
        card::reset_flag_effect(d, h, ID);
        // return true
        true
    // elseif Duel.GetFlagEffect(e:GetHandlerPlayer(),id)==0 then
    } else if duel::get_flag_effect(d, effect::get_handler_player(d, e), ID) == 0 {
        // e:GetHandler():RegisterFlagEffect(id,0,0,1)
        card::register_flag_effect(d, h, ID, 0, 0, 1, 0, 0);
        // return true
        true
    // else return false end
    } else {
        false
    }
}
/// `s.sdfilter(c)`
fn sdfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsFaceup() and not c:IsSetCard(SET_SPEEDROID)
    card::is_faceup(d, c) && !card::is_set_card(d, c, &[SET_SPEEDROID as u16])
}
/// `s.sdcon(e)`
fn sdcon(d: &mut Duel, a: Args) -> bool {
    // return Duel.IsExistingMatchingCard(s.sdfilter,e:GetHandlerPlayer(),LOCATION_MZONE,0,1,nil)
    let p = effect::get_handler_player(d, a.e(0));
    duel::is_existing_matching_card(d, Some(&sdfilter), p, LOCATION_MZONE, 0, 1, NO_EX)
}
