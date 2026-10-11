//! Earthbound Immortal Red Nova
use super::prelude::*;

pub const ID: u32 = 5267507;

pub const INFO: ScriptInfo = ScriptInfo {
    listed_names: &[CARD_RED_DRAGON_ARCHFIEND, 97489701],
    listed_series: &[SET_EARTHBOUND, SET_EARTHBOUND_IMMORTAL],
    ..ScriptInfo::DEFAULT
};

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // There can only be 1 "Earthbound Immortal" monster on the field
    // c:SetUniqueOnField(1,1,aux.FilterBoolFunction(Card.IsSetCard,SET_EARTHBOUND_IMMORTAL),LOCATION_MZONE)
    card::set_unique_on_field(
        d,
        c,
        1,
        1,
        card::UniqueCode::Function(lua_fn(|d, a| card::is_set_card(d, a.c(0), &[SET_EARTHBOUND_IMMORTAL as u16]))),
        Some(LOCATION_MZONE),
    );
    // Send 1 "Earthbound Immortal" monster or "Red Dragon Archfiend" from your hand or face-up Monster Zone to the GY
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_TOGRAVE+CATEGORY_SPECIAL_SUMMON)
    effect::set_category(d, e1, CATEGORY_TOGRAVE + CATEGORY_SPECIAL_SUMMON);
    // e1:SetType(EFFECT_TYPE_QUICK_O)
    effect::set_type(d, e1, EFFECT_TYPE_QUICK_O);
    // e1:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    // e1:SetRange(LOCATION_HAND|LOCATION_GRAVE)
    effect::set_range(d, e1, LOCATION_HAND | LOCATION_GRAVE);
    // e1:SetCountLimit(1,id)
    effect::set_count_limit(d, e1, 1, ID, 0, 0);
    // e1:SetHintTiming(0,TIMING_MAIN_END)
    effect::set_hint_timing(d, e1, 0, Some(TIMING_MAIN_END));
    // e1:SetCondition(function() return Duel.IsMainPhase() end)
    effect::set_condition(d, e1, Some(condition(|d, _a| duel::is_main_phase(d, None))));
    // e1:SetCost(Cost.SelfBanish)
    effect::set_cost(d, e1, Some(cost::self_banish()));
    // e1:SetTarget(s.tgtg)
    effect::set_target(d, e1, Some(cost_target(tgtg)));
    // e1:SetOperation(s.tgop)
    effect::set_operation(d, e1, Some(operation(tgop)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
fn tgfilter(d: &mut Duel, c: CardId) -> bool {
    // return (c:IsSetCard(SET_EARTHBOUND_IMMORTAL) or c:IsCode(CARD_RED_DRAGON_ARCHFIEND)) and c:IsMonster() and c:IsAbleToGrave()
    //   and (c:IsFaceup() or not c:IsOnField())
    (card::is_set_card(d, c, &[SET_EARTHBOUND_IMMORTAL as u16]) || card::is_code(d, c, &[CARD_RED_DRAGON_ARCHFIEND]))
        && card::is_monster(d, c)
        && card::is_able_to_grave(d, c, None)
        && (card::is_faceup(d, c) || !card::is_on_field(d, c))
}
fn tgtg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return Duel.IsExistingMatchingCard(s.tgfilter,tp,LOCATION_HAND|LOCATION_MZONE,0,1,e:GetHandler()) end
    if a.chk == Some(0) {
        let c = effect::get_handler(d, a.e).nil();
        return duel::is_existing_matching_card(d, Some(&tgfilter), a.tp, LOCATION_HAND | LOCATION_MZONE, 0, 1, c.into());
    }
    // Duel.SetOperationInfo(0,CATEGORY_TOGRAVE,nil,1,tp,LOCATION_HAND|LOCATION_MZONE)
    duel::set_operation_info(d, 0, CATEGORY_TOGRAVE, None, 1, a.tp, (LOCATION_HAND | LOCATION_MZONE) as i32);
    // Duel.SetPossibleOperationInfo(0,CATEGORY_SPECIAL_SUMMON,nil,1,tp,LOCATION_DECK|LOCATION_EXTRA)
    duel::set_possible_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, None, 1, a.tp, (LOCATION_DECK | LOCATION_EXTRA) as i32);
    true
}
fn earthbound_sp_filter(d: &mut Duel, c: CardId, e: EffectId, tp: u8) -> bool {
    // if not (c:IsSetCard(SET_EARTHBOUND) and c:IsCanBeSpecialSummoned(e,0,tp,false,false)) then return false end
    if !(card::is_set_card(d, c, &[SET_EARTHBOUND as u16])
        && card::is_can_be_special_summoned(d, c, e, 0, tp, false, false, None, None, None))
    {
        return false;
    }
    // if c:IsLocation(LOCATION_DECK) then
    if card::is_location(d, c, LOCATION_DECK) {
        // return Duel.GetLocationCount(tp,LOCATION_MZONE)>0
        duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 > 0
    // elseif c:IsLocation(LOCATION_EXTRA) then
    } else if card::is_location(d, c, LOCATION_EXTRA) {
        // return Duel.GetLocationCountFromEx(tp,tp,nil,c)
        // (a number is always true as a filter result: the count is not compared)
        let _ = duel::get_location_count_from_ex(d, tp, Some(tp), NO_EX, duel::ExtraCard::Card(c), None);
        true
    } else {
        false
    }
}
fn rednova_sp_filter(d: &mut Duel, c: CardId, e: EffectId, tp: u8) -> bool {
    // return c:IsCode(97489701) and c:IsCanBeSpecialSummoned(e,SUMMON_TYPE_SYNCHRO,tp,false,false)
    //   and Duel.GetLocationCountFromEx(tp,tp,nil,c)>0
    card::is_code(d, c, &[97489701])
        && card::is_can_be_special_summoned(d, c, e, SUMMON_TYPE_SYNCHRO, tp, false, false, None, None, None)
        && duel::get_location_count_from_ex(d, tp, Some(tp), NO_EX, duel::ExtraCard::Card(c), None).0 > 0
}
fn tgop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_TOGRAVE)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_TOGRAVE as u64);
    // local gc=Duel.SelectMatchingCard(tp,s.tgfilter,tp,LOCATION_HAND|LOCATION_MZONE,0,1,1,nil):GetFirst()
    let sel = duel::select_matching_card(d, tp, Some(&tgfilter), tp, LOCATION_HAND | LOCATION_MZONE, 0, 1, 1, false, NO_EX).nil();
    let gc = group::get_first(d, &sel);
    // if gc and Duel.SendtoGrave(gc,REASON_EFFECT)>0 and gc:IsLocation(LOCATION_GRAVE) then
    if let Some(gc) = gc {
        if duel::send_to_grave(d, CardSet::from_card(gc), REASON_EFFECT, None, None) > 0
            && card::is_location(d, gc, LOCATION_GRAVE)
        {
            // local b1=Duel.IsExistingMatchingCard(s.earthboundspfilter,tp,LOCATION_DECK|LOCATION_EXTRA,0,1,nil,e,tp)
            let b1 = duel::is_existing_matching_card(
                d,
                Some(&|d, c| earthbound_sp_filter(d, c, a.e, tp)),
                tp,
                LOCATION_DECK | LOCATION_EXTRA,
                0,
                1,
                NO_EX,
            );
            // local b2=Duel.IsExistingMatchingCard(s.rednovaspfilter,tp,LOCATION_EXTRA,0,1,nil,e,tp)
            let b2 = duel::is_existing_matching_card(d, Some(&|d, c| rednova_sp_filter(d, c, a.e, tp)), tp, LOCATION_EXTRA, 0, 1, NO_EX);
            // if not ((b1 or b2) and Duel.SelectYesNo(tp,aux.Stringid(id,1))) then return end
            if !((b1 || b2) && duel::select_yes_no(d, tp, aux::stringid(ID, 1))) {
                return;
            }
            // local op=Duel.SelectEffect(tp,
            //   {b1,aux.Stringid(id,2)},
            //   {b2,aux.Stringid(id,3)})
            let op = duel::select_effect(d, tp, &[(b1, aux::stringid(ID, 2)), (b2, aux::stringid(ID, 3))]);
            // if op==1 then
            if op == Some(1) {
                // Special Summon 1 "Earthbound" monster from your Deck or Extra Deck
                // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_SPSUMMON)
                duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_SPSUMMON as u64);
                // local g=Duel.SelectMatchingCard(tp,s.earthboundspfilter,tp,LOCATION_DECK|LOCATION_EXTRA,0,1,1,nil,e,tp)
                let g = duel::select_matching_card(
                    d,
                    tp,
                    Some(&|d, c| earthbound_sp_filter(d, c, a.e, tp)),
                    tp,
                    LOCATION_DECK | LOCATION_EXTRA,
                    0,
                    1,
                    1,
                    false,
                    NO_EX,
                )
                .nil();
                // if #g>0 then
                if group::get_count(&g) > 0 {
                    // Duel.BreakEffect()
                    duel::break_effect(d);
                    // Duel.SpecialSummon(g,0,tp,tp,false,false,POS_FACEUP)
                    duel::special_summon(d, g, 0, tp, tp, false, false, POS_FACEUP as u8, None);
                }
            // elseif op==2 then
            } else if op == Some(2) {
                // Special Summon "Red Nova Dragon" from your Extra Deck
                // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_SPSUMMON)
                duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_SPSUMMON as u64);
                // local sc=Duel.SelectMatchingCard(tp,s.rednovaspfilter,tp,LOCATION_EXTRA,0,1,1,nil,e,tp):GetFirst()
                let sel2 = duel::select_matching_card(
                    d,
                    tp,
                    Some(&|d, c| rednova_sp_filter(d, c, a.e, tp)),
                    tp,
                    LOCATION_EXTRA,
                    0,
                    1,
                    1,
                    false,
                    NO_EX,
                )
                .nil();
                // if not sc then return end
                let Some(sc) = group::get_first(d, &sel2) else { return };
                // Duel.BreakEffect()
                duel::break_effect(d);
                // if Duel.SpecialSummon(sc,SUMMON_TYPE_SYNCHRO,tp,tp,false,false,POS_FACEUP) then
                //   sc:CompleteProcedure()
                // end
                // (the result is a number, which is always true)
                duel::special_summon(d, CardSet::from_card(sc), SUMMON_TYPE_SYNCHRO, tp, tp, false, false, POS_FACEUP as u8, None);
                card::complete_procedure(d, sc);
            }
        }
    }
}
