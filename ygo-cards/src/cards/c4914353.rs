//! Space Insulator
use super::prelude::*;

pub const ID: u32 = 4914353;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Must be properly summoned before reviving
    // c:EnableReviveLimit()
    card::enable_revive_limit(d, c);
    // Link summon procedure
    // Link.AddProcedure(c,nil,2,2)
    link::add_procedure(d, c, None, 2, Some(2), None, None, None);
    // Monsters this card points to loses 800 ATK/DEF
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e1, EFFECT_TYPE_FIELD);
    // e1:SetCode(EFFECT_UPDATE_ATTACK)
    effect::set_code(d, e1, EFFECT_UPDATE_ATTACK);
    // e1:SetRange(LOCATION_MZONE)
    effect::set_range(d, e1, LOCATION_MZONE);
    // e1:SetTargetRange(LOCATION_MZONE,LOCATION_MZONE)
    effect::set_target_range(d, e1, LOCATION_MZONE, LOCATION_MZONE);
    // e1:SetTarget(s.tgtg)
    effect::set_target(d, e1, Some(effect_card_fn(tgtg)));
    // e1:SetValue(-800)
    effect::set_value(d, e1, -800);
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // local e2=e1:Clone()
    let e2 = effect::clone(d, e1);
    // e2:SetCode(EFFECT_UPDATE_DEFENSE)
    effect::set_code(d, e2, EFFECT_UPDATE_DEFENSE);
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
    // Special summon itself from GY
    // local e3=Effect.CreateEffect(c)
    let e3 = effect::create_effect(d, c);
    // e3:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e3, aux::stringid(ID, 0));
    // e3:SetCategory(CATEGORY_SPECIAL_SUMMON)
    effect::set_category(d, e3, CATEGORY_SPECIAL_SUMMON);
    // e3:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e3, EFFECT_TYPE_FIELD + EFFECT_TYPE_TRIGGER_O);
    // e3:SetCode(EVENT_SPSUMMON_SUCCESS)
    effect::set_code(d, e3, EVENT_SPSUMMON_SUCCESS);
    // e3:SetRange(LOCATION_GRAVE)
    effect::set_range(d, e3, LOCATION_GRAVE);
    // e3:SetCondition(s.spcon)
    effect::set_condition(d, e3, Some(condition(spcon)));
    // e3:SetTarget(s.sptg)
    effect::set_target(d, e3, Some(cost_target(sptg)));
    // e3:SetOperation(s.spop)
    effect::set_operation(d, e3, Some(operation(spop)));
    // c:RegisterEffect(e3)
    card::register_effect(d, c, e3, false);
}
/// `s.tgtg(e,c)`
fn tgtg(d: &mut Duel, e: EffectId, c: CardId) -> bool {
    // return e:GetHandler():GetLinkedGroup():IsContains(c)
    let h = effect::get_handler(d, e).nil();
    card::get_linked_group(d, h).contains(c)
}
/// `s.cfilter(c,tp)`
fn cfilter(d: &mut Duel, c: CardId, tp: u8) -> bool {
    // return c:IsControler(tp) and c:IsRace(RACE_CYBERSE) and c:IsLinkMonster() and c:IsLinkSummoned()
    card::is_controler(d, c, tp) && card::is_race(d, c, RACE_CYBERSE) && card::is_link_monster(d, c) && card::is_link_summoned(d, c)
}
/// `s.spcon(e,tp,eg,ep,ev,re,r,rp)`
fn spcon(d: &mut Duel, a: &EffArgs) -> bool {
    // return eg:IsExists(s.cfilter,1,nil,tp) and aux.exccon(e)
    let eg = a.eg.clone().nil();
    if !group::is_exists(d, &eg, &|d, c| cfilter(d, c, a.tp), 1, NO_EX) {
        return false;
    }
    aux::exccon().call(d, &[Value::Effect(a.e)]).truthy()
}
/// The zone of the linked cards that `s.cfilter` picks from `eg` (`zone=zone|tc:GetLinkedZone()`).
fn linked_zone(d: &mut Duel, a: &EffArgs) -> u32 {
    let mut zone = 0u32;
    // local lg=eg:Filter(s.cfilter,nil,tp)
    let eg = a.eg.clone().nil();
    let lg = group::filter(d, &eg, &|d, c| cfilter(d, c, a.tp), NO_EX);
    // for tc in aux.Next(lg) do zone=(zone|tc:GetLinkedZone()) end
    for tc in group::iter(d, &lg) {
        zone |= card::get_linked_zone(d, tc, None);
    }
    zone
}
/// `s.sptg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn sptg(d: &mut Duel, a: &EffArgs) -> bool {
    // local zone=0 ... (see linked_zone)
    let zone = linked_zone(d, a);
    // if chk==0 then return Duel.GetLocationCount(tp,LOCATION_MZONE)>0
    //     and e:GetHandler():IsCanBeSpecialSummoned(e,0,tp,false,false,POS_FACEUP,tp,zone) end
    if a.chk == Some(0) {
        let h = effect::get_handler(d, a.e).nil();
        return duel::get_location_count(d, a.tp, LOCATION_MZONE as u8, None, None, None).0 > 0
            && card::is_can_be_special_summoned(d, h, a.e, 0, a.tp, false, false, Some(POS_FACEUP as u8), Some(a.tp), Some(zone));
    }
    // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,e:GetHandler(),1,0,0)
    let h = effect::get_handler(d, a.e).nil();
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, Some(CardSet::from_card(h)), 1, 0, 0);
    true
}
/// `s.spop(e,tp,eg,ep,ev,re,r,rp)`
fn spop(d: &mut Duel, a: &EffArgs) {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // local zone=0 ... (see linked_zone)
    let zone = linked_zone(d, a);
    // if c:IsRelateToEffect(e) and zone~=0 and Duel.SpecialSummonStep(c,0,tp,tp,false,false,POS_FACEUP,zone) then
    if card::is_relate_to_effect(d, c, a.e) && zone != 0 && duel::special_summon_step(d, c, 0, a.tp, a.tp, false, false, POS_FACEUP as u8, Some(zone)) {
        // Cannot be used as link material
        // local e1=Effect.CreateEffect(c)
        let e1 = effect::create_effect(d, c);
        // e1:SetDescription(3312)
        effect::set_description(d, e1, 3312);
        // e1:SetType(EFFECT_TYPE_SINGLE)
        effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
        // e1:SetProperty(EFFECT_FLAG_CANNOT_DISABLE+EFFECT_FLAG_CLIENT_HINT)
        effect::set_property(d, e1, EFFECT_FLAG_CANNOT_DISABLE + EFFECT_FLAG_CLIENT_HINT, 0);
        // e1:SetCode(EFFECT_CANNOT_BE_LINK_MATERIAL)
        effect::set_code(d, e1, EFFECT_CANNOT_BE_LINK_MATERIAL);
        // e1:SetValue(1)
        effect::set_value(d, e1, 1);
        // e1:SetReset(RESET_EVENT|RESETS_STANDARD)
        effect::set_reset(d, e1, RESET_EVENT | RESETS_STANDARD, 1);
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
    }
    // Duel.SpecialSummonComplete()
    duel::special_summon_complete(d);
}
