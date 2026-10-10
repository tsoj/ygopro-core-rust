//! Number 1: Infection Buzzking
//! scripted by pyrQ, adapted from the anime script
use super::prelude::*;

pub const ID: u32 = 10666000;

pub const INFO: ScriptInfo = ScriptInfo { xyz_number: Some(1), ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // c:EnableReviveLimit()
    card::enable_revive_limit(d, c);
    // Xyz Summon procedure
    // Xyz.AddProcedure(c,nil,8,2,nil,nil,Xyz.InfiniteMats)
    xyz::add_procedure(d, c, None, Some(8), 2, None, None, Some(xyz::INFINITE_MATS), None, false, None);
    // Attach 1 card from your opponent's GY to this card
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_LEAVE_GRAVE)
    effect::set_category(d, e1, CATEGORY_LEAVE_GRAVE);
    // e1:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e1, EFFECT_TYPE_FIELD + EFFECT_TYPE_TRIGGER_O);
    // e1:SetCode(EVENT_PHASE|PHASE_STANDBY)
    effect::set_code(d, e1, EVENT_PHASE | PHASE_STANDBY);
    // e1:SetRange(LOCATION_MZONE)
    effect::set_range(d, e1, LOCATION_MZONE);
    // e1:SetCountLimit(1)
    effect::set_count_limit(d, e1, 1, 0, 0, 0);
    // e1:SetCondition(function(_,tp) return Duel.IsTurnPlayer(tp) end)
    effect::set_condition(d, e1, Some(condition(|d, a| duel::is_turn_player(d, a.tp))));
    // e1:SetTarget(s.mattg)
    effect::set_target(d, e1, Some(cost_target(mattg)));
    // e1:SetOperation(s.matop)
    effect::set_operation(d, e1, Some(operation(matop)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // Look at your opponent's Extra Deck
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e2, aux::stringid(ID, 1));
    // e2:SetCategory(CATEGORY_TOGRAVE)
    effect::set_category(d, e2, CATEGORY_TOGRAVE);
    // e2:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e2, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_O);
    // e2:SetProperty(EFFECT_FLAG_DELAY)
    effect::set_property(d, e2, EFFECT_FLAG_DELAY, 0);
    // e2:SetCode(EVENT_SPSUMMON_SUCCESS)
    effect::set_code(d, e2, EVENT_SPSUMMON_SUCCESS);
    // e2:SetCountLimit(1,id)
    effect::set_count_limit(d, e2, 1, ID, 0, 0);
    // e2:SetCondition(function(e) return e:GetHandler():IsXyzSummoned() end)
    effect::set_condition(d, e2, Some(condition(|d, a| {
        let h = effect::get_handler(d, a.e).nil();
        card::is_xyz_summoned(d, h)
    })));
    // e2:SetTarget(s.tgtg)
    effect::set_target(d, e2, Some(cost_target(tgtg)));
    // e2:SetOperation(s.tgop)
    effect::set_operation(d, e2, Some(operation(tgop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
    // Destroy 1 card your opponent controls
    // local e3=Effect.CreateEffect(c)
    let e3 = effect::create_effect(d, c);
    // e3:SetDescription(aux.Stringid(id,2))
    effect::set_description(d, e3, aux::stringid(ID, 2));
    // e3:SetCategory(CATEGORY_DESTROY+CATEGORY_DAMAGE)
    effect::set_category(d, e3, CATEGORY_DESTROY + CATEGORY_DAMAGE);
    // e3:SetType(EFFECT_TYPE_IGNITION)
    effect::set_type(d, e3, EFFECT_TYPE_IGNITION);
    // e3:SetProperty(EFFECT_FLAG_CARD_TARGET)
    effect::set_property(d, e3, EFFECT_FLAG_CARD_TARGET, 0);
    // e3:SetRange(LOCATION_MZONE)
    effect::set_range(d, e3, LOCATION_MZONE);
    // e3:SetCountLimit(1,{id,1})
    effect::set_count_limit(d, e3, 1, ID, 1, 0);
    // e3:SetCost(Cost.DetachFromSelf(1,1,nil))
    effect::set_cost(d, e3, Some(cost::detach_from_self(cost::Count::N(1), Some(cost::Count::N(1)), None)));
    // e3:SetTarget(s.destg)
    effect::set_target(d, e3, Some(cost_target(destg)));
    // e3:SetOperation(s.desop)
    effect::set_operation(d, e3, Some(operation(desop)));
    // c:RegisterEffect(e3)
    card::register_effect(d, c, e3, false);
}
/// `s.mattg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn mattg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // if chk==0 then return Duel.IsExistingMatchingCard(nil,tp,0,LOCATION_GRAVE,1,nil) end
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, None, tp, 0, LOCATION_GRAVE, 1, NO_EX);
    }
    // Duel.SetOperationInfo(0,CATEGORY_LEAVE_GRAVE,nil,1,1-tp,0)
    duel::set_operation_info(d, 0, CATEGORY_LEAVE_GRAVE, None, 1, 1 - tp, 0);
    true
}
/// `s.matop(e,tp,eg,ep,ev,re,r,rp)`
fn matop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if not c:IsRelateToEffect(e) then return end
    if !card::is_relate_to_effect(d, c, a.e) {
        return;
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_XMATERIAL)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_XMATERIAL as u64);
    // local g=Duel.SelectMatchingCard(tp,nil,tp,0,LOCATION_GRAVE,1,1,nil)
    let g = duel::select_matching_card(d, tp, None, tp, 0, LOCATION_GRAVE, 1, 1, false, NO_EX).nil();
    // if #g>0 then
    if !g.is_empty() {
        // Duel.HintSelection(g,true)
        duel::hint_selection(d, g.clone(), true);
        // Duel.Overlay(c,g)
        duel::overlay(d, c, g, false);
    }
}
/// `s.tgtg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn tgtg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // if chk==0 then return Duel.IsExistingMatchingCard(Card.IsAbleToGrave,tp,0,LOCATION_EXTRA,1,nil) end
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, Some(&|d, c| card::is_able_to_grave(d, c, None)), tp, 0, LOCATION_EXTRA, 1, NO_EX);
    }
    // Duel.SetOperationInfo(0,CATEGORY_TOGRAVE,nil,1,1-tp,LOCATION_EXTRA)
    duel::set_operation_info(d, 0, CATEGORY_TOGRAVE, None, 1, 1 - tp, LOCATION_EXTRA as i32);
    true
}
/// `s.tgop(e,tp,eg,ep,ev,re,r,rp)`
fn tgop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // local g=Duel.GetFieldGroup(tp,0,LOCATION_EXTRA)
    let g = duel::get_field_group(d, tp, 0, LOCATION_EXTRA);
    // if #g==0 then return end
    if g.is_empty() {
        return;
    }
    // Duel.ConfirmCards(tp,g)
    duel::confirm_cards(d, tp, g);
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_TOGRAVE)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_TOGRAVE as u64);
    // local g=Duel.SelectMatchingCard(tp,Card.IsAbleToGrave,tp,0,LOCATION_EXTRA,1,1,nil)
    let g = duel::select_matching_card(d, tp, Some(&|d, c| card::is_able_to_grave(d, c, None)), tp, 0, LOCATION_EXTRA, 1, 1, false, NO_EX).nil();
    // if #g>0 then
    if !g.is_empty() {
        // Duel.SendtoGrave(g,REASON_EFFECT)
        duel::send_to_grave(d, g, REASON_EFFECT, None, None);
    }
    // Duel.ShuffleExtra(1-tp)
    duel::shuffle_extra(d, 1 - tp);
}
/// `s.destg(e,tp,eg,ep,ev,re,r,rp,chk,chkc)`
fn destg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // if chkc then return chkc:IsOnField() and chkc:IsControler(1-tp) end
    if let Some(chkc) = a.chkc() {
        return card::is_on_field(d, chkc) && card::is_controler(d, chkc, 1 - tp);
    }
    // if chk==0 then return Duel.IsExistingTarget(nil,tp,0,LOCATION_ONFIELD,1,nil) end
    if a.chk == Some(0) {
        return duel::is_existing_target(d, None, tp, 0, LOCATION_ONFIELD, 1, NO_EX);
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_DESTROY)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_DESTROY as u64);
    // local tc=Duel.SelectTarget(tp,nil,tp,0,LOCATION_ONFIELD,1,1,nil):GetFirst()
    let sg = duel::select_target(d, tp, None, tp, 0, LOCATION_ONFIELD, 1, 1, false, NO_EX).nil();
    let tc = group::get_first(d, &sg);
    // Duel.SetOperationInfo(0,CATEGORY_DESTROY,tc,1,tp,0)
    duel::set_operation_info(d, 0, CATEGORY_DESTROY, tc.map(CardSet::from_card), 1, tp, 0);
    let tc = tc.nil();
    // if tc:IsMonster() then
    if card::is_monster(d, tc) {
        // if tc:IsFaceup() then
        if card::is_faceup(d, tc) {
            // Duel.SetOperationInfo(0,CATEGORY_DAMAGE,nil,0,1-tp,tc:GetAttack()/2)
            let dam = card::get_attack(d, tc) as f64 / 2.0;
            duel::set_operation_info(d, 0, CATEGORY_DAMAGE, None, 0, 1 - tp, lua_int(dam) as i32);
        } else {
            // Duel.SetPossibleOperationInfo(0,CATEGORY_DAMAGE,nil,0,1-tp,0)
            duel::set_possible_operation_info(d, 0, CATEGORY_DAMAGE, None, 0, 1 - tp, 0);
        }
    }
    true
}
/// `s.desop(e,tp,eg,ep,ev,re,r,rp)`
fn desop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // local tc=Duel.GetFirstTarget()
    let tc = duel::first_target(d).nil();
    // if not tc:IsRelateToEffect(e) then return end
    if !card::is_relate_to_effect(d, tc, a.e) {
        return;
    }
    // local dam=0
    let mut dam = 0.0_f64;
    // if tc:IsMonster() and tc:IsFaceup() then
    if card::is_monster(d, tc) && card::is_faceup(d, tc) {
        // dam=tc:GetAttack()/2
        dam = card::get_attack(d, tc) as f64 / 2.0;
    }
    // if Duel.Destroy(tc,REASON_EFFECT)>0 and dam>0 then
    if duel::destroy(d, tc, REASON_EFFECT, None, None) > 0 && dam > 0.0 {
        // Duel.BreakEffect()
        duel::break_effect(d);
        // Duel.Damage(1-tp,dam,REASON_EFFECT)
        duel::damage(d, 1 - tp, lua_int(dam), REASON_EFFECT, false, None);
    }
}
