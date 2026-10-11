//! Battlewasp - Rapier the Onslaught
use super::prelude::*;

pub const ID: u32 = 56146300;
pub const INFO: ScriptInfo = ScriptInfo { listed_names: &[67441879], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Place 1 "Battlewasp - Wind" from your Deck face-up in your Spell & Trap Zone
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_SPECIAL_SUMMON)
    effect::set_category(d, e1, CATEGORY_SPECIAL_SUMMON);
    // e1:SetType(EFFECT_TYPE_IGNITION)
    effect::set_type(d, e1, EFFECT_TYPE_IGNITION);
    // e1:SetRange(LOCATION_HAND)
    effect::set_range(d, e1, LOCATION_HAND);
    // e1:SetCountLimit(1,id)
    effect::set_count_limit(d, e1, 1, ID, 0, 0);
    // e1:SetCondition(s.plcon)
    effect::set_condition(d, e1, Some(condition(plcon)));
    // e1:SetCost(s.plcost)
    effect::set_cost(d, e1, Some(cost_target(plcost)));
    // e1:SetTarget(s.pltg)
    effect::set_target(d, e1, Some(cost_target(pltg)));
    // e1:SetOperation(s.plop)
    effect::set_operation(d, e1, Some(operation(plop)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // Reduce the Level of 1 Insect monster you control by 1
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e2, aux::stringid(ID, 1));
    // e2:SetCategory(CATEGORY_LVCHANGE)
    effect::set_category(d, e2, CATEGORY_LVCHANGE);
    // e2:SetType(EFFECT_TYPE_IGNITION)
    effect::set_type(d, e2, EFFECT_TYPE_IGNITION);
    // e2:SetRange(LOCATION_GRAVE)
    effect::set_range(d, e2, LOCATION_GRAVE);
    // e2:SetCountLimit(1,{id,1})
    effect::set_count_limit(d, e2, 1, ID, 1, 0);
    // e2:SetCost(Cost.SelfBanish)
    effect::set_cost(d, e2, Some(cost::self_banish()));
    // e2:SetTarget(s.lvtg)
    effect::set_target(d, e2, Some(cost_target(lvtg)));
    // e2:SetOperation(s.lvop)
    effect::set_operation(d, e2, Some(operation(lvop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
}
/// `s.plcon(e,tp,eg,ep,ev,re,r,rp)`
fn plcon(d: &mut Duel, a: &EffArgs) -> bool {
    // local g=Duel.GetMatchingGroup(Card.IsFaceup,tp,LOCATION_MZONE,0,nil)
    let g = duel::get_matching_group(d, Some(&|d, c| card::is_faceup(d, c)), a.tp, LOCATION_MZONE, 0, NO_EX);
    // return #g==0 or g:FilterCount(Card.IsRace,nil,RACE_INSECT)==#g
    g.is_empty() || group::filter_count(d, &g, &|d, c| card::is_race(d, c, RACE_INSECT), NO_EX) == g.len()
}
/// `s.plcostfilter(c)`
fn plcostfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsRace(RACE_INSECT) and c:IsDiscardable()
    card::is_race(d, c, RACE_INSECT) && card::is_discardable(d, c, None, None)
}
/// `s.plcost(e,tp,eg,ep,ev,re,r,rp,chk)`
fn plcost(d: &mut Duel, a: &EffArgs) -> bool {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if chk==0 then return Duel.IsExistingMatchingCard(s.plcostfilter,tp,LOCATION_HAND,0,1,c) end
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, Some(&plcostfilter), a.tp, LOCATION_HAND, 0, 1, c.into());
    }
    // Duel.DiscardHand(tp,s.plcostfilter,1,1,REASON_COST|REASON_DISCARD,c)
    duel::discard_hand(d, a.tp, Some(&plcostfilter), 1, 1, REASON_COST | REASON_DISCARD, c.into());
    true
}
/// `s.plfilter(c,tp)`
fn plfilter(d: &mut Duel, c: CardId, tp: u8) -> bool {
    // return c:IsCode(67441879) and not c:IsForbidden() and c:CheckUniqueOnField(tp)
    card::is_code(d, c, &[67441879]) && !card::is_forbidden(d, c) && card::check_unique_on_field(d, c, tp, Some(LOCATION_ONFIELD), None)
}
/// `s.pltg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn pltg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if chk==0 then return Duel.GetLocationCount(tp,LOCATION_SZONE)>0
    //     and Duel.GetLocationCount(tp,LOCATION_MZONE)>0
    //     and c:IsCanBeSpecialSummoned(e,0,tp,false,false)
    //     and Duel.IsExistingMatchingCard(s.plfilter,tp,LOCATION_DECK,0,1,nil,tp) end
    if a.chk == Some(0) {
        return duel::get_location_count(d, tp, LOCATION_SZONE as u8, None, None, None).0 > 0
            && duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 > 0
            && card::is_can_be_special_summoned(d, c, a.e, 0, tp, false, false, None, None, None)
            && duel::is_existing_matching_card(d, Some(&|d, cc| plfilter(d, cc, tp)), tp, LOCATION_DECK, 0, 1, NO_EX);
    }
    // Duel.SetPossibleOperationInfo(0,CATEGORY_SPECIAL_SUMMON,c,1,tp,0)
    duel::set_possible_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, Some(CardSet::from_card(c)), 1, tp, 0);
    true
}
/// `s.plop(e,tp,eg,ep,ev,re,r,rp)`
fn plop(d: &mut Duel, a: &EffArgs) {
    let e = a.e;
    let tp = a.tp;
    // local c=e:GetHandler()
    let c = effect::get_handler(d, e).nil();
    // if Duel.GetLocationCount(tp,LOCATION_SZONE)>0 then
    if duel::get_location_count(d, tp, LOCATION_SZONE as u8, None, None, None).0 > 0 {
        // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_TOFIELD)
        duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_TOFIELD as u64);
        // local tc=Duel.SelectMatchingCard(tp,s.plfilter,tp,LOCATION_DECK,0,1,1,nil,tp):GetFirst()
        let g = duel::select_matching_card(d, tp, Some(&|d, cc| plfilter(d, cc, tp)), tp, LOCATION_DECK, 0, 1, 1, false, NO_EX).nil();
        let tc = group::get_first(d, &g);
        // if tc and Duel.MoveToField(tc,tp,tp,LOCATION_SZONE,POS_FACEUP,true) and c:IsRelateToEffect(e) then
        if let Some(tc) = tc {
            if duel::move_to_field(d, tc, tp, tp, LOCATION_SZONE as u16, POS_FACEUP as u8, true, None) && card::is_relate_to_effect(d, c, e) {
                // Duel.BreakEffect()
                duel::break_effect(d);
                // Duel.SpecialSummon(c,0,tp,tp,false,false,POS_FACEUP)
                duel::special_summon(d, c, 0, tp, tp, false, false, POS_FACEUP as u8, None);
            }
        }
    }
    // Cannot Special Summon from the Extra Deck for the rest of this turn, except Insect monsters
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,2))
    effect::set_description(d, e1, aux::stringid(ID, 2));
    // e1:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e1, EFFECT_TYPE_FIELD);
    // e1:SetProperty(EFFECT_FLAG_PLAYER_TARGET+EFFECT_FLAG_CLIENT_HINT)
    effect::set_property(d, e1, EFFECT_FLAG_PLAYER_TARGET + EFFECT_FLAG_CLIENT_HINT, 0);
    // e1:SetCode(EFFECT_CANNOT_SPECIAL_SUMMON)
    effect::set_code(d, e1, EFFECT_CANNOT_SPECIAL_SUMMON);
    // e1:SetTargetRange(1,0)
    effect::set_target_range(d, e1, 1, 0);
    // e1:SetTarget(function(e,c) return c:IsLocation(LOCATION_EXTRA) and not c:IsRace(RACE_INSECT) end)
    effect::set_target(d, e1, Some(effect_card_fn(|d, _e, cc| card::is_location(d, cc, LOCATION_EXTRA) && !card::is_race(d, cc, RACE_INSECT))));
    // e1:SetReset(RESET_PHASE|PHASE_END)
    effect::set_reset(d, e1, RESET_PHASE | PHASE_END, 1);
    // Duel.RegisterEffect(e1,tp)
    duel::register_effect(d, e1, tp);
    // aux.addTempLizardCheck(c,tp,function(e,c) return not c:IsOriginalRace(RACE_INSECT) end)
    aux::add_temp_lizard_check(d, c, tp, Some(effect_card_fn(|d, _e, cc| !card::is_original_race(d, cc, RACE_INSECT))), None, None, None, None);
}
/// `s.lvfilter(c)`
fn lvfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsRace(RACE_INSECT) and c:IsLevelAbove(2) and c:IsFaceup()
    card::is_race(d, c, RACE_INSECT) && card::is_level_above(d, c, 2) && card::is_faceup(d, c)
}
/// `s.lvtg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn lvtg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return Duel.IsExistingMatchingCard(s.lvfilter,tp,LOCATION_MZONE,0,1,nil) end
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, Some(&lvfilter), a.tp, LOCATION_MZONE, 0, 1, NO_EX);
    }
    true
}
/// `s.lvop(e,tp,eg,ep,ev,re,r,rp)`
fn lvop(d: &mut Duel, a: &EffArgs) {
    let e = a.e;
    let tp = a.tp;
    // Duel.Hint(HINT_SELECTMSG,tp,aux.Stringid(id,3))
    duel::hint(d, HINT_SELECTMSG as u8, tp, aux::stringid(ID, 3));
    // local sc=Duel.SelectMatchingCard(tp,s.lvfilter,tp,LOCATION_MZONE,0,1,1,nil):GetFirst()
    let g = duel::select_matching_card(d, tp, Some(&lvfilter), tp, LOCATION_MZONE, 0, 1, 1, false, NO_EX).nil();
    let sc = group::get_first(d, &g);
    // if sc then
    if let Some(sc) = sc {
        // Duel.HintSelection(sc)
        duel::hint_selection(d, CardSet::from_card(sc), true);
        // local e1=Effect.CreateEffect(e:GetHandler())
        let c = effect::get_handler(d, e).nil();
        let e1 = effect::create_effect(d, c);
        // e1:SetType(EFFECT_TYPE_SINGLE)
        effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
        // e1:SetProperty(EFFECT_FLAG_CANNOT_DISABLE)
        effect::set_property(d, e1, EFFECT_FLAG_CANNOT_DISABLE, 0);
        // e1:SetCode(EFFECT_UPDATE_LEVEL)
        effect::set_code(d, e1, EFFECT_UPDATE_LEVEL);
        // e1:SetValue(-1)
        effect::set_value(d, e1, -1);
        // e1:SetReset(RESET_EVENT|RESETS_STANDARD)
        effect::set_reset(d, e1, RESET_EVENT | RESETS_STANDARD, 1);
        // sc:RegisterEffect(e1)
        card::register_effect(d, sc, e1, false);
    }
}
