//! Crystal Abundance
use super::prelude::*;

pub const ID: u32 = 72881007;

pub const INFO: ScriptInfo = ScriptInfo {
    listed_series: &[SET_CRYSTAL_BEAST],
    ..ScriptInfo::DEFAULT
};

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate
    let e1 = effect::create_effect(d, c);
    effect::set_category(d, e1, CATEGORY_TOGRAVE | CATEGORY_SPECIAL_SUMMON);
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    effect::set_cost(d, e1, Some(cost_target(cost_fn)));
    effect::set_target(d, e1, Some(cost_target(target)));
    effect::set_operation(d, e1, Some(operation(activate)));
    card::register_effect(d, c, e1, false);
}
// s.costfilter(c)
fn costfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsFaceup() and c:IsSetCard(SET_CRYSTAL_BEAST) and c:IsAbleToGraveAsCost() and c:GetSequence()<5
    card::is_faceup(d, c) && card::is_set_card(d, c, &[SET_CRYSTAL_BEAST as u16]) && card::is_able_to_grave_as_cost(d, c) && card::get_sequence(d, c) < 5
}
// s.rescon(sg,e,tp,mg)
fn rescon(d: &mut Duel, sg: &CardSet, e: Option<EffectId>, _tp: u8, _mg: &CardSet, _c: Option<CardId>) -> (bool, bool) {
    // return Duel.IsExistingMatchingCard(Card.IsAbleToGrave,0,LOCATION_ONFIELD,LOCATION_ONFIELD,1,sg+e:GetHandler())
    let h = effect::get_handler(d, e.nil()).nil();
    let mut ex = sg.clone();
    group::add_card(&mut ex, &CardSet::from_card(h));
    (
        duel::is_existing_matching_card(d, Some(&|d, c| card::is_able_to_grave(d, c, None)), 0, LOCATION_ONFIELD, LOCATION_ONFIELD, 1, (&ex).into()),
        false,
    )
}
// s.cost
fn cost_fn(d: &mut Duel, a: &EffArgs) -> bool {
    // e:SetLabel(1)
    effect::set_label(d, a.e, &[1]);
    // local cg=Duel.GetMatchingGroup(s.costfilter,tp,LOCATION_SZONE,0,nil)
    let cg = duel::get_matching_group(d, Some(&costfilter), a.tp, LOCATION_SZONE, 0, NO_EX);
    // if chk==0 then return aux.SelectUnselectGroup(cg,e,tp,4,4,s.rescon,0) end
    if a.chk == Some(0) {
        return matches!(
            aux::select_unselect_group(d, &cg, Some(a.e), a.tp, Some(4), Some(4), Some(&rescon), 0, a.tp, None, None, None, false),
            Err(true)
        );
    }
    // local g=aux.SelectUnselectGroup(cg,e,tp,4,4,s.rescon,1,tp,HINTMSG_TOGRAVE)
    let g = aux::select_unselect_group(d, &cg, Some(a.e), a.tp, Some(4), Some(4), Some(&rescon), 1, a.tp, Some(HINTMSG_TOGRAVE as u64), None, None, false)
        .unwrap_or_default();
    // Duel.SendtoGrave(g,REASON_COST)
    duel::send_to_grave(d, g, REASON_COST, None, None);
    true
}
// s.target
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    // local g=Duel.GetMatchingGroup(Card.IsAbleToGrave,0,LOCATION_ONFIELD,LOCATION_ONFIELD,e:GetHandler())
    let h = effect::get_handler(d, a.e).nil();
    let g = duel::get_matching_group(d, Some(&|d, c| card::is_able_to_grave(d, c, None)), 0, LOCATION_ONFIELD, LOCATION_ONFIELD, h.into());
    if a.chk == Some(0) {
        // if e:GetLabel()~=1 then return #g>0 else e:SetLabel(0) return true end
        if effect::get_label(d, a.e) != 1 {
            return !g.is_empty();
        }
        effect::set_label(d, a.e, &[0]);
        return true;
    }
    // e:SetLabel(0)
    effect::set_label(d, a.e, &[0]);
    // Duel.SetOperationInfo(0,CATEGORY_TOGRAVE,g,#g,tp,0)
    let n = g.len() as u8;
    duel::set_operation_info(d, 0, CATEGORY_TOGRAVE, Some(g), n, a.tp, 0);
    // Duel.SetPossibleOperationInfo(0,CATEGORY_SPECIAL_SUMMON,nil,1,tp,LOCATION_GRAVE)
    duel::set_possible_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, None, 1, a.tp, LOCATION_GRAVE as i32);
    true
}
// s.ctfilter(c,tp)
fn ctfilter(d: &mut Duel, c: CardId, tp: u8) -> bool {
    // return c:IsPreviousControler(1-tp) and c:IsLocation(LOCATION_GRAVE)
    card::is_previous_controler(d, c, 1 - tp) && card::is_location(d, c, LOCATION_GRAVE)
}
// s.spfilter(c,e,tp)
fn spfilter(d: &mut Duel, c: CardId, e: EffectId, tp: u8) -> bool {
    // return c:IsSetCard(SET_CRYSTAL_BEAST) and c:IsCanBeSpecialSummoned(e,0,tp,false,false)
    card::is_set_card(d, c, &[SET_CRYSTAL_BEAST as u16]) && card::is_can_be_special_summoned(d, c, e, 0, tp, false, false, None, None, None)
}
// s.activate
fn activate(d: &mut Duel, a: &EffArgs) {
    // local g=Duel.GetMatchingGroup(Card.IsAbleToGrave,0,LOCATION_ONFIELD,LOCATION_ONFIELD,e:GetHandler())
    let h = effect::get_handler(d, a.e).nil();
    let g = duel::get_matching_group(d, Some(&|d, c| card::is_able_to_grave(d, c, None)), 0, LOCATION_ONFIELD, LOCATION_ONFIELD, h.into());
    // Duel.SendtoGrave(g,REASON_EFFECT)
    duel::send_to_grave(d, g, REASON_EFFECT, None, None);
    // local ft=Duel.GetLocationCount(tp,LOCATION_MZONE)
    let mut ft = duel::get_location_count(d, a.tp, LOCATION_MZONE as u8, None, None, None).0;
    // local og=Duel.GetOperatedGroup()
    let og = duel::get_operated_group(d);
    // local ct=og:FilterCount(s.ctfilter,nil,tp)
    let ct = group::filter_count(d, &og, &|d, c| ctfilter(d, c, a.tp), NO_EX) as i32;
    // if ct==0 then return end
    if ct == 0 {
        return;
    }
    // if ft>ct then ft=ct end
    if ft > ct {
        ft = ct;
    }
    // if Duel.IsPlayerAffectedByEffect(tp,CARD_BLUEEYES_SPIRIT) then ft=1 end
    if duel::is_player_affected_by_effect(d, a.tp, CARD_BLUEEYES_SPIRIT).is_some() {
        ft = 1;
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_SPSUMMON)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_SPSUMMON as u64);
    // local sg=Duel.SelectMatchingCard(tp,aux.NecroValleyFilter(s.spfilter),tp,LOCATION_GRAVE,0,ft,ft,nil,e,tp)
    let f = aux::necro_valley_filter(|d, c| spfilter(d, c, a.e, a.tp));
    let sg = duel::select_matching_card(d, a.tp, Some(&f), a.tp, LOCATION_GRAVE, 0, ft as u16, ft as u16, false, NO_EX).nil();
    // if #sg>0 then
    if !sg.is_empty() {
        // Duel.BreakEffect()
        duel::break_effect(d);
        // Duel.SpecialSummon(sg,0,tp,tp,false,false,POS_FACEUP)
        duel::special_summon(d, sg, 0, a.tp, a.tp, false, false, POS_FACEUP as u8, None);
    }
}
