//! Magia Magic - Thunder of Judgment
use super::prelude::*;

pub const ID: u32 = 59080;

pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_MAGISTUS, SET_ENDYMION], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate 1 of these effects
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    // e1:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    // e1:SetCost(s.cost)
    effect::set_cost(d, e1, Some(cost_target(cost)));
    // e1:SetTarget(s.target)
    effect::set_target(d, e1, Some(cost_target(target)));
    // e1:SetOperation(s.activate)
    effect::set_operation(d, e1, Some(operation(activate)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
// function s.costfilter(c,e,tp)
fn costfilter(d: &mut Duel, c: CardId, e: EffectId, tp: u8) -> bool {
    // return c:IsSetCard(SET_MAGISTUS) and c:IsFaceup() and c:IsAbleToGraveAsCost()
    //     and ((not Duel.HasFlagEffect(tp,id) and Duel.IsExistingMatchingCard(s.spfilter,tp,LOCATION_HAND|LOCATION_GRAVE|LOCATION_EXTRA,0,1,nil,e,tp))
    //     or (not Duel.HasFlagEffect(tp,id+1) and Duel.IsExistingMatchingCard(Card.IsAbleToRemove,tp,LOCATION_ONFIELD,LOCATION_ONFIELD,1,Group.FromCards(e:GetHandler(),c))))
    if !(card::is_set_card(d, c, &[SET_MAGISTUS as u16]) && card::is_faceup(d, c) && card::is_able_to_grave_as_cost(d, c)) {
        return false;
    }
    if !duel::has_flag_effect(d, tp, ID, None)
        && duel::is_existing_matching_card(d, Some(&|d, x| spfilter(d, x, e, tp)), tp, LOCATION_HAND | LOCATION_GRAVE | LOCATION_EXTRA, 0, 1, NO_EX)
    {
        return true;
    }
    // Group.FromCards(e:GetHandler(),c)
    let h = effect::get_handler(d, e);
    let mut ex = CardSet::new();
    if let Some(h) = h {
        group::add_card(&mut ex, &CardSet::from_card(h));
    }
    group::add_card(&mut ex, &CardSet::from_card(c));
    !duel::has_flag_effect(d, tp, ID + 1, None)
        && duel::is_existing_matching_card(d, Some(&|d, x| card::is_able_to_remove(d, x, None, None, None)), tp, LOCATION_ONFIELD, LOCATION_ONFIELD, 1, (&ex).into())
}
// function s.cost(e,tp,eg,ep,ev,re,r,rp,chk)
fn cost(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // local b1=Duel.IsExistingMatchingCard(s.costfilter,tp,LOCATION_ONFIELD,0,1,c,e,tp)
    let b1 = duel::is_existing_matching_card(d, Some(&|d, x| costfilter(d, x, a.e, tp)), tp, LOCATION_ONFIELD, 0, 1, c.into());
    // local b2=Duel.IsCanRemoveCounter(tp,1,0,COUNTER_SPELL,2,REASON_COST)
    let b2 = duel::is_can_remove_counter(d, tp, 1, 0, COUNTER_SPELL as u16, 2, REASON_COST);
    // if chk==0 then return b1 or b2 end
    if a.chk == Some(0) {
        return b1 || b2;
    }
    // local op=nil
    // if b1 and b2 then
    //     op=Duel.SelectEffect(tp, {b1,aux.Stringid(id,1)}, {b2,aux.Stringid(id,2)})
    // else
    //     op=(b1 and 1) or (b2 and 2)
    // end
    let op: Option<usize> = if b1 && b2 {
        duel::select_effect(d, tp, &[(b1, aux::stringid(ID, 1)), (b2, aux::stringid(ID, 2))])
    } else if b1 {
        Some(1)
    } else if b2 {
        Some(2)
    } else {
        None
    };
    if op == Some(1) {
        // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_TOGRAVE)
        duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_TOGRAVE as u64);
        // local g=Duel.SelectMatchingCard(tp,s.costfilter,tp,LOCATION_ONFIELD,0,1,1,c,e,tp)
        let g = duel::select_matching_card(d, tp, Some(&|d, x| costfilter(d, x, a.e, tp)), tp, LOCATION_ONFIELD, 0, 1, 1, false, c.into()).nil();
        // Duel.SendtoGrave(g,REASON_COST)
        duel::send_to_grave(d, g, REASON_COST, None, None);
    } else if op == Some(2) {
        // Duel.RemoveCounter(tp,1,0,COUNTER_SPELL,2,REASON_COST)
        duel::remove_counter(d, tp, 1, 0, COUNTER_SPELL as u16, 2, REASON_COST);
    }
    true
}
// function s.spfilter(c,e,tp)
fn spfilter(d: &mut Duel, c: CardId, e: EffectId, tp: u8) -> bool {
    // if c:IsLocation(LOCATION_EXTRA) and Duel.GetLocationCountFromEx(tp,tp,nil,c)<=0 then return false end
    if card::is_location(d, c, LOCATION_EXTRA) && duel::get_location_count_from_ex(d, tp, Some(tp), NO_EX, duel::ExtraCard::Card(c), None).0 <= 0 {
        return false;
    }
    // if not c:IsLocation(LOCATION_EXTRA) and Duel.GetLocationCount(tp,LOCATION_MZONE)<=0 then return false end
    if !card::is_location(d, c, LOCATION_EXTRA) && duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 <= 0 {
        return false;
    }
    // return c:IsRace(RACE_SPELLCASTER) and c:IsCanBeSpecialSummoned(e,0,tp,false,false) and (c:IsFaceup() or not c:IsLocation(LOCATION_EXTRA))
    card::is_race(d, c, RACE_SPELLCASTER)
        && card::is_can_be_special_summoned(d, c, e, 0, tp, false, false, None, None, None)
        && (card::is_faceup(d, c) || !card::is_location(d, c, LOCATION_EXTRA))
}
// function s.target(e,tp,eg,ep,ev,re,r,rp,chk)
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // local b1=not Duel.HasFlagEffect(tp,id)
    //     and Duel.IsExistingMatchingCard(s.spfilter,tp,LOCATION_HAND|LOCATION_GRAVE|LOCATION_EXTRA,0,1,nil,e,tp)
    let b1 = !duel::has_flag_effect(d, tp, ID, None)
        && duel::is_existing_matching_card(d, Some(&|d, x| spfilter(d, x, a.e, tp)), tp, LOCATION_HAND | LOCATION_GRAVE | LOCATION_EXTRA, 0, 1, NO_EX);
    // local b2=not Duel.HasFlagEffect(tp,id+1)
    //     and Duel.IsExistingMatchingCard(Card.IsAbleToRemove,tp,LOCATION_ONFIELD,LOCATION_ONFIELD,1,e:GetHandler())
    let h = effect::get_handler(d, a.e);
    let b2 = !duel::has_flag_effect(d, tp, ID + 1, None)
        && duel::is_existing_matching_card(d, Some(&|d, x| card::is_able_to_remove(d, x, None, None, None)), tp, LOCATION_ONFIELD, LOCATION_ONFIELD, 1, h.into());
    // if chk==0 then return b1 or b2 end
    if a.chk == Some(0) {
        return b1 || b2;
    }
    // local op=Duel.SelectEffect(tp, {b1,aux.Stringid(id,3)}, {b2,aux.Stringid(id,4)})
    let op = duel::select_effect(d, tp, &[(b1, aux::stringid(ID, 3)), (b2, aux::stringid(ID, 4))]);
    // e:SetLabel(op)
    effect::set_label(d, a.e, &[op.unwrap_or(0) as i64]);
    if op == Some(1) {
        // e:SetCategory(CATEGORY_SPECIAL_SUMMON)
        effect::set_category(d, a.e, CATEGORY_SPECIAL_SUMMON);
        // Duel.RegisterFlagEffect(tp,id,RESET_PHASE|PHASE_END,0,1)
        duel::register_flag_effect(d, tp, ID, RESET_PHASE | PHASE_END, 0, 1, 0);
        // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,nil,1,tp,LOCATION_HAND|LOCATION_GRAVE|LOCATION_EXTRA)
        duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, None, 1, tp, (LOCATION_HAND | LOCATION_GRAVE | LOCATION_EXTRA) as i32);
    } else if op == Some(2) {
        // e:SetCategory(CATEGORY_REMOVE)
        effect::set_category(d, a.e, CATEGORY_REMOVE);
        // Duel.RegisterFlagEffect(tp,id+1,RESET_PHASE|PHASE_END,0,1)
        duel::register_flag_effect(d, tp, ID + 1, RESET_PHASE | PHASE_END, 0, 1, 0);
        // local g=Duel.GetMatchingGroup(Card.IsAbleToRemove,tp,LOCATION_ONFIELD,LOCATION_ONFIELD,nil)
        let g = duel::get_matching_group(d, Some(&|d, x| card::is_able_to_remove(d, x, None, None, None)), tp, LOCATION_ONFIELD, LOCATION_ONFIELD, NO_EX);
        // Duel.SetOperationInfo(0,CATEGORY_REMOVE,g,1,tp,0)
        duel::set_operation_info(d, 0, CATEGORY_REMOVE, Some(g), 1, tp, 0);
    }
    true
}
// function s.activate(e,tp,eg,ep,ev,re,r,rp)
fn activate(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // local op=e:GetLabel()
    let op = effect::get_label(d, a.e);
    if op == 1 {
        // Special Summon 1 Spellcaster monster from your hand, face-up Extra Deck, or GY
        // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_SPSUMMON)
        duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_SPSUMMON as u64);
        // local g=Duel.SelectMatchingCard(tp,aux.NecroValleyFilter(s.spfilter),tp,LOCATION_HAND|LOCATION_GRAVE|LOCATION_EXTRA,0,1,1,nil,e,tp)
        let g = duel::select_matching_card(
            d,
            tp,
            Some(&aux::necro_valley_filter(|d, x| spfilter(d, x, a.e, tp))),
            tp,
            LOCATION_HAND | LOCATION_GRAVE | LOCATION_EXTRA,
            0,
            1,
            1,
            false,
            NO_EX,
        )
        .nil();
        // if #g>0 then
        if !g.is_empty() {
            // Duel.SpecialSummon(g,0,tp,tp,false,false,POS_FACEUP)
            duel::special_summon(d, g, 0, tp, tp, false, false, POS_FACEUP as u8, None);
        }
    } else if op == 2 {
        // Banish 1 other card on the field
        // local c=e:GetHandler()
        let c = effect::get_handler(d, a.e).nil();
        // local exc=c:IsRelateToEffect(e) and c or nil
        let exc = if card::is_relate_to_effect(d, c, a.e) { Some(c) } else { None };
        // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_REMOVE)
        duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_REMOVE as u64);
        // local g=Duel.SelectMatchingCard(tp,Card.IsAbleToRemove,tp,LOCATION_ONFIELD,LOCATION_ONFIELD,1,1,exc)
        let g = duel::select_matching_card(
            d,
            tp,
            Some(&|d, x| card::is_able_to_remove(d, x, None, None, None)),
            tp,
            LOCATION_ONFIELD,
            LOCATION_ONFIELD,
            1,
            1,
            false,
            exc.into(),
        )
        .nil();
        // if #g>0 then
        if !g.is_empty() {
            // Duel.HintSelection(g)
            duel::hint_selection(d, g.clone(), true);
            // Duel.Remove(g,POS_FACEUP,REASON_EFFECT)
            duel::remove(d, g, POS_FACEUP as u8, REASON_EFFECT, None, None);
        }
    }
}
