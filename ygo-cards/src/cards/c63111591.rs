//! Corpse-Seller Crow
use super::prelude::*;

pub const ID: u32 = 63111591;
pub const INFO: ScriptInfo = ScriptInfo { listed_names: &[ID], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Send cards from the top of your Deck to the GY up to the number of Traps in the GYs with different names +1 (max. 4), then you can apply 1 of these effects
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_TOGRAVE+CATEGORY_SPECIAL_SUMMON+CATEGORY_SET)
    effect::set_category(d, e1, CATEGORY_TOGRAVE | CATEGORY_SPECIAL_SUMMON | CATEGORY_SET);
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    // e1:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    // e1:SetTarget(s.target)
    effect::set_target(d, e1, Some(cost_target(target)));
    // e1:SetOperation(s.activate)
    effect::set_operation(d, e1, Some(operation(activate)));
    // e1:SetHintTiming(0,TIMING_STANDBY_PHASE|TIMING_MAIN_END|TIMINGS_CHECK_MONSTER_E)
    effect::set_hint_timing(d, e1, 0, Some(TIMING_STANDBY_PHASE | TIMING_MAIN_END | TIMINGS_CHECK_MONSTER_E));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
fn setfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsSpellTrap() and c:IsSSetable() and not c:IsCode(id)
    card::is_spell_trap(d, c) && card::is_s_setable(d, c, false, None) && !card::is_code(d, c, &[ID])
}
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return Duel.IsPlayerCanDiscardDeck(tp,1) end
    if a.chk == Some(0) {
        return duel::is_player_can_discard_deck(d, a.tp, 1);
    }
    // Duel.SetOperationInfo(0,CATEGORY_DECKDES,nil,0,tp,1)
    duel::set_operation_info(d, 0, CATEGORY_DECKDES, None, 0, a.tp, 1);
    // Duel.SetPossibleOperationInfo(0,CATEGORY_SPECIAL_SUMMON,nil,1,tp,LOCATION_GRAVE)
    duel::set_possible_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, None, 1, a.tp, LOCATION_GRAVE as i32);
    // Duel.SetPossibleOperationInfo(0,CATEGORY_SET,nil,1,tp,LOCATION_GRAVE)
    duel::set_possible_operation_info(d, 0, CATEGORY_SET, None, 1, a.tp, LOCATION_GRAVE as i32);
    true
}
fn activate(d: &mut Duel, a: &EffArgs) {
    // if not Duel.IsPlayerCanDiscardDeck(tp,1) then return end
    if !duel::is_player_can_discard_deck(d, a.tp, 1) {
        return;
    }
    // local trap_count=Duel.GetMatchingGroup(Card.IsTrap,tp,LOCATION_GRAVE,LOCATION_GRAVE,nil):GetClassCount(Card.GetCode)+1
    let trap_group = duel::get_matching_group(d, Some(&|d, c| card::is_trap(d, c)), a.tp, LOCATION_GRAVE as u32, LOCATION_GRAVE as u32, NO_EX);
    let trap_count = group::get_class_count(d, &trap_group, &|d, c| card::get_code(d, c)[0] as i64) as i64 + 1;
    // local deck_count=Duel.GetFieldGroupCount(tp,LOCATION_DECK,0)
    let deck_count = duel::get_field_group_count(d, a.tp, LOCATION_DECK as u32, 0) as i64;
    // local max_count=math.min(trap_count,deck_count,4)
    let max_count = trap_count.min(deck_count).min(4);
    // local mill_count=1
    let mut mill_count: i64 = 1;
    // if max_count>1 then
    if max_count > 1 {
        // mill_count=Duel.AnnounceNumberRange(tp,1,max_count)
        mill_count = duel::announce_number_range(d, a.tp, Some(1), Some(max_count as u32), &[]).nil().0 as i64;
    }
    // if Duel.DiscardDeck(tp,mill_count,REASON_EFFECT)==0 then return end
    if duel::discard_deck(d, a.tp, mill_count as u16, REASON_EFFECT) == 0 {
        return;
    }
    // local og=Duel.GetOperatedGroup():Match(Card.IsLocation,nil,LOCATION_GRAVE)
    let mut og = duel::get_operated_group(d);
    group::match_(d, &mut og, &|d, c| card::is_location(d, c, LOCATION_GRAVE), NO_EX);
    // if #og==0 then return end
    if og.is_empty() {
        return;
    }
    // Special Summon 1 of the monsters sent to the GY
    // local b1=Duel.GetLocationCount(tp,LOCATION_MZONE)>0
    //     and og:IsExists(aux.NecroValleyFilter(Card.IsCanBeSpecialSummoned),1,nil,e,0,tp,false,false)
    let b1 = duel::get_location_count(d, a.tp, LOCATION_MZONE as u8, None, None, None).0 > 0
        && group::is_exists(
            d,
            &og,
            &aux::necro_valley_filter(|d, c| card::is_can_be_special_summoned(d, c, a.e, 0, a.tp, false, false, None, None, None)),
            1,
            NO_EX,
        );
    // Set 1 of the Spells/Traps sent to the GY, except "Corpse-Seller Crow", and if you Set a Trap or Quick-Play Spell, it can be activated this turn
    // local b2=og:IsExists(aux.NecroValleyFilter(s.setfilter),1,nil)
    let b2 = group::is_exists(d, &og, &aux::necro_valley_filter(|d, c| setfilter(d, c)), 1, NO_EX);
    // if not ((b1 or b2) and Duel.SelectYesNo(tp,aux.Stringid(id,1))) then return end
    if !((b1 || b2) && duel::select_yes_no(d, a.tp, aux::stringid(ID, 1))) {
        return;
    }
    // local op=Duel.SelectEffect(tp, {b1,aux.Stringid(id,2)}, {b2,aux.Stringid(id,3)})
    let op = duel::select_effect(d, a.tp, &[(b1, aux::stringid(ID, 2)), (b2, aux::stringid(ID, 3))]);
    if op == Some(1) {
        // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_SPSUMMON)
        duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_SPSUMMON as u64);
        // local g=og:FilterSelect(tp,aux.NecroValleyFilter(Card.IsCanBeSpecialSummoned),1,1,nil,e,0,tp,false,false)
        let g = group::filter_select(
            d,
            &og,
            a.tp,
            &aux::necro_valley_filter(|d, c| card::is_can_be_special_summoned(d, c, a.e, 0, a.tp, false, false, None, None, None)),
            1,
            1,
            false,
            NO_EX,
        )
        .nil();
        // if #g>0 then
        if !g.is_empty() {
            // Duel.BreakEffect()
            duel::break_effect(d);
            // Duel.SpecialSummon(g,0,tp,tp,false,false,POS_FACEUP)
            duel::special_summon(d, g, 0, a.tp, a.tp, false, false, POS_FACEUP as u8, None);
        }
    } else if op == Some(2) {
        // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_SET)
        duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_SET as u64);
        // local sc=og:FilterSelect(tp,aux.NecroValleyFilter(s.setfilter),1,1,nil):GetFirst()
        let sel = group::filter_select(d, &og, a.tp, &aux::necro_valley_filter(|d, c| setfilter(d, c)), 1, 1, false, NO_EX).nil();
        // if not sc then return end
        let Some(sc) = group::get_first(d, &sel) else {
            return;
        };
        // Duel.BreakEffect()
        duel::break_effect(d);
        // if Duel.SSet(tp,sc)>0 and (sc:IsTrap() or sc:IsQuickPlaySpell()) then
        if duel::s_set(d, a.tp, sc, None, true) > 0 && (card::is_trap(d, sc) || card::is_quick_play_spell(d, sc)) {
            // local eff_code=sc:IsTrap() and EFFECT_TRAP_ACT_IN_SET_TURN or EFFECT_QP_ACT_IN_SET_TURN
            let eff_code = if card::is_trap(d, sc) { EFFECT_TRAP_ACT_IN_SET_TURN } else { EFFECT_QP_ACT_IN_SET_TURN };
            // local e1=Effect.CreateEffect(e:GetHandler())
            let h = effect::get_handler(d, a.e).nil();
            let e1 = effect::create_effect(d, h);
            // e1:SetDescription(aux.Stringid(id,4))
            effect::set_description(d, e1, aux::stringid(ID, 4));
            // e1:SetType(EFFECT_TYPE_SINGLE)
            effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
            // e1:SetProperty(EFFECT_FLAG_SET_AVAILABLE+EFFECT_FLAG_CLIENT_HINT)
            effect::set_property(d, e1, EFFECT_FLAG_SET_AVAILABLE + EFFECT_FLAG_CLIENT_HINT, 0);
            // e1:SetCode(eff_code)
            effect::set_code(d, e1, eff_code);
            // e1:SetReset(RESETS_STANDARD_PHASE_END)
            effect::set_reset(d, e1, RESETS_STANDARD_PHASE_END, 1);
            // sc:RegisterEffect(e1)
            card::register_effect(d, sc, e1, false);
        }
    }
}
