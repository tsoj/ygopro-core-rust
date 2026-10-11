//! Elfnote Lucina
use super::prelude::*;

pub const ID: u32 = 13597785;

pub const INFO: ScriptInfo = ScriptInfo { listed_names: &[ID], listed_series: &[SET_ELFNOTE], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // You can Special Summon this card (from your hand) to your center Main Monster Zone
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
    // Add 1 "Elfnote" monster from your Deck to your hand, except "Elfnote Lucina"
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e2, aux::stringid(ID, 1));
    // e2:SetCategory(CATEGORY_TOHAND+CATEGORY_SEARCH)
    effect::set_category(d, e2, CATEGORY_TOHAND | CATEGORY_SEARCH);
    // e2:SetType(EFFECT_TYPE_IGNITION)
    effect::set_type(d, e2, EFFECT_TYPE_IGNITION);
    // e2:SetRange(LOCATION_MZONE)
    effect::set_range(d, e2, LOCATION_MZONE);
    // e2:SetCountLimit(1,{id,1})
    effect::set_count_limit(d, e2, 1, ID, 1, 0);
    // e2:SetTarget(s.thtg)
    effect::set_target(d, e2, Some(cost_target(thtg)));
    // e2:SetOperation(s.thop)
    effect::set_operation(d, e2, Some(operation(thop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
    // Switch the locations of this card in your Main Monster Zone and the monster in your center Main Monster Zone
    // local e3=Effect.CreateEffect(c)
    let e3 = effect::create_effect(d, c);
    // e3:SetDescription(aux.Stringid(id,2))
    effect::set_description(d, e3, aux::stringid(ID, 2));
    // e3:SetCategory(CATEGORY_TOHAND)
    effect::set_category(d, e3, CATEGORY_TOHAND);
    // e3:SetType(EFFECT_TYPE_QUICK_O)
    effect::set_type(d, e3, EFFECT_TYPE_QUICK_O);
    // e3:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e3, EVENT_FREE_CHAIN);
    // e3:SetRange(LOCATION_MMZONE)
    effect::set_range(d, e3, LOCATION_MMZONE);
    // e3:SetCountLimit(1,{id,2})
    effect::set_count_limit(d, e3, 1, ID, 2, 0);
    // e3:SetCondition(function(e,tp) return Duel.IsTurnPlayer(1-tp) end)
    effect::set_condition(d, e3, Some(lua_fn(|d, a: Args| duel::is_turn_player(d, 1 - a.p(1)))));
    // e3:SetTarget(s.swtg)
    effect::set_target(d, e3, Some(cost_target(swtg)));
    // e3:SetOperation(s.swop)
    effect::set_operation(d, e3, Some(operation(swop)));
    // e3:SetHintTiming(0,TIMING_STANDBY_PHASE|TIMING_MAIN_END|TIMINGS_CHECK_MONSTER_E)
    effect::set_hint_timing(d, e3, 0, Some(TIMING_STANDBY_PHASE | TIMING_MAIN_END | TIMINGS_CHECK_MONSTER_E));
    // c:RegisterEffect(e3)
    card::register_effect(d, c, e3, false);
}
/// `s.thfilter(c)`
fn thfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsSetCard(SET_ELFNOTE) and c:IsMonster() and not c:IsCode(id) and c:IsAbleToHand()
    card::is_set_card(d, c, &[SET_ELFNOTE as u16])
        && card::is_monster(d, c)
        && !card::is_code(d, c, &[ID])
        && card::is_able_to_hand(d, c, None)
}
/// `s.thtg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn thtg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return Duel.IsExistingMatchingCard(s.thfilter,tp,LOCATION_DECK,0,1,nil) end
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, Some(&thfilter), a.tp, LOCATION_DECK, 0, 1, NO_EX);
    }
    // Duel.SetOperationInfo(0,CATEGORY_TOHAND,nil,1,tp,LOCATION_DECK)
    duel::set_operation_info(d, 0, CATEGORY_TOHAND, None, 1, a.tp, LOCATION_DECK as i32);
    true
}
/// `s.thop(e,tp,eg,ep,ev,re,r,rp)`
fn thop(d: &mut Duel, a: &EffArgs) {
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_ATOHAND)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_ATOHAND as u64);
    // local g=Duel.SelectMatchingCard(tp,s.thfilter,tp,LOCATION_DECK,0,1,1,nil)
    let g = duel::select_matching_card(d, a.tp, Some(&thfilter), a.tp, LOCATION_DECK, 0, 1, 1, false, NO_EX).nil();
    // if #g>0 then
    if !g.is_empty() {
        // Duel.SendtoHand(g,nil,REASON_EFFECT)
        duel::send_to_hand(d, g.clone(), None, REASON_EFFECT, None);
        // Duel.ConfirmCards(1-tp,g)
        duel::confirm_cards(d, 1 - a.tp, g);
    }
}
/// `s.swtg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn swtg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return not e:GetHandler():IsSequence(2) and Duel.GetFieldCard(tp,LOCATION_MZONE,2) end
    if a.chk == Some(0) {
        let c = effect::get_handler(d, a.e).nil();
        return !card::is_sequence(d, c, &[2]) && duel::get_field_card(d, a.tp, LOCATION_MZONE, 2).is_some();
    }
    // Duel.SetPossibleOperationInfo(0,CATEGORY_TOHAND,nil,1,1-tp,LOCATION_MZONE)
    duel::set_possible_operation_info(d, 0, CATEGORY_TOHAND, None, 1, 1 - a.tp, LOCATION_MZONE as i32);
    true
}
/// `s.rthfilter(c)`
fn rthfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsLevelBelow(6) and c:IsFaceup() and c:IsAbleToHand()
    card::is_level_below(d, c, 6) && card::is_faceup(d, c) && card::is_able_to_hand(d, c, None)
}
/// `s.swop(e,tp,eg,ep,ev,re,r,rp)`
fn swop(d: &mut Duel, a: &EffArgs) {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // local cc=Duel.GetFieldCard(tp,LOCATION_MZONE,2)
    let cc = duel::get_field_card(d, a.tp, LOCATION_MZONE, 2);
    // if c:IsRelateToEffect(e) and c:IsInMainMZone(tp) and cc and c~=cc and Duel.SwapSequence(c,cc) then
    if card::is_relate_to_effect(d, c, a.e) && card::is_in_main_m_zone(d, c, Some(a.tp)) {
        if let Some(cc) = cc {
            if c != cc && duel::swap_sequence(d, c, cc) {
                // local g=Duel.GetMatchingGroup(s.rthfilter,tp,0,LOCATION_MZONE,nil)
                let g = duel::get_matching_group(d, Some(&rthfilter), a.tp, 0, LOCATION_MZONE, NO_EX);
                // if #g==0 or not Duel.SelectYesNo(tp,aux.Stringid(id,3)) then return end
                if g.is_empty() || !duel::select_yes_no(d, a.tp, aux::stringid(ID, 3)) {
                    return;
                }
                // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_RTOHAND)
                duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_RTOHAND as u64);
                // local sg=g:Select(tp,1,1,nil)
                let sg = group::select(d, &g, a.tp, 1, 1, false, NO_EX).nil();
                // if #sg>0 then
                if !sg.is_empty() {
                    // Duel.HintSelection(sg)
                    duel::hint_selection(d, sg.clone(), true);
                    // Duel.BreakEffect()
                    duel::break_effect(d);
                    // Duel.SendtoHand(sg,nil,REASON_EFFECT)
                    duel::send_to_hand(d, sg, None, REASON_EFFECT, None);
                }
            }
        }
    }
}
