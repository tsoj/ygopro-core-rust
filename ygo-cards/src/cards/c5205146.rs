//! Infinitrack Road Roller
use super::prelude::*;

pub const ID: u32 = 5205146;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Special Summon itself if an EARTH Machine monster is Tributed
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_SPECIAL_SUMMON)
    effect::set_category(d, e1, CATEGORY_SPECIAL_SUMMON);
    // e1:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e1, EFFECT_TYPE_FIELD + EFFECT_TYPE_TRIGGER_O);
    // e1:SetProperty(EFFECT_FLAG_DELAY,EFFECT_FLAG2_CHECK_SIMULTANEOUS)
    effect::set_property(d, e1, EFFECT_FLAG_DELAY, EFFECT_FLAG2_CHECK_SIMULTANEOUS);
    // e1:SetCode(EVENT_RELEASE)
    effect::set_code(d, e1, EVENT_RELEASE);
    // e1:SetRange(LOCATION_HAND|LOCATION_GRAVE)
    effect::set_range(d, e1, LOCATION_HAND | LOCATION_GRAVE);
    // e1:SetLabel(0)
    effect::set_label(d, e1, &[0]);
    // e1:SetCountLimit(1,id)
    effect::set_count_limit(d, e1, 1, ID, 0, 0);
    // e1:SetCondition(s.spcon)
    effect::set_condition(d, e1, Some(condition(spcon)));
    // e1:SetTarget(s.sptg)
    effect::set_target(d, e1, Some(cost_target(sptg)));
    // e1:SetOperation(s.spop)
    effect::set_operation(d, e1, Some(operation(spop)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // Special Summon itself if an EARTH Machine monster is banished face-up
    // local e2=e1:Clone()
    let e2 = effect::clone(d, e1);
    // e2:SetCode(EVENT_REMOVE)
    effect::set_code(d, e2, EVENT_REMOVE);
    // e2:SetLabel(1)
    effect::set_label(d, e2, &[1]);
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
    // If attached to a Machine Xyz monster, monsters your opponent controls are changed to Defense Position
    // local e3=Effect.CreateEffect(c)
    let e3 = effect::create_effect(d, c);
    // e3:SetType(EFFECT_TYPE_XMATERIAL+EFFECT_TYPE_FIELD)
    effect::set_type(d, e3, EFFECT_TYPE_XMATERIAL + EFFECT_TYPE_FIELD);
    // e3:SetProperty(EFFECT_FLAG_SINGLE_RANGE)
    effect::set_property(d, e3, EFFECT_FLAG_SINGLE_RANGE, 0);
    // e3:SetCode(EFFECT_SET_POSITION)
    effect::set_code(d, e3, EFFECT_SET_POSITION);
    // e3:SetRange(LOCATION_MZONE)
    effect::set_range(d, e3, LOCATION_MZONE);
    // e3:SetTargetRange(0,LOCATION_MZONE)
    effect::set_target_range(d, e3, 0, LOCATION_MZONE);
    // e3:SetCondition(s.poscond)
    effect::set_condition(d, e3, Some(condition(poscond)));
    // e3:SetTarget(function(_,c) return c:IsFaceup() end)
    effect::set_target(d, e3, Some(effect_card_fn(|d, _e, c| card::is_faceup(d, c))));
    // e3:SetValue(POS_FACEUP_DEFENSE)
    effect::set_value(d, e3, POS_FACEUP_DEFENSE as i64);
    // c:RegisterEffect(e3)
    card::register_effect(d, c, e3, false);
    // If attached to a Machine Xyz monster, monsters your opponent controls lose 1000 DEF
    // local e4=Effect.CreateEffect(c)
    let e4 = effect::create_effect(d, c);
    // e4:SetType(EFFECT_TYPE_XMATERIAL+EFFECT_TYPE_FIELD)
    effect::set_type(d, e4, EFFECT_TYPE_XMATERIAL + EFFECT_TYPE_FIELD);
    // e4:SetProperty(EFFECT_FLAG_SINGLE_RANGE)
    effect::set_property(d, e4, EFFECT_FLAG_SINGLE_RANGE, 0);
    // e4:SetCode(EFFECT_UPDATE_DEFENSE)
    effect::set_code(d, e4, EFFECT_UPDATE_DEFENSE);
    // e4:SetRange(LOCATION_MZONE)
    effect::set_range(d, e4, LOCATION_MZONE);
    // e4:SetTargetRange(0,LOCATION_MZONE)
    effect::set_target_range(d, e4, 0, LOCATION_MZONE);
    // e4:SetCondition(s.poscond)
    effect::set_condition(d, e4, Some(condition(poscond)));
    // e4:SetValue(-1000)
    effect::set_value(d, e4, -1000);
    // c:RegisterEffect(e4)
    card::register_effect(d, c, e4, false);
}
/// `s.cfilter(c,label)`
fn cfilter(d: &mut Duel, c: CardId, label: i64) -> bool {
    // if label==1 and c:IsFacedown() then return false end
    if label == 1 && card::is_facedown(d, c) {
        return false;
    }
    // if c:IsPreviousLocation(LOCATION_MZONE) then
    if card::is_previous_location(d, c, LOCATION_MZONE) {
        // return c:GetPreviousAttributeOnField()&ATTRIBUTE_EARTH>0 and c:GetPreviousRaceOnField()&RACE_MACHINE>0
        (card::get_previous_attribute_on_field(d, c) & ATTRIBUTE_EARTH) > 0
            && (card::get_previous_race_on_field(d, c) & RACE_MACHINE) > 0
    } else {
        // return c:IsMonster() and c:IsOriginalAttribute(ATTRIBUTE_EARTH) and c:IsOriginalRace(RACE_MACHINE)
        card::is_monster(d, c)
            && card::is_original_attribute(d, c, ATTRIBUTE_EARTH)
            && card::is_original_race(d, c, RACE_MACHINE)
    }
}
/// `s.spcon(e,tp,eg,ep,ev,re,r,rp)`
fn spcon(d: &mut Duel, a: &EffArgs) -> bool {
    // return eg:IsExists(s.cfilter,1,nil,e:GetLabel())
    let label = effect::get_label(d, a.e);
    let eg = a.eg.clone().nil();
    group::is_exists(d, &eg, &|d, c| cfilter(d, c, label), 1, NO_EX)
}
/// `s.sptg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn sptg(d: &mut Duel, a: &EffArgs) -> bool {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if chk==0 then return Duel.GetLocationCount(tp,LOCATION_MZONE)>0
    //     and c:IsCanBeSpecialSummoned(e,0,tp,false,false) end
    if a.chk == Some(0) {
        let (ft, _) = duel::get_location_count(d, a.tp, LOCATION_MZONE as u8, None, None, None);
        return ft > 0 && card::is_can_be_special_summoned(d, c, a.e, 0, a.tp, false, false, None, None, None);
    }
    // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,c,1,0,0)
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, Some(CardSet::from_card(c)), 1, 0, 0);
    true
}
/// `s.spop(e,tp,eg,ep,ev,re,r,rp)`
fn spop(d: &mut Duel, a: &EffArgs) {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if c:IsRelateToEffect(e) and Duel.SpecialSummon(c,0,tp,tp,false,false,POS_FACEUP)>0 then
    if card::is_relate_to_effect(d, c, a.e)
        && duel::special_summon(d, c, 0, a.tp, a.tp, false, false, POS_FACEUP as u8, None) > 0
    {
        // Banish it when it leaves the field
        // local e1=Effect.CreateEffect(c)
        let e1 = effect::create_effect(d, c);
        // e1:SetDescription(3300)
        effect::set_description(d, e1, 3300);
        // e1:SetType(EFFECT_TYPE_SINGLE)
        effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
        // e1:SetProperty(EFFECT_FLAG_CANNOT_DISABLE+EFFECT_FLAG_CLIENT_HINT)
        effect::set_property(d, e1, EFFECT_FLAG_CANNOT_DISABLE + EFFECT_FLAG_CLIENT_HINT, 0);
        // e1:SetCode(EFFECT_LEAVE_FIELD_REDIRECT)
        effect::set_code(d, e1, EFFECT_LEAVE_FIELD_REDIRECT);
        // e1:SetValue(LOCATION_REMOVED)
        effect::set_value(d, e1, LOCATION_REMOVED as i64);
        // e1:SetReset(RESET_EVENT|RESETS_REDIRECT)
        effect::set_reset(d, e1, RESET_EVENT | RESETS_REDIRECT, 1);
        // c:RegisterEffect(e1,true)
        card::register_effect(d, c, e1, true);
    }
}
/// `s.poscond(e,tp,eg,ep,ev,re,r,rp)`
fn poscond(d: &mut Duel, a: &EffArgs) -> bool {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // return c:IsOriginalRace(RACE_MACHINE) and c:IsType(TYPE_XYZ)
    card::is_original_race(d, c, RACE_MACHINE) && card::is_type(d, c, TYPE_XYZ)
}
