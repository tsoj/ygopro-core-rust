//! Twilight Eraser
use super::prelude::*;

pub const ID: u32 = 57348141;

pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_LIGHTSWORN as u32], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // activate
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetCategory(CATEGORY_REMOVE)
    effect::set_category(d, e1, CATEGORY_REMOVE);
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    // e1:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    // e1:SetProperty(EFFECT_FLAG_CARD_TARGET)
    effect::set_property(d, e1, EFFECT_FLAG_CARD_TARGET, 0);
    // e1:SetHintTiming(0,TIMINGS_CHECK_MONSTER_E)
    effect::set_hint_timing(d, e1, 0, Some(TIMINGS_CHECK_MONSTER_E));
    // e1:SetCondition(s.condition)
    effect::set_condition(d, e1, Some(condition(condition_fn)));
    // e1:SetCost(s.cost)
    effect::set_cost(d, e1, Some(cost_target(cost)));
    // e1:SetTarget(s.target)
    effect::set_target(d, e1, Some(cost_target(target)));
    // e1:SetOperation(s.activate)
    effect::set_operation(d, e1, Some(operation(activate)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // to hand
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetCategory(CATEGORY_SPECIAL_SUMMON)
    effect::set_category(d, e2, CATEGORY_SPECIAL_SUMMON);
    // e2:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e2, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_O);
    // e2:SetCode(EVENT_TO_GRAVE)
    effect::set_code(d, e2, EVENT_TO_GRAVE);
    // e2:SetProperty(EFFECT_FLAG_DELAY)
    effect::set_property(d, e2, EFFECT_FLAG_DELAY, 0);
    // e2:SetCondition(s.spcon)
    effect::set_condition(d, e2, Some(condition(spcon)));
    // e2:SetTarget(s.sptg)
    effect::set_target(d, e2, Some(cost_target(sptg)));
    // e2:SetOperation(s.spop)
    effect::set_operation(d, e2, Some(operation(spop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
}
fn filter1(d: &mut Duel, c: CardId, tp: u8) -> bool {
    // return c:IsFaceup() and c:IsSetCard(SET_LIGHTSWORN)
    //   and Duel.IsExistingMatchingCard(s.filter2,tp,LOCATION_MZONE,0,1,c,c:GetRace(),c:GetCode())
    let race = card::get_race(d, c);
    let code = card::get_code(d, c)[0];
    card::is_faceup(d, c)
        && card::is_set_card(d, c, &[SET_LIGHTSWORN as u16])
        && duel::is_existing_matching_card(d, Some(&|d, x| filter2(d, x, race, code)), tp, LOCATION_MZONE, 0, 1, c.into())
}
fn filter2(d: &mut Duel, c: CardId, race: u64, code: u32) -> bool {
    // return c:IsFaceup() and c:IsSetCard(SET_LIGHTSWORN) and c:IsRace(race) and not c:IsCode(code)
    card::is_faceup(d, c)
        && card::is_set_card(d, c, &[SET_LIGHTSWORN as u16])
        && card::is_race(d, c, race)
        && !card::is_code(d, c, &[code])
}
fn condition_fn(d: &mut Duel, a: &EffArgs) -> bool {
    // return Duel.IsExistingMatchingCard(s.filter1,tp,LOCATION_MZONE,0,1,nil,tp)
    duel::is_existing_matching_card(d, Some(&|d, x| filter1(d, x, a.tp)), a.tp, LOCATION_MZONE, 0, 1, NO_EX)
}
fn cfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsMonster() and c:IsSetCard(SET_LIGHTSWORN) and c:IsAbleToRemoveAsCost() and aux.SpElimFilter(c,true)
    card::is_monster(d, c)
        && card::is_set_card(d, c, &[SET_LIGHTSWORN as u16])
        && card::is_able_to_remove_as_cost(d, c, None)
        && aux::sp_elim_filter(d, c, true, false)
}
fn rescon(d: &mut Duel, sg: &CardSet, e: Option<EffectId>, tp: u8, _mg: &CardSet, _c: Option<CardId>) -> (bool, bool) {
    // sg:AddCard(e:GetHandler())
    let h = effect::get_handler(d, e.nil()).nil();
    let mut sg = sg.clone();
    group::add_card(&mut sg, &CardSet::from_card(h));
    // local res=Duel.IsExistingTarget(Card.IsAbleToRemove,tp,LOCATION_ONFIELD,LOCATION_ONFIELD,2,sg)
    let res = duel::is_existing_target(d, Some(&|d, x| card::is_able_to_remove(d, x, None, None, None)), tp, LOCATION_ONFIELD, LOCATION_ONFIELD, 2, (&sg).into());
    // return res
    (res, false)
}
fn cost(d: &mut Duel, a: &EffArgs) -> bool {
    let (e, tp) = (a.e, a.tp);
    // e:SetLabel(1)
    effect::set_label(d, e, &[1]);
    // local cg=Duel.GetMatchingGroup(s.cfilter,tp,LOCATION_MZONE|LOCATION_GRAVE,0,nil)
    let cg = duel::get_matching_group(d, Some(&|d, x| cfilter(d, x)), tp, LOCATION_MZONE | LOCATION_GRAVE, 0, NO_EX);
    // if chk==0 then return aux.SelectUnselectGroup(cg,e,tp,2,2,s.rescon,0) end
    if a.chk == Some(0) {
        return matches!(
            aux::select_unselect_group(d, &cg, Some(e), tp, Some(2), Some(2), Some(&rescon), 0, tp, None, None, None, false),
            Err(true)
        );
    }
    // local rg=aux.SelectUnselectGroup(cg,e,tp,2,2,s.rescon,1,tp,HINTMSG_REMOVE)
    let rg = aux::select_unselect_group(d, &cg, Some(e), tp, Some(2), Some(2), Some(&rescon), 1, tp, Some(HINTMSG_REMOVE as u64), None, None, false).unwrap_or_default();
    // Duel.Remove(rg,POS_FACEUP,REASON_COST)
    duel::remove(d, rg, POS_FACEUP as u8, REASON_COST, None, None);
    true
}
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    let (e, tp) = (a.e, a.tp);
    // if chkc then return chkc:IsOnField() and chkc~=e:GetHandler() and chkc:IsAbleToRemove() end
    if let Some(chkc) = a.chkc() {
        let h = effect::get_handler(d, e).nil();
        return card::is_on_field(d, chkc) && chkc != h && card::is_able_to_remove(d, chkc, None, None, None);
    }
    // if chk==0 then
    if a.chk == Some(0) {
        // local label=e:GetLabel()
        let label = effect::get_label(d, e);
        // e:SetLabel(0)
        effect::set_label(d, e, &[0]);
        // return label==1 or Duel.IsExistingTarget(Card.IsAbleToRemove,tp,LOCATION_ONFIELD,LOCATION_ONFIELD,2,e:GetHandler()) end
        let h = effect::get_handler(d, e).nil();
        return label == 1
            || duel::is_existing_target(d, Some(&|d, x| card::is_able_to_remove(d, x, None, None, None)), tp, LOCATION_ONFIELD, LOCATION_ONFIELD, 2, h.into());
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_REMOVE)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_REMOVE as u64);
    // local g=Duel.SelectTarget(tp,Card.IsAbleToRemove,tp,LOCATION_ONFIELD,LOCATION_ONFIELD,2,2,e:GetHandler())
    let h = effect::get_handler(d, e).nil();
    let g = duel::select_target(d, tp, Some(&|d, x| card::is_able_to_remove(d, x, None, None, None)), tp, LOCATION_ONFIELD, LOCATION_ONFIELD, 2, 2, false, h.into()).nil();
    // Duel.SetOperationInfo(0,CATEGORY_REMOVE,g,#g,0,0)
    let n = g.len() as u8;
    duel::set_operation_info(d, 0, CATEGORY_REMOVE, Some(g), n, 0, 0);
    true
}
fn activate(d: &mut Duel, a: &EffArgs) {
    // local g=Duel.GetTargetCards(e)
    let g = duel::get_target_cards(d, a.e).nil();
    // if #g>0 then
    if !g.is_empty() {
        // Duel.Remove(g,POS_FACEUP,REASON_EFFECT)
        duel::remove(d, g, POS_FACEUP as u8, REASON_EFFECT, None, None);
    }
}
fn spcon(d: &mut Duel, a: &EffArgs) -> bool {
    // return e:GetHandler():IsPreviousLocation(LOCATION_DECK) and re:IsMonsterEffect()
    //   and re:GetHandler():IsSetCard(SET_LIGHTSWORN) and r&REASON_EFFECT~=0
    let h = effect::get_handler(d, a.e).nil();
    card::is_previous_location(d, h, LOCATION_DECK)
        && effect::is_monster_effect(d, a.re.nil())
        && card::is_set_card(d, effect::get_handler(d, a.re.nil()).nil(), &[SET_LIGHTSWORN as u16])
        && (a.r & REASON_EFFECT) != 0
}
fn spfilter(d: &mut Duel, c: CardId, e: EffectId, tp: u8) -> bool {
    // return c:IsSetCard(SET_LIGHTSWORN) and c:IsCanBeSpecialSummoned(e,0,tp,false,false)
    card::is_set_card(d, c, &[SET_LIGHTSWORN as u16]) && card::is_can_be_special_summoned(d, c, e, 0, tp, false, false, None, None, None)
}
fn sptg(d: &mut Duel, a: &EffArgs) -> bool {
    let (e, tp) = (a.e, a.tp);
    // if chk==0 then return Duel.GetLocationCount(tp,LOCATION_MZONE)>0
    //   and Duel.IsExistingMatchingCard(s.spfilter,tp,LOCATION_HAND,0,1,nil,e,tp) end
    if a.chk == Some(0) {
        return duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 > 0
            && duel::is_existing_matching_card(d, Some(&|d, x| spfilter(d, x, e, tp)), tp, LOCATION_HAND, 0, 1, NO_EX);
    }
    // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,nil,1,tp,LOCATION_HAND)
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, None, 1, tp, LOCATION_HAND as i32);
    true
}
fn spop(d: &mut Duel, a: &EffArgs) {
    let (e, tp) = (a.e, a.tp);
    // if Duel.GetLocationCount(tp,LOCATION_MZONE)<1 then return end
    if duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 < 1 {
        return;
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_SPSUMMON)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_SPSUMMON as u64);
    // local g=Duel.SelectMatchingCard(tp,s.spfilter,tp,LOCATION_HAND,0,1,1,nil,e,tp)
    let g = duel::select_matching_card(d, tp, Some(&|d, x| spfilter(d, x, e, tp)), tp, LOCATION_HAND, 0, 1, 1, false, NO_EX).nil();
    // if #g>0 then
    if !g.is_empty() {
        // Duel.SpecialSummon(g,0,tp,tp,false,false,POS_FACEUP)
        duel::special_summon(d, g, 0, tp, tp, false, false, POS_FACEUP as u8, None);
    }
}
