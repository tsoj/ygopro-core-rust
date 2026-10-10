//! Majesty of the White Dragons
//! scripted by pyrQ
use super::prelude::*;

pub const ID: u32 = 43219114;

pub const INFO: ScriptInfo = ScriptInfo { listed_names: &[CARD_BLUEEYES_W_DRAGON], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Show up to 3 "Blue-Eyes White Dragon(s)" from your hand, face-up Monster Zone, and/or GY, then destroy that many cards your opponent controls
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_DESTROY)
    effect::set_category(d, e1, CATEGORY_DESTROY);
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    // e1:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    // e1:SetCountLimit(1,id)
    effect::set_count_limit(d, e1, 1, ID, 0, 0);
    // e1:SetHintTiming(0,TIMING_STANDBY_PHASE|TIMING_MAIN_END|TIMINGS_CHECK_MONSTER_E)
    effect::set_hint_timing(d, e1, 0, Some(TIMING_STANDBY_PHASE | TIMING_MAIN_END | TIMINGS_CHECK_MONSTER_E));
    // e1:SetTarget(s.target)
    effect::set_target(d, e1, Some(cost_target(target)));
    // e1:SetOperation(s.activate)
    effect::set_operation(d, e1, Some(operation(activate)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // Ritual Summon 1 Ritual Monster from your hand, by Tributing "Blue-Eyes White Dragon(s)" from your hand or field
    // local e2=Ritual.CreateProc({handler=c,lvtype=RITPROC_EQUAL,matfilter=function(c) return c:IsCode(CARD_BLUEEYES_W_DRAGON) end,desc=aux.Stringid(id,1)})
    let e2 = ritual::create_proc(
        d,
        c,
        ritual::RitualProc {
            lvtype: RITPROC_EQUAL,
            matfilter: Some(ritual::MatFilt::F(std::sync::Arc::new(|d, c, _e, _tp| card::is_code(d, c, &[CARD_BLUEEYES_W_DRAGON])))),
            desc: Some(aux::stringid(ID, 1)),
            ..Default::default()
        },
    );
    // e2:SetType(EFFECT_TYPE_QUICK_O)
    effect::set_type(d, e2, EFFECT_TYPE_QUICK_O);
    // e2:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e2, EVENT_FREE_CHAIN);
    // e2:SetRange(LOCATION_GRAVE)
    effect::set_range(d, e2, LOCATION_GRAVE);
    // e2:SetCountLimit(1,{id,1})
    effect::set_count_limit(d, e2, 1, ID, 1, 0);
    // e2:SetHintTiming(0,TIMING_STANDBY_PHASE|TIMING_MAIN_END|TIMINGS_CHECK_MONSTER_E)
    effect::set_hint_timing(d, e2, 0, Some(TIMING_STANDBY_PHASE | TIMING_MAIN_END | TIMINGS_CHECK_MONSTER_E));
    // e2:SetCost(Cost.SelfBanish)
    effect::set_cost(d, e2, Some(cost::self_banish()));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
}
/// `s.showfilter(c)`: `return c:IsCode(CARD_BLUEEYES_W_DRAGON) and (c:IsFaceup() or not c:IsLocation(LOCATION_MZONE))`
fn showfilter(d: &mut Duel, c: CardId) -> bool {
    card::is_code(d, c, &[CARD_BLUEEYES_W_DRAGON]) && (card::is_faceup(d, c) || !card::is_location(d, c, LOCATION_MZONE))
}
/// `s.target(e,tp,eg,ep,ev,re,r,rp,chk)`
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return Duel.IsExistingMatchingCard(s.showfilter,tp,LOCATION_HAND|LOCATION_MZONE|LOCATION_GRAVE,0,1,nil)
    //   and Duel.GetFieldGroupCount(tp,0,LOCATION_ONFIELD)>0 end
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, Some(&showfilter), a.tp, LOCATION_HAND | LOCATION_MZONE | LOCATION_GRAVE, 0, 1, NO_EX)
            && duel::get_field_group_count(d, a.tp, 0, LOCATION_ONFIELD) > 0;
    }
    // local g=Duel.GetMatchingGroup(nil,tp,0,LOCATION_ONFIELD,nil)
    let g = duel::get_matching_group(d, None, a.tp, 0, LOCATION_ONFIELD, NO_EX);
    // Duel.SetOperationInfo(0,CATEGORY_DESTROY,g,1,tp,0)
    duel::set_operation_info(d, 0, CATEGORY_DESTROY, Some(g), 1, a.tp, 0);
    true
}
/// `s.rescon(sg,e,tp,mg)`: `return Duel.GetFieldGroupCount(tp,0,LOCATION_ONFIELD)>=#sg`
fn rescon(d: &mut Duel, sg: &CardSet, _e: Option<EffectId>, tp: u8, _mg: &CardSet, _c: Option<CardId>) -> (bool, bool) {
    (duel::get_field_group_count(d, tp, 0, LOCATION_ONFIELD) >= sg.len() as i32, false)
}
/// `s.activate(e,tp,eg,ep,ev,re,r,rp)`
fn activate(d: &mut Duel, a: &EffArgs) {
    // if Duel.GetFieldGroupCount(tp,0,LOCATION_ONFIELD)==0 then return end
    if duel::get_field_group_count(d, a.tp, 0, LOCATION_ONFIELD) == 0 {
        return;
    }
    // local showg=Duel.GetMatchingGroup(s.showfilter,tp,LOCATION_HAND|LOCATION_MZONE|LOCATION_GRAVE,0,nil)
    let showg = duel::get_matching_group(d, Some(&showfilter), a.tp, LOCATION_HAND | LOCATION_MZONE | LOCATION_GRAVE, 0, NO_EX);
    // if #showg==0 then return end
    if showg.is_empty() {
        return;
    }
    // local g=aux.SelectUnselectGroup(showg,e,tp,1,3,s.rescon,1,tp,HINTMSG_CONFIRM)
    let g = aux::select_unselect_group(d, &showg, Some(a.e), a.tp, Some(1), Some(3), Some(&rescon), 1, a.tp, Some(HINTMSG_CONFIRM as u64), None, None, false).unwrap_or_default();
    // local ct=#g
    let ct = g.len();
    // if ct==0 then return end
    if ct == 0 {
        return;
    }
    // local confirmg,hintg=g:Split(Card.IsLocation,nil,LOCATION_HAND)
    let (confirmg, hintg) = group::split(d, &g, &|d, c| card::is_location(d, c, LOCATION_HAND), NO_EX);
    // if #confirmg>0 then Duel.ConfirmCards(1-tp,confirmg) Duel.ShuffleHand(tp) end
    if !confirmg.is_empty() {
        duel::confirm_cards(d, 1 - a.tp, confirmg);
        duel::shuffle_hand(d, a.tp);
    }
    // if #hintg>0 then Duel.HintSelection(hintg) end
    if !hintg.is_empty() {
        duel::hint_selection(d, hintg, true);
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_DESTROY)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_DESTROY as u64);
    // local desg=Duel.SelectMatchingCard(tp,nil,tp,0,LOCATION_ONFIELD,ct,ct,nil)
    let desg = duel::select_matching_card(d, a.tp, None, a.tp, 0, LOCATION_ONFIELD, ct as u16, ct as u16, false, NO_EX).nil();
    // if #desg>0 then
    if !desg.is_empty() {
        // Duel.HintSelection(desg)
        duel::hint_selection(d, desg.clone(), true);
        // Duel.BreakEffect()
        duel::break_effect(d);
        // Duel.Destroy(desg,REASON_EFFECT)
        duel::destroy(d, desg, REASON_EFFECT, None, None);
    }
}
