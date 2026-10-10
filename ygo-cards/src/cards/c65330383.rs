//! Knightmare Gryphon
use super::prelude::*;

pub const ID: u32 = 65330383;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // c:EnableReviveLimit()
    card::enable_revive_limit(d, c);
    // Link Summon procedure: 2+ monsters with different names
    // Link.AddProcedure(c,nil,2,99,s.matcheck)
    link::add_procedure(d, c, None, 2, Some(99), Some(std::sync::Arc::new(matcheck)), None, None);
    // If this card is Link Summoned: You can discard 1 card, then target 1 Spell/Trap in your GY; ...
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_O);
    // e1:SetProperty(EFFECT_FLAG_DELAY+EFFECT_FLAG_CARD_TARGET)
    effect::set_property(d, e1, EFFECT_FLAG_DELAY + EFFECT_FLAG_CARD_TARGET, 0);
    // e1:SetCode(EVENT_SPSUMMON_SUCCESS)
    effect::set_code(d, e1, EVENT_SPSUMMON_SUCCESS);
    // e1:SetCountLimit(1,id)
    effect::set_count_limit(d, e1, 1, ID, 0, 0);
    // e1:SetCondition(function(e) return e:GetHandler():IsLinkSummoned() end)
    effect::set_condition(d, e1, Some(condition(e1_condition)));
    // e1:SetCost(Cost.Discard())
    effect::set_cost(d, e1, Some(cost::discard(None, false, None, None, None)));
    // e1:SetTarget(s.settg)
    effect::set_target(d, e1, Some(cost_target(settg)));
    // e1:SetOperation(s.setop)
    effect::set_operation(d, e1, Some(operation(setop)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // Special Summoned monsters on the field cannot activate their effects, unless they are linked
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e2, EFFECT_TYPE_FIELD);
    // e2:SetProperty(EFFECT_FLAG_PLAYER_TARGET)
    effect::set_property(d, e2, EFFECT_FLAG_PLAYER_TARGET, 0);
    // e2:SetCode(EFFECT_CANNOT_ACTIVATE)
    effect::set_code(d, e2, EFFECT_CANNOT_ACTIVATE);
    // e2:SetRange(LOCATION_MZONE)
    effect::set_range(d, e2, LOCATION_MZONE);
    // e2:SetTargetRange(1,1)
    effect::set_target_range(d, e2, 1, 1);
    // e2:SetValue(s.aclimit)
    effect::set_value_fn(d, e2, lua_fn(aclimit));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
}
// s.matcheck(g,lc,sumtype,tp)
fn matcheck(d: &mut Duel, g: &CardSet, _lc: CardId, _sumtype: u64, _tp: u8) -> bool {
    // return g:CheckDifferentProperty(Card.GetCode,lc,sumtype,tp)
    group::check_different_property(d, g, &|d, c| card::get_code(d, c).into_iter().map(i64::from).collect())
}
// function(e) return e:GetHandler():IsLinkSummoned() end
fn e1_condition(d: &mut Duel, a: &EffArgs) -> bool {
    card::is_link_summoned(d, effect::get_handler(d, a.e).nil())
}
// s.settg(e,tp,eg,ep,ev,re,r,rp,chk,chkc)
fn settg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chkc then return chkc:IsControler(tp) and chkc:IsLocation(LOCATION_GRAVE) and chkc:IsSpellTrap() and chkc:IsSSetable() end
    if let Some(chkc) = a.chkc() {
        return card::is_controler(d, chkc, a.tp) && card::is_location(d, chkc, LOCATION_GRAVE) && card::is_spell_trap(d, chkc) && card::is_s_setable(d, chkc, false, None);
    }
    // if chk==0 then return Duel.IsExistingTarget(aux.AND(Card.IsSpellTrap,Card.IsSSetable),tp,LOCATION_GRAVE,0,1,nil) end
    if a.chk == Some(0) {
        return duel::is_existing_target(d, Some(&|d, c| card::is_spell_trap(d, c) && card::is_s_setable(d, c, false, None)), a.tp, LOCATION_GRAVE, 0, 1, NO_EX);
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_SET)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_SET as u64);
    // local g=Duel.SelectTarget(tp,aux.AND(Card.IsSpellTrap,Card.IsSSetable),tp,LOCATION_GRAVE,0,1,1,nil)
    let g = duel::select_target(d, a.tp, Some(&|d, c| card::is_spell_trap(d, c) && card::is_s_setable(d, c, false, None)), a.tp, LOCATION_GRAVE, 0, 1, 1, false, NO_EX);
    // local extra_categ_if_colinked=0
    let mut extra_categ_if_colinked = 0;
    // if e:GetHandler():IsCoLinked() then
    if card::is_co_linked(d, effect::get_handler(d, a.e).nil(), None) {
        // e:SetLabel(1)
        effect::set_label(d, a.e, &[1]);
        // extra_categ_if_colinked=CATEGORY_DRAW
        extra_categ_if_colinked = CATEGORY_DRAW;
        // Duel.SetPossibleOperationInfo(0,CATEGORY_DRAW,nil,0,tp,1)
        duel::set_possible_operation_info(d, 0, CATEGORY_DRAW, None, 0, a.tp, 1);
    } else {
        // e:SetLabel(0)
        effect::set_label(d, a.e, &[0]);
    }
    // e:SetCategory(CATEGORY_SET|extra_categ_if_colinked)
    effect::set_category(d, a.e, CATEGORY_SET | extra_categ_if_colinked);
    // Duel.SetOperationInfo(0,CATEGORY_SET,g,1,tp,0)
    duel::set_operation_info(d, 0, CATEGORY_SET, g, 1, a.tp, 0);
    true
}
// s.setop(e,tp,eg,ep,ev,re,r,rp)
fn setop(d: &mut Duel, a: &EffArgs) {
    // local tc=Duel.GetFirstTarget()
    let tc = duel::first_target(d).nil();
    // if tc:IsRelateToEffect(e) and Duel.SSet(tp,tc)>0 then
    if card::is_relate_to_effect(d, tc, a.e) && duel::s_set(d, a.tp, CardSet::from_card(tc), None, true) > 0 {
        // It cannot be activated this turn
        // local e1=Effect.CreateEffect(e:GetHandler())
        let h = effect::get_handler(d, a.e).nil();
        let e1 = effect::create_effect(d, h);
        // e1:SetDescription(aux.Stringid(id,1))
        effect::set_description(d, e1, aux::stringid(ID, 1));
        // e1:SetType(EFFECT_TYPE_SINGLE)
        effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
        // e1:SetProperty(EFFECT_FLAG_CANNOT_DISABLE+EFFECT_FLAG_CLIENT_HINT)
        effect::set_property(d, e1, EFFECT_FLAG_CANNOT_DISABLE + EFFECT_FLAG_CLIENT_HINT, 0);
        // e1:SetCode(EFFECT_CANNOT_TRIGGER)
        effect::set_code(d, e1, EFFECT_CANNOT_TRIGGER);
        // e1:SetReset(RESET_EVENT|RESETS_CANNOT_ACT|RESET_PHASE|PHASE_END)
        effect::set_reset(d, e1, RESET_EVENT | RESETS_CANNOT_ACT | RESET_PHASE | PHASE_END, 1);
        // tc:RegisterEffect(e1)
        card::register_effect(d, tc, e1, false);
        // if e:GetLabel()==1 and Duel.IsPlayerCanDraw(tp,1) and Duel.SelectYesNo(tp,aux.Stringid(id,2)) then
        if effect::get_label(d, a.e) == 1 && duel::is_player_can_draw(d, a.tp, 1) && duel::select_yes_no(d, a.tp, aux::stringid(ID, 2)) {
            // Duel.BreakEffect()
            duel::break_effect(d);
            // Duel.Draw(tp,1,REASON_EFFECT)
            duel::draw(d, a.tp, 1, REASON_EFFECT);
        }
    }
}
// s.aclimit(e,re,tp)
fn aclimit(d: &mut Duel, a: Args) -> bool {
    // local rc=re:GetHandler()
    let re = a.e(1);
    let rc = effect::get_handler(d, re).nil();
    // return rc:IsSpecialSummoned() and rc:IsLocation(LOCATION_MZONE) and re:IsMonsterEffect() and not rc:IsLinked()
    card::is_special_summoned(d, rc) && card::is_location(d, rc, LOCATION_MZONE) && effect::is_monster_effect(d, re) && !card::is_linked(d, rc)
}
