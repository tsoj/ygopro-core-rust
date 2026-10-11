//! Lunalight Scarlet Tiger
use super::prelude::*;

pub const ID: u32 = 11876803;
pub const INFO: ScriptInfo = ScriptInfo { listed_names: &[CARD_POLYMERIZATION], listed_series: &[SET_LUNALIGHT], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_SPECIAL_SUMMON+CATEGORY_TOHAND+CATEGORY_SEARCH)
    effect::set_category(d, e1, CATEGORY_SPECIAL_SUMMON + CATEGORY_TOHAND + CATEGORY_SEARCH);
    // e1:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e1, EFFECT_TYPE_FIELD + EFFECT_TYPE_TRIGGER_O);
    // e1:SetProperty(EFFECT_FLAG_DELAY)
    effect::set_property(d, e1, EFFECT_FLAG_DELAY, 0);
    // e1:SetCode(EVENT_CHAINING)
    effect::set_code(d, e1, EVENT_CHAINING);
    // e1:SetRange(LOCATION_HAND)
    effect::set_range(d, e1, LOCATION_HAND);
    // e1:SetCountLimit(1,{id,0})
    effect::set_count_limit(d, e1, 1, ID, 0, 0);
    // e1:SetCondition(function(e,tp,eg,ep,ev,re,r,rp) return rp==tp and re:IsCardSetcode(SET_LUNALIGHT) and re:IsMonsterEffect() end)
    effect::set_condition(d, e1, Some(condition(cond)));
    // e1:SetTarget(s.selfsptg)
    effect::set_target(d, e1, Some(cost_target(selfsptg)));
    // e1:SetOperation(s.selfspop)
    effect::set_operation(d, e1, Some(operation(selfspop)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // Duel.AddCustomActivityCounter(id,ACTIVITY_CHAIN,aux.FALSE)
    duel::add_custom_activity_counter(d, ID, ACTIVITY_CHAIN, lua_fn(aux_false));
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e2, aux::stringid(ID, 1));
    // e2:SetCategory(CATEGORY_SPECIAL_SUMMON)
    effect::set_category(d, e2, CATEGORY_SPECIAL_SUMMON);
    // e2:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e2, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_O);
    // e2:SetProperty(EFFECT_FLAG_DELAY)
    effect::set_property(d, e2, EFFECT_FLAG_DELAY, 0);
    // e2:SetCode(EVENT_TO_GRAVE)
    effect::set_code(d, e2, EVENT_TO_GRAVE);
    // e2:SetCountLimit(1,{id,1})
    effect::set_count_limit(d, e2, 1, ID, 1, 0);
    // e2:SetCondition(function(e,tp,eg,ep,ev,re,r,rp) return e:GetHandler():IsReason(REASON_EFFECT) end)
    effect::set_condition(d, e2, Some(condition(cond2)));
    // e2:SetTarget(s.exgysptg)
    effect::set_target(d, e2, Some(cost_target(exgysptg)));
    // e2:SetOperation(s.exgyspop)
    effect::set_operation(d, e2, Some(operation(exgyspop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
}
/// `aux.FALSE`
fn aux_false(_d: &mut Duel, _a: Args) -> bool {
    false
}
/// e1's condition
fn cond(d: &mut Duel, a: &EffArgs) -> bool {
    // return rp==tp and re:IsCardSetcode(SET_LUNALIGHT) and re:IsMonsterEffect()
    if a.rp != a.tp {
        return false;
    }
    let re = a.re.nil();
    effect::is_card_setcode(d, re, &[SET_LUNALIGHT as u16]) && effect::is_monster_effect(d, re)
}
/// e2's condition
fn cond2(d: &mut Duel, a: &EffArgs) -> bool {
    // return e:GetHandler():IsReason(REASON_EFFECT)
    let h = effect::get_handler(d, a.e).nil();
    card::is_reason(d, h, REASON_EFFECT)
}
/// `s.selfspfilter`-less target of e1
fn selfsptg(d: &mut Duel, a: &EffArgs) -> bool {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if chk==0 then return Duel.GetLocationCount(tp,LOCATION_MZONE)>0 and c:IsCanBeSpecialSummoned(e,0,tp,false,false) end
    if a.chk == Some(0) {
        return duel::get_location_count(d, a.tp, LOCATION_MZONE as u8, None, None, None).0 > 0
            && card::is_can_be_special_summoned(d, c, a.e, 0, a.tp, false, false, None, None, None);
    }
    // e:GetChainData().extra_effect=Duel.GetCustomActivityCount(id,1-tp,ACTIVITY_CHAIN)>0
    let extra = duel::get_custom_activity_count(d, ID, 1 - a.tp, ACTIVITY_CHAIN) > 0;
    chain::set_effect_data(d, a.e, None, chain::field::CARD, Value::Bool(extra));
    // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,c,1,tp,0)
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, Some(CardSet::from_card(c)), 1, a.tp, 0);
    // Duel.SetPossibleOperationInfo(0,CATEGORY_TOHAND,nil,1,tp,LOCATION_DECK|LOCATION_GRAVE)
    duel::set_possible_operation_info(d, 0, CATEGORY_TOHAND, None, 1, a.tp, (LOCATION_DECK | LOCATION_GRAVE) as i32);
    true
}
/// `s.thfilter(c)`
fn thfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsCode(CARD_POLYMERIZATION) and c:IsAbleToHand()
    card::is_code(d, c, &[CARD_POLYMERIZATION]) && card::is_able_to_hand(d, c, None)
}
/// `s.selfspop`
fn selfspop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if c:IsRelateToEffect(e) and Duel.SpecialSummon(c,0,tp,tp,false,false,POS_FACEUP)>0 and e:GetChainData().extra_effect then
    if card::is_relate_to_effect(d, c, a.e)
        && duel::special_summon(d, c, 0, tp, tp, false, false, POS_FACEUP as u8, None) > 0
        && chain::effect_data(d, a.e, None, chain::field::CARD).truthy()
    {
        // local g=Duel.GetMatchingGroup(aux.NecroValleyFilter(s.thfilter),tp,LOCATION_DECK|LOCATION_GRAVE,0,nil)
        let nf = aux::necro_valley_filter(|d, c| thfilter(d, c));
        let g = duel::get_matching_group(d, Some(&nf), tp, LOCATION_DECK | LOCATION_GRAVE, 0, NO_EX);
        // if #g>0 and Duel.SelectYesNo(tp,aux.Stringid(id,2)) then
        if !g.is_empty() && duel::select_yes_no(d, tp, aux::stringid(ID, 2)) {
            // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_ATOHAND)
            duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_ATOHAND as u64);
            // local sc=g:Select(tp,1,1,nil):GetFirst()
            let sel = group::select(d, &g, tp, 1, 1, false, NO_EX).nil();
            let sc = group::get_first(d, &sel);
            // if sc then
            if let Some(sc) = sc {
                // if sc:IsLocation(LOCATION_GRAVE) then Duel.HintSelection(sc) end
                if card::is_location(d, sc, LOCATION_GRAVE) {
                    duel::hint_selection(d, CardSet::from_card(sc), true);
                }
                // Duel.BreakEffect()
                duel::break_effect(d);
                // Duel.SendtoHand(sc,nil,REASON_EFFECT)
                duel::send_to_hand(d, CardSet::from_card(sc), None, REASON_EFFECT, None);
                // if sc:IsPreviousLocation(LOCATION_DECK) then Duel.ConfirmCards(1-tp,sc) end
                if card::is_previous_location(d, sc, LOCATION_DECK) {
                    duel::confirm_cards(d, 1 - tp, CardSet::from_card(sc));
                }
            }
        }
    }
}
/// `s.exgyspfilter(c,e,tp,mmz_chk)`
fn exgyspfilter(d: &mut Duel, c: CardId, e: EffectId, tp: u8, mmz_chk: bool) -> bool {
    // if not (c:IsSetCard(SET_LUNALIGHT) and not c:IsLevel(5) and c:IsFaceup() and c:IsCanBeSpecialSummoned(e,0,tp,false,false)) then return false end
    if !(card::is_set_card(d, c, &[SET_LUNALIGHT as u16])
        && !card::is_level(d, c, &[5])
        && card::is_faceup(d, c)
        && card::is_can_be_special_summoned(d, c, e, 0, tp, false, false, None, None, None))
    {
        return false;
    }
    // if c:IsLocation(LOCATION_EXTRA) then
    if card::is_location(d, c, LOCATION_EXTRA) {
        // return Duel.GetLocationCountFromEx(tp,tp,nil,c)>0
        duel::get_location_count_from_ex(d, tp, Some(tp), NO_EX, duel::ExtraCard::Card(c), None).0 > 0
    } else if card::is_location(d, c, LOCATION_GRAVE) {
        // return mmz_chk
        mmz_chk
    } else {
        // (falls through: nil, falsy)
        false
    }
}
/// `s.exgysptg(e,...,chk)`
fn exgysptg(d: &mut Duel, a: &EffArgs) -> bool {
    let (e, tp) = (a.e, a.tp);
    if a.chk == Some(0) {
        // local mmz_chk=Duel.GetLocationCount(tp,LOCATION_MZONE)>0
        let mmz_chk = duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 > 0;
        // return Duel.IsExistingMatchingCard(s.exgyspfilter,tp,LOCATION_EXTRA|LOCATION_GRAVE,0,1,nil,e,tp,mmz_chk)
        return duel::is_existing_matching_card(
            d,
            Some(&|d, c| exgyspfilter(d, c, e, tp, mmz_chk)),
            tp,
            LOCATION_EXTRA | LOCATION_GRAVE,
            0,
            1,
            NO_EX,
        );
    }
    // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,nil,1,tp,LOCATION_EXTRA|LOCATION_GRAVE)
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, None, 1, tp, (LOCATION_EXTRA | LOCATION_GRAVE) as i32);
    true
}
/// `s.exgyspop(e,tp,...)`
fn exgyspop(d: &mut Duel, a: &EffArgs) {
    let (e, tp) = (a.e, a.tp);
    // local mmz_chk=Duel.GetLocationCount(tp,LOCATION_MZONE)>0
    let mmz_chk = duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 > 0;
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_SPSUMMON)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_SPSUMMON as u64);
    // local sc=Duel.SelectMatchingCard(tp,aux.NecroValleyFilter(s.exgyspfilter),tp,LOCATION_EXTRA|LOCATION_GRAVE,0,1,1,nil,e,tp,mmz_chk):GetFirst()
    let nf = aux::necro_valley_filter(move |d, c| exgyspfilter(d, c, e, tp, mmz_chk));
    let sel = duel::select_matching_card(d, tp, Some(&nf), tp, LOCATION_EXTRA | LOCATION_GRAVE, 0, 1, 1, false, NO_EX).nil();
    let sc = group::get_first(d, &sel);
    // if sc and Duel.SpecialSummonStep(sc,0,tp,tp,false,false,POS_FACEUP) then
    if let Some(sc) = sc {
        if duel::special_summon_step(d, sc, 0, tp, tp, false, false, POS_FACEUP as u8, None) {
            // local e1=Effect.CreateEffect(e:GetHandler())
            let h = effect::get_handler(d, e).nil();
            let e1 = effect::create_effect(d, h);
            // e1:SetDescription(3302)
            effect::set_description(d, e1, 3302);
            // e1:SetType(EFFECT_TYPE_SINGLE)
            effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
            // e1:SetProperty(EFFECT_FLAG_CLIENT_HINT)
            effect::set_property(d, e1, EFFECT_FLAG_CLIENT_HINT, 0);
            // e1:SetCode(EFFECT_CANNOT_TRIGGER)
            effect::set_code(d, e1, EFFECT_CANNOT_TRIGGER);
            // e1:SetReset(RESET_EVENT|RESETS_STANDARD)
            effect::set_reset(d, e1, RESET_EVENT | RESETS_STANDARD, 1);
            // sc:RegisterEffect(e1)
            card::register_effect(d, sc, e1, false);
        }
    }
    // Duel.SpecialSummonComplete()
    duel::special_summon_complete(d);
}
