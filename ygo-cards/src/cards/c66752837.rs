//! Keeper of the Shrine
use super::prelude::*;

pub const ID: u32 = 66752837;

pub const INFO: ScriptInfo = ScriptInfo { listed_names: &[ID], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Double tribute for the Tribute Summon of Dragon monsters
    let e1 = effect::create_effect(d, c);
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
    effect::set_code(d, e1, EFFECT_DOUBLE_TRIBUTE);
    effect::set_value_fn(d, e1, effect_card_fn(val));
    card::register_effect(d, c, e1, false);
    // Special Summon itself from the GY and add 1 Dragon monster from the GY to the hand
    let e2 = effect::create_effect(d, c);
    effect::set_category(d, e2, CATEGORY_SPECIAL_SUMMON + CATEGORY_TOHAND);
    effect::set_type(d, e2, EFFECT_TYPE_FIELD + EFFECT_TYPE_TRIGGER_O);
    effect::set_code(d, e2, EVENT_TO_GRAVE);
    effect::set_range(d, e2, LOCATION_HAND | LOCATION_GRAVE);
    effect::set_property(d, e2, EFFECT_FLAG_DAMAGE_STEP + EFFECT_FLAG_DELAY, 0);
    effect::set_count_limit(d, e2, 1, ID, 0, 0);
    effect::set_condition(d, e2, Some(condition(spcon)));
    effect::set_target(d, e2, Some(cost_target(sptg)));
    effect::set_operation(d, e2, Some(operation(spop)));
    card::register_effect(d, c, e2, false);
}
/// `function (e,c) return c:IsRace(RACE_DRAGON) end` (the value of the double tribute effect)
fn val(d: &mut Duel, _e: EffectId, c: CardId) -> bool {
    card::is_race(d, c, RACE_DRAGON)
}
/// `s.cfilter(c)`
fn cfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsRace(RACE_DRAGON) and c:GetPreviousRaceOnField()==RACE_DRAGON
    //     and c:IsPreviousPosition(POS_FACEUP) and c:IsPreviousLocation(LOCATION_MZONE)
    //     and c:IsReason(REASON_EFFECT|REASON_BATTLE) and not c:IsCode(id)
    card::is_race(d, c, RACE_DRAGON)
        && card::get_previous_race_on_field(d, c) == RACE_DRAGON
        && card::is_previous_position(d, c, POS_FACEUP)
        && card::is_previous_location(d, c, LOCATION_MZONE)
        && card::is_reason(d, c, REASON_EFFECT | REASON_BATTLE)
        && !card::is_code(d, c, &[ID])
}
/// `s.spcon(e,tp,eg,ep,ev,re,r,rp)`
fn spcon(d: &mut Duel, a: &EffArgs) -> bool {
    let h = effect::get_handler(d, a.e).nil();
    // if eg:IsContains(e:GetHandler()) then return false end
    if group::is_contains(&a.eg.clone().nil(), h) {
        return false;
    }
    // local g=eg:Filter(s.cfilter,nil)
    let g = group::filter(d, &a.eg.clone().nil(), &cfilter, NO_EX);
    // if #g==0 then return false end
    if g.is_empty() {
        return false;
    }
    // e:SetLabel(0)
    effect::set_label(d, a.e, &[0]);
    // if g:IsExists(Card.IsType,1,nil,TYPE_NORMAL) then
    if group::is_exists(d, &g, &|d, c| card::is_type(d, c, TYPE_NORMAL), 1, NO_EX) {
        // e:SetLabel(1)
        effect::set_label(d, a.e, &[1]);
    }
    // return true
    true
}
/// `s.sptg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn sptg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return Duel.GetLocationCount(tp,LOCATION_MZONE)>0
    //     and e:GetHandler():IsCanBeSpecialSummoned(e,0,tp,false,false) end
    if a.chk == Some(0) {
        let h = effect::get_handler(d, a.e).nil();
        return duel::get_location_count(d, a.tp, LOCATION_MZONE as u8, None, None, None).0 > 0
            && card::is_can_be_special_summoned(d, h, a.e, 0, a.tp, false, false, None, None, None);
    }
    // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,e:GetHandler(),1,0,0)
    let h = effect::get_handler(d, a.e).nil();
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, Some(CardSet::from_card(h)), 1, 0, 0);
    // Duel.SetPossibleOperationInfo(0,CATEGORY_TOHAND,nil,1,tp,LOCATION_GRAVE)
    duel::set_possible_operation_info(d, 0, CATEGORY_TOHAND, None, 1, a.tp, LOCATION_GRAVE as i32);
    true
}
/// `s.thfilter(c)`
fn thfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsRace(RACE_DRAGON) and c:IsType(TYPE_NORMAL) and c:IsAbleToHand()
    card::is_race(d, c, RACE_DRAGON) && card::is_type(d, c, TYPE_NORMAL) && card::is_able_to_hand(d, c, None)
}
/// `s.spop(e,tp,eg,ep,ev,re,r,rp)`
fn spop(d: &mut Duel, a: &EffArgs) {
    // if Duel.GetLocationCount(tp,LOCATION_MZONE)<=0 then return end
    if duel::get_location_count(d, a.tp, LOCATION_MZONE as u8, None, None, None).0 <= 0 {
        return;
    }
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // local g=Duel.GetMatchingGroup(aux.NecroValleyFilter(s.thfilter),tp,LOCATION_GRAVE,0,nil)
    let g = duel::get_matching_group(d, Some(&aux::necro_valley_filter(thfilter)), a.tp, LOCATION_GRAVE, 0, NO_EX);
    // if not c:IsRelateToEffect(e) then return end
    if !card::is_relate_to_effect(d, c, a.e) {
        return;
    }
    // if Duel.SpecialSummon(c,0,tp,tp,false,false,POS_FACEUP)~=0 and e:GetLabel()==1
    //     and #g>0 and Duel.SelectYesNo(tp,aux.Stringid(id,0)) then
    if duel::special_summon(d, c, 0, a.tp, a.tp, false, false, POS_FACEUP as u8, None) != 0
        && effect::get_label(d, a.e) == 1
        && !g.is_empty()
        && duel::select_yes_no(d, a.tp, aux::stringid(ID, 0))
    {
        // Duel.BreakEffect()
        duel::break_effect(d);
        // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_ATOHAND)
        duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_ATOHAND as u64);
        // local sg=g:Select(tp,1,1,nil)
        let sg = group::select(d, &g, a.tp, 1, 1, false, NO_EX).nil();
        // Duel.SendtoHand(sg,nil,REASON_EFFECT)
        duel::send_to_hand(d, &sg, None, REASON_EFFECT, None);
        // Duel.ConfirmCards(1-tp,sg)
        duel::confirm_cards(d, 1 - a.tp, &sg);
    }
}
