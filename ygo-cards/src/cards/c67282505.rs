//! Mitsu the Insect Ninja
use super::prelude::*;

pub const ID: u32 = 67282505;

pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_NINJA], listed_names: &[ID], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Special Summon itself from your hand
    let e1 = effect::create_effect(d, c);
    effect::set_description(d, e1, aux::stringid(ID, 0));
    effect::set_category(d, e1, CATEGORY_SPECIAL_SUMMON);
    effect::set_type(d, e1, EFFECT_TYPE_IGNITION);
    effect::set_range(d, e1, LOCATION_HAND);
    effect::set_count_limit(d, e1, 1, ID, 0, 0);
    effect::set_condition(d, e1, Some(condition(spcon)));
    effect::set_target(d, e1, Some(cost_target(sptg)));
    effect::set_operation(d, e1, Some(operation(spop)));
    card::register_effect(d, c, e1, false);
    // Change positions and negate activated effect
    let e2 = effect::create_effect(d, c);
    effect::set_description(d, e2, aux::stringid(ID, 1));
    effect::set_category(d, e2, CATEGORY_POSITION + CATEGORY_DISABLE + CATEGORY_SET);
    effect::set_type(d, e2, EFFECT_TYPE_QUICK_O);
    effect::set_property(d, e2, EFFECT_FLAG_CARD_TARGET, 0);
    effect::set_code(d, e2, EVENT_CHAINING);
    effect::set_range(d, e2, LOCATION_MZONE);
    effect::set_count_limit(d, e2, 1, ID, 1, 0);
    effect::set_condition(d, e2, Some(condition(negcon)));
    effect::set_target(d, e2, Some(cost_target(negtg)));
    effect::set_operation(d, e2, Some(operation(negop)));
    card::register_effect(d, c, e2, false);
}
fn cfilter(d: &mut Duel, c: CardId) -> bool {
    // return (c:IsFaceup() and c:IsSetCard(SET_NINJA)) or (c:IsPosition(POS_FACEDOWN_DEFENSE) and c:IsLocation(LOCATION_MZONE))
    (card::is_faceup(d, c) && card::is_set_card(d, c, &[SET_NINJA as u16]))
        || (card::is_position(d, c, POS_FACEDOWN_DEFENSE) && card::is_location(d, c, LOCATION_MZONE))
}
fn spcon(d: &mut Duel, a: &EffArgs) -> bool {
    // return Duel.IsExistingMatchingCard(s.cfilter,tp,LOCATION_ONFIELD,0,1,nil)
    duel::is_existing_matching_card(d, Some(&cfilter), a.tp, LOCATION_ONFIELD, 0, 1, NO_EX)
}
fn sptg(d: &mut Duel, a: &EffArgs) -> bool {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if chk==0 then return Duel.GetLocationCount(tp,LOCATION_MZONE)>0
    //   and c:IsCanBeSpecialSummoned(e,0,tp,false,false,POS_FACEUP) end
    if a.chk == Some(0) {
        return duel::get_location_count(d, a.tp, LOCATION_MZONE as u8, None, None, None).0 > 0
            && card::is_can_be_special_summoned(d, c, a.e, 0, a.tp, false, false, Some(POS_FACEUP as u8), None, None);
    }
    // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,c,1,tp,LOCATION_HAND)
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, Some(CardSet::from_card(c)), 1, a.tp, LOCATION_HAND as i32);
    true
}
fn spop(d: &mut Duel, a: &EffArgs) {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if not c:IsRelateToEffect(e) then return end
    if !card::is_relate_to_effect(d, c, a.e) {
        return;
    }
    // Duel.SpecialSummon(c,0,tp,tp,false,false,POS_FACEUP)
    duel::special_summon(d, c, 0, a.tp, a.tp, false, false, POS_FACEUP as u8, None);
}
fn negcon(d: &mut Duel, a: &EffArgs) -> bool {
    // return ep==1-tp and re:IsMonsterEffect() and Duel.IsChainDisablable(ev)
    a.ep == 1 - a.tp && effect::is_monster_effect(d, a.re.nil()) && duel::is_chain_disablable(d, a.ev as u8)
}
fn negtg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chkc then return chkc:IsLocation(LOCATION_MZONE) and chkc:IsControler(tp) and chkc:IsPosition(POS_FACEDOWN_DEFENSE) end
    if let Some(chkc) = a.chkc() {
        return card::is_location(d, chkc, LOCATION_MZONE) && card::is_controler(d, chkc, a.tp) && card::is_position(d, chkc, POS_FACEDOWN_DEFENSE);
    }
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if chk==0 then return Duel.IsExistingTarget(Card.IsPosition,tp,LOCATION_MZONE,0,1,nil,POS_FACEDOWN_DEFENSE) and c:IsCanTurnSet() end
    if a.chk == Some(0) {
        return duel::is_existing_target(d, Some(&|d, c| card::is_position(d, c, POS_FACEDOWN_DEFENSE)), a.tp, LOCATION_MZONE, 0, 1, NO_EX)
            && card::is_can_turn_set(d, c);
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_POSCHANGE)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_POSCHANGE as u64);
    // local tc=Duel.SelectTarget(tp,Card.IsPosition,tp,LOCATION_MZONE,0,1,1,nil,POS_FACEDOWN_DEFENSE):GetFirst()
    let g = duel::select_target(
        d,
        a.tp,
        Some(&|d, c| card::is_position(d, c, POS_FACEDOWN_DEFENSE)),
        a.tp,
        LOCATION_MZONE,
        0,
        1,
        1,
        false,
        NO_EX,
    )
    .nil();
    let tc = group::get_first(d, &g);
    // Duel.SetOperationInfo(0,CATEGORY_POSITION,Group.FromCards(c,tc),1,tp,POS_FACEUP_DEFENSE|POS_FACEDOWN_DEFENSE)
    let mut cards = CardSet::from_card(c);
    if let Some(tc) = tc {
        group::add_card(&mut cards, &CardSet::from_card(tc));
    }
    duel::set_operation_info(d, 0, CATEGORY_POSITION, Some(cards), 1, a.tp, (POS_FACEUP_DEFENSE | POS_FACEDOWN_DEFENSE) as i32);
    // if tc:IsSetCard(SET_NINJA) and not tc:IsCode(id) then
    if card::is_set_card(d, tc.nil(), &[SET_NINJA as u16]) && !card::is_code(d, tc.nil(), &[ID]) {
        // Duel.SetOperationInfo(0,CATEGORY_DISABLE,eg,1,0,0)
        duel::set_operation_info(d, 0, CATEGORY_DISABLE, a.eg.clone(), 1, 0, 0);
    }
    true
}
fn negop(d: &mut Duel, a: &EffArgs) {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // local tc=Duel.GetFirstTarget()
    let tc = duel::first_target(d).nil();
    // if tc:IsRelateToEffect(e) and tc:IsFacedown() and Duel.ChangePosition(tc,POS_FACEUP_DEFENSE)>0
    //   and c:IsRelateToEffect(e) and c:IsFaceup() and Duel.ChangePosition(c,POS_FACEDOWN_DEFENSE)>0
    //   and tc:IsSetCard(SET_NINJA) and not tc:IsCode(id) then
    if card::is_relate_to_effect(d, tc, a.e)
        && card::is_facedown(d, tc)
        && duel::change_position(d, tc, POS_FACEUP_DEFENSE as u8, None, None, None, false) > 0
        && card::is_relate_to_effect(d, c, a.e)
        && card::is_faceup(d, c)
        && duel::change_position(d, c, POS_FACEDOWN_DEFENSE as u8, None, None, None, false) > 0
        && card::is_set_card(d, tc, &[SET_NINJA as u16])
        && !card::is_code(d, tc, &[ID])
    {
        // Duel.BreakEffect()
        duel::break_effect(d);
        // Duel.NegateEffect(ev)
        duel::negate_effect(d, a.ev as u8);
    }
}
