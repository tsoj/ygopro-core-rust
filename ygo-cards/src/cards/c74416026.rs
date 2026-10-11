//! Frightfur March
use super::prelude::*;

pub const ID: u32 = 74416026;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetCategory(CATEGORY_SPECIAL_SUMMON+CATEGORY_NEGATE+CATEGORY_DESTROY+CATEGORY_FUSION_SUMMON)
    effect::set_category(d, e1, CATEGORY_SPECIAL_SUMMON + CATEGORY_NEGATE + CATEGORY_DESTROY + CATEGORY_FUSION_SUMMON);
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    // e1:SetCode(EVENT_CHAINING)
    effect::set_code(d, e1, EVENT_CHAINING);
    // e1:SetCondition(s.condition)
    effect::set_condition(d, e1, Some(condition(cond)));
    // e1:SetTarget(s.target)
    effect::set_target(d, e1, Some(cost_target(target)));
    // e1:SetOperation(s.activate)
    effect::set_operation(d, e1, Some(operation(activate)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
// s.listed_series={SET_FRIGHTFUR}
pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_FRIGHTFUR], ..ScriptInfo::DEFAULT };
/// `s.filter(c,tp)`
fn filter(d: &mut Duel, c: CardId, tp: u8) -> bool {
    // return c:IsControler(tp) and c:IsLocation(LOCATION_MZONE) and c:IsSetCard(SET_FRIGHTFUR)
    card::is_controler(d, c, tp) && card::is_location(d, c, LOCATION_MZONE) && card::is_set_card(d, c, &[SET_FRIGHTFUR as u16])
}
/// `s.condition(e,tp,eg,ep,ev,re,r,rp)`
fn cond(d: &mut Duel, a: &EffArgs) -> bool {
    // if rp==tp or not re:IsHasProperty(EFFECT_FLAG_CARD_TARGET) then return false end
    let re = a.re.nil();
    if a.rp == a.tp || !effect::is_has_property(d, re, EFFECT_FLAG_CARD_TARGET, 0) {
        return false;
    }
    // local g=Duel.GetChainInfo(ev,CHAININFO_TARGET_CARDS)
    let g = duel::get_chain_info(d, a.ev as u8, &[CHAININFO_TARGET_CARDS]).first().and_then(|v| v.as_group().cloned());
    // return g and g:IsExists(s.filter,1,nil,tp)
    //   and Duel.IsChainNegatable(ev) and (re:IsMonsterEffect() or re:IsHasType(EFFECT_TYPE_ACTIVATE))
    let exists = match g {
        Some(g) => group::is_exists(d, &g, &|d, c| filter(d, c, a.tp), 1, NO_EX),
        None => false,
    };
    exists
        && duel::is_chain_negatable(d, a.ev as u8)
        && (effect::is_monster_effect(d, re) || effect::is_has_type(d, re, EFFECT_TYPE_ACTIVATE))
}
/// `s.spfilter(c,e,tp)`
fn spfilter(d: &mut Duel, c: CardId, e: EffectId, tp: u8) -> bool {
    // return c:IsType(TYPE_FUSION) and c:IsLevelAbove(8) and c:IsSetCard(SET_FRIGHTFUR)
    //   and c:IsCanBeSpecialSummoned(e,SUMMON_TYPE_FUSION,tp,false,false) and c:CheckFusionMaterial()
    card::is_type(d, c, TYPE_FUSION)
        && card::is_level_above(d, c, 8)
        && card::is_set_card(d, c, &[SET_FRIGHTFUR as u16])
        && card::is_can_be_special_summoned(d, c, e, SUMMON_TYPE_FUSION as u32, tp, false, false, Some(POS_FACEUP as u8), Some(tp), Some(0xff))
        && card::check_fusion_material(d, c, None, None, None)
}
/// `s.target(e,tp,eg,ep,ev,re,r,rp,chk)`
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return true end
    if a.chk == Some(0) {
        return true;
    }
    // Duel.SetOperationInfo(0,CATEGORY_NEGATE,eg,1,0,0)
    duel::set_operation_info(d, 0, CATEGORY_NEGATE, a.eg.clone(), 1, 0, 0);
    // if re:GetHandler():IsRelateToEffect(re) then
    let re = a.re.nil();
    let h = effect::get_handler(d, re).nil();
    if card::is_relate_to_effect(d, h, re) {
        // Duel.SetOperationInfo(0,CATEGORY_DESTROY,eg,1,0,0)
        duel::set_operation_info(d, 0, CATEGORY_DESTROY, a.eg.clone(), 1, 0, 0);
    }
    true
}
/// `s.activate(e,tp,eg,ep,ev,re,r,rp)`
fn activate(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    let re = a.re.nil();
    // local g=Duel.GetChainInfo(ev,CHAININFO_TARGET_CARDS):Filter(s.filter,nil,tp)
    let chain_cards = duel::get_chain_info(d, a.ev as u8, &[CHAININFO_TARGET_CARDS]).first().and_then(|v| v.as_group().cloned()).nil();
    let g = group::filter(d, &chain_cards, &|d, c| filter(d, c, tp), NO_EX);
    // if Duel.NegateActivation(ev) and re:GetHandler():IsRelateToEffect(re) and Duel.Destroy(eg,REASON_EFFECT)~=0 then
    if !duel::negate_activation(d, a.ev as u8) {
        return;
    }
    let h = effect::get_handler(d, re).nil();
    if !card::is_relate_to_effect(d, h, re) {
        return;
    }
    if duel::destroy(d, a.eg(), REASON_EFFECT, None, None) == 0 {
        return;
    }
    // local tg=g:Filter(Card.IsRelateToEffect,nil,re)
    let tg = group::filter(d, &g, &|d, c| card::is_relate_to_effect(d, c, re), NO_EX);
    // local sg=Duel.GetMatchingGroup(s.spfilter,tp,LOCATION_EXTRA,0,nil,e,tp)
    let sg = duel::get_matching_group(d, Some(&|d, c| spfilter(d, c, a.e, tp)), tp, LOCATION_EXTRA, 0, NO_EX);
    // if #tg>0 and #sg>0 and Duel.SelectYesNo(tp,aux.Stringid(id,0)) then
    if !tg.is_empty() && !sg.is_empty() && duel::select_yes_no(d, tp, aux::stringid(ID, 0)) {
        // Duel.BreakEffect()
        duel::break_effect(d);
        // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_TOGRAVE)
        duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_TOGRAVE as u64);
        // local tc=tg:Select(tp,1,1,nil):GetFirst()
        let tc_sel = group::select(d, &tg, tp, 1, 1, false, NO_EX).nil();
        let tc = group::get_first(d, &tc_sel).nil();
        // if Duel.SendtoGrave(tc,REASON_EFFECT)==0 or not tc:IsLocation(LOCATION_GRAVE) then return end
        if duel::send_to_grave(d, tc, REASON_EFFECT, None, None) == 0 || !card::is_location(d, tc, LOCATION_GRAVE) {
            return;
        }
        // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_SPSUMMON)
        duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_SPSUMMON as u64);
        // local sc=sg:Select(tp,1,1,nil):GetFirst()
        let sc_sel = group::select(d, &sg, tp, 1, 1, false, NO_EX).nil();
        let sc = group::get_first(d, &sc_sel).nil();
        // if Duel.SpecialSummonStep(sc,SUMMON_TYPE_FUSION,tp,tp,false,false,POS_FACEUP) then
        if duel::special_summon_step(d, sc, SUMMON_TYPE_FUSION as u32, tp, tp, false, false, POS_FACEUP as u8, None) {
            // local e1=Effect.CreateEffect(e:GetHandler())
            let eh = effect::get_handler(d, a.e).nil();
            let e1 = effect::create_effect(d, eh);
            // e1:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_CONTINUOUS)
            effect::set_type(d, e1, EFFECT_TYPE_FIELD + EFFECT_TYPE_CONTINUOUS);
            // e1:SetCode(EVENT_PHASE+PHASE_END)
            effect::set_code(d, e1, EVENT_PHASE + PHASE_END);
            // e1:SetCountLimit(1)
            effect::set_count_limit(d, e1, 1, 0, 0, 0);
            // e1:SetProperty(EFFECT_FLAG_IGNORE_IMMUNE)
            effect::set_property(d, e1, EFFECT_FLAG_IGNORE_IMMUNE, 0);
            // e1:SetLabelObject(sc)
            effect::set_label_object(d, e1, Value::Card(sc));
            // e1:SetCondition(s.rmcon)
            effect::set_condition(d, e1, Some(condition(rmcon)));
            // e1:SetOperation(s.rmop)
            effect::set_operation(d, e1, Some(operation(rmop)));
            // if Duel.IsTurnPlayer(tp) and Duel.IsPhase(PHASE_END) then
            if duel::is_turn_player(d, tp) && duel::is_phase(d, PHASE_END) {
                // e1:SetLabel(Duel.GetTurnCount())
                let turn = duel::get_turn_count(d, None) as i64;
                effect::set_label(d, e1, &[turn]);
                // e1:SetReset(RESET_PHASE|PHASE_END|RESET_SELF_TURN,2)
                effect::set_reset(d, e1, RESET_PHASE | PHASE_END | RESET_SELF_TURN, 2);
            } else {
                // e:SetLabel(0)
                effect::set_label(d, a.e, &[0]);
                // e1:SetReset(RESET_PHASE|PHASE_END|RESET_SELF_TURN)
                effect::set_reset(d, e1, RESET_PHASE | PHASE_END | RESET_SELF_TURN, 1);
            }
            // Duel.RegisterEffect(e1,tp)
            duel::register_effect(d, e1, tp);
            // Duel.SpecialSummonComplete()
            duel::special_summon_complete(d);
            // sc:CompleteProcedure()
            card::complete_procedure(d, sc);
            // sc:RegisterFlagEffect(id,RESET_EVENT|RESETS_STANDARD,0,1)
            card::register_flag_effect(d, sc, ID, RESET_EVENT | RESETS_STANDARD, 0, 1, 0, 0);
        }
    }
}
/// `s.rmcon(e,tp,eg,ep,ev,re,r,rp)`
fn rmcon(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // local tc=e:GetLabelObject()
    let tc = effect::get_label_object(d, a.e).as_card();
    // return Duel.IsTurnPlayer(tp) and Duel.GetTurnCount()~=e:GetLabel() and tc:GetFlagEffect(id)~=0
    duel::is_turn_player(d, tp)
        && duel::get_turn_count(d, None) as i64 != effect::get_label(d, a.e)
        && card::get_flag_effect(d, tc.nil(), ID) != 0
}
/// `s.rmop(e,tp,eg,ep,ev,re,r,rp)`
fn rmop(d: &mut Duel, a: &EffArgs) {
    // local tc=e:GetLabelObject()
    let tc = effect::get_label_object(d, a.e).as_card().nil();
    // Duel.Remove(tc,POS_FACEUP,REASON_EFFECT)
    duel::remove(d, tc, POS_FACEUP as u8, REASON_EFFECT, None, None);
}
