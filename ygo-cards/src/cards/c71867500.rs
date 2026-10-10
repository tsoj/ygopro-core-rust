//! Dragon Revival Rhapsody
use super::prelude::*;

pub const ID: u32 = 71867500;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Special Summon up to 2 Dragon monsters from the GY, including a Normal monster
    let e1 = effect::create_effect(d, c);
    effect::set_description(d, e1, aux::stringid(ID, 0));
    effect::set_category(d, e1, CATEGORY_SPECIAL_SUMMON);
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    effect::set_property(d, e1, EFFECT_FLAG_CARD_TARGET, 0);
    effect::set_count_limit(d, e1, 1, ID, 0, EFFECT_COUNT_CODE_OATH as u8);
    effect::set_condition(d, e1, Some(condition(cond)));
    effect::set_target(d, e1, Some(cost_target(target)));
    effect::set_operation(d, e1, Some(operation(activate)));
    card::register_effect(d, c, e1, false);
}
/// `s.condition(e,tp,eg,ep,ev,re,r,rp)`
fn cond(d: &mut Duel, a: &EffArgs) -> bool {
    // return Duel.IsExistingMatchingCard(aux.FaceupFilter(Card.IsRace,RACE_SPELLCASTER),tp,LOCATION_MZONE,0,1,nil)
    duel::is_existing_matching_card(d, Some(&aux::faceup_filter(|d, c| card::is_race(d, c, RACE_SPELLCASTER))), a.tp, LOCATION_MZONE, 0, 1, NO_EX)
}
/// `s.spfilter(c,e,tp)`
fn spfilter(d: &mut Duel, c: CardId, e: EffectId, tp: u8) -> bool {
    // return c:IsRace(RACE_DRAGON) and c:IsCanBeSpecialSummoned(e,0,tp,false,false)
    card::is_race(d, c, RACE_DRAGON) && card::is_can_be_special_summoned(d, c, e, 0, tp, false, false, None, None, None)
}
/// `s.rescon(sg,e,tp,mg)`
fn rescon(d: &mut Duel, sg: &CardSet, _e: Option<EffectId>, _tp: u8, _mg: &CardSet, _c: Option<CardId>) -> (bool, bool) {
    // return sg:IsExists(Card.IsType,1,nil,TYPE_NORMAL)
    (group::is_exists(d, sg, &|d, x| card::is_type(d, x, TYPE_NORMAL), 1, NO_EX), false)
}
/// `s.target(e,tp,eg,ep,ev,re,r,rp,chk,chkc)`
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // if chkc then return false end
    if a.chkc().is_some() {
        return false;
    }
    // local ft=math.min(Duel.GetLocationCount(tp,LOCATION_MZONE),Duel.IsPlayerAffectedByEffect(tp,CARD_BLUEEYES_SPIRIT) and 1 or 2)
    let zone = duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0;
    let spirit = duel::is_player_affected_by_effect(d, tp, CARD_BLUEEYES_SPIRIT).is_some();
    let ft = zone.min(if spirit { 1 } else { 2 });
    // local g=Duel.GetMatchingGroup(s.spfilter,tp,LOCATION_GRAVE,0,nil,e,tp)
    let g = duel::get_matching_group(d, Some(&|d, c| spfilter(d, c, a.e, tp)), tp, LOCATION_GRAVE, 0, NO_EX);
    // if chk==0 then return ft>0 and #g>0 and aux.SelectUnselectGroup(g,e,tp,1,ft,s.rescon,0) end
    if a.chk == Some(0) {
        return ft > 0
            && !g.is_empty()
            && matches!(
                aux::select_unselect_group(d, &g, Some(a.e), tp, Some(1), Some(ft.max(0) as usize), Some(&rescon), 0, tp, None, None, None, false),
                Err(true)
            );
    }
    // local sg=aux.SelectUnselectGroup(g,e,tp,1,ft,s.rescon,1,tp,HINTMSG_SPSUMMON,s.rescon)
    let sg = aux::select_unselect_group(
        d,
        &g,
        Some(a.e),
        tp,
        Some(1),
        Some(ft.max(0) as usize),
        Some(&rescon),
        1,
        tp,
        Some(HINTMSG_SPSUMMON as u64),
        Some(&rescon),
        None,
        false,
    )
    .unwrap_or_default();
    // Duel.SetTargetCard(sg)
    duel::set_target_card(d, sg.clone());
    // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,sg,#sg,tp,0)
    let n = sg.len() as u8;
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, Some(sg), n, tp, 0);
    true
}
/// `s.activate(e,tp,eg,ep,ev,re,r,rp)`
fn activate(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // local e1=Effect.CreateEffect(e:GetHandler())
    let h = effect::get_handler(d, a.e).nil();
    let e1 = effect::create_effect(d, h);
    // e1:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e1, EFFECT_TYPE_FIELD);
    // e1:SetCode(EFFECT_CHANGE_DAMAGE)
    effect::set_code(d, e1, EFFECT_CHANGE_DAMAGE);
    // e1:SetProperty(EFFECT_FLAG_PLAYER_TARGET)
    effect::set_property(d, e1, EFFECT_FLAG_PLAYER_TARGET, 0);
    // e1:SetTargetRange(0,1)
    effect::set_target_range(d, e1, 0, 1);
    // e1:SetValue(0)
    effect::set_value(d, e1, 0);
    // e1:SetReset(RESET_PHASE|PHASE_END)
    effect::set_reset(d, e1, RESET_PHASE | PHASE_END, 1);
    // Duel.RegisterEffect(e1,tp)
    duel::register_effect(d, e1, tp);
    // local e2=e1:Clone()
    let e2 = effect::clone(d, e1);
    // e2:SetCode(EFFECT_NO_EFFECT_DAMAGE)
    effect::set_code(d, e2, EFFECT_NO_EFFECT_DAMAGE);
    // e2:SetReset(RESET_PHASE|PHASE_END)
    effect::set_reset(d, e2, RESET_PHASE | PHASE_END, 1);
    // Duel.RegisterEffect(e2,tp)
    duel::register_effect(d, e2, tp);
    // local ct=Duel.GetLocationCount(tp,LOCATION_MZONE)
    let ct = duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0;
    // if ct<1 then return end
    if ct < 1 {
        return;
    }
    // local g=Duel.GetTargetCards(e)
    let mut g = duel::get_target_cards(d, a.e).unwrap_or_default();
    // if #g==0 then return end
    if g.is_empty() {
        return;
    }
    // if #g>ct or (#g>1 and Duel.IsPlayerAffectedByEffect(tp,CARD_BLUEEYES_SPIRIT)) then
    if g.len() as i32 > ct || (g.len() > 1 && duel::is_player_affected_by_effect(d, tp, CARD_BLUEEYES_SPIRIT).is_some()) {
        // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_SPSUMMON)
        duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_SPSUMMON as u64);
        // g=g:Select(tp,1,1,nil)
        g = group::select(d, &g, tp, 1, 1, false, NO_EX).nil();
    }
    // Duel.SpecialSummon(g,0,tp,tp,false,false,POS_FACEUP)
    duel::special_summon(d, g, 0, tp, tp, false, false, POS_FACEUP as u8, None);
}
