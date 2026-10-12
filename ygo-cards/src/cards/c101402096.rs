//! Angelechy Verdict
use super::prelude::*;

pub const ID: u32 = 101402096;
pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_ANGELECHY], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Apply this effect based on the number of "Angelechy" Monster Cards on the field
    // ● 1 to 4: Negate the effects of 1 face-up card your opponent controls
    // ● 5+: Banish all cards your opponent controls
    let e1 = effect::create_effect(d, c);
    effect::set_description(d, e1, aux::stringid(ID, 0));
    effect::set_category(d, e1, CATEGORY_DISABLE + CATEGORY_REMOVE);
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    effect::set_count_limit(d, e1, 1, ID, 0, 0);
    effect::set_target(d, e1, Some(cost_target(efftg)));
    effect::set_operation(d, e1, Some(operation(effop)));
    effect::set_hint_timing(d, e1, 0, Some(TIMING_STANDBY_PHASE | TIMING_MAIN_END | TIMINGS_CHECK_MONSTER_E));
    card::register_effect(d, c, e1, false);
    // You can banish this card from your GY; send 1 "Angelechy" card from your hand or face-up field to the GY, then you can place 1 "Angelechy" monster with a different name than the cards in your Spell & Trap Zone, from your Extra Deck in your Spell & Trap Zone as a face-up Continuous Spell
    let e2 = effect::create_effect(d, c);
    effect::set_description(d, e2, aux::stringid(ID, 1));
    effect::set_category(d, e2, CATEGORY_TOGRAVE);
    effect::set_type(d, e2, EFFECT_TYPE_QUICK_O);
    effect::set_code(d, e2, EVENT_FREE_CHAIN);
    effect::set_range(d, e2, LOCATION_GRAVE);
    effect::set_count_limit(d, e2, 1, ID, 1, 0);
    effect::set_cost(d, e2, Some(cost::self_banish()));
    effect::set_target(d, e2, Some(cost_target(tgtg)));
    effect::set_operation(d, e2, Some(operation(tgop)));
    effect::set_hint_timing(d, e2, 0, Some(TIMING_STANDBY_PHASE | TIMING_MAIN_END | TIMINGS_CHECK_MONSTER_E));
    card::register_effect(d, c, e2, false);
}
// s.countfilter(c)
fn countfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsSetCard(SET_ANGELECHY) and c:IsMonsterCard() and c:IsFaceup()
    card::is_set_card(d, c, &[SET_ANGELECHY as u16]) && card::is_monster_card(d, c) && card::is_faceup(d, c)
}
// s.efftg(e,tp,eg,ep,ev,re,r,rp,chk)
fn efftg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    if a.chk == Some(0) {
        // local ct=Duel.GetMatchingGroupCount(s.countfilter,tp,LOCATION_ONFIELD,LOCATION_ONFIELD,1,nil)
        let ct = duel::get_matching_group_count(d, Some(&countfilter), tp, LOCATION_ONFIELD, LOCATION_ONFIELD, NO_EX);
        // if ct==0 then return false end
        if ct == 0 {
            return false;
        }
        // if ct<5 then return Duel.IsExistingMatchingCard(Card.IsNegatable,tp,0,LOCATION_ONFIELD,1,nil) end
        if ct < 5 {
            return duel::is_existing_matching_card(d, Some(&|d, c| card::is_negatable(d, c)), tp, 0, LOCATION_ONFIELD, 1, NO_EX);
        }
        // return Duel.IsExistingMatchingCard(Card.IsAbleToRemove,tp,0,LOCATION_ONFIELD,1,nil)
        return duel::is_existing_matching_card(d, Some(&|d, c| card::is_able_to_remove(d, c, None, None, None)), tp, 0, LOCATION_ONFIELD, 1, NO_EX);
    }
    // Duel.SetPossibleOperationInfo(0,CATEGORY_DISABLE,nil,1,1-tp,LOCATION_ONFIELD)
    duel::set_possible_operation_info(d, 0, CATEGORY_DISABLE, None, 1, 1 - tp, LOCATION_ONFIELD as i32);
    // Duel.SetPossibleOperationInfo(0,CATEGORY_REMOVE,nil,1,1-tp,LOCATION_ONFIELD)
    duel::set_possible_operation_info(d, 0, CATEGORY_REMOVE, None, 1, 1 - tp, LOCATION_ONFIELD as i32);
    true
}
// s.effop(e,tp,eg,ep,ev,re,r,rp)
fn effop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // local ct=Duel.GetMatchingGroupCount(s.countfilter,tp,LOCATION_ONFIELD,LOCATION_ONFIELD,1,nil)
    let ct = duel::get_matching_group_count(d, Some(&countfilter), tp, LOCATION_ONFIELD, LOCATION_ONFIELD, NO_EX);
    // if ct==0 then return end
    if ct == 0 {
        return;
    }
    if ct < 5 {
        // ● 1 to 4: Negate the effects of 1 face-up card your opponent controls
        // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_NEGATE)
        duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_NEGATE as u64);
        // local tc=Duel.SelectMatchingCard(tp,Card.IsNegatable,tp,0,LOCATION_ONFIELD,1,1,nil):GetFirst()
        let g = duel::select_matching_card(d, tp, Some(&|d, c| card::is_negatable(d, c)), tp, 0, LOCATION_ONFIELD, 1, 1, false, NO_EX).nil();
        // if tc then
        if let Some(tc) = group::get_first(d, &g) {
            // Duel.HintSelection(tc)
            duel::hint_selection(d, CardSet::from_card(tc), true);
            // tc:NegateEffects(e:GetHandler(),nil,true)
            let h = effect::get_handler(d, a.e).nil();
            card::negate_effects(d, tc, h, None, true, None);
        }
    } else {
        // ● 5+: Banish all cards your opponent controls
        // local g=Duel.GetMatchingGroup(Card.IsAbleToRemove,tp,0,LOCATION_ONFIELD,nil)
        let g = duel::get_matching_group(d, Some(&|d, c| card::is_able_to_remove(d, c, None, None, None)), tp, 0, LOCATION_ONFIELD, NO_EX);
        // if #g>0 then Duel.Remove(g,POS_FACEUP,REASON_EFFECT) end
        if !g.is_empty() {
            duel::remove(d, g, POS_FACEUP as u8, REASON_EFFECT, None, None);
        }
    }
}
// s.tgfilter(c)
fn tgfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsSetCard(SET_ANGELECHY) and c:IsAbleToGrave() and (c:IsFaceup() or c:IsLocation(LOCATION_HAND))
    card::is_set_card(d, c, &[SET_ANGELECHY as u16])
        && card::is_able_to_grave(d, c, None)
        && (card::is_faceup(d, c) || card::is_location(d, c, LOCATION_HAND))
}
// s.plfilter(c,...) with codes
fn plfilter(d: &mut Duel, c: CardId, codes: &[u32]) -> bool {
    // return c:IsSetCard(SET_ANGELECHY) and not c:IsForbidden() and not (... and c:IsCode(...))
    card::is_set_card(d, c, &[SET_ANGELECHY as u16]) && !card::is_forbidden(d, c) && !card::is_code(d, c, codes)
}
// Duel.GetMatchingGroup(Card.IsFaceup,tp,LOCATION_STZONE,0,nil):GetClass(Card.GetCode)
fn stzone_codes(d: &mut Duel, tp: u8) -> Vec<u32> {
    let g = duel::get_matching_group(d, Some(&|d, c| card::is_faceup(d, c)), tp, LOCATION_STZONE, 0, NO_EX);
    group::get_class(d, &g, &|d, c| card::get_code(d, c)[0] as i64).into_iter().map(|x| x as u32).collect()
}
// s.tgtg(e,tp,eg,ep,ev,re,r,rp,chk)
fn tgtg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return Duel.IsExistingMatchingCard(s.tgfilter,tp,LOCATION_HAND|LOCATION_ONFIELD,0,1,nil) end
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, Some(&tgfilter), a.tp, LOCATION_HAND | LOCATION_ONFIELD, 0, 1, NO_EX);
    }
    // Duel.SetOperationInfo(0,CATEGORY_TOGRAVE,nil,1,tp,LOCATION_HAND|LOCATION_ONFIELD)
    duel::set_operation_info(d, 0, CATEGORY_TOGRAVE, None, 1, a.tp, (LOCATION_HAND | LOCATION_ONFIELD) as i32);
    true
}
// s.tgop(e,tp,eg,ep,ev,re,r,rp)
fn tgop(d: &mut Duel, a: &EffArgs) {
    let (e, tp) = (a.e, a.tp);
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_TOGRAVE)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_TOGRAVE as u64);
    // local g=Duel.SelectMatchingCard(tp,s.tgfilter,tp,LOCATION_HAND|LOCATION_ONFIELD,0,1,1,nil)
    let g = duel::select_matching_card(d, tp, Some(&tgfilter), tp, LOCATION_HAND | LOCATION_ONFIELD, 0, 1, 1, false, NO_EX).nil();
    // if #g==0 or Duel.SendtoGrave(g,REASON_EFFECT)==0
    //   or not g:GetFirst():IsLocation(LOCATION_GRAVE)
    //   or Duel.GetLocationCount(tp,LOCATION_SZONE,0)==0 then return end
    if g.is_empty()
        || duel::send_to_grave(d, g.clone(), REASON_EFFECT, None, None) == 0
        || !card::is_location(d, group::get_first(d, &g).nil(), LOCATION_GRAVE)
        || duel::get_location_count(d, tp, LOCATION_SZONE as u8, Some(0), None, None).0 == 0
    {
        return;
    }
    // local codes=Duel.GetMatchingGroup(Card.IsFaceup,tp,LOCATION_STZONE,0,nil):GetClass(Card.GetCode)
    let codes = stzone_codes(d, tp);
    // local plg=Duel.GetMatchingGroup(s.plfilter,tp,LOCATION_EXTRA,0,nil,codes)
    let plg = duel::get_matching_group(d, Some(&|d, c| plfilter(d, c, &codes)), tp, LOCATION_EXTRA, 0, NO_EX);
    // if #plg==0 or not Duel.SelectYesNo(tp,aux.Stringid(id,2)) then return end
    if plg.is_empty() || !duel::select_yes_no(d, tp, aux::stringid(ID, 2)) {
        return;
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_TOFIELD)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_TOFIELD as u64);
    // local plc=plg:Select(tp,1,1,nil):GetFirst()
    let sel = group::select(d, &plg, tp, 1, 1, false, NO_EX).nil();
    // if not plc then return end
    let Some(plc) = group::get_first(d, &sel) else { return };
    // Duel.BreakEffect()
    duel::break_effect(d);
    // if Duel.MoveToField(plc,tp,tp,LOCATION_SZONE,POS_FACEUP,true) then
    if duel::move_to_field(d, plc, tp, tp, LOCATION_SZONE as u16, POS_FACEUP as u8, true, None) {
        // Treated as a Continuous Spell
        // local e1=Effect.CreateEffect(e:GetHandler())
        let h = effect::get_handler(d, e).nil();
        let e1 = effect::create_effect(d, h);
        // e1:SetType(EFFECT_TYPE_SINGLE)
        effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
        // e1:SetProperty(EFFECT_FLAG_CANNOT_DISABLE)
        effect::set_property(d, e1, EFFECT_FLAG_CANNOT_DISABLE, 0);
        // e1:SetCode(EFFECT_CHANGE_TYPE)
        effect::set_code(d, e1, EFFECT_CHANGE_TYPE);
        // e1:SetValue(TYPE_SPELL|TYPE_CONTINUOUS)
        effect::set_value(d, e1, (TYPE_SPELL | TYPE_CONTINUOUS) as i64);
        // e1:SetReset(RESET_EVENT|(RESETS_STANDARD&~RESET_TURN_SET))
        effect::set_reset(d, e1, RESET_EVENT | (RESETS_STANDARD & !RESET_TURN_SET), 1);
        // plc:RegisterEffect(e1)
        card::register_effect(d, plc, e1, false);
    }
}
