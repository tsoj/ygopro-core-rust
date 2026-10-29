//! Albaz the Ashen
use super::prelude::*;

pub const ID: u32 = 37683547;

pub const INFO: ScriptInfo = ScriptInfo { listed_names: &[CARD_ALBAZ], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Change name
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
    // e1:SetProperty(EFFECT_FLAG_SINGLE_RANGE)
    effect::set_property(d, e1, EFFECT_FLAG_SINGLE_RANGE, 0);
    // e1:SetCode(EFFECT_CHANGE_CODE)
    effect::set_code(d, e1, EFFECT_CHANGE_CODE);
    // e1:SetRange(LOCATION_MZONE|LOCATION_GRAVE)
    effect::set_range(d, e1, LOCATION_MZONE | LOCATION_GRAVE);
    // e1:SetValue(CARD_ALBAZ)
    effect::set_value(d, e1, CARD_ALBAZ as i64);
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // Increase ATK
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e2, EFFECT_TYPE_SINGLE);
    // e2:SetCode(EFFECT_UPDATE_ATTACK)
    effect::set_code(d, e2, EFFECT_UPDATE_ATTACK);
    // e2:SetProperty(EFFECT_FLAG_SINGLE_RANGE)
    effect::set_property(d, e2, EFFECT_FLAG_SINGLE_RANGE, 0);
    // e2:SetRange(LOCATION_MZONE)
    effect::set_range(d, e2, LOCATION_MZONE);
    // e2:SetCondition(s.fuscon)
    effect::set_condition(d, e2, Some(condition(fuscon)));
    // e2:SetValue(s.atkval)
    effect::set_value_fn(d, e2, effect_value(atkval));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
    // Opponent cannot target other monsters you control with effects
    // local e3=Effect.CreateEffect(c)
    let e3 = effect::create_effect(d, c);
    // e3:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e3, EFFECT_TYPE_FIELD);
    // e3:SetCode(EFFECT_CANNOT_BE_EFFECT_TARGET)
    effect::set_code(d, e3, EFFECT_CANNOT_BE_EFFECT_TARGET);
    // e3:SetProperty(EFFECT_FLAG_IGNORE_IMMUNE)
    effect::set_property(d, e3, EFFECT_FLAG_IGNORE_IMMUNE, 0);
    // e3:SetTargetRange(LOCATION_MZONE,0)
    effect::set_target_range(d, e3, LOCATION_MZONE, 0);
    // e3:SetRange(LOCATION_MZONE)
    effect::set_range(d, e3, LOCATION_MZONE);
    // e3:SetCondition(s.fuscon)
    effect::set_condition(d, e3, Some(condition(fuscon)));
    // e3:SetTarget(function(e,c) return c~=e:GetHandler() end)
    effect::set_target(d, e3, Some(effect_card_fn(|d, e, c| effect::get_handler(d, e) != Some(c))));
    // e3:SetValue(aux.tgoval)
    effect::set_value_fn(d, e3, aux::tgoval());
    // c:RegisterEffect(e3)
    card::register_effect(d, c, e3, false);
    // Special Summon this card
    // local e4=Effect.CreateEffect(c)
    let e4 = effect::create_effect(d, c);
    // e4:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e4, aux::stringid(ID, 0));
    // e4:SetCategory(CATEGORY_SPECIAL_SUMMON)
    effect::set_category(d, e4, CATEGORY_SPECIAL_SUMMON);
    // e4:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e4, EFFECT_TYPE_FIELD + EFFECT_TYPE_TRIGGER_O);
    // e4:SetProperty(EFFECT_FLAG_DELAY)
    effect::set_property(d, e4, EFFECT_FLAG_DELAY, 0);
    // e4:SetCode(EVENT_LEAVE_FIELD)
    effect::set_code(d, e4, EVENT_LEAVE_FIELD);
    // e4:SetRange(LOCATION_GRAVE)
    effect::set_range(d, e4, LOCATION_GRAVE);
    // e4:SetCountLimit(1,id)
    effect::set_count_limit(d, e4, 1, ID, 0, 0);
    // e4:SetCondition(s.spcon)
    effect::set_condition(d, e4, Some(condition(spcon)));
    // e4:SetTarget(s.sptg)
    effect::set_target(d, e4, Some(cost_target(sptg)));
    // e4:SetOperation(s.spop)
    effect::set_operation(d, e4, Some(operation(spop)));
    // c:RegisterEffect(e4)
    card::register_effect(d, c, e4, false);
}
/// `s.cfilter(c)`
fn cfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsType(TYPE_FUSION) and c:IsLevel(8)
    card::is_type(d, c, TYPE_FUSION) && card::is_level(d, c, &[8])
}
/// `s.fuscon(e)`
fn fuscon(d: &mut Duel, a: &EffArgs) -> bool {
    // return Duel.IsExistingMatchingCard(s.cfilter,e:GetHandlerPlayer(),LOCATION_GRAVE,0,1,nil)
    let p = effect::get_handler_player(d, a.e);
    duel::is_existing_matching_card(d, Some(&cfilter), p, LOCATION_GRAVE, 0, 1, NO_EX)
}
/// `s.atkval(e,c)`
fn atkval(d: &mut Duel, e: EffectId) -> i64 {
    // return Duel.GetMatchingGroupCount(Card.IsMonster,e:GetHandlerPlayer(),LOCATION_GRAVE,0,nil)*200
    let p = effect::get_handler_player(d, e);
    duel::get_matching_group_count(d, Some(&card::is_monster), p, LOCATION_GRAVE, 0, NO_EX) as i64 * 200
}
/// `s.spcfilter(c,tp,rp)`
fn spcfilter(d: &mut Duel, c: CardId, tp: u8, rp: u8) -> bool {
    // return c:IsPreviousLocation(LOCATION_MZONE) and c:IsPreviousControler(tp)
    //   and rp==1-tp and c:IsReason(REASON_EFFECT)
    card::is_previous_location(d, c, LOCATION_MZONE) && card::is_previous_controler(d, c, tp) && rp == 1 - tp && card::is_reason(d, c, REASON_EFFECT)
}
/// `s.spcon(e,tp,eg,ep,ev,re,r,rp)`
fn spcon(d: &mut Duel, a: &EffArgs) -> bool {
    // if not (not eg:IsContains(e:GetHandler()) and eg:IsExists(s.spcfilter,1,nil,tp,rp)) then return false end
    let eg = a.eg.clone().nil();
    let h = effect::get_handler(d, a.e).nil();
    if !(!eg.contains(h) && group::is_exists(d, &eg, &|d, c| spcfilter(d, c, a.tp, a.rp), 1, NO_EX)) {
        return false;
    }
    // local fg=Duel.GetMatchingGroup(Card.IsType,tp,LOCATION_GRAVE,0,nil,TYPE_FUSION)
    let fg = duel::get_matching_group(d, Some(&|d, c| card::is_type(d, c, TYPE_FUSION)), a.tp, LOCATION_GRAVE, 0, NO_EX);
    // return fg:FilterCount(aux.TRUE,eg)>0
    group::filter_count(d, &fg, &|_d, _c| true, (&eg).into()) > 0
}
/// `s.sptg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn sptg(d: &mut Duel, a: &EffArgs) -> bool {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if chk==0 then return Duel.GetLocationCount(tp,LOCATION_MZONE)>0
    //   and c:IsCanBeSpecialSummoned(e,0,tp,false,false) end
    if a.chk == Some(0) {
        return duel::get_location_count(d, a.tp, LOCATION_MZONE as u8, None, None, None).0 > 0
            && card::is_can_be_special_summoned(d, c, a.e, 0, a.tp, false, false, None, None, None);
    }
    // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,c,1,0,LOCATION_GRAVE)
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, Some(CardSet::from_card(c)), 1, 0, LOCATION_GRAVE as i32);
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
