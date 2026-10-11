//! Abyss Actors' Curtain Call
//! 魔界劇団のカーテンコール
use super::prelude::*;

pub const ID: u32 = 4682617;

pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_ABYSS_SCRIPT, SET_ABYSS_ACTOR], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // activate
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetCategory(CATEGORY_TOHAND+CATEGORY_SPECIAL_SUMMON)
    effect::set_category(d, e1, CATEGORY_TOHAND + CATEGORY_SPECIAL_SUMMON);
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    // e1:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    // e1:SetCountLimit(1,id,EFFECT_COUNT_CODE_OATH)
    effect::set_count_limit(d, e1, 1, ID, 0, EFFECT_COUNT_CODE_OATH as u8);
    // e1:SetCondition(s.condition)
    effect::set_condition(d, e1, Some(condition(cond)));
    // e1:SetTarget(s.target)
    effect::set_target(d, e1, Some(cost_target(target)));
    // e1:SetOperation(s.activate)
    effect::set_operation(d, e1, Some(operation(activate)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // activity check
    // Duel.AddCustomActivityCounter(id,ACTIVITY_CHAIN,s.chainfilter)
    duel::add_custom_activity_counter(d, ID, ACTIVITY_CHAIN as u32, lua_fn(chainfilter));
}
/// `s.chainfilter(re,tp,cid)`
fn chainfilter(d: &mut Duel, a: Args) -> bool {
    // return not (re:IsSpellEffect() and re:GetHandler():IsSetCard(SET_ABYSS_SCRIPT))
    let re = a.e(0);
    if effect::is_spell_effect(d, re) {
        let h = effect::get_handler(d, re).nil();
        if card::is_set_card(d, h, &[SET_ABYSS_SCRIPT as u16]) {
            return false;
        }
    }
    true
}
/// `s.thfilter(c)`
fn thfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsFaceup() and c:IsSetCard(SET_ABYSS_ACTOR) and c:IsType(TYPE_PENDULUM) and c:IsAbleToHand()
    card::is_faceup(d, c)
        && card::is_set_card(d, c, &[SET_ABYSS_ACTOR as u16])
        && card::is_type(d, c, TYPE_PENDULUM)
        && card::is_able_to_hand(d, c, None)
}
/// `s.spfilter(c,e,tp)`
fn spfilter(d: &mut Duel, c: CardId, e: EffectId, tp: u8) -> bool {
    // return c:IsSetCard(SET_ABYSS_ACTOR) and c:IsType(TYPE_PENDULUM) and c:IsCanBeSpecialSummoned(e,0,tp,false,false)
    card::is_set_card(d, c, &[SET_ABYSS_ACTOR as u16])
        && card::is_type(d, c, TYPE_PENDULUM)
        && card::is_can_be_special_summoned(d, c, e, 0, tp, false, false, None, None, None)
}
/// `s.ctfilter(c)`
fn ctfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsSpell() and c:IsSetCard(SET_ABYSS_SCRIPT)
    card::is_spell(d, c) && card::is_set_card(d, c, &[SET_ABYSS_SCRIPT as u16])
}
/// `s.condition(e,tp,eg,ep,ev,re,r,rp)`
fn cond(d: &mut Duel, a: &EffArgs) -> bool {
    // return Duel.GetCustomActivityCount(id,tp,ACTIVITY_CHAIN)>0
    duel::get_custom_activity_count(d, ID, a.tp, ACTIVITY_CHAIN as u32) > 0
}
/// `s.target(e,tp,eg,ep,ev,re,r,rp,chk)`
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // if chk==0 then return Duel.IsExistingMatchingCard(s.ctfilter,tp,LOCATION_GRAVE,0,1,nil) and
    //     Duel.IsExistingMatchingCard(s.thfilter,tp,LOCATION_EXTRA,0,1,nil) end
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, Some(&ctfilter), tp, LOCATION_GRAVE, 0, 1, NO_EX)
            && duel::is_existing_matching_card(d, Some(&thfilter), tp, LOCATION_EXTRA, 0, 1, NO_EX);
    }
    // Duel.SetOperationInfo(0,CATEGORY_TOHAND,nil,1,tp,LOCATION_EXTRA)
    duel::set_operation_info(d, 0, CATEGORY_TOHAND, None, 1, tp, LOCATION_EXTRA as i32);
    // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,nil,1,tp,LOCATION_HAND)
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, None, 1, tp, LOCATION_HAND as i32);
    true
}
/// `s.splimit(e,c)`
fn splimit(d: &mut Duel, _e: EffectId, c: CardId) -> bool {
    // return not (c:IsSetCard(SET_ABYSS_ACTOR) and c:IsType(TYPE_PENDULUM))
    !(card::is_set_card(d, c, &[SET_ABYSS_ACTOR as u16]) && card::is_type(d, c, TYPE_PENDULUM))
}
/// `s.activate(e,tp,eg,ep,ev,re,r,rp)`
fn activate(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // local e1=Effect.CreateEffect(e:GetHandler())
    let h = effect::get_handler(d, a.e).nil();
    let e1 = effect::create_effect(d, h);
    // e1:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e1, EFFECT_TYPE_FIELD);
    // e1:SetCode(EFFECT_CANNOT_SPECIAL_SUMMON)
    effect::set_code(d, e1, EFFECT_CANNOT_SPECIAL_SUMMON);
    // e1:SetProperty(EFFECT_FLAG_PLAYER_TARGET+EFFECT_FLAG_CLIENT_HINT)
    effect::set_property(d, e1, EFFECT_FLAG_PLAYER_TARGET + EFFECT_FLAG_CLIENT_HINT, 0);
    // e1:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e1, aux::stringid(ID, 1));
    // e1:SetTargetRange(1,0)
    effect::set_target_range(d, e1, 1, 0);
    // e1:SetTarget(s.splimit)
    effect::set_target(d, e1, Some(effect_card_fn(splimit)));
    // e1:SetReset(RESET_PHASE|PHASE_END)
    effect::set_reset(d, e1, RESET_PHASE | PHASE_END, 1);
    // Duel.RegisterEffect(e1,tp)
    duel::register_effect(d, e1, tp);
    // local ct=Duel.GetMatchingGroupCount(s.ctfilter,tp,LOCATION_GRAVE,0,nil)
    let ct = duel::get_matching_group_count(d, Some(&ctfilter), tp, LOCATION_GRAVE, 0, NO_EX);
    // if ct<1 then return end
    if ct < 1 {
        return;
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_ATOHAND)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_ATOHAND as u64);
    // local hg=Duel.SelectMatchingCard(tp,s.thfilter,tp,LOCATION_EXTRA,0,1,ct,nil)
    let hg = duel::select_matching_card(d, tp, Some(&thfilter), tp, LOCATION_EXTRA, 0, 1, ct as u16, false, NO_EX).nil();
    // if #hg>0 and Duel.SendtoHand(hg,nil,REASON_EFFECT)~=0 then
    if !hg.is_empty() && duel::send_to_hand(d, hg, None, REASON_EFFECT, None) != 0 {
        // local sct=Duel.GetOperatedGroup():FilterCount(Card.IsControler,nil,tp)
        let operated = duel::get_operated_group(d);
        let sct = group::filter_count(d, &operated, &|d, c| card::is_controler(d, c, tp), NO_EX) as i32;
        // local sg=Duel.GetMatchingGroup(s.spfilter,tp,LOCATION_HAND,0,nil,e,tp)
        let sg = duel::get_matching_group(d, Some(&|d, c| spfilter(d, c, a.e, tp)), tp, LOCATION_HAND, 0, NO_EX);
        // local ft=math.min(Duel.GetLocationCount(tp,LOCATION_MZONE),sct)
        let mut ft = duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0.min(sct);
        // if sct>0 and ft>0 and #sg>0 and Duel.SelectYesNo(tp,aux.Stringid(id,0)) then
        if sct > 0 && ft > 0 && !sg.is_empty() && duel::select_yes_no(d, tp, aux::stringid(ID, 0)) {
            // Duel.BreakEffect()
            duel::break_effect(d);
            // if Duel.IsPlayerAffectedByEffect(tp,CARD_BLUEEYES_SPIRIT) then ft=1 end
            if duel::is_player_affected_by_effect(d, tp, CARD_BLUEEYES_SPIRIT).is_some() {
                ft = 1;
            }
            // local g=aux.SelectUnselectGroup(sg,e,tp,1,ft,aux.dncheck,1,tp,HINTMSG_SPSUMMON,false)
            let g = aux::select_unselect_group(
                d,
                &sg,
                Some(a.e),
                tp,
                Some(1),
                Some(ft as usize),
                Some(&aux::dncheck()),
                1,
                tp,
                Some(HINTMSG_SPSUMMON as u64),
                None,
                None,
                false,
            )
            .unwrap_or_default();
            // Duel.SpecialSummon(g,0,tp,tp,false,false,POS_FACEUP)
            duel::special_summon(d, g, 0, tp, tp, false, false, POS_FACEUP as u8, None);
        }
    }
}
