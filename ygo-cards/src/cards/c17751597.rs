//! Branded Retribution
//! scripted by pyrQ
use super::prelude::*;

pub const ID: u32 = 17751597;

pub const INFO: ScriptInfo = ScriptInfo { listed_names: &[CARD_ALBAZ, ID], listed_series: &[SET_BRANDED], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_TOEXTRA+CATEGORY_NEGATE+CATEGORY_DESTROY)
    effect::set_category(d, e1, CATEGORY_TOEXTRA + CATEGORY_NEGATE + CATEGORY_DESTROY);
    // e1:SetProperty(EFFECT_FLAG_DAMAGE_STEP+EFFECT_FLAG_DAMAGE_CAL)
    effect::set_property(d, e1, EFFECT_FLAG_DAMAGE_STEP + EFFECT_FLAG_DAMAGE_CAL, 0);
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    // e1:SetCode(EVENT_CHAINING)
    effect::set_code(d, e1, EVENT_CHAINING);
    // e1:SetCountLimit(1,id)
    effect::set_count_limit(d, e1, 1, ID, 0, 0);
    // e1:SetCondition(s.condition)
    effect::set_condition(d, e1, Some(condition(cond)));
    // e1:SetTarget(s.target)
    effect::set_target(d, e1, Some(cost_target(target)));
    // e1:SetOperation(s.activate)
    effect::set_operation(d, e1, Some(operation(activate)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // Return 1 "Branded" Spell/Trap to your hand
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e2, aux::stringid(ID, 1));
    // e2:SetCategory(CATEGORY_TOHAND)
    effect::set_category(d, e2, CATEGORY_TOHAND);
    // e2:SetProperty(EFFECT_FLAG_CARD_TARGET)
    effect::set_property(d, e2, EFFECT_FLAG_CARD_TARGET, 0);
    // e2:SetType(EFFECT_TYPE_QUICK_O)
    effect::set_type(d, e2, EFFECT_TYPE_QUICK_O);
    // e2:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e2, EVENT_FREE_CHAIN);
    // e2:SetRange(LOCATION_GRAVE)
    effect::set_range(d, e2, LOCATION_GRAVE);
    // e2:SetHintTiming(0,TIMING_END_PHASE)
    effect::set_hint_timing(d, e2, 0, Some(TIMING_END_PHASE));
    // e2:SetCountLimit(1,id)
    effect::set_count_limit(d, e2, 1, ID, 0, 0);
    // e2:SetCost(Cost.SelfBanish)
    effect::set_cost(d, e2, Some(cost::self_banish()));
    // e2:SetTarget(s.thtg)
    effect::set_target(d, e2, Some(cost_target(thtg)));
    // e2:SetOperation(s.thop)
    effect::set_operation(d, e2, Some(operation(thop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
}
/// `s.condition(e,tp,eg,ep,ev,re,r,rp)`
fn cond(d: &mut Duel, a: &EffArgs) -> bool {
    // if not Duel.IsChainNegatable(ev) then return false end
    if !duel::is_chain_negatable(d, a.ev as u8) {
        return false;
    }
    // if not re or (not re:IsMonsterEffect() and not re:IsHasType(EFFECT_TYPE_ACTIVATE)) then return false end
    let Some(re) = a.re else { return false };
    if !effect::is_monster_effect(d, re) && !effect::is_has_type(d, re, EFFECT_TYPE_ACTIVATE) {
        return false;
    }
    // return re:IsHasCategory(CATEGORY_SPECIAL_SUMMON)
    effect::is_has_category(d, re, CATEGORY_SPECIAL_SUMMON)
}
/// `s.texfilter(c)`
fn texfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsFaceup() and c:IsType(TYPE_FUSION) and c:IsMonster() and c:IsAbleToExtra() and c:ListsCodeAsMaterial(CARD_ALBAZ)
    card::is_faceup(d, c)
        && card::is_type(d, c, TYPE_FUSION)
        && card::is_monster(d, c)
        && card::is_able_to_extra(d, c)
        && card::lists_code_as_material(d, c, &[CARD_ALBAZ])
}
/// `s.rescon(sg,e,tp,mg)`
fn rescon(d: &mut Duel, sg: &CardSet, _e: Option<EffectId>, _tp: u8, _mg: &CardSet, _c: Option<CardId>) -> (bool, bool) {
    // return (#sg==1 and sg:IsExists(Card.IsLocation,1,nil,LOCATION_MZONE))
    //   or (#sg==2 and sg:IsExists(Card.IsLocation,2,nil,LOCATION_GRAVE))
    let res = (sg.len() == 1 && group::is_exists(d, sg, &|d, c| card::is_location(d, c, LOCATION_MZONE), 1, NO_EX))
        || (sg.len() == 2 && group::is_exists(d, sg, &|d, c| card::is_location(d, c, LOCATION_GRAVE), 2, NO_EX));
    (res, false)
}
/// `s.target(e,tp,eg,ep,ev,re,r,rp,chk)`
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    // local rg=Duel.GetMatchingGroup(s.texfilter,tp,LOCATION_MZONE|LOCATION_GRAVE,0,nil)
    let rg = duel::get_matching_group(d, Some(&texfilter), a.tp, LOCATION_MZONE | LOCATION_GRAVE, 0, NO_EX);
    // if chk==0 then return #rg>0 and aux.SelectUnselectGroup(rg,e,tp,1,2,s.rescon,0) end
    if a.chk == Some(0) {
        let rs: aux::Rescon = &rescon;
        return !rg.is_empty()
            && matches!(aux::select_unselect_group(d, &rg, Some(a.e), a.tp, Some(1), Some(2), Some(rs), 0, a.tp, None, None, None, false), Err(true));
    }
    // Duel.SetOperationInfo(0,CATEGORY_NEGATE,eg,1,0,0)
    duel::set_operation_info(d, 0, CATEGORY_NEGATE, a.eg.clone(), 1, 0, 0);
    // if re:GetHandler():IsDestructable() and re:GetHandler():IsRelateToEffect(re) then
    let rh = effect::get_handler(d, a.re.nil()).nil();
    if card::is_destructable(d, rh, None) && card::is_relate_to_effect(d, rh, a.re.nil()) {
        // Duel.SetOperationInfo(0,CATEGORY_DESTROY,eg,1,0,0)
        duel::set_operation_info(d, 0, CATEGORY_DESTROY, a.eg.clone(), 1, 0, 0);
    }
    // Duel.SetOperationInfo(0,CATEGORY_TOEXTRA,rg,1,0,0)
    duel::set_operation_info(d, 0, CATEGORY_TOEXTRA, Some(rg), 1, 0, 0);
    true
}
/// `s.activate(e,tp,eg,ep,ev,re,r,rp)`
fn activate(d: &mut Duel, a: &EffArgs) {
    // local rg=Duel.GetMatchingGroup(s.texfilter,tp,LOCATION_MZONE|LOCATION_GRAVE,0,nil)
    let rg = duel::get_matching_group(d, Some(&texfilter), a.tp, LOCATION_MZONE | LOCATION_GRAVE, 0, NO_EX);
    // if #rg==0 then return end
    if rg.is_empty() {
        return;
    }
    // local sg=aux.SelectUnselectGroup(rg,e,tp,1,2,s.rescon,1,tp,HINTMSG_TODECK,s.rescon)
    let rs: aux::Rescon = &rescon;
    let sg = aux::select_unselect_group(d, &rg, Some(a.e), a.tp, Some(1), Some(2), Some(rs), 1, a.tp, Some(HINTMSG_TODECK as u64), Some(rs), None, false)
        .unwrap_or_default();
    // if #sg==0 then return end
    if sg.is_empty() {
        return;
    }
    // Duel.HintSelection(sg,true)
    duel::hint_selection(d, &sg, true);
    // if Duel.SendtoDeck(sg,nil,SEQ_DECKTOP,REASON_EFFECT)>0 and sg:IsExists(Card.IsLocation,1,nil,LOCATION_EXTRA)
    //   and Duel.NegateActivation(ev) and re:GetHandler():IsRelateToEffect(re) then
    if duel::send_to_deck(d, sg.clone(), None, SEQ_DECKTOP as i32, REASON_EFFECT, None) > 0
        && group::is_exists(d, &sg, &|d, c| card::is_location(d, c, LOCATION_EXTRA), 1, NO_EX)
        && duel::negate_activation(d, a.ev as u8)
    {
        let rh = effect::get_handler(d, a.re.nil()).nil();
        if card::is_relate_to_effect(d, rh, a.re.nil()) {
            // Duel.Destroy(eg,REASON_EFFECT)
            duel::destroy(d, a.eg.clone().nil(), REASON_EFFECT, None, None);
        }
    }
}
/// `s.thfilter(c)`
fn thfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsSetCard(SET_BRANDED) and c:IsSpellTrap() and not c:IsCode(id) and c:IsAbleToHand()
    card::is_set_card(d, c, &[SET_BRANDED as u16]) && card::is_spell_trap(d, c) && !card::is_code(d, c, &[ID]) && card::is_able_to_hand(d, c, None)
}
/// `s.thtg(e,tp,eg,ep,ev,re,r,rp,chk,chkc)`
fn thtg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chkc then return chkc:IsLocation(LOCATION_GRAVE) and chkc:IsControler(tp) and s.thfilter(chkc) end
    if let Some(chkc) = a.chkc() {
        return card::is_location(d, chkc, LOCATION_GRAVE) && card::is_controler(d, chkc, a.tp) && thfilter(d, chkc);
    }
    // local h=e:GetHandler()
    let h = effect::get_handler(d, a.e).nil();
    // if chk==0 then return Duel.IsExistingTarget(s.thfilter,tp,LOCATION_GRAVE,0,1,e:GetHandler()) end
    if a.chk == Some(0) {
        return duel::is_existing_target(d, Some(&thfilter), a.tp, LOCATION_GRAVE, 0, 1, h.into());
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_ATOHAND)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_ATOHAND as u64);
    // local g=Duel.SelectTarget(tp,s.thfilter,tp,LOCATION_GRAVE,0,1,1,nil)
    let g = duel::select_target(d, a.tp, Some(&thfilter), a.tp, LOCATION_GRAVE, 0, 1, 1, false, NO_EX);
    // Duel.SetOperationInfo(0,CATEGORY_TOHAND,g,1,tp,0)
    duel::set_operation_info(d, 0, CATEGORY_TOHAND, g, 1, a.tp, 0);
    true
}
/// `s.thop(e,tp,eg,ep,ev,re,r,rp)`
fn thop(d: &mut Duel, a: &EffArgs) {
    // local tc=Duel.GetFirstTarget()
    let tc = duel::first_target(d).nil();
    // if tc:IsRelateToEffect(e) and Duel.SendtoHand(tc,nil,REASON_EFFECT)>0 then
    if card::is_relate_to_effect(d, tc, a.e) && duel::send_to_hand(d, tc, None, REASON_EFFECT, None) > 0 {
        // Duel.ConfirmCards(1-tp,tc)
        duel::confirm_cards(d, 1 - a.tp, tc);
    }
}
