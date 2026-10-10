//! Thelematech Clatis
use super::prelude::*;

pub const ID: u32 = 98986900;

pub const INFO: ScriptInfo = ScriptInfo { counter_place_list: &[COUNTER_SPELL], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // c:EnableCounterPermit(COUNTER_SPELL)
    card::enable_counter_permit(d, c, COUNTER_SPELL as u16, None, None);
    // c:SetCounterLimit(COUNTER_SPELL,9)
    card::set_counter_limit(d, c, COUNTER_SPELL as u16, 9);
    // c:EnableReviveLimit()
    card::enable_revive_limit(d, c);
    // 2 Level 8 monsters
    // Xyz.AddProcedure(c,nil,8,2)
    xyz::add_procedure(d, c, None, Some(8), 2, None, None, None, None, false, None);
    // Place Spell Counter on this card
    // local e0=Effect.CreateEffect(c)
    let e0 = effect::create_effect(d, c);
    // e0:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_CONTINUOUS)
    effect::set_type(d, e0, EFFECT_TYPE_FIELD + EFFECT_TYPE_CONTINUOUS);
    // e0:SetProperty(EFFECT_FLAG_CANNOT_DISABLE)
    effect::set_property(d, e0, EFFECT_FLAG_CANNOT_DISABLE, 0);
    // e0:SetCode(EVENT_CHAINING)
    effect::set_code(d, e0, EVENT_CHAINING);
    // e0:SetRange(LOCATION_MZONE)
    effect::set_range(d, e0, LOCATION_MZONE);
    // e0:SetOperation(aux.chainreg)
    effect::set_operation(d, e0, Some(aux::chainreg()));
    // c:RegisterEffect(e0)
    card::register_effect(d, c, e0, false);
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_CONTINUOUS)
    effect::set_type(d, e1, EFFECT_TYPE_FIELD + EFFECT_TYPE_CONTINUOUS);
    // e1:SetCode(EVENT_CHAIN_SOLVED)
    effect::set_code(d, e1, EVENT_CHAIN_SOLVED);
    // e1:SetRange(LOCATION_MZONE)
    effect::set_range(d, e1, LOCATION_MZONE);
    // e1:SetOperation(s.acop)
    effect::set_operation(d, e1, Some(operation(acop)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // Search 1 Spell or 1 Spellcaster monster, OR Special Summon 1 Spellcaster monster
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e2, aux::stringid(ID, 0));
    // e2:SetCategory(CATEGORY_SEARCH+CATEGORY_TOHAND+CATEGORY_SPECIAL_SUMMON)
    effect::set_category(d, e2, CATEGORY_SEARCH + CATEGORY_TOHAND + CATEGORY_SPECIAL_SUMMON);
    // e2:SetType(EFFECT_TYPE_IGNITION)
    effect::set_type(d, e2, EFFECT_TYPE_IGNITION);
    // e2:SetRange(LOCATION_MZONE)
    effect::set_range(d, e2, LOCATION_MZONE);
    // e2:SetCountLimit(1,id)
    effect::set_count_limit(d, e2, 1, ID, 0, 0);
    // e2:SetCost(s.ctcost)
    effect::set_cost(d, e2, Some(cost_target(ctcost)));
    // e2:SetTarget(s.cttg)
    effect::set_target(d, e2, Some(cost_target(cttg)));
    // e2:SetOperation(s.ctop)
    effect::set_operation(d, e2, Some(operation(ctop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
    // Destruction replacement
    // local e3=Effect.CreateEffect(c)
    let e3 = effect::create_effect(d, c);
    // e3:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_CONTINUOUS)
    effect::set_type(d, e3, EFFECT_TYPE_SINGLE + EFFECT_TYPE_CONTINUOUS);
    // e3:SetProperty(EFFECT_FLAG_SINGLE_RANGE)
    effect::set_property(d, e3, EFFECT_FLAG_SINGLE_RANGE, 0);
    // e3:SetCode(EFFECT_DESTROY_REPLACE)
    effect::set_code(d, e3, EFFECT_DESTROY_REPLACE);
    // e3:SetRange(LOCATION_MZONE)
    effect::set_range(d, e3, LOCATION_MZONE);
    // e3:SetTarget(s.reptg)
    effect::set_target(d, e3, Some(cost_target(reptg)));
    // c:RegisterEffect(e3)
    card::register_effect(d, c, e3, false);
}
/// `s.acop(e,tp,eg,ep,ev,re,r,rp)`
fn acop(d: &mut Duel, a: &EffArgs) {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if ep==1-tp and not re:IsSpellEffect() and c:HasFlagEffect(1) then
    if a.ep == 1 - a.tp && !effect::is_spell_effect(d, a.re.nil()) && card::has_flag_effect(d, c, 1, None) {
        // c:AddCounter(COUNTER_SPELL,1)
        card::add_counter(d, c, COUNTER_SPELL as u16, 1, false);
    }
}
/// `s.ctcost(e,tp,eg,ep,ev,re,r,rp,chk)`
fn ctcost(d: &mut Duel, a: &EffArgs) -> bool {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if chk==0 then return c:IsCanRemoveCounter(tp,COUNTER_SPELL,3,REASON_COST) end
    if a.chk == Some(0) {
        return card::is_can_remove_counter(d, c, a.tp, COUNTER_SPELL as u16, 3, REASON_COST);
    }
    // c:RemoveCounter(tp,COUNTER_SPELL,3,REASON_COST)
    card::remove_counter(d, c, a.tp, COUNTER_SPELL as u16, 3, REASON_COST);
    true
}
/// `s.thfilter(c)`
fn thfilter(d: &mut Duel, c: CardId) -> bool {
    // return (c:IsSpell() or (c:IsRace(RACE_SPELLCASTER) and c:IsType(TYPE_EFFECT))) and c:IsAbleToHand()
    (card::is_spell(d, c) || (card::is_race(d, c, RACE_SPELLCASTER) && card::is_type(d, c, TYPE_EFFECT))) && card::is_able_to_hand(d, c, None)
}
/// `s.spfilter(c,e,tp)`
fn spfilter(d: &mut Duel, c: CardId, e: EffectId, tp: u8) -> bool {
    // return c:IsRace(RACE_SPELLCASTER) and c:IsCanBeSpecialSummoned(e,0,tp,false,false)
    card::is_race(d, c, RACE_SPELLCASTER) && card::is_can_be_special_summoned(d, c, e, 0, tp, false, false, None, None, None)
}
/// `s.cttg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn cttg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return Duel.IsExistingMatchingCard(s.thfilter,tp,LOCATION_DECK,0,1,nil)
    //   or (Duel.GetLocationCount(tp,LOCATION_MZONE)>0
    //   and Duel.IsExistingMatchingCard(s.spfilter,tp,LOCATION_HAND|LOCATION_DECK,0,1,nil,e,tp)) end
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, Some(&thfilter), a.tp, LOCATION_DECK, 0, 1, NO_EX)
            || (duel::get_location_count(d, a.tp, LOCATION_MZONE as u8, None, None, None).0 > 0
                && duel::is_existing_matching_card(d, Some(&|d, x| spfilter(d, x, a.e, a.tp)), a.tp, LOCATION_HAND | LOCATION_DECK, 0, 1, NO_EX));
    }
    // Duel.SetPossibleOperationInfo(0,CATEGORY_TOHAND,nil,1,tp,LOCATION_DECK)
    duel::set_possible_operation_info(d, 0, CATEGORY_TOHAND, None, 1, a.tp, LOCATION_DECK as i32);
    // Duel.SetPossibleOperationInfo(0,CATEGORY_SPECIAL_SUMMON,nil,1,tp,LOCATION_HAND|LOCATION_DECK)
    duel::set_possible_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, None, 1, a.tp, (LOCATION_HAND | LOCATION_DECK) as i32);
    true
}
/// `s.ctop(e,tp,eg,ep,ev,re,r,rp)`
fn ctop(d: &mut Duel, a: &EffArgs) {
    // local thg=Duel.GetMatchingGroup(s.thfilter,tp,LOCATION_DECK,0,nil)
    let thg = duel::get_matching_group(d, Some(&thfilter), a.tp, LOCATION_DECK, 0, NO_EX);
    // local spg=Duel.GetMatchingGroup(s.spfilter,tp,LOCATION_HAND|LOCATION_DECK,0,nil,e,tp)
    let spg = duel::get_matching_group(d, Some(&|d, x| spfilter(d, x, a.e, a.tp)), a.tp, LOCATION_HAND | LOCATION_DECK, 0, NO_EX);
    // local b1=#thg>0
    let b1 = !thg.is_empty();
    // local b2=#spg>0 and Duel.GetLocationCount(tp,LOCATION_MZONE)>0
    let b2 = !spg.is_empty() && duel::get_location_count(d, a.tp, LOCATION_MZONE as u8, None, None, None).0 > 0;
    // local op=Duel.SelectEffect(tp,{b1,aux.Stringid(id,1)},{b2,aux.Stringid(id,2)})
    let op = duel::select_effect(d, a.tp, &[(b1, aux::stringid(ID, 1)), (b2, aux::stringid(ID, 2))]);
    // if op==1 then
    if op == Some(1) {
        // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_ATOHAND)
        duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_ATOHAND as u64);
        // local g=thg:Select(tp,1,1,nil)
        let g = group::select(d, &thg, a.tp, 1, 1, false, NO_EX).nil();
        // if #g==0 then return end
        if g.is_empty() {
            return;
        }
        // Duel.SendtoHand(g,nil,REASON_EFFECT)
        duel::send_to_hand(d, g.clone(), None, REASON_EFFECT, None);
        // Duel.ConfirmCards(1-tp,g)
        duel::confirm_cards(d, 1 - a.tp, g);
    // elseif op==2 then
    } else if op == Some(2) {
        // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_SPSUMMON)
        duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_SPSUMMON as u64);
        // local g=spg:Select(tp,1,1,nil)
        let g = group::select(d, &spg, a.tp, 1, 1, false, NO_EX).nil();
        // if #g==0 then return end
        if g.is_empty() {
            return;
        }
        // Duel.SpecialSummon(g,0,tp,tp,false,false,POS_FACEUP)
        duel::special_summon(d, g, 0, a.tp, a.tp, false, false, POS_FACEUP as u8, None);
    }
}
/// `s.reptg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn reptg(d: &mut Duel, a: &EffArgs) -> bool {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if chk==0 then return not c:IsReason(REASON_REPLACE) and c:CheckRemoveOverlayCard(tp,1,REASON_EFFECT) end
    if a.chk == Some(0) {
        return !card::is_reason(d, c, REASON_REPLACE) && card::check_remove_overlay_card(d, c, a.tp, 1, REASON_EFFECT);
    }
    // if Duel.SelectEffectYesNo(tp,c,96) then
    if duel::select_effect_yes_no(d, a.tp, c, Some(96)) {
        // c:RemoveOverlayCard(tp,1,1,REASON_EFFECT|REASON_REPLACE)
        card::remove_overlay_card(d, c, a.tp, 1, 1, REASON_EFFECT | REASON_REPLACE);
        // return true
        return true;
    }
    // else return false end
    false
}
