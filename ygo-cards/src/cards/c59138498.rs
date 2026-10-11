//! Martyr of the Sacred Beasts
use super::prelude::*;

pub const ID: u32 = 59138498;
pub const INFO: ScriptInfo = ScriptInfo { listed_names: &[ID], listed_series: &[SET_SACRED_BEAST], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // local e1a=Effect.CreateEffect(c)
    let e1a = effect::create_effect(d, c);
    // e1a:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1a, aux::stringid(ID, 0));
    // e1a:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e1a, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_O);
    // e1a:SetProperty(EFFECT_FLAG_DELAY)
    effect::set_property(d, e1a, EFFECT_FLAG_DELAY, 0);
    // e1a:SetCode(EVENT_SUMMON_SUCCESS)
    effect::set_code(d, e1a, EVENT_SUMMON_SUCCESS);
    // e1a:SetCountLimit(1,{id,0})
    effect::set_count_limit(d, e1a, 1, ID, 0, 0);
    // e1a:SetTarget(s.pltg)
    effect::set_target(d, e1a, Some(cost_target(pltg)));
    // e1a:SetOperation(s.plop)
    effect::set_operation(d, e1a, Some(operation(plop)));
    // c:RegisterEffect(e1a)
    card::register_effect(d, c, e1a, false);
    // local e1b=e1a:Clone()
    let e1b = effect::clone(d, e1a);
    // e1b:SetCode(EVENT_SPSUMMON_SUCCESS)
    effect::set_code(d, e1b, EVENT_SPSUMMON_SUCCESS);
    // c:RegisterEffect(e1b)
    card::register_effect(d, c, e1b, false);
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e2, aux::stringid(ID, 1));
    // e2:SetCategory(CATEGORY_SPECIAL_SUMMON)
    effect::set_category(d, e2, CATEGORY_SPECIAL_SUMMON);
    // e2:SetType(EFFECT_TYPE_IGNITION)
    effect::set_type(d, e2, EFFECT_TYPE_IGNITION);
    // e2:SetRange(LOCATION_MZONE)
    effect::set_range(d, e2, LOCATION_MZONE);
    // e2:SetCountLimit(1,{id,1})
    effect::set_count_limit(d, e2, 1, ID, 1, 0);
    // e2:SetCondition(s.spcon)
    effect::set_condition(d, e2, Some(condition(spcon)));
    // e2:SetTarget(s.sptg)
    effect::set_target(d, e2, Some(cost_target(sptg)));
    // e2:SetOperation(s.spop)
    effect::set_operation(d, e2, Some(operation(spop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
    // local e3=Effect.CreateEffect(c)
    let e3 = effect::create_effect(d, c);
    // e3:SetDescription(aux.Stringid(id,2))
    effect::set_description(d, e3, aux::stringid(ID, 2));
    // e3:SetCategory(CATEGORY_TOHAND)
    effect::set_category(d, e3, CATEGORY_TOHAND);
    // e3:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e3, EFFECT_TYPE_FIELD + EFFECT_TYPE_TRIGGER_O);
    // e3:SetCode(EVENT_PHASE+PHASE_END)
    effect::set_code(d, e3, EVENT_PHASE + PHASE_END);
    // e3:SetRange(LOCATION_GRAVE)
    effect::set_range(d, e3, LOCATION_GRAVE);
    // e3:SetCountLimit(1,{id,2})
    effect::set_count_limit(d, e3, 1, ID, 2, 0);
    // e3:SetCondition(s.thcon)
    effect::set_condition(d, e3, Some(condition(thcon)));
    // e3:SetTarget(s.thtg)
    effect::set_target(d, e3, Some(cost_target(thtg)));
    // e3:SetOperation(s.thop)
    effect::set_operation(d, e3, Some(operation(thop)));
    // c:RegisterEffect(e3)
    card::register_effect(d, c, e3, false);
}
/// `s.plfilter(c,tp,szone_chk)`
fn plfilter(d: &mut Duel, c: CardId, tp: u8, szone_chk: bool) -> bool {
    // return c:IsSetCard(SET_SACRED_BEAST) and not c:IsForbidden() and c:CheckUniqueOnField(tp)
    //     and (c:IsFieldSpell() or (szone_chk and c:IsContinuousSpellTrap()))
    card::is_set_card(d, c, &[SET_SACRED_BEAST as u16])
        && !card::is_forbidden(d, c)
        && card::check_unique_on_field(d, c, tp, None, None)
        && (card::is_field_spell(d, c) || (szone_chk && card::is_continuous_spell_trap(d, c)))
}
/// `s.pltg(e,tp,...,chk)`
fn pltg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // if chk==0 then
    if a.chk == Some(0) {
        // local szone_chk=Duel.GetLocationCount(tp,LOCATION_SZONE)>0
        let szone_chk = duel::get_location_count(d, tp, LOCATION_SZONE as u8, None, None, None).0 > 0;
        // return Duel.IsExistingMatchingCard(s.plfilter,tp,LOCATION_DECK,0,1,nil,tp,szone_chk)
        return duel::is_existing_matching_card(d, Some(&|d, c| plfilter(d, c, tp, szone_chk)), tp, LOCATION_DECK, 0, 1, NO_EX);
    }
    true
}
/// `s.plop(e,tp,...)`
fn plop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // local szone_chk=Duel.GetLocationCount(tp,LOCATION_SZONE)>0
    let szone_chk = duel::get_location_count(d, tp, LOCATION_SZONE as u8, None, None, None).0 > 0;
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_TOFIELD)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_TOFIELD as u64);
    // local sc=Duel.SelectMatchingCard(tp,s.plfilter,tp,LOCATION_DECK,0,1,1,nil,tp,szone_chk):GetFirst()
    let sel = duel::select_matching_card(d, tp, Some(&|d, c| plfilter(d, c, tp, szone_chk)), tp, LOCATION_DECK, 0, 1, 1, false, NO_EX).nil();
    let sc = group::get_first(d, &sel);
    // if sc then
    if let Some(sc) = sc {
        // if sc:IsFieldSpell() then
        if card::is_field_spell(d, sc) {
            // local fc=Duel.GetFieldCard(tp,LOCATION_FZONE,0)
            // if fc then
            if let Some(fc) = duel::get_field_card(d, tp, LOCATION_FZONE, 0) {
                // Duel.SendtoGrave(fc,REASON_RULE)
                duel::send_to_grave(d, CardSet::from_card(fc), REASON_RULE, None, None);
                // Duel.BreakEffect()
                duel::break_effect(d);
            }
            // Duel.MoveToField(sc,tp,tp,LOCATION_FZONE,POS_FACEUP,true)
            duel::move_to_field(d, sc, tp, tp, LOCATION_FZONE as u16, POS_FACEUP as u8, true, None);
        } else {
            // Duel.MoveToField(sc,tp,tp,LOCATION_SZONE,POS_FACEUP,true)
            duel::move_to_field(d, sc, tp, tp, LOCATION_SZONE as u16, POS_FACEUP as u8, true, None);
        }
    }
}
/// `s.spcon(e,tp,...)`
fn spcon(d: &mut Duel, a: &EffArgs) -> bool {
    let h = effect::get_handler(d, a.e).nil();
    // return Duel.IsExistingMatchingCard(aux.FaceupFilter(Card.IsSetCard,SET_SACRED_BEAST),tp,LOCATION_MZONE,0,1,e:GetHandler())
    duel::is_existing_matching_card(
        d,
        Some(&|d, c| card::is_faceup(d, c) && card::is_set_card(d, c, &[SET_SACRED_BEAST as u16])),
        a.tp,
        LOCATION_MZONE,
        0,
        1,
        h.into(),
    )
}
/// `s.spfilter(c,e,tp)`
fn spfilter(d: &mut Duel, c: CardId, e: EffectId, tp: u8) -> bool {
    // return c:IsCode(id) and c:IsCanBeSpecialSummoned(e,0,tp,false,false)
    card::is_code(d, c, &[ID]) && card::is_can_be_special_summoned(d, c, e, 0, tp, false, false, None, None, None)
}
/// `s.sptg(e,tp,...,chk)`
fn sptg(d: &mut Duel, a: &EffArgs) -> bool {
    let (e, tp) = (a.e, a.tp);
    // if chk==0 then return Duel.GetLocationCount(tp,LOCATION_MZONE)>=2
    //     and not Duel.IsPlayerAffectedByEffect(tp,CARD_BLUEEYES_SPIRIT)
    //     and Duel.IsExistingMatchingCard(s.spfilter,tp,LOCATION_HAND|LOCATION_DECK|LOCATION_GRAVE,0,2,nil,e,tp) end
    if a.chk == Some(0) {
        return duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 >= 2
            && duel::is_player_affected_by_effect(d, tp, CARD_BLUEEYES_SPIRIT).is_none()
            && duel::is_existing_matching_card(d, Some(&|d, c| spfilter(d, c, e, tp)), tp, LOCATION_HAND | LOCATION_DECK | LOCATION_GRAVE, 0, 2, NO_EX);
    }
    // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,nil,2,tp,LOCATION_HAND|LOCATION_DECK|LOCATION_GRAVE)
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, None, 2, tp, (LOCATION_HAND | LOCATION_DECK | LOCATION_GRAVE) as i32);
    true
}
/// `s.spop(e,tp,...)`
fn spop(d: &mut Duel, a: &EffArgs) {
    let (e, tp) = (a.e, a.tp);
    // if Duel.GetLocationCount(tp,LOCATION_MZONE)<2 or Duel.IsPlayerAffectedByEffect(tp,CARD_BLUEEYES_SPIRIT) then return end
    if duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 < 2
        || duel::is_player_affected_by_effect(d, tp, CARD_BLUEEYES_SPIRIT).is_some()
    {
        return;
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_SPSUMMON)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_SPSUMMON as u64);
    // local g=Duel.SelectMatchingCard(tp,aux.NecroValleyFilter(s.spfilter),tp,LOCATION_HAND|LOCATION_DECK|LOCATION_GRAVE,0,2,2,nil,e,tp)
    let nf = aux::necro_valley_filter(move |d, c| spfilter(d, c, e, tp));
    let g = duel::select_matching_card(d, tp, Some(&nf), tp, LOCATION_HAND | LOCATION_DECK | LOCATION_GRAVE, 0, 2, 2, false, NO_EX).nil();
    // if #g==2 then
    if g.len() == 2 {
        // Duel.SpecialSummon(g,0,tp,tp,false,false,POS_FACEUP)
        duel::special_summon(d, g, 0, tp, tp, false, false, POS_FACEUP as u8, None);
    }
}
/// `s.thconfilter(c)`
fn thconfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsSetCard(SET_SACRED_BEAST) and c:IsLevel(10)
    card::is_set_card(d, c, &[SET_SACRED_BEAST as u16]) && card::is_level(d, c, &[10])
}
/// `s.thcon(e,tp,...)`
fn thcon(d: &mut Duel, a: &EffArgs) -> bool {
    let h = effect::get_handler(d, a.e).nil();
    // return Duel.IsTurnPlayer(1-tp) and Duel.IsExistingMatchingCard(s.thconfilter,tp,LOCATION_GRAVE,0,1,e:GetHandler())
    duel::is_turn_player(d, 1 - a.tp) && duel::is_existing_matching_card(d, Some(&thconfilter), a.tp, LOCATION_GRAVE, 0, 1, h.into())
}
/// `s.thtg(e,tp,...,chk)`
fn thtg(d: &mut Duel, a: &EffArgs) -> bool {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if chk==0 then return c:IsAbleToHand() end
    if a.chk == Some(0) {
        return card::is_able_to_hand(d, c, None);
    }
    // Duel.SetOperationInfo(0,CATEGORY_TOHAND,c,1,tp,0)
    duel::set_operation_info(d, 0, CATEGORY_TOHAND, Some(CardSet::from_card(c)), 1, a.tp, 0);
    true
}
/// `s.thop(e,tp,...)`
fn thop(d: &mut Duel, a: &EffArgs) {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if c:IsRelateToEffect(e) then
    if card::is_relate_to_effect(d, c, a.e) {
        // Duel.SendtoHand(c,nil,REASON_EFFECT)
        duel::send_to_hand(d, CardSet::from_card(c), None, REASON_EFFECT, None);
    }
}
