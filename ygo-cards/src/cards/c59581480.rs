//! Elfnote Tinia
//! 耀聖の波詩ディーナ
use super::prelude::*;

pub const ID: u32 = 59581480;
pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_ELFNOTE], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // --You can Special Summon this card (from your hand) to your center Main Monster Zone
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e1, EFFECT_TYPE_FIELD);
    // e1:SetProperty(EFFECT_FLAG_UNCOPYABLE)
    effect::set_property(d, e1, EFFECT_FLAG_UNCOPYABLE, 0);
    // e1:SetCode(EFFECT_SPSUMMON_PROC)
    effect::set_code(d, e1, EFFECT_SPSUMMON_PROC);
    // e1:SetRange(LOCATION_HAND)
    effect::set_range(d, e1, LOCATION_HAND);
    // e1:SetCountLimit(1,id,EFFECT_COUNT_CODE_OATH)
    effect::set_count_limit(d, e1, 1, ID, 0, EFFECT_COUNT_CODE_OATH as u8);
    // e1:SetValue(function() return 0,ZONE_CENTER_MMZ end)
    effect::set_value_fn(d, e1, lua_fn(|_d, _a| Value::Multi(vec![Value::Int(0), Value::Int(ZONE_CENTER_MMZ as i64)])));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // --Place 1 "Elfnote" Continuous Spell from your Deck face-up on your field
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e2, aux::stringid(ID, 1));
    // e2:SetType(EFFECT_TYPE_IGNITION)
    effect::set_type(d, e2, EFFECT_TYPE_IGNITION);
    // e2:SetRange(LOCATION_MZONE)
    effect::set_range(d, e2, LOCATION_MZONE);
    // e2:SetCountLimit(1,{id,1})
    effect::set_count_limit(d, e2, 1, ID, 1, 0);
    // e2:SetTarget(s.pltg)
    effect::set_target(d, e2, Some(cost_target(pltg)));
    // e2:SetOperation(s.plop)
    effect::set_operation(d, e2, Some(operation(plop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
    // --Switch the locations of this card in your Main Monster Zone and the monster in your center Main Monster Zone
    // local e3=Effect.CreateEffect(c)
    let e3 = effect::create_effect(d, c);
    // e3:SetDescription(aux.Stringid(id,2))
    effect::set_description(d, e3, aux::stringid(ID, 2));
    // e3:SetCategory(CATEGORY_REMOVE)
    effect::set_category(d, e3, CATEGORY_REMOVE);
    // e3:SetType(EFFECT_TYPE_QUICK_O)
    effect::set_type(d, e3, EFFECT_TYPE_QUICK_O);
    // e3:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e3, EVENT_FREE_CHAIN);
    // e3:SetRange(LOCATION_MMZONE)
    effect::set_range(d, e3, LOCATION_MMZONE);
    // e3:SetCountLimit(1,{id,2})
    effect::set_count_limit(d, e3, 1, ID, 2, 0);
    // e3:SetCondition(function(e,tp) return Duel.IsTurnPlayer(1-tp) end)
    effect::set_condition(d, e3, Some(condition(|d, a| duel::is_turn_player(d, 1 - a.tp))));
    // e3:SetTarget(s.swtg)
    effect::set_target(d, e3, Some(cost_target(swtg)));
    // e3:SetOperation(s.swop)
    effect::set_operation(d, e3, Some(operation(swop)));
    // e3:SetHintTiming(0,TIMING_STANDBY_PHASE|TIMING_MAIN_END|TIMINGS_CHECK_MONSTER_E)
    effect::set_hint_timing(d, e3, 0, Some(TIMING_STANDBY_PHASE | TIMING_MAIN_END | TIMINGS_CHECK_MONSTER_E));
    // c:RegisterEffect(e3)
    card::register_effect(d, c, e3, false);
}
/// `s.plfilter(c,tp)`
fn plfilter(d: &mut Duel, c: CardId, tp: u8) -> bool {
    // return c:IsSetCard(SET_ELFNOTE) and c:IsContinuousSpell() and not c:IsForbidden() and c:CheckUniqueOnField(tp)
    card::is_set_card(d, c, &[SET_ELFNOTE as u16]) && card::is_continuous_spell(d, c) && !card::is_forbidden(d, c) && card::check_unique_on_field(d, c, tp, Some(LOCATION_ONFIELD), None)
}
/// `s.pltg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn pltg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return Duel.GetLocationCount(tp,LOCATION_SZONE)>0
    //     and Duel.IsExistingMatchingCard(s.plfilter,tp,LOCATION_DECK,0,1,nil,tp) end
    if a.chk == Some(0) {
        return duel::get_location_count(d, a.tp, LOCATION_SZONE as u8, None, None, None).0 > 0
            && duel::is_existing_matching_card(d, Some(&|d, c| plfilter(d, c, a.tp)), a.tp, LOCATION_DECK, 0, 1, NO_EX);
    }
    true
}
/// `s.plop(e,tp,eg,ep,ev,re,r,rp)`
fn plop(d: &mut Duel, a: &EffArgs) {
    // if Duel.GetLocationCount(tp,LOCATION_SZONE)<=0 then return end
    if duel::get_location_count(d, a.tp, LOCATION_SZONE as u8, None, None, None).0 <= 0 {
        return;
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_TOFIELD)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_TOFIELD as u64);
    // local sc=Duel.SelectMatchingCard(tp,s.plfilter,tp,LOCATION_DECK,0,1,1,nil,tp):GetFirst()
    let g = duel::select_matching_card(d, a.tp, Some(&|d, c| plfilter(d, c, a.tp)), a.tp, LOCATION_DECK, 0, 1, 1, false, NO_EX).nil();
    // if sc then
    if let Some(sc) = group::get_first(d, &g) {
        // Duel.MoveToField(sc,tp,tp,LOCATION_SZONE,POS_FACEUP,true)
        duel::move_to_field(d, sc, a.tp, a.tp, LOCATION_SZONE as u16, POS_FACEUP as u8, true, None);
    }
}
/// `s.swtg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn swtg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return not e:GetHandler():IsSequence(2) and Duel.GetFieldCard(tp,LOCATION_MZONE,2) end
    if a.chk == Some(0) {
        let c = effect::get_handler(d, a.e).nil();
        return !card::is_sequence(d, c, &[2]) && duel::get_field_card(d, a.tp, LOCATION_MZONE, 2).is_some();
    }
    // Duel.SetPossibleOperationInfo(0,CATEGORY_REMOVE,nil,1,1-tp,LOCATION_HAND)
    duel::set_possible_operation_info(d, 0, CATEGORY_REMOVE, None, 1, 1 - a.tp, LOCATION_HAND as i32);
    true
}
/// `s.swop(e,tp,eg,ep,ev,re,r,rp)`
fn swop(d: &mut Duel, a: &EffArgs) {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // local cc=Duel.GetFieldCard(tp,LOCATION_MZONE,2)
    let cc = duel::get_field_card(d, a.tp, LOCATION_MZONE, 2);
    // if c:IsRelateToEffect(e) and c:IsInMainMZone(tp) and cc and c~=cc and Duel.SwapSequence(c,cc) then
    if !card::is_relate_to_effect(d, c, a.e) || !card::is_in_main_m_zone(d, c, Some(a.tp)) {
        return;
    }
    let cc = match cc {
        Some(cc) => cc,
        None => return,
    };
    if c == cc || !duel::swap_sequence(d, c, cc) {
        return;
    }
    // local g=Duel.GetMatchingGroup(Card.IsAbleToRemove,tp,0,LOCATION_HAND,nil)
    let g = duel::get_matching_group(d, Some(&|d, c| card::is_able_to_remove(d, c, None, None, None)), a.tp, 0, LOCATION_HAND, NO_EX);
    // if #g==0 or not Duel.SelectYesNo(tp,aux.Stringid(id,3)) then return end
    if g.is_empty() || !duel::select_yes_no(d, a.tp, aux::stringid(ID, 3)) {
        return;
    }
    // local rg=g:RandomSelect(tp,1)
    let rg = group::random_select(d, &g, a.tp, 1);
    // if #rg>0 then
    if !rg.is_empty() {
        // Duel.BreakEffect()
        duel::break_effect(d);
        // aux.RemoveUntil(rg,POS_FACEUP,REASON_EFFECT|REASON_TEMPORARY,PHASE_END,id,e,tp,function(ag,e,tp) Duel.SendtoHand(ag,nil,REASON_EFFECT) end)
        aux::remove_until(
            d,
            rg,
            Some(POS_FACEUP as u8),
            REASON_EFFECT | REASON_TEMPORARY,
            PHASE_END,
            ID,
            a.e,
            a.tp,
            Some(std::sync::Arc::new(|d: &mut Duel, ag: &CardSet, _a: &EffArgs| {
                duel::send_to_hand(d, ag.clone(), None, REASON_EFFECT, None);
                true
            })),
            None,
            None,
            None,
            None,
            None,
        );
    }
}
