//! A Bao A Qu, the Lightless Shadow
use super::prelude::*;

pub const ID: u32 = 4731783;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // c:EnableReviveLimit()
    card::enable_revive_limit(d, c);
    // Link Summon procedure: 2+ monsters, including a Fiend monster
    // Link.AddProcedure(c,nil,2,4,s.matcheck)
    link::add_procedure(d, c, None, 2, Some(4), Some(group_check(matcheck)), None, None);
    // Activate 1 of these effects
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetType(EFFECT_TYPE_QUICK_O)
    effect::set_type(d, e1, EFFECT_TYPE_QUICK_O);
    // e1:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    // e1:SetRange(LOCATION_MZONE)
    effect::set_range(d, e1, LOCATION_MZONE);
    // e1:SetCountLimit(1,{id,0})
    effect::set_count_limit(d, e1, 1, ID, 0, 0);
    // e1:SetHintTiming(0,TIMING_MAIN_END|TIMINGS_CHECK_MONSTER)
    effect::set_hint_timing(d, e1, 0, Some(TIMING_MAIN_END | TIMINGS_CHECK_MONSTER));
    // e1:SetCondition(function() return Duel.IsMainPhase() end)
    effect::set_condition(d, e1, Some(condition(|d, _a| duel::is_main_phase(d, None))));
    // e1:SetCost(s.effcost)
    effect::set_cost(d, e1, Some(cost_target(effcost)));
    // e1:SetTarget(s.efftg)
    effect::set_target(d, e1, Some(cost_target(efftg)));
    // e1:SetOperation(s.effop)
    effect::set_operation(d, e1, Some(operation(effop)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // Draw cards equal to the number of different Monster Types among the monsters in your GY, then place the same number of cards from your hand on the bottom of the Deck in any order
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e2, aux::stringid(ID, 1));
    // e2:SetCategory(CATEGORY_DRAW+CATEGORY_TODECK)
    effect::set_category(d, e2, CATEGORY_DRAW + CATEGORY_TODECK);
    // e2:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e2, EFFECT_TYPE_FIELD + EFFECT_TYPE_TRIGGER_O);
    // e2:SetCode(EVENT_PHASE|PHASE_STANDBY)
    effect::set_code(d, e2, EVENT_PHASE | PHASE_STANDBY);
    // e2:SetRange(LOCATION_MZONE)
    effect::set_range(d, e2, LOCATION_MZONE);
    // e2:SetCountLimit(1,{id,1})
    effect::set_count_limit(d, e2, 1, ID, 1, 0);
    // e2:SetCondition(function(e,tp) return Duel.IsTurnPlayer(tp) end)
    effect::set_condition(d, e2, Some(condition(|d, a| duel::is_turn_player(d, a.tp))));
    // e2:SetTarget(s.drtg)
    effect::set_target(d, e2, Some(cost_target(drtg)));
    // e2:SetOperation(s.drop)
    effect::set_operation(d, e2, Some(operation(drop_op)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
}
/// `s.matcheck(g,lc,sumtype,tp)`
fn matcheck(d: &mut Duel, g: &CardSet, lc: CardId, sumtype: u64, tp: u8) -> bool {
    // return g:IsExists(Card.IsRace,1,nil,RACE_FIEND,lc,sumtype,tp)
    group::is_exists(d, g, &|d, x| card::is_race_ctx(d, x, RACE_FIEND, Some(lc), sumtype, Some(tp)), 1, NO_EX)
}
/// `s.effcost(e,tp,eg,ep,ev,re,r,rp,chk)`
fn effcost(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return Duel.IsExistingMatchingCard(Card.IsDiscardable,tp,LOCATION_HAND,0,1,nil) end
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, Some(&|d, c| card::is_discardable(d, c, None, None)), a.tp, LOCATION_HAND, 0, 1, NO_EX);
    }
    // Duel.DiscardHand(tp,Card.IsDiscardable,1,1,REASON_COST|REASON_DISCARD)
    duel::discard_hand(d, a.tp, Some(&|d, c| card::is_discardable(d, c, None, None)), 1, 1, REASON_COST | REASON_DISCARD, NO_EX);
    true
}
/// `s.spfilter(c,e,tp)`
fn spfilter(d: &mut Duel, c: CardId, e: EffectId, tp: u8) -> bool {
    // return c:IsAttribute(ATTRIBUTE_LIGHT|ATTRIBUTE_DARK) and c:IsCanBeSpecialSummoned(e,0,tp,false,false)
    card::is_attribute(d, c, ATTRIBUTE_LIGHT | ATTRIBUTE_DARK) && card::is_can_be_special_summoned(d, c, e, 0, tp, false, false, None, None, None)
}
/// `s.efftg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn efftg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // local b1=Duel.GetFieldGroupCount(tp,LOCATION_ONFIELD,LOCATION_ONFIELD)>0
    let b1 = duel::get_field_group_count(d, tp, LOCATION_ONFIELD, LOCATION_ONFIELD) > 0;
    // local b2=c:IsAbleToRemove() and Duel.GetMZoneCount(tp,c)>0
    //     and Duel.IsExistingMatchingCard(s.spfilter,tp,LOCATION_GRAVE,0,1,nil,e,tp)
    let b2 = card::is_able_to_remove(d, c, None, None, None)
        && duel::get_m_zone_count(d, tp, c.into(), None, None, None).0 > 0
        && duel::is_existing_matching_card(d, Some(&|d, x| spfilter(d, x, a.e, tp)), tp, LOCATION_GRAVE, 0, 1, NO_EX);
    // if chk==0 then return b1 or b2 end
    if a.chk == Some(0) {
        return b1 || b2;
    }
    // local op=Duel.SelectEffect(tp,{b1,aux.Stringid(id,2)},{b2,aux.Stringid(id,3)})
    let op = duel::select_effect(d, tp, &[(b1, aux::stringid(ID, 2)), (b2, aux::stringid(ID, 3))]).nil();
    // e:SetLabel(op)
    effect::set_label(d, a.e, &[op as i64]);
    // if op==1 then
    if op == 1 {
        // e:SetCategory(CATEGORY_DESTROY)
        effect::set_category(d, a.e, CATEGORY_DESTROY);
        // local g=Duel.GetFieldGroup(tp,LOCATION_ONFIELD,LOCATION_ONFIELD)
        let g = duel::get_field_group(d, tp, LOCATION_ONFIELD, LOCATION_ONFIELD);
        // Duel.SetOperationInfo(0,CATEGORY_DESTROY,g,1,tp,0)
        duel::set_operation_info(d, 0, CATEGORY_DESTROY, Some(g), 1, tp, 0);
    } else if op == 2 {
        // e:SetCategory(CATEGORY_REMOVE+CATEGORY_SPECIAL_SUMMON)
        effect::set_category(d, a.e, CATEGORY_REMOVE + CATEGORY_SPECIAL_SUMMON);
        // Duel.SetOperationInfo(0,CATEGORY_REMOVE,c,1,tp,0)
        duel::set_operation_info(d, 0, CATEGORY_REMOVE, Some(CardSet::from_card(c)), 1, tp, 0);
        // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,nil,1,tp,LOCATION_GRAVE)
        duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, None, 1, tp, LOCATION_GRAVE as i32);
    }
    true
}
/// `s.effop(e,tp,eg,ep,ev,re,r,rp)`
fn effop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // local op=e:GetLabel()
    let op = effect::get_label(d, a.e);
    // if op==1 then
    if op == 1 {
        // Destroy 1 card on the field
        // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_DESTROY)
        duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_DESTROY as u64);
        // local g=Duel.SelectMatchingCard(tp,nil,tp,LOCATION_ONFIELD,LOCATION_ONFIELD,1,1,nil)
        let g = duel::select_matching_card(d, tp, None, tp, LOCATION_ONFIELD, LOCATION_ONFIELD, 1, 1, false, NO_EX).nil();
        // if #g>0 then
        if !g.is_empty() {
            // Duel.HintSelection(g)
            duel::hint_selection(d, &g, true);
            // Duel.Destroy(g,REASON_EFFECT)
            duel::destroy(d, &g, REASON_EFFECT, None, None);
        }
    } else if op == 2 {
        // Banish this card (until the End Phase), and if you do, Special Summon 1 LIGHT or DARK monster from your GY
        // local c=e:GetHandler()
        let c = effect::get_handler(d, a.e).nil();
        // if c:IsRelateToEffect(e) and aux.RemoveUntil(c,nil,REASON_EFFECT,PHASE_END,id,e,tp,aux.DefaultFieldReturnOp)
        //     and Duel.GetLocationCount(tp,LOCATION_MZONE)>0 then
        if card::is_relate_to_effect(d, c, a.e)
            && aux::remove_until(d, c, None, REASON_EFFECT, PHASE_END, ID, a.e, tp, Some(aux::default_field_return()), None, None, None, None, None).is_some()
            && duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 > 0
        {
            // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_SPSUMMON)
            duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_SPSUMMON as u64);
            // local g=Duel.SelectMatchingCard(tp,aux.NecroValleyFilter(s.spfilter),tp,LOCATION_GRAVE,0,1,1,nil,e,tp)
            let e = a.e;
            let g = duel::select_matching_card(d, tp, Some(&aux::necro_valley_filter(move |d, x| spfilter(d, x, e, tp))), tp, LOCATION_GRAVE, 0, 1, 1, false, NO_EX).nil();
            // if #g>0 then
            if !g.is_empty() {
                // Duel.SpecialSummon(g,0,tp,tp,false,false,POS_FACEUP)
                duel::special_summon(d, &g, 0, tp, tp, false, false, POS_FACEUP as u8, None);
            }
        }
    }
}
/// `s.drtg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn drtg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // local ct=Duel.GetMatchingGroup(Card.IsMonster,tp,LOCATION_GRAVE,0,nil):GetClassCount(Card.GetRace)
    let g = duel::get_matching_group(d, Some(&card::is_monster), tp, LOCATION_GRAVE, 0, NO_EX);
    let ct = group::get_class_count(d, &g, &|d, x| card::get_race(d, x) as i64) as u32;
    // if chk==0 then return ct>0 and Duel.IsPlayerCanDraw(tp,ct) end
    if a.chk == Some(0) {
        return ct > 0 && duel::is_player_can_draw(d, tp, ct);
    }
    // Duel.SetTargetPlayer(tp)
    duel::set_target_player(d, tp);
    // Duel.SetOperationInfo(0,CATEGORY_DRAW,nil,0,tp,ct)
    duel::set_operation_info(d, 0, CATEGORY_DRAW, None, 0, tp, ct as i32);
    // Duel.SetOperationInfo(0,CATEGORY_TODECK,nil,ct,tp,LOCATION_HAND)
    duel::set_operation_info(d, 0, CATEGORY_TODECK, None, ct as u8, tp, LOCATION_HAND as i32);
    true
}
/// `s.drop(e,tp,eg,ep,ev,re,r,rp)`
fn drop_op(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // local p=Duel.GetChainInfo(0,CHAININFO_TARGET_PLAYER)
    let p = duel::get_chain_info(d, 0, &[CHAININFO_TARGET_PLAYER])[0].int() as u8;
    // local dn=Duel.GetMatchingGroup(Card.IsMonster,tp,LOCATION_GRAVE,0,nil):GetClassCount(Card.GetRace)
    let g = duel::get_matching_group(d, Some(&card::is_monster), tp, LOCATION_GRAVE, 0, NO_EX);
    let dn = group::get_class_count(d, &g, &|d, x| card::get_race(d, x) as i64) as u32;
    // local ct=Duel.Draw(p,d,REASON_EFFECT)
    let ct = duel::draw(d, p, dn, REASON_EFFECT);
    // if ct==0 then return end
    if ct == 0 {
        return;
    }
    // Duel.ShuffleHand(tp)
    duel::shuffle_hand(d, tp);
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_TODECK)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_TODECK as u64);
    // local g=Duel.SelectMatchingCard(tp,Card.IsAbleToDeck,tp,LOCATION_HAND,0,ct,ct,nil)
    let g = duel::select_matching_card(d, tp, Some(&card::is_able_to_deck), tp, LOCATION_HAND, 0, ct as u16, ct as u16, false, NO_EX).nil();
    // if #g==0 then return end
    if g.is_empty() {
        return;
    }
    // Duel.BreakEffect()
    duel::break_effect(d);
    // if Duel.SendtoDeck(g,nil,SEQ_DECKBOTTOM,REASON_EFFECT)>0 then
    if duel::send_to_deck(d, &g, None, SEQ_DECKBOTTOM as i32, REASON_EFFECT, None) > 0 {
        // Duel.SortDeckbottom(tp,tp,#g)
        duel::sort_deckbottom(d, tp, tp, g.len() as u16);
    }
}
