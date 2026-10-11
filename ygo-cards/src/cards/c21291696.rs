//! Electrode Beast Cation
use super::prelude::*;

pub const ID: u32 = 21291696;

pub const INFO: ScriptInfo = ScriptInfo { listed_names: &[58680635], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Add to the hand 1 "Electrode Beast Anion" from the Deck or GY and Normal Summon 1 Level 4 or lower Thunder monster
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_TOHAND+CATEGORY_SEARCH+CATEGORY_SUMMON)
    effect::set_category(d, e1, CATEGORY_TOHAND + CATEGORY_SEARCH + CATEGORY_SUMMON);
    // e1:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_O);
    // e1:SetProperty(EFFECT_FLAG_DELAY)
    effect::set_property(d, e1, EFFECT_FLAG_DELAY, 0);
    // e1:SetCode(EVENT_SUMMON_SUCCESS)
    effect::set_code(d, e1, EVENT_SUMMON_SUCCESS);
    // e1:SetCountLimit(1,id)
    effect::set_count_limit(d, e1, 1, ID, 0, 0);
    // e1:SetTarget(s.thtg)
    effect::set_target(d, e1, Some(cost_target(thtg)));
    // e1:SetOperation(s.thop)
    effect::set_operation(d, e1, Some(operation(thop)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // Change its level
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e2, aux::stringid(ID, 1));
    // e2:SetCategory(CATEGORY_LVCHANGE)
    effect::set_category(d, e2, CATEGORY_LVCHANGE);
    // e2:SetType(EFFECT_TYPE_IGNITION)
    effect::set_type(d, e2, EFFECT_TYPE_IGNITION);
    // e2:SetProperty(EFFECT_FLAG_CARD_TARGET)
    effect::set_property(d, e2, EFFECT_FLAG_CARD_TARGET, 0);
    // e2:SetRange(LOCATION_MZONE)
    effect::set_range(d, e2, LOCATION_MZONE);
    // e2:SetCountLimit(1,{id,1})
    effect::set_count_limit(d, e2, 1, ID, 1, 0);
    // e2:SetTarget(s.lvtg)
    effect::set_target(d, e2, Some(cost_target(lvtg)));
    // e2:SetOperation(s.lvop)
    effect::set_operation(d, e2, Some(operation(lvop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
}
/// `s.thfilter(c)`
fn thfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsCode(58680635) and c:IsAbleToHand()
    card::is_code(d, c, &[58680635]) && card::is_able_to_hand(d, c, None)
}
/// `s.thtg(e,tp,...,chk)`
fn thtg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return Duel.IsExistingMatchingCard(s.thfilter,tp,LOCATION_DECK|LOCATION_GRAVE,0,1,nil) end
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, Some(&thfilter), a.tp, LOCATION_DECK | LOCATION_GRAVE, 0, 1, NO_EX);
    }
    // Duel.SetOperationInfo(0,CATEGORY_TOHAND,nil,1,tp,LOCATION_DECK|LOCATION_GRAVE)
    duel::set_operation_info(d, 0, CATEGORY_TOHAND, None, 1, a.tp, (LOCATION_DECK | LOCATION_GRAVE) as i32);
    // Duel.SetPossibleOperationInfo(0,CATEGORY_SUMMON,nil,1,tp,LOCATION_HAND|LOCATION_MZONE)
    duel::set_possible_operation_info(d, 0, CATEGORY_SUMMON, None, 1, a.tp, (LOCATION_HAND | LOCATION_MZONE) as i32);
    true
}
/// `s.nsfilter(c)`
fn nsfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsLevelBelow(4) and c:IsRace(RACE_THUNDER) and c:IsSummonable(true,nil)
    card::is_level_below(d, c, 4) && card::is_race(d, c, RACE_THUNDER as u64) && card::is_summonable(d, c, true, None, 0, None)
}
/// `s.thop(e,tp,...)`
fn thop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // Cannot Special Summon from the Extra Deck, except LIGHT Xyz Monsters
    // local e1=Effect.CreateEffect(e:GetHandler())
    let h = effect::get_handler(d, a.e).nil();
    let e1 = effect::create_effect(d, h);
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
    // e1:SetTarget(function(e,c) return c:IsLocation(LOCATION_EXTRA) and not (c:IsAttribute(ATTRIBUTE_LIGHT) and c:IsType(TYPE_XYZ)) end)
    effect::set_target(d, e1, Some(effect_card_fn(|d, _e, c| {
        card::is_location(d, c, LOCATION_EXTRA) && !(card::is_attribute(d, c, ATTRIBUTE_LIGHT) && card::is_type(d, c, TYPE_XYZ))
    })));
    // e1:SetReset(RESET_PHASE|PHASE_END)
    effect::set_reset(d, e1, RESET_PHASE | PHASE_END, 1);
    // Duel.RegisterEffect(e1,tp)
    duel::register_effect(d, e1, tp);
    // Clock Lizard check
    // aux.addTempLizardCheck(e:GetHandler(),tp,function(e,c) return not (c:IsOriginalAttribute(ATTRIBUTE_LIGHT) and c:IsOriginalType(TYPE_XYZ)) end)
    aux::add_temp_lizard_check(
        d,
        h,
        tp,
        Some(effect_card_fn(|d, _e, c| !(card::is_original_attribute(d, c, ATTRIBUTE_LIGHT) && card::is_original_type(d, c, TYPE_XYZ)))),
        None,
        None,
        None,
        None,
    );
    // Add to the hand 1 "Electrode Beast Anion" from the Deck or GY
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_ATOHAND)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_ATOHAND as u64);
    // local g=Duel.SelectMatchingCard(tp,aux.NecroValleyFilter(s.thfilter),tp,LOCATION_DECK|LOCATION_GRAVE,0,1,1,nil)
    let g = duel::select_matching_card(d, tp, Some(&aux::necro_valley_filter(thfilter)), tp, LOCATION_DECK | LOCATION_GRAVE, 0, 1, 1, false, NO_EX).nil();
    // if #g>0 and Duel.SendtoHand(g,nil,REASON_EFFECT)>0 then
    if !g.is_empty() && duel::send_to_hand(d, g.clone(), None, REASON_EFFECT, None) > 0 {
        // Duel.ConfirmCards(1-tp,g)
        duel::confirm_cards(d, 1 - tp, g.clone());
        // Duel.ShuffleHand(tp)
        duel::shuffle_hand(d, tp);
        // if Duel.IsExistingMatchingCard(s.nsfilter,tp,LOCATION_HAND|LOCATION_MZONE,0,1,nil) and Duel.SelectYesNo(tp,aux.Stringid(id,3)) then
        if duel::is_existing_matching_card(d, Some(&nsfilter), tp, LOCATION_HAND | LOCATION_MZONE, 0, 1, NO_EX) && duel::select_yes_no(d, tp, aux::stringid(ID, 3)) {
            // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_SUMMON)
            duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_SUMMON as u64);
            // local sc=Duel.SelectMatchingCard(tp,s.nsfilter,tp,LOCATION_HAND|LOCATION_MZONE,0,1,1,nil):GetFirst()
            let sg = duel::select_matching_card(d, tp, Some(&nsfilter), tp, LOCATION_HAND | LOCATION_MZONE, 0, 1, 1, false, NO_EX).nil();
            let sc = group::get_first(d, &sg);
            // if sc then
            if let Some(sc) = sc {
                // Duel.BreakEffect()
                duel::break_effect(d);
                // Duel.Summon(tp,sc,true,nil)
                duel::summon(d, tp, sc, true, None, 0, None);
            }
        }
    }
}
/// `s.lvfilter(c,lv)`
fn lvfilter(d: &mut Duel, c: CardId, lv: u32) -> bool {
    // return c:IsFaceup() and c:IsRace(RACE_THUNDER) and c:HasLevel() and not c:IsLevel(lv)
    card::is_faceup(d, c) && card::is_race(d, c, RACE_THUNDER as u64) && card::has_level(d, c) && !card::is_level(d, c, &[lv])
}
/// `s.lvtg(e,tp,...,chk,chkc)`
fn lvtg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // local lv=c:GetLevel()
    let lv = card::get_level(d, c);
    // if chkc then return chkc:IsControler(tp) and chkc:IsLocation(LOCATION_MZONE) and chkc~=c and s.lvfilter(chkc,lv) end
    if let Some(chkc) = a.chkc() {
        return card::is_controler(d, chkc, tp) && card::is_location(d, chkc, LOCATION_MZONE) && chkc != c && lvfilter(d, chkc, lv);
    }
    // if chk==0 then return Duel.IsExistingTarget(s.lvfilter,tp,LOCATION_MZONE,0,1,c,lv) end
    if a.chk == Some(0) {
        return duel::is_existing_target(d, Some(&|d, cc| lvfilter(d, cc, lv)), tp, LOCATION_MZONE, 0, 1, c.into());
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_TARGET)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_TARGET as u64);
    // Duel.SelectTarget(tp,s.lvfilter,tp,LOCATION_MZONE,0,1,1,c,lv)
    duel::select_target(d, tp, Some(&|d, cc| lvfilter(d, cc, lv)), tp, LOCATION_MZONE, 0, 1, 1, false, c.into());
    true
}
/// `s.lvop(e,tp,...)`
fn lvop(d: &mut Duel, a: &EffArgs) {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // local tc=Duel.GetFirstTarget()
    let tc = duel::first_target(d).nil();
    // if tc:IsFaceup() and tc:IsRelateToEffect(e) and c:IsFaceup() and c:IsRelateToEffect(e) then
    if card::is_faceup(d, tc) && card::is_relate_to_effect(d, tc, a.e) && card::is_faceup(d, c) && card::is_relate_to_effect(d, c, a.e) {
        // This card's Level becomes the same as that monster's
        // local e1=Effect.CreateEffect(c)
        let e1 = effect::create_effect(d, c);
        // e1:SetType(EFFECT_TYPE_SINGLE)
        effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
        // e1:SetCode(EFFECT_CHANGE_LEVEL)
        effect::set_code(d, e1, EFFECT_CHANGE_LEVEL);
        // e1:SetValue(tc:GetLevel())
        let lv = card::get_level(d, tc) as i64;
        effect::set_value(d, e1, lv);
        // e1:SetReset(RESETS_STANDARD_DISABLE_PHASE_END)
        effect::set_reset(d, e1, RESETS_STANDARD_DISABLE_PHASE_END, 1);
        // c:RegisterEffect(e1)
        card::register_effect(d, c, e1, false);
    }
}
