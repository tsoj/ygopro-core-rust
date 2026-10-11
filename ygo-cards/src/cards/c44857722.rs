//! Full Moon Mirror
//! --月輪鏡
use super::prelude::*;

pub const ID: u32 = 44857722;
const COUNTER_FULL_MOON: u16 = 0x219;

pub const INFO: ScriptInfo = ScriptInfo {
    counter_place_list: &[COUNTER_FULL_MOON as u32],
    ..ScriptInfo::DEFAULT
};

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // c:EnableCounterPermit(COUNTER_FULL_MOON)
    card::enable_counter_permit(d, c, COUNTER_FULL_MOON, None, None);
    // Activate
    // local e0=Effect.CreateEffect(c)
    let e0 = effect::create_effect(d, c);
    // e0:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e0, EFFECT_TYPE_ACTIVATE);
    // e0:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e0, EVENT_FREE_CHAIN);
    // c:RegisterEffect(e0)
    card::register_effect(d, c, e0, false);
    // Each time a monster(s) is destroyed by battle or card effect, place 1 Full Moon Counter on this card for each
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_CONTINUOUS)
    effect::set_type(d, e1, EFFECT_TYPE_FIELD + EFFECT_TYPE_CONTINUOUS);
    // e1:SetCode(EVENT_DESTROYED)
    effect::set_code(d, e1, EVENT_DESTROYED);
    // e1:SetRange(LOCATION_SZONE)
    effect::set_range(d, e1, LOCATION_SZONE);
    // e1:SetOperation(s.ctop)
    effect::set_operation(d, e1, Some(operation(ctop)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // You can remove Full Moon Counters from this card, then activate 1 of these effects;
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e2, aux::stringid(ID, 0));
    // e2:SetType(EFFECT_TYPE_IGNITION)
    effect::set_type(d, e2, EFFECT_TYPE_IGNITION);
    // e2:SetRange(LOCATION_SZONE)
    effect::set_range(d, e2, LOCATION_SZONE);
    // e2:SetCountLimit(1,id)
    effect::set_count_limit(d, e2, 1, ID, 0, 0);
    // e2:SetTarget(s.efftg)
    effect::set_target(d, e2, Some(cost_target(efftg)));
    // e2:SetOperation(s.effop)
    effect::set_operation(d, e2, Some(operation(effop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
}
fn ctfilter(d: &mut Duel, c: CardId) -> bool {
    // return ((c:IsMonster() and not c:IsPreviousLocation(LOCATION_ONFIELD)) or c:IsPreviousLocation(LOCATION_MZONE))
    //     and c:IsReason(REASON_BATTLE|REASON_EFFECT)
    ((card::is_monster(d, c) && !card::is_previous_location(d, c, LOCATION_ONFIELD)) || card::is_previous_location(d, c, LOCATION_MZONE))
        && card::is_reason(d, c, REASON_BATTLE | REASON_EFFECT)
}
fn ctop(d: &mut Duel, a: &EffArgs) {
    // local des_count=eg:FilterCount(s.ctfilter,nil)
    let eg = a.eg.clone().nil();
    let des_count = group::filter_count(d, &eg, &ctfilter, NO_EX) as u16;
    // if des_count==0 then return end
    if des_count == 0 {
        return;
    }
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if not Duel.IsChainSolving() then
    if !duel::is_chain_solving(d) {
        // c:AddCounter(COUNTER_FULL_MOON,des_count)
        card::add_counter(d, c, COUNTER_FULL_MOON, des_count, false);
    } else {
        // Place 1 Full Moon Counter on this card at the end of the Chain Link
        // local e1=Effect.CreateEffect(c)
        let e1 = effect::create_effect(d, c);
        // e1:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_CONTINUOUS)
        effect::set_type(d, e1, EFFECT_TYPE_FIELD + EFFECT_TYPE_CONTINUOUS);
        // e1:SetCode(EVENT_CHAIN_SOLVED)
        effect::set_code(d, e1, EVENT_CHAIN_SOLVED);
        // e1:SetRange(LOCATION_SZONE)
        effect::set_range(d, e1, LOCATION_SZONE);
        // the captured des_count is kept as the label of e1
        effect::set_label(d, e1, &[des_count as i64]);
        // e1:SetOperation(function() c:AddCounter(COUNTER_FULL_MOON,des_count) end)
        effect::set_operation(d, e1, Some(operation(|d, a| {
            let c = effect::get_handler(d, a.e).nil();
            let n = effect::get_label(d, a.e) as u16;
            card::add_counter(d, c, COUNTER_FULL_MOON, n, false);
        })));
        // e1:SetReset(RESET_EVENT|RESETS_STANDARD|RESET_CHAIN)
        effect::set_reset(d, e1, RESET_EVENT | RESETS_STANDARD | RESET_CHAIN, 1);
        // c:RegisterEffect(e1)
        card::register_effect(d, c, e1, false);
        // Reset "e1" at the end of the Chain Link
        // local e2=Effect.CreateEffect(c)
        let e2 = effect::create_effect(d, c);
        // e2:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_CONTINUOUS)
        effect::set_type(d, e2, EFFECT_TYPE_FIELD + EFFECT_TYPE_CONTINUOUS);
        // e2:SetCode(EVENT_CHAIN_SOLVED)
        effect::set_code(d, e2, EVENT_CHAIN_SOLVED);
        // the effect e1 is kept as the label object of e2
        effect::set_label_object(d, e2, Value::Effect(e1));
        // e2:SetOperation(function() e1:Reset() end)
        effect::set_operation(d, e2, Some(operation(|d, a| {
            let e1 = effect::get_label_object(d, a.e).as_effect().nil();
            effect::reset(d, e1);
        })));
        // e2:SetReset(RESET_CHAIN)
        effect::set_reset(d, e2, RESET_CHAIN, 1);
        // Duel.RegisterEffect(e2,tp)
        duel::register_effect(d, e2, a.tp);
    }
}
fn lv6spfilter(d: &mut Duel, c: CardId, e: EffectId, tp: u8) -> bool {
    // return c:IsLevelBelow(6) and c:IsRace(RACE_FIEND|RACE_FAIRY) and c:IsCanBeSpecialSummoned(e,0,tp,false,false)
    card::is_level_below(d, c, 6)
        && card::is_race(d, c, RACE_FIEND | RACE_FAIRY)
        && card::is_can_be_special_summoned(d, c, e, 0, tp, false, false, None, None, None)
}
fn thfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsLevel(10) and c:IsAttribute(ATTRIBUTE_DARK) and c:IsAbleToHand()
    card::is_level(d, c, &[10]) && card::is_attribute(d, c, ATTRIBUTE_DARK) && card::is_able_to_hand(d, c, None)
}
fn lv10spfilter(d: &mut Duel, c: CardId, e: EffectId, tp: u8) -> bool {
    // return c:IsLevel(10) and c:IsCanBeSpecialSummoned(e,0,tp,false,false)
    card::is_level(d, c, &[10]) && card::is_can_be_special_summoned(d, c, e, 0, tp, false, false, None, None, None)
}
fn efftg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    let e = a.e;
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // local mmz_chk=Duel.GetLocationCount(tp,LOCATION_MZONE)>0
    let mmz_chk = duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 > 0;
    // local b1=c:IsCanRemoveCounter(tp,COUNTER_FULL_MOON,1,REASON_COST) and mmz_chk
    //     and Duel.IsExistingMatchingCard(s.lv6spfilter,tp,LOCATION_HAND|LOCATION_GRAVE,0,1,nil,e,tp)
    let b1 = card::is_can_remove_counter(d, c, tp, COUNTER_FULL_MOON, 1, REASON_COST)
        && mmz_chk
        && duel::is_existing_matching_card(d, Some(&|d, x| lv6spfilter(d, x, e, tp)), tp, LOCATION_HAND | LOCATION_GRAVE, 0, 1, NO_EX);
    // local b2=c:IsCanRemoveCounter(tp,COUNTER_FULL_MOON,3,REASON_COST)
    //     and Duel.IsExistingMatchingCard(s.thfilter,tp,LOCATION_DECK,0,1,nil)
    let b2 = card::is_can_remove_counter(d, c, tp, COUNTER_FULL_MOON, 3, REASON_COST)
        && duel::is_existing_matching_card(d, Some(&thfilter), tp, LOCATION_DECK, 0, 1, NO_EX);
    // local b3=c:IsCanRemoveCounter(tp,COUNTER_FULL_MOON,5,REASON_COST) and mmz_chk
    //     and Duel.IsExistingMatchingCard(s.lv10spfilter,tp,LOCATION_HAND|LOCATION_GRAVE,0,1,nil,e,tp)
    let b3 = card::is_can_remove_counter(d, c, tp, COUNTER_FULL_MOON, 5, REASON_COST)
        && mmz_chk
        && duel::is_existing_matching_card(d, Some(&|d, x| lv10spfilter(d, x, e, tp)), tp, LOCATION_HAND | LOCATION_GRAVE, 0, 1, NO_EX);
    // if chk==0 then return b1 or b2 or b3 end
    if a.chk == Some(0) {
        return b1 || b2 || b3;
    }
    // local op=Duel.SelectEffect(tp,{b1,aux.Stringid(id,1)},{b2,aux.Stringid(id,2)},{b3,aux.Stringid(id,3)})
    let op = duel::select_effect(
        d,
        tp,
        &[(b1, aux::stringid(ID, 1)), (b2, aux::stringid(ID, 2)), (b3, aux::stringid(ID, 3))],
    )
    .nil();
    // e:SetLabel(op)
    effect::set_label(d, e, &[op as i64]);
    // c:RemoveCounter(tp,COUNTER_FULL_MOON,2*op-1,REASON_COST)
    card::remove_counter(d, c, tp, COUNTER_FULL_MOON, (2 * op - 1) as u16, REASON_COST);
    // if (op==1 or op==3) then
    if op == 1 || op == 3 {
        // e:SetCategory(CATEGORY_SPECIAL_SUMMON)
        effect::set_category(d, e, CATEGORY_SPECIAL_SUMMON);
        // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,nil,1,tp,LOCATION_HAND|LOCATION_GRAVE)
        duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, None, 1, tp, (LOCATION_HAND | LOCATION_GRAVE) as i32);
    } else if op == 2 {
        // e:SetCategory(CATEGORY_TOHAND+CATEGORY_SEARCH)
        effect::set_category(d, e, CATEGORY_TOHAND + CATEGORY_SEARCH);
        // Duel.SetOperationInfo(0,CATEGORY_TOHAND,nil,1,tp,LOCATION_DECK)
        duel::set_operation_info(d, 0, CATEGORY_TOHAND, None, 1, tp, LOCATION_DECK as i32);
    }
    true
}
fn effop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    let e = a.e;
    // local op=e:GetLabel()
    let op = effect::get_label(d, e);
    // local mmz_chk=Duel.GetLocationCount(tp,LOCATION_MZONE)>0
    let mmz_chk = duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 > 0;
    if op == 1 {
        // if not mmz_chk then return end
        if !mmz_chk {
            return;
        }
        // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_SPSUMMON)
        duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_SPSUMMON as u64);
        // local g=Duel.SelectMatchingCard(tp,aux.NecroValleyFilter(s.lv6spfilter),tp,LOCATION_HAND|LOCATION_GRAVE,0,1,1,nil,e,tp)
        let g = duel::select_matching_card(
            d,
            tp,
            Some(&aux::necro_valley_filter(|d, x| lv6spfilter(d, x, e, tp))),
            tp,
            LOCATION_HAND | LOCATION_GRAVE,
            0,
            1,
            1,
            false,
            NO_EX,
        )
        .nil();
        // if #g>0 then Duel.SpecialSummon(g,0,tp,tp,false,false,POS_FACEUP) end
        if !g.is_empty() {
            duel::special_summon(d, g, 0, tp, tp, false, false, POS_FACEUP as u8, None);
        }
    } else if op == 2 {
        // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_ATOHAND)
        duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_ATOHAND as u64);
        // local g=Duel.SelectMatchingCard(tp,s.thfilter,tp,LOCATION_DECK,0,1,1,nil)
        let g = duel::select_matching_card(d, tp, Some(&thfilter), tp, LOCATION_DECK, 0, 1, 1, false, NO_EX).nil();
        // if #g>0 then
        if !g.is_empty() {
            // Duel.SendtoHand(g,nil,REASON_EFFECT)
            duel::send_to_hand(d, g.clone(), None, REASON_EFFECT, None);
            // Duel.ConfirmCards(1-tp,g)
            duel::confirm_cards(d, 1 - tp, g);
        }
    } else if op == 3 {
        // if not mmz_chk then return end
        if !mmz_chk {
            return;
        }
        // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_SPSUMMON)
        duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_SPSUMMON as u64);
        // local g=Duel.SelectMatchingCard(tp,aux.NecroValleyFilter(s.lv10spfilter),tp,LOCATION_HAND|LOCATION_GRAVE,0,1,1,nil,e,tp)
        let g = duel::select_matching_card(
            d,
            tp,
            Some(&aux::necro_valley_filter(|d, x| lv10spfilter(d, x, e, tp))),
            tp,
            LOCATION_HAND | LOCATION_GRAVE,
            0,
            1,
            1,
            false,
            NO_EX,
        )
        .nil();
        // if #g>0 then Duel.SpecialSummon(g,0,tp,tp,false,false,POS_FACEUP) end
        if !g.is_empty() {
            duel::special_summon(d, g, 0, tp, tp, false, false, POS_FACEUP as u8, None);
        }
    }
}
