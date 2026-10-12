//! The King of Beasts that Rises from the Valley of Flames
//! Scripted by Eerie Code
use super::prelude::*;

pub const ID: u32 = 101403048;

pub const INFO: ScriptInfo = ScriptInfo { listed_names: &[41463181], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // c:EnableReviveLimit()
    card::enable_revive_limit(d, c);
    // aux.EnableCheckReincarnation(c)
    aux::enable_check_reincarnation(d, c);
    // Link Summon procedure: 3 FIRE Effect Monsters
    // Link.AddProcedure(c,s.matfilter,3,3)
    link::add_procedure(d, c, Some(mat_filter(|d, c, sc, st, tp| matfilter(d, c, sc, st, tp))), 3, Some(3), None, None, None);
    // Face-up monsters on the field become FIRE
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e1, EFFECT_TYPE_FIELD);
    // e1:SetCode(EFFECT_CHANGE_ATTRIBUTE)
    effect::set_code(d, e1, EFFECT_CHANGE_ATTRIBUTE);
    // e1:SetRange(LOCATION_MZONE)
    effect::set_range(d, e1, LOCATION_MZONE);
    // e1:SetTargetRange(LOCATION_MZONE,LOCATION_MZONE)
    effect::set_target_range(d, e1, LOCATION_MZONE, LOCATION_MZONE);
    // e1:SetValue(ATTRIBUTE_FIRE)
    effect::set_value(d, e1, ATTRIBUTE_FIRE as i64);
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // During the Main Phase, if this card was Link Summoned using "Salamangreat Heatleo" (Quick Effect): You can target 1 card your opponent controls; shuffle it into the Deck, then you can make the ATK of 1 FIRE monster your opponent controls become the ATK of 1 monster in your GY. You can only use this effect of "The King of Beasts that Rises from the Valley of Flames" once per turn
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e2, aux::stringid(ID, 0));
    // e2:SetCategory(CATEGORY_TODECK+CATEGORY_ATKCHANGE)
    effect::set_category(d, e2, CATEGORY_TODECK + CATEGORY_ATKCHANGE);
    // e2:SetType(EFFECT_TYPE_QUICK_O)
    effect::set_type(d, e2, EFFECT_TYPE_QUICK_O);
    // e2:SetProperty(EFFECT_FLAG_CARD_TARGET)
    effect::set_property(d, e2, EFFECT_FLAG_CARD_TARGET, 0);
    // e2:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e2, EVENT_FREE_CHAIN);
    // e2:SetRange(LOCATION_MZONE)
    effect::set_range(d, e2, LOCATION_MZONE);
    // e2:SetCountLimit(1,id)
    effect::set_count_limit(d, e2, 1, ID, 0, 0);
    // e2:SetCondition(s.tdcon)
    effect::set_condition(d, e2, Some(condition(tdcon)));
    // e2:SetTarget(s.tdtg)
    effect::set_target(d, e2, Some(cost_target(tdtg)));
    // e2:SetOperation(s.tdop)
    effect::set_operation(d, e2, Some(operation(tdop)));
    // e2:SetHintTiming(0,TIMING_MAIN_END|TIMINGS_CHECK_MONSTER)
    effect::set_hint_timing(d, e2, 0, Some((TIMING_MAIN_END | TIMINGS_CHECK_MONSTER) as u32));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
}
// function s.matfilter(c,scard,sumtype,tp)
fn matfilter(d: &mut Duel, c: CardId, sc: Option<CardId>, st: u64, tp: u8) -> bool {
    // return c:IsAttribute(ATTRIBUTE_FIRE,scard,sumtype,tp) and c:IsType(TYPE_EFFECT,scard,sumtype,tp)
    card::is_attribute_ctx(d, c, ATTRIBUTE_FIRE, sc, st, Some(tp)) && card::is_type_ctx(d, c, TYPE_EFFECT, sc, st, Some(tp))
}
// function s.tdcon(e,tp,eg,ep,ev,re,r,rp)
fn tdcon(d: &mut Duel, a: &EffArgs) -> bool {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // return Duel.IsMainPhase() and c:IsReincarnationSummoned() and c:IsLinkSummoned()
    duel::is_main_phase(d, None) && card::is_reincarnation_summoned(d, c) && card::is_link_summoned(d, c)
}
// function s.tdtg(e,tp,eg,ep,ev,re,r,rp,chk,chkc)
fn tdtg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chkc then return chkc:IsOnField() and chkc:IsControler(1-tp) and chkc:IsAbleToDeck() end
    if let Some(chkc) = a.chkc() {
        return card::is_on_field(d, chkc) && card::is_controler(d, chkc, 1 - a.tp) && card::is_able_to_deck(d, chkc);
    }
    // if chk==0 then return Duel.IsExistingTarget(Card.IsAbleToDeck,tp,0,LOCATION_ONFIELD,1,nil) end
    if a.chk == Some(0) {
        return duel::is_existing_target(d, Some(&card::is_able_to_deck), a.tp, 0, LOCATION_ONFIELD, 1, NO_EX);
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_TODECK)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_TODECK as u64);
    // local g=Duel.SelectTarget(tp,Card.IsAbleToDeck,tp,0,LOCATION_ONFIELD,1,1,nil)
    let g = duel::select_target(d, a.tp, Some(&card::is_able_to_deck), a.tp, 0, LOCATION_ONFIELD, 1, 1, false, NO_EX).nil();
    // Duel.SetOperationInfo(0,CATEGORY_TODECK,g,1,tp,0)
    duel::set_operation_info(d, 0, CATEGORY_TODECK, Some(g), 1, a.tp, 0);
    true
}
// function s.atkfilter(c,tp)
fn atkfilter(d: &mut Duel, c: CardId, tp: u8) -> bool {
    // return c:IsAttribute(ATTRIBUTE_FIRE) and c:IsFaceup() and Duel.IsExistingMatchingCard(s.gyfilter,tp,LOCATION_GRAVE,0,1,nil,c:GetAttack())
    if !(card::is_attribute(d, c, ATTRIBUTE_FIRE) && card::is_faceup(d, c)) {
        return false;
    }
    let atk = card::get_attack(d, c);
    duel::is_existing_matching_card(d, Some(&|d, x| gyfilter(d, x, atk)), tp, LOCATION_GRAVE, 0, 1, NO_EX)
}
// function s.gyfilter(c,atk)
fn gyfilter(d: &mut Duel, c: CardId, atk: i32) -> bool {
    // return c:IsMonster() and not c:IsAttack(atk)
    card::is_monster(d, c) && !card::is_attack(d, c, &[atk as u32])
}
// function s.tdop(e,tp,eg,ep,ev,re,r,rp)
fn tdop(d: &mut Duel, a: &EffArgs) {
    let (e, tp) = (a.e, a.tp);
    // local tc=Duel.GetFirstTarget()
    let tc = duel::first_target(d).nil();
    // if tc:IsRelateToEffect(e) and Duel.SendtoDeck(tc,nil,SEQ_DECKSHUFFLE,REASON_EFFECT)>0 and tc:IsLocation(LOCATION_DECK|LOCATION_EXTRA)
    //     and Duel.IsExistingMatchingCard(s.atkfilter,tp,0,LOCATION_MZONE,1,nil,tp)
    //     and Duel.SelectYesNo(tp,aux.Stringid(id,1)) then
    if card::is_relate_to_effect(d, tc, e)
        && duel::send_to_deck(d, CardSet::from_card(tc), None, SEQ_DECKSHUFFLE as i32, REASON_EFFECT, None) > 0
        && card::is_location(d, tc, LOCATION_DECK | LOCATION_EXTRA)
        && duel::is_existing_matching_card(d, Some(&|d, c| atkfilter(d, c, tp)), tp, 0, LOCATION_MZONE, 1, NO_EX)
        && duel::select_yes_no(d, tp, aux::stringid(ID, 1))
    {
        // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_ATKDEF)
        duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_ATKDEF as u64);
        // local oppc=Duel.SelectMatchingCard(tp,s.atkfilter,tp,0,LOCATION_MZONE,1,1,nil,tp):GetFirst()
        let g = duel::select_matching_card(d, tp, Some(&|d, c| atkfilter(d, c, tp)), tp, 0, LOCATION_MZONE, 1, 1, false, NO_EX).nil();
        let Some(oppc) = group::get_first(d, &g) else { return };
        // Duel.HintSelection(oppc)
        duel::hint_selection(d, CardSet::from_card(oppc), true);
        // Duel.Hint(HINT_SELECTMSG,tp,aux.Stringid(id,2))
        duel::hint(d, HINT_SELECTMSG as u8, tp, aux::stringid(ID, 2));
        // local gyc=Duel.SelectMatchingCard(tp,s.gyfilter,tp,LOCATION_GRAVE,0,1,1,nil,oppc:GetAttack()):GetFirst()
        let oppc_atk = card::get_attack(d, oppc);
        let g2 = duel::select_matching_card(d, tp, Some(&|d, x| gyfilter(d, x, oppc_atk)), tp, LOCATION_GRAVE, 0, 1, 1, false, NO_EX).nil();
        let Some(gyc) = group::get_first(d, &g2) else { return };
        // Duel.HintSelection(gyc)
        duel::hint_selection(d, CardSet::from_card(gyc), true);
        // Duel.BreakEffect()
        duel::break_effect(d);
        // Then you can make the ATK of 1 FIRE monster your opponent controls become the ATK of 1 monster in your GY
        // local e1=Effect.CreateEffect(e:GetHandler())
        let h = effect::get_handler(d, e).nil();
        let e1 = effect::create_effect(d, h);
        // e1:SetType(EFFECT_TYPE_SINGLE)
        effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
        // e1:SetProperty(EFFECT_FLAG_CANNOT_DISABLE)
        effect::set_property(d, e1, EFFECT_FLAG_CANNOT_DISABLE, 0);
        // e1:SetCode(EFFECT_SET_ATTACK_FINAL)
        effect::set_code(d, e1, EFFECT_SET_ATTACK_FINAL);
        // e1:SetValue(gyc:GetAttack())
        let gyc_atk = card::get_attack(d, gyc) as i64;
        effect::set_value(d, e1, gyc_atk);
        // e1:SetReset(RESET_EVENT|RESETS_STANDARD)
        effect::set_reset(d, e1, RESET_EVENT | RESETS_STANDARD, 1);
        // oppc:RegisterEffect(e1)
        card::register_effect(d, oppc, e1, false);
    }
}
