//! Materiactor Exareptor
use super::prelude::*;

pub const ID: u32 = 37426272;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Add 1 Level 3 Normal Monster from your Deck or GY to your hand
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_TOHAND+CATEGORY_SPECIAL_SUMMON)
    effect::set_category(d, e1, CATEGORY_TOHAND + CATEGORY_SPECIAL_SUMMON);
    // e1:SetType(EFFECT_TYPE_IGNITION)
    effect::set_type(d, e1, EFFECT_TYPE_IGNITION);
    // e1:SetRange(LOCATION_HAND)
    effect::set_range(d, e1, LOCATION_HAND);
    // e1:SetCountLimit(1,id)
    effect::set_count_limit(d, e1, 1, ID, 0, 0);
    // e1:SetCost(s.thcost)
    effect::set_cost(d, e1, Some(cost_target(thcost)));
    // e1:SetTarget(s.thtg)
    effect::set_target(d, e1, Some(cost_target(thtg)));
    // e1:SetOperation(s.thop)
    effect::set_operation(d, e1, Some(operation(thop)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // Make 1 Rank 3 Xyz Monster you control gain 1500 ATK until the end of that turn
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e2, aux::stringid(ID, 1));
    // e2:SetCategory(CATEGORY_ATKCHANGE)
    effect::set_category(d, e2, CATEGORY_ATKCHANGE);
    // e2:SetType(EFFECT_TYPE_QUICK_O)
    effect::set_type(d, e2, EFFECT_TYPE_QUICK_O);
    // e2:SetProperty(EFFECT_FLAG_CARD_TARGET+EFFECT_FLAG_DAMAGE_STEP)
    effect::set_property(d, e2, EFFECT_FLAG_CARD_TARGET + EFFECT_FLAG_DAMAGE_STEP, 0);
    // e2:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e2, EVENT_FREE_CHAIN);
    // e2:SetRange(LOCATION_HAND)
    effect::set_range(d, e2, LOCATION_HAND);
    // e2:SetHintTiming(TIMING_DAMAGE_STEP)
    effect::set_hint_timing(d, e2, TIMING_DAMAGE_STEP, None);
    // e2:SetCondition(function() return not (Duel.IsPhase(PHASE_DAMAGE) and Duel.IsDamageCalculated()) end)
    effect::set_condition(d, e2, Some(condition(e2_cond)));
    // e2:SetCost(Cost.SelfDiscard)
    effect::set_cost(d, e2, Some(cost::self_discard()));
    // e2:SetTarget(s.atktg)
    effect::set_target(d, e2, Some(cost_target(atktg)));
    // e2:SetOperation(s.atkop)
    effect::set_operation(d, e2, Some(operation(atkop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
}
/// `function() return not (Duel.IsPhase(PHASE_DAMAGE) and Duel.IsDamageCalculated()) end`
fn e2_cond(d: &mut Duel, _a: &EffArgs) -> bool {
    !(duel::is_phase(d, PHASE_DAMAGE) && duel::is_damage_calculated(d))
}
/// `s.thcost(e,tp,eg,ep,ev,re,r,rp,chk)`
fn thcost(d: &mut Duel, a: &EffArgs) -> bool {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if chk==0 then return not c:IsPublic() and Duel.IsExistingMatchingCard(nil,tp,LOCATION_HAND,0,1,c) end
    if a.chk == Some(0) {
        return !card::is_public(d, c) && duel::is_existing_matching_card(d, None, a.tp, LOCATION_HAND, 0, 1, c.into());
    }
    // Duel.ConfirmCards(1-tp,c)
    duel::confirm_cards(d, 1 - a.tp, CardSet::from_card(c));
    // Duel.DiscardHand(tp,nil,1,1,REASON_COST|REASON_DISCARD,c)
    duel::discard_hand(d, a.tp, None, 1, 1, REASON_COST | REASON_DISCARD, c.into());
    // Duel.SetTargetCard(Duel.GetOperatedGroup():GetFirst())
    let operated = duel::get_operated_group(d);
    if let Some(tc) = group::get_first(d, &operated) {
        duel::set_target_card(d, CardSet::from_card(tc));
    }
    // Duel.ShuffleHand(tp)
    duel::shuffle_hand(d, a.tp);
    true
}
/// `s.thfilter(c)`
fn thfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsLevel(3) and c:IsType(TYPE_NORMAL) and c:IsAbleToHand()
    card::is_level(d, c, &[3]) && card::is_type(d, c, TYPE_NORMAL) && card::is_able_to_hand(d, c, None)
}
/// `s.thtg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn thtg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return Duel.IsExistingMatchingCard(s.thfilter,tp,LOCATION_DECK|LOCATION_GRAVE,0,1,nil) end
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, Some(&thfilter), a.tp, LOCATION_DECK | LOCATION_GRAVE, 0, 1, NO_EX);
    }
    // Duel.SetOperationInfo(0,CATEGORY_TOHAND,nil,1,tp,LOCATION_DECK|LOCATION_GRAVE)
    duel::set_operation_info(d, 0, CATEGORY_TOHAND, None, 1, a.tp, (LOCATION_DECK | LOCATION_GRAVE) as i32);
    // Duel.SetPossibleOperationInfo(0,CATEGORY_SPECIAL_SUMMON,e:GetHandler(),1,tp,0)
    let h = effect::get_handler(d, a.e).nil();
    duel::set_possible_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, Some(CardSet::from_card(h)), 1, a.tp, 0);
    true
}
/// `s.thop(e,tp,eg,ep,ev,re,r,rp)`
fn thop(d: &mut Duel, a: &EffArgs) {
    // local tc=Duel.GetFirstTarget()
    let tc = duel::first_target(d).nil();
    // local exc=tc:IsRelateToEffect(e) and tc or nil
    let exc: Exclude = if card::is_relate_to_effect(d, tc, a.e) { tc.into() } else { NO_EX };
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_ATOHAND)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_ATOHAND as u64);
    // local g=Duel.SelectMatchingCard(tp,aux.NecroValleyFilter(s.thfilter),tp,LOCATION_DECK|LOCATION_GRAVE,0,1,1,exc)
    let nv = aux::necro_valley_filter(|d: &mut Duel, c: CardId| thfilter(d, c));
    let g = duel::select_matching_card(d, a.tp, Some(&nv), a.tp, LOCATION_DECK | LOCATION_GRAVE, 0, 1, 1, false, exc).nil();
    // if #g>0 and Duel.SendtoHand(g,nil,REASON_EFFECT)>0 then
    if !g.is_empty() && duel::send_to_hand(d, g.clone(), None, REASON_EFFECT, None) > 0 {
        // Duel.ConfirmCards(1-tp,g)
        duel::confirm_cards(d, 1 - a.tp, g);
        // local c=e:GetHandler()
        let c = effect::get_handler(d, a.e).nil();
        // if c:IsRelateToEffect(e) and Duel.GetLocationCount(tp,LOCATION_MZONE)>0
        //     and c:IsCanBeSpecialSummoned(e,0,tp,false,false,POS_FACEUP_DEFENSE)
        //     and Duel.SelectYesNo(tp,aux.Stringid(id,2)) then
        if card::is_relate_to_effect(d, c, a.e)
            && duel::get_location_count(d, a.tp, LOCATION_MZONE as u8, None, None, None).0 > 0
            && card::is_can_be_special_summoned(d, c, a.e, 0, a.tp, false, false, Some(POS_FACEUP_DEFENSE as u8), None, None)
            && duel::select_yes_no(d, a.tp, aux::stringid(ID, 2))
        {
            // Duel.BreakEffect()
            duel::break_effect(d);
            // Duel.SpecialSummon(c,0,tp,tp,false,false,POS_FACEUP_DEFENSE)
            duel::special_summon(d, CardSet::from_card(c), 0, a.tp, a.tp, false, false, POS_FACEUP_DEFENSE as u8, None);
        }
    }
}
/// `s.atkfilter(c)`
fn atkfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsRank(3) and c:IsType(TYPE_XYZ) and c:IsFaceup()
    card::is_rank(d, c, &[3]) && card::is_type(d, c, TYPE_XYZ) && card::is_faceup(d, c)
}
/// `s.atktg(e,tp,eg,ep,ev,re,r,rp,chk,chkc)`
fn atktg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chkc then return chkc:IsLocation(LOCATION_MZONE) and chkc:IsControler(tp) and s.atkfilter(chkc) end
    if let Some(chkc) = a.chkc() {
        return card::is_location(d, chkc, LOCATION_MZONE) && card::is_controler(d, chkc, a.tp) && atkfilter(d, chkc);
    }
    // if chk==0 then return Duel.IsExistingTarget(s.atkfilter,tp,LOCATION_MZONE,0,1,nil) end
    if a.chk == Some(0) {
        return duel::is_existing_target(d, Some(&atkfilter), a.tp, LOCATION_MZONE, 0, 1, NO_EX);
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_ATKDEF)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_ATKDEF as u64);
    // Duel.SelectTarget(tp,s.atkfilter,tp,LOCATION_MZONE,0,1,1,nil)
    duel::select_target(d, a.tp, Some(&atkfilter), a.tp, LOCATION_MZONE, 0, 1, 1, false, NO_EX);
    true
}
/// `s.atkop(e,tp,eg,ep,ev,re,r,rp)`
fn atkop(d: &mut Duel, a: &EffArgs) {
    // local tc=Duel.GetFirstTarget()
    let tc = duel::first_target(d).nil();
    // if tc:IsRelateToEffect(e) and tc:IsFaceup() then
    if card::is_relate_to_effect(d, tc, a.e) && card::is_faceup(d, tc) {
        // local e1=Effect.CreateEffect(e:GetHandler())
        let h = effect::get_handler(d, a.e).nil();
        let e1 = effect::create_effect(d, h);
        // e1:SetType(EFFECT_TYPE_SINGLE)
        effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
        // e1:SetProperty(EFFECT_FLAG_CANNOT_DISABLE)
        effect::set_property(d, e1, EFFECT_FLAG_CANNOT_DISABLE, 0);
        // e1:SetCode(EFFECT_UPDATE_ATTACK)
        effect::set_code(d, e1, EFFECT_UPDATE_ATTACK);
        // e1:SetValue(1500)
        effect::set_value(d, e1, 1500);
        // e1:SetReset(RESETS_STANDARD_PHASE_END)
        effect::set_reset(d, e1, RESETS_STANDARD_PHASE_END, 1);
        // tc:RegisterEffect(e1)
        card::register_effect(d, tc, e1, false);
    }
}
