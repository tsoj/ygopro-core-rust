//! Tao Tao the Chanter
//! 陰陽師 タオタオ
use super::prelude::*;

pub const ID: u32 = 45016904;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // If this card battles a monster, neither can be destroyed by that battle
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e1, EFFECT_TYPE_FIELD);
    // e1:SetCode(EFFECT_INDESTRUCTABLE_BATTLE)
    effect::set_code(d, e1, EFFECT_INDESTRUCTABLE_BATTLE);
    // e1:SetRange(LOCATION_MZONE)
    effect::set_range(d, e1, LOCATION_MZONE);
    // e1:SetTargetRange(LOCATION_MZONE,LOCATION_MZONE)
    effect::set_target_range(d, e1, LOCATION_MZONE, LOCATION_MZONE);
    // e1:SetTarget(s.indestg)
    effect::set_target(d, e1, Some(effect_card_fn(indestg)));
    // e1:SetValue(1)
    effect::set_value(d, e1, 1);
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // Special Summon 1 Level 3 or higher Illusion monster from your hand or GY, or, if you took 2000 or more damage, you can Special Summon it from your Deck or Extra Deck instead
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e2, aux::stringid(ID, 0));
    // e2:SetCategory(CATEGORY_SPECIAL_SUMMON)
    effect::set_category(d, e2, CATEGORY_SPECIAL_SUMMON);
    // e2:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e2, EFFECT_TYPE_FIELD + EFFECT_TYPE_TRIGGER_O);
    // e2:SetProperty(EFFECT_FLAG_DELAY+EFFECT_FLAG_DAMAGE_STEP+EFFECT_FLAG_DAMAGE_CAL)
    effect::set_property(d, e2, EFFECT_FLAG_DELAY + EFFECT_FLAG_DAMAGE_STEP + EFFECT_FLAG_DAMAGE_CAL, 0);
    // e2:SetRange(LOCATION_MZONE)
    effect::set_range(d, e2, LOCATION_MZONE);
    // e2:SetCode(EVENT_DAMAGE)
    effect::set_code(d, e2, EVENT_DAMAGE);
    // e2:SetCountLimit(1,id)
    effect::set_count_limit(d, e2, 1, ID, 0, 0);
    // e2:SetCondition(function(e,tp,eg,ep,ev,re,r,rp) return ep==tp and r&(REASON_BATTLE|REASON_EFFECT)>0 end)
    effect::set_condition(d, e2, Some(condition(|_d, a| a.ep == a.tp && (a.r & (REASON_BATTLE | REASON_EFFECT)) > 0)));
    // e2:SetTarget(s.sptg)
    effect::set_target(d, e2, Some(cost_target(sptg)));
    // e2:SetOperation(s.spop)
    effect::set_operation(d, e2, Some(operation(spop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
}
/// `s.indestg(e,c)`
fn indestg(d: &mut Duel, e: EffectId, c: CardId) -> bool {
    // local handler=e:GetHandler()
    let handler = effect::get_handler(d, e).nil();
    // return c==handler or c==handler:GetBattleTarget()
    c == handler || card::get_battle_target(d, handler) == Some(c)
}
/// `s.spfilter(c,e,tp,mmz_chk)`
fn spfilter(d: &mut Duel, c: CardId, e: EffectId, tp: u8, _mmz_chk: i32) -> bool {
    // if not (c:IsLevelAbove(3) and c:IsRace(RACE_ILLUSION) and c:IsCanBeSpecialSummoned(e,0,tp,false,false)) then return false end
    if !(card::is_level_above(d, c, 3) && card::is_race(d, c, RACE_ILLUSION as u64) && card::is_can_be_special_summoned(d, c, e, 0, tp, false, false, None, None, None)) {
        return false;
    }
    // local is_in_extra_deck=c:IsLocation(LOCATION_EXTRA)
    let is_in_extra_deck = card::is_location(d, c, LOCATION_EXTRA);
    // return (not is_in_extra_deck and mmz_chk) or (is_in_extra_deck and Duel.GetLocationCountFromEx(tp,tp,nil,c)>0)
    // In Lua, `not is_in_extra_deck and mmz_chk` is the number mmz_chk itself when it is true, and a number is always
    // true in a condition (even 0), so a monster outside the Extra Deck always passes here.
    if !is_in_extra_deck {
        return true;
    }
    duel::get_location_count_from_ex(d, tp, Some(tp), NO_EX, duel::ExtraCard::Card(c), None).0 > 0
}
/// `s.sptg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn sptg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // local extra_locations=ev>=2000 and LOCATION_DECK|LOCATION_EXTRA or 0
    let extra_locations = if a.ev >= 2000 { LOCATION_DECK | LOCATION_EXTRA } else { 0 };
    // if chk==0 then return Duel.IsExistingMatchingCard(s.spfilter,tp,LOCATION_HAND|LOCATION_GRAVE|extra_locations,0,1,nil,e,tp,Duel.GetLocationCount(tp,LOCATION_MZONE)) end
    if a.chk == Some(0) {
        let mmz = duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0;
        return duel::is_existing_matching_card(d, Some(&|d, c| spfilter(d, c, a.e, tp, mmz)), tp, LOCATION_HAND | LOCATION_GRAVE | extra_locations, 0, 1, NO_EX);
    }
    // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,nil,1,tp,LOCATION_HAND|LOCATION_GRAVE|extra_locations)
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, None, 1, tp, (LOCATION_HAND | LOCATION_GRAVE | extra_locations) as i32);
    true
}
/// `s.spop(e,tp,eg,ep,ev,re,r,rp)`
fn spop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // local extra_locations=ev>=2000 and LOCATION_DECK|LOCATION_EXTRA or 0
    let extra_locations = if a.ev >= 2000 { LOCATION_DECK | LOCATION_EXTRA } else { 0 };
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_SPSUMMON)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_SPSUMMON as u64);
    // local g=Duel.SelectMatchingCard(tp,aux.NecroValleyFilter(s.spfilter),tp,LOCATION_HAND|LOCATION_GRAVE|extra_locations,0,1,1,nil,e,tp,Duel.GetLocationCount(tp,LOCATION_MZONE))
    let mmz = duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0;
    let nv = aux::necro_valley_filter(|d, c| spfilter(d, c, a.e, tp, mmz));
    let g = duel::select_matching_card(d, tp, Some(&nv), tp, LOCATION_HAND | LOCATION_GRAVE | extra_locations, 0, 1, 1, false, NO_EX).nil();
    // if #g>0 then
    if !g.is_empty() {
        // Duel.SpecialSummon(g,0,tp,tp,false,false,POS_FACEUP)
        duel::special_summon(d, g, 0, tp, tp, false, false, POS_FACEUP as u8, None);
    }
}
