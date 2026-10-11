//! Traptrix Allomerus
use super::prelude::*;

pub const ID: u32 = 59071624;

pub const INFO: ScriptInfo = ScriptInfo {
    listed_series: &[SET_TRAPTRIX as u32],
    ..ScriptInfo::DEFAULT
};

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Must be properly summoned before reviving
    // c:EnableReviveLimit()
    card::enable_revive_limit(d, c);
    // Xyz.AddProcedure(c,nil,4,2,nil,nil,Xyz.InfiniteMats)
    xyz::add_procedure(d, c, None, Some(4), 2, None, None, Some(xyz::INFINITE_MATS), None, false, None);
    // Unaffected by trap effects while has material(s) attached
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
    // e1:SetProperty(EFFECT_FLAG_SINGLE_RANGE)
    effect::set_property(d, e1, EFFECT_FLAG_SINGLE_RANGE, 0);
    // e1:SetRange(LOCATION_MZONE)
    effect::set_range(d, e1, LOCATION_MZONE);
    // e1:SetCode(EFFECT_IMMUNE_EFFECT)
    effect::set_code(d, e1, EFFECT_IMMUNE_EFFECT);
    // e1:SetCondition(s.imcon)
    effect::set_condition(d, e1, Some(lua_fn(imcon)));
    // e1:SetValue(s.efilter)
    effect::set_value_fn(d, e1, lua_fn(efilter));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // Detach 2 materials; special summon 1 level 4 plant/insect from GY
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e2, aux::stringid(ID, 0));
    // e2:SetCategory(CATEGORY_SPECIAL_SUMMON)
    effect::set_category(d, e2, CATEGORY_SPECIAL_SUMMON);
    // e2:SetType(EFFECT_TYPE_IGNITION)
    effect::set_type(d, e2, EFFECT_TYPE_IGNITION);
    // e2:SetRange(LOCATION_MZONE)
    effect::set_range(d, e2, LOCATION_MZONE);
    // e2:SetCountLimit(1,id)
    effect::set_count_limit(d, e2, 1, ID, 0, 0);
    // e2:SetCost(Cost.DetachFromSelf(2))
    effect::set_cost(d, e2, Some(cost::detach_from_self(cost::Count::N(2), None, None)));
    // e2:SetTarget(s.sptg)
    effect::set_target(d, e2, Some(cost_target(sptg)));
    // e2:SetOperation(s.spop)
    effect::set_operation(d, e2, Some(operation(spop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
    // If an opponent's monster is in GY/banished due to your effect, special summon
    // local e3=Effect.CreateEffect(c)
    let e3 = effect::create_effect(d, c);
    // e3:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e3, aux::stringid(ID, 1));
    // e3:SetCategory(CATEGORY_SPECIAL_SUMMON)
    effect::set_category(d, e3, CATEGORY_SPECIAL_SUMMON);
    // e3:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e3, EFFECT_TYPE_FIELD + EFFECT_TYPE_TRIGGER_O);
    // e3:SetProperty(EFFECT_FLAG_CARD_TARGET+EFFECT_FLAG_DELAY)
    effect::set_property(d, e3, EFFECT_FLAG_CARD_TARGET + EFFECT_FLAG_DELAY, 0);
    // e3:SetCode(EVENT_TO_GRAVE)
    effect::set_code(d, e3, EVENT_TO_GRAVE);
    // e3:SetRange(LOCATION_MZONE)
    effect::set_range(d, e3, LOCATION_MZONE);
    // e3:SetCountLimit(1,{id,1})
    effect::set_count_limit(d, e3, 1, ID, 1, 0);
    // e3:SetCost(Cost.DetachFromSelf(1))
    effect::set_cost(d, e3, Some(cost::detach_from_self(cost::Count::N(1), None, None)));
    // e3:SetCondition(s.sscon)
    effect::set_condition(d, e3, Some(condition(sscon)));
    // e3:SetTarget(s.sstg)
    effect::set_target(d, e3, Some(cost_target(sstg)));
    // e3:SetOperation(s.ssop)
    effect::set_operation(d, e3, Some(operation(ssop)));
    // c:RegisterEffect(e3)
    card::register_effect(d, c, e3, false);
    // local e4=e3:Clone()
    let e4 = effect::clone(d, e3);
    // e4:SetCode(EVENT_REMOVE)
    effect::set_code(d, e4, EVENT_REMOVE);
    // c:RegisterEffect(e4)
    card::register_effect(d, c, e4, false);
}
// Has material(s) attached
/// `function(e) return e:GetHandler():GetOverlayCount()>0 end`
fn imcon(d: &mut Duel, a: Args) -> bool {
    let h = effect::get_handler(d, a.e(0)).nil();
    card::get_overlay_count(d, h) > 0
}
// Trap effects
/// `function(e,te) return te:IsTrapEffect() end`
fn efilter(d: &mut Duel, a: Args) -> bool {
    let te = a.e(1);
    effect::is_trap_effect(d, te)
}
// Check for level 4 plant/insect monster
fn spfilter(d: &mut Duel, c: CardId, e: EffectId, tp: u8) -> bool {
    // return c:IsLevel(4) and c:IsRace(RACE_PLANT|RACE_INSECT) and c:IsCanBeSpecialSummoned(e,0,tp,false,false)
    card::is_level(d, c, &[4]) && card::is_race(d, c, RACE_PLANT | RACE_INSECT) && card::is_can_be_special_summoned(d, c, e, 0, tp, false, false, None, None, None)
}
// Activation legality
fn sptg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return Duel.GetLocationCount(tp,LOCATION_MZONE)>0
    //     and Duel.IsExistingMatchingCard(s.spfilter,tp,LOCATION_GRAVE,0,1,nil,e,tp) end
    if a.chk == Some(0) {
        return duel::get_location_count(d, a.tp, LOCATION_MZONE as u8, None, None, None).0 > 0
            && duel::is_existing_matching_card(d, Some(&|d, c| spfilter(d, c, a.e, a.tp)), a.tp, LOCATION_GRAVE, 0, 1, NO_EX);
    }
    // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,nil,1,tp,LOCATION_GRAVE)
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, None, 1, a.tp, LOCATION_GRAVE as i32);
    true
}
// Special summon 1 level 4 plant/insect monster from GY
fn spop(d: &mut Duel, a: &EffArgs) {
    // if Duel.GetLocationCount(tp,LOCATION_MZONE)<=0 then return end
    if duel::get_location_count(d, a.tp, LOCATION_MZONE as u8, None, None, None).0 <= 0 {
        return;
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_SPSUMMON)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_SPSUMMON as u64);
    // local g=Duel.SelectMatchingCard(tp,aux.NecroValleyFilter(s.spfilter),tp,LOCATION_GRAVE,0,1,1,nil,e,tp)
    let g = duel::select_matching_card(d, a.tp, Some(&aux::necro_valley_filter(|d, x| spfilter(d, x, a.e, a.tp))), a.tp, LOCATION_GRAVE, 0, 1, 1, false, NO_EX).nil();
    // if #g>0 then
    if group::get_count(&g) > 0 {
        // Duel.SpecialSummon(g,0,tp,tp,false,false,POS_FACEUP)
        duel::special_summon(d, g, 0, a.tp, a.tp, false, false, POS_FACEUP as u8, None);
    }
}
// Check if ever happened
fn sscon(d: &mut Duel, a: &EffArgs) -> bool {
    // return eg:IsExists(s.cfilter,1,nil,tp)
    let eg = a.eg.clone().nil();
    group::is_exists(d, &eg, &|d, c| cfilter(d, c, a.tp), 1, NO_EX)
}
// Check for opponent's monster that sent there by your effect
fn cfilter(d: &mut Duel, c: CardId, tp: u8) -> bool {
    // return c:IsMonster() and c:GetPreviousControler()==1-tp and c:IsPreviousLocation(LOCATION_MZONE)
    //     and c:IsReason(REASON_EFFECT) and c:GetReasonPlayer()==tp
    card::is_monster(d, c)
        && card::get_previous_controler(d, c) == 1 - tp
        && card::is_previous_location(d, c, LOCATION_MZONE)
        && card::is_reason(d, c, REASON_EFFECT)
        && card::get_reason_player(d, c) == tp
}
// Check for opponent's sent monster in GY/banished
fn ssfilter(d: &mut Duel, c: CardId, e: EffectId, tp: u8) -> bool {
    // return c:IsCanBeSpecialSummoned(e,0,tp,false,false) and s.cfilter(c,tp)
    //     and c:IsCanBeEffectTarget(e) and c:IsLocation(LOCATION_GRAVE|LOCATION_REMOVED)
    card::is_can_be_special_summoned(d, c, e, 0, tp, false, false, None, None, None)
        && cfilter(d, c, tp)
        && card::is_can_be_effect_target(d, c, Some(e))
        && card::is_location(d, c, LOCATION_GRAVE | LOCATION_REMOVED)
}
// Activation legality
fn sstg(d: &mut Duel, a: &EffArgs) -> bool {
    // local g=eg:Filter(s.ssfilter,nil,e,tp)
    let eg = a.eg.clone().nil();
    let g = group::filter(d, &eg, &|d, c| ssfilter(d, c, a.e, a.tp), NO_EX);
    // if chk==0 then return Duel.GetLocationCount(tp,LOCATION_MZONE)>0 and #g>0 end
    if a.chk == Some(0) {
        return duel::get_location_count(d, a.tp, LOCATION_MZONE as u8, None, None, None).0 > 0 && group::get_count(&g) > 0;
    }
    // local c=nil
    // if #g>1 then
    let c = if group::get_count(&g) > 1 {
        // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_SPSUMMON)
        duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_SPSUMMON as u64);
        // c=g:Select(tp,1,1,nil):GetFirst()
        let sel = group::select(d, &g, a.tp, 1, 1, false, NO_EX).nil();
        group::get_first(d, &sel)
    } else {
        // c=g:GetFirst()
        group::get_first(d, &g)
    };
    // Duel.SetTargetCard(c)
    duel::set_target_card(d, c.nil());
    // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,c,1,0,g:GetFirst():GetLocation())
    let first_loc = group::get_first(d, &g).nil();
    let loc = card::get_location(d, first_loc);
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, Some(CardSet::from_card(c.nil())), 1, 0, loc as i32);
    true
}
// Special summon the opponent's monster that was sent there by your effect
fn ssop(d: &mut Duel, a: &EffArgs) {
    // local tc=Duel.GetFirstTarget()
    let tc = duel::first_target(d).nil();
    // if tc:IsRelateToEffect(e) then
    if card::is_relate_to_effect(d, tc, a.e) {
        // Duel.SpecialSummon(tc,0,tp,tp,false,false,POS_FACEUP)
        duel::special_summon(d, tc, 0, a.tp, a.tp, false, false, POS_FACEUP as u8, None);
    }
}
