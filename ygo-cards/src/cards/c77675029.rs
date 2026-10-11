//! Exosister Karmael
use super::prelude::*;

pub const ID: u32 = 77675029;

pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_EXOSISTER], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // c:EnableReviveLimit()
    card::enable_revive_limit(d, c);
    // Xyz Summon procedure: 2 Level 4 "Exosister" monsters
    // Xyz.AddProcedure(c,aux.FilterBoolFunctionEx(Card.IsSetCard,SET_EXOSISTER),4,2)
    xyz::add_procedure(d, c, Some(mat_filter(|d, c, sc, st, tp| card::is_set_card_ctx(d, c, &[SET_EXOSISTER as u16], sc, st, Some(tp)))), Some(4), 2, None, None, None, None, false, None);
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_SET)
    effect::set_category(d, e1, CATEGORY_SET);
    // e1:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_O);
    // e1:SetProperty(EFFECT_FLAG_DELAY)
    effect::set_property(d, e1, EFFECT_FLAG_DELAY, 0);
    // e1:SetCode(EVENT_SPSUMMON_SUCCESS)
    effect::set_code(d, e1, EVENT_SPSUMMON_SUCCESS);
    // e1:SetCountLimit(1,id)
    effect::set_count_limit(d, e1, 1, ID, 0, 0);
    // e1:SetCondition(function(e) return e:GetHandler():IsXyzSummoned() end)
    effect::set_condition(d, e1, Some(condition(|d, a| {
        let h = effect::get_handler(d, a.e).nil();
        card::is_xyz_summoned(d, h)
    })));
    // e1:SetOperation(s.regop)
    effect::set_operation(d, e1, Some(operation(regop)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e2, aux::stringid(ID, 1));
    // e2:SetType(EFFECT_TYPE_QUICK_O)
    effect::set_type(d, e2, EFFECT_TYPE_QUICK_O);
    // e2:SetCode(EVENT_CHAINING)
    effect::set_code(d, e2, EVENT_CHAINING);
    // e2:SetRange(LOCATION_MZONE)
    effect::set_range(d, e2, LOCATION_MZONE);
    // e2:SetCountLimit(1,{id,1})
    effect::set_count_limit(d, e2, 1, ID, 1, 0);
    // e2:SetCondition(s.chcon)
    effect::set_condition(d, e2, Some(condition(chcon)));
    // e2:SetTarget(s.chtg)
    effect::set_target(d, e2, Some(cost_target(chtg)));
    // e2:SetOperation(s.chop)
    effect::set_operation(d, e2, Some(operation(chop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
}
/// `s.regop(e,tp,...)`
fn regop(d: &mut Duel, a: &EffArgs) {
    // local e1=Effect.CreateEffect(e:GetHandler())
    let h = effect::get_handler(d, a.e).nil();
    let e1 = effect::create_effect(d, h);
    // e1:SetDescription(aux.Stringid(id,2))
    effect::set_description(d, e1, aux::stringid(ID, 2));
    // e1:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_CONTINUOUS)
    effect::set_type(d, e1, EFFECT_TYPE_FIELD + EFFECT_TYPE_CONTINUOUS);
    // e1:SetCode(EVENT_PHASE+PHASE_END)
    effect::set_code(d, e1, EVENT_PHASE + PHASE_END);
    // e1:SetCountLimit(1)
    effect::set_count_limit(d, e1, 1, 0, 0, 0);
    // e1:SetOperation(s.setop)
    effect::set_operation(d, e1, Some(operation(setop)));
    // e1:SetReset(RESET_PHASE|PHASE_END)
    effect::set_reset(d, e1, RESET_PHASE | PHASE_END, 1);
    // Duel.RegisterEffect(e1,tp)
    duel::register_effect(d, e1, a.tp);
}
/// `s.setfilter(c)`
fn setfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsSetCard(SET_EXOSISTER) and c:IsSpellTrap() and c:IsSSetable() and (c:IsFaceup() or c:IsLocation(LOCATION_DECK))
    card::is_set_card(d, c, &[SET_EXOSISTER as u16])
        && card::is_spell_trap(d, c)
        && card::is_s_setable(d, c, false, None)
        && (card::is_faceup(d, c) || card::is_location(d, c, LOCATION_DECK))
}
/// `s.setop(e,tp,...)`
fn setop(d: &mut Duel, a: &EffArgs) {
    // Duel.Hint(HINT_CARD,0,id)
    duel::hint(d, HINT_CARD as u8, 0, ID as u64);
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_SET)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_SET as u64);
    // local g=Duel.SelectMatchingCard(tp,aux.NecroValleyFilter(s.setfilter),tp,LOCATION_DECK|LOCATION_GRAVE|LOCATION_REMOVED,0,1,1,nil)
    let g = duel::select_matching_card(d, a.tp, Some(&aux::necro_valley_filter(setfilter)), a.tp, LOCATION_DECK | LOCATION_GRAVE | LOCATION_REMOVED, 0, 1, 1, false, NO_EX).nil();
    // if #g>0 then Duel.SSet(tp,g) end
    if !g.is_empty() {
        duel::s_set(d, a.tp, g, None, true);
    }
}
/// `s.chcon(e,tp,eg,ep,ev,re,r,rp)`
fn chcon(d: &mut Duel, a: &EffArgs) -> bool {
    // local rc=re:GetHandler()
    let re = a.re.nil();
    let rc = effect::get_handler(d, re).nil();
    // return rp==1-tp and (re:IsMonsterEffect() or (rc:IsNormalSpellTrap() and re:IsHasType(EFFECT_TYPE_ACTIVATE)))
    a.rp == 1 - a.tp
        && (effect::is_monster_effect(d, re) || (card::is_normal_spell_trap(d, rc) && effect::is_has_type(d, re, EFFECT_TYPE_ACTIVATE)))
}
/// `s.chtg(e,tp,...,chk)`
fn chtg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // if chk==0 then
    if a.chk == Some(0) {
        // local c=e:GetHandler()
        let c = effect::get_handler(d, a.e).nil();
        // local mat_ct=c:GetOverlayCount()
        let mat_ct = card::get_overlay_count(d, c) as u16;
        // return mat_ct>0 and c:CheckRemoveOverlayCard(tp,mat_ct,REASON_EFFECT)
        //     and Duel.IsExistingMatchingCard(Card.IsAbleToHand,tp,LOCATION_GRAVE,0,1,nil)
        return mat_ct > 0
            && card::check_remove_overlay_card(d, c, tp, mat_ct, REASON_EFFECT)
            && duel::is_existing_matching_card(d, Some(&|d, c| card::is_able_to_hand(d, c, None)), tp, LOCATION_GRAVE, 0, 1, NO_EX);
    }
    true
}
/// `s.chop(e,tp,eg,ep,ev,re,r,rp)`
fn chop(d: &mut Duel, a: &EffArgs) {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // local mg=c:GetOverlayGroup()
    let mg = card::get_overlay_group(d, c);
    // if c:IsRelateToEffect(e) and #mg>0 and Duel.SendtoGrave(mg,REASON_EFFECT)>0 then
    if card::is_relate_to_effect(d, c, a.e) && !mg.is_empty() && duel::send_to_grave(d, mg, REASON_EFFECT, None, None) > 0 {
        // local g=Group.CreateGroup()
        let g = CardSet::new();
        // Duel.ChangeTargetCard(ev,g)
        duel::change_target_card(d, a.ev as u8, &g);
        // Duel.ChangeChainOperation(ev,s.repop)
        duel::change_chain_operation(d, a.ev as u8, operation(repop));
    }
}
/// `s.repop(e,tp,...)`
fn repop(d: &mut Duel, a: &EffArgs) {
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_ATOHAND)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_ATOHAND as u64);
    // local g=Duel.SelectMatchingCard(tp,aux.NecroValleyFilter(Card.IsAbleToHand),tp,0,LOCATION_GRAVE,1,1,nil)
    let g = duel::select_matching_card(d, a.tp, Some(&aux::necro_valley_filter(|d, c| card::is_able_to_hand(d, c, None))), a.tp, 0, LOCATION_GRAVE, 1, 1, false, NO_EX).nil();
    // if #g>0 then
    if !g.is_empty() {
        // Duel.HintSelection(g)
        duel::hint_selection(d, g.clone(), true);
        // Duel.SendtoHand(g,nil,REASON_EFFECT)
        duel::send_to_hand(d, g, None, REASON_EFFECT, None);
    }
}
