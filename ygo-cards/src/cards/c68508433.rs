//! Master Spirit Tech Force - Pendulum Ruler
use super::prelude::*;

pub const ID: u32 = 68508433;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // c:EnableReviveLimit()
    card::enable_revive_limit(d, c);
    // Pendulum.AddProcedure(c,false)
    pendulum::add_procedure(d, c, Some(false), None);
    // Fusion.AddProcMix(c,true,true,s.matfilter,aux.FilterBoolFunctionEx(Card.IsType,TYPE_PENDULUM))
    fusion::add_proc_mix(
        d,
        c,
        true,
        true,
        vec![
            fusion::Mat::F(fusion::fus_mat(|d, c, fc, st, tp| matfilter(d, c, fc, st, tp))),
            fusion::Mat::F(fusion::fus_mat(|d, c, fc, st, tp| card::is_type_ctx(d, c, TYPE_PENDULUM, Some(fc), st, Some(tp)))),
        ],
    );
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_TOHAND+CATEGORY_SEARCH)
    effect::set_category(d, e1, CATEGORY_TOHAND + CATEGORY_SEARCH);
    // e1:SetType(EFFECT_TYPE_IGNITION)
    effect::set_type(d, e1, EFFECT_TYPE_IGNITION);
    // e1:SetRange(LOCATION_PZONE)
    effect::set_range(d, e1, LOCATION_PZONE);
    // e1:SetCountLimit(1,{id,0})
    effect::set_count_limit(d, e1, 1, ID, 0, 0);
    // e1:SetTarget(s.textg)
    effect::set_target(d, e1, Some(cost_target(textg)));
    // e1:SetOperation(s.texop)
    effect::set_operation(d, e1, Some(operation(texop)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e2, EFFECT_TYPE_SINGLE);
    // e2:SetProperty(EFFECT_FLAG_SINGLE_RANGE)
    effect::set_property(d, e2, EFFECT_FLAG_SINGLE_RANGE, 0);
    // e2:SetCode(EFFECT_INDESTRUCTABLE_EFFECT)
    effect::set_code(d, e2, EFFECT_INDESTRUCTABLE_EFFECT);
    // e2:SetRange(LOCATION_MZONE)
    effect::set_range(d, e2, LOCATION_MZONE);
    // e2:SetValue(1)
    effect::set_value(d, e2, 1);
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
    // local e3=Effect.CreateEffect(c)
    let e3 = effect::create_effect(d, c);
    // e3:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e3, EFFECT_TYPE_FIELD);
    // e3:SetCode(EFFECT_CANNOT_ATTACK)
    effect::set_code(d, e3, EFFECT_CANNOT_ATTACK);
    // e3:SetRange(LOCATION_MZONE)
    effect::set_range(d, e3, LOCATION_MZONE);
    // e3:SetTarget(function(e,c) return not (c:IsPendulumMonster() and not c:HasLevel()) end)
    effect::set_target(d, e3, Some(effect_card_fn(|d, _e, c| !(card::is_pendulum_monster(d, c) && !card::has_level(d, c)))));
    // e3:SetTargetRange(0,LOCATION_MZONE)
    effect::set_target_range(d, e3, 0, LOCATION_MZONE);
    // c:RegisterEffect(e3)
    card::register_effect(d, c, e3, false);
    // local e4=Effect.CreateEffect(c)
    let e4 = effect::create_effect(d, c);
    // e4:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e4, aux::stringid(ID, 1));
    // e4:SetCategory(CATEGORY_DESTROY+CATEGORY_DAMAGE)
    effect::set_category(d, e4, CATEGORY_DESTROY + CATEGORY_DAMAGE);
    // e4:SetType(EFFECT_TYPE_IGNITION)
    effect::set_type(d, e4, EFFECT_TYPE_IGNITION);
    // e4:SetProperty(EFFECT_FLAG_CARD_TARGET)
    effect::set_property(d, e4, EFFECT_FLAG_CARD_TARGET, 0);
    // e4:SetRange(LOCATION_MZONE)
    effect::set_range(d, e4, LOCATION_MZONE);
    // e4:SetCountLimit(2,{id,1})
    effect::set_count_limit(d, e4, 2, ID, 1, 0);
    // e4:SetCost(s.descost)
    effect::set_cost(d, e4, Some(cost_target(descost)));
    // e4:SetTarget(s.destg)
    effect::set_target(d, e4, Some(cost_target(destg)));
    // e4:SetOperation(s.desop)
    effect::set_operation(d, e4, Some(operation(desop)));
    // c:RegisterEffect(e4)
    card::register_effect(d, c, e4, false);
    // local e5=Effect.CreateEffect(c)
    let e5 = effect::create_effect(d, c);
    // e5:SetDescription(aux.Stringid(id,2))
    effect::set_description(d, e5, aux::stringid(ID, 2));
    // e5:SetType(EFFECT_TYPE_IGNITION)
    effect::set_type(d, e5, EFFECT_TYPE_IGNITION);
    // e5:SetRange(LOCATION_MZONE)
    effect::set_range(d, e5, LOCATION_MZONE);
    // e5:SetCountLimit(1)
    effect::set_count_limit(d, e5, 1, 0, 0, 0);
    // e5:SetTarget(function(e,tp,eg,ep,ev,re,r,rp,chk) if chk==0 then return Duel.CheckPendulumZones(tp) end end)
    effect::set_target(d, e5, Some(cost_target(e5_target)));
    // e5:SetOperation(s.plop)
    effect::set_operation(d, e5, Some(operation(plop)));
    // c:RegisterEffect(e5)
    card::register_effect(d, c, e5, false);
}
/// `s.matfilter(c,fc,sumtype,tp)`
fn matfilter(d: &mut Duel, c: CardId, fc: CardId, sumtype: u64, tp: u8) -> bool {
    // return c:IsLevelAbove(7) and c:IsType(TYPE_PENDULUM,fc,sumtype,tp)
    card::is_level_above(d, c, 7) && card::is_type_ctx(d, c, TYPE_PENDULUM, Some(fc), sumtype, Some(tp))
}
/// `s.textg(e,tp,...,chk)`
fn textg(d: &mut Duel, a: &EffArgs) -> bool {
    let c = effect::get_handler(d, a.e).nil();
    // if chk==0 then return c:IsAbleToExtra() end
    if a.chk == Some(0) {
        return card::is_able_to_extra(d, c);
    }
    // Duel.SetOperationInfo(0,CATEGORY_TOEXTRA,c,1,tp,0)
    duel::set_operation_info(d, 0, CATEGORY_TOEXTRA, Some(CardSet::from_card(c)), 1, a.tp, 0);
    // Duel.SetPossibleOperationInfo(0,CATEGORY_TOHAND,nil,1,tp,LOCATION_DECK|LOCATION_EXTRA)
    duel::set_possible_operation_info(d, 0, CATEGORY_TOHAND, None, 1, a.tp, (LOCATION_DECK | LOCATION_EXTRA) as i32);
    true
}
/// `s.thfilter(c)`
fn thfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsLevel(7) and c:IsPendulumMonster() and (c:IsLocation(LOCATION_DECK) or c:IsFaceup()) and c:IsAbleToHand()
    card::is_level(d, c, &[7])
        && card::is_pendulum_monster(d, c)
        && (card::is_location(d, c, LOCATION_DECK) || card::is_faceup(d, c))
        && card::is_able_to_hand(d, c, None)
}
/// `s.texop(e,tp,...)`
fn texop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if c:IsRelateToEffect(e) and Duel.SendtoDeck(c,nil,SEQ_DECKSHUFFLE,REASON_EFFECT)>0
    //     and Duel.IsExistingMatchingCard(s.thfilter,tp,LOCATION_DECK|LOCATION_EXTRA,0,1,nil)
    //     and Duel.SelectYesNo(tp,aux.Stringid(id,3)) then
    if card::is_relate_to_effect(d, c, a.e)
        && duel::send_to_deck(d, CardSet::from_card(c), None, SEQ_DECKSHUFFLE as i32, REASON_EFFECT, None) > 0
        && duel::is_existing_matching_card(d, Some(&thfilter), tp, LOCATION_DECK | LOCATION_EXTRA, 0, 1, NO_EX)
        && duel::select_yes_no(d, tp, aux::stringid(ID, 3))
    {
        // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_ATOHAND)
        duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_ATOHAND as u64);
        // local g=Duel.SelectMatchingCard(tp,s.thfilter,tp,LOCATION_DECK|LOCATION_EXTRA,0,1,1,nil)
        let g = duel::select_matching_card(d, tp, Some(&thfilter), tp, LOCATION_DECK | LOCATION_EXTRA, 0, 1, 1, false, NO_EX).nil();
        // if #g>0 then
        if !g.is_empty() {
            // Duel.BreakEffect()
            duel::break_effect(d);
            // Duel.SendtoHand(g,nil,REASON_EFFECT)
            duel::send_to_hand(d, g.clone(), None, REASON_EFFECT, None);
            // Duel.ConfirmCards(1-tp,g)
            duel::confirm_cards(d, 1 - tp, g);
        }
    }
}
/// `function(e,c,tp,r) ...` the filter of `s.descostfilter(c,tp)`
fn descostfilter(d: &mut Duel, c: CardId, tp: u8) -> bool {
    // local exg=c:GetEquipGroup()+c
    let mut exg = card::get_equip_group(d, c);
    group::add_card(&mut exg, &CardSet::from_card(c));
    // return Duel.IsExistingTarget(nil,tp,0,LOCATION_ONFIELD,1,exg)
    duel::is_existing_target(d, None, tp, 0, LOCATION_ONFIELD, 1, (&exg).into())
}
/// `s.descost(e,tp,...,chk)`
fn descost(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // if chk==0 then return Duel.CheckReleaseGroupCost(tp,s.descostfilter,1,false,nil,nil,tp) end
    if a.chk == Some(0) {
        return duel::check_release_group_cost(d, tp, Some(&|d, c| descostfilter(d, c, tp)), 1, 1, false, None, NO_EX);
    }
    // local g=Duel.SelectReleaseGroupCost(tp,s.descostfilter,1,1,false,nil,nil,tp)
    let g = duel::select_release_group_cost(d, tp, Some(&|d, c| descostfilter(d, c, tp)), 1, 1, false, None, NO_EX);
    // Duel.Release(g,REASON_COST)
    duel::release(d, g, REASON_COST, None);
    true
}
/// `s.destg(e,tp,...,chk,chkc)`
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
    // local g=Duel.SelectTarget(tp,nil,tp,0,LOCATION_ONFIELD,1,1,nil)
    let g = duel::select_target(d, tp, None, tp, 0, LOCATION_ONFIELD, 1, 1, false, NO_EX);
    // Duel.SetOperationInfo(0,CATEGORY_DESTROY,g,1,tp,0)
    duel::set_operation_info(d, 0, CATEGORY_DESTROY, g, 1, tp, 0);
    // Duel.SetOperationInfo(0,CATEGORY_DAMAGE,nil,0,1-tp,1600)
    duel::set_operation_info(d, 0, CATEGORY_DAMAGE, None, 0, 1 - tp, 1600);
    true
}
/// `s.desop(e,tp,...)`
fn desop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // local tc=Duel.GetFirstTarget()
    let tc = duel::first_target(d).nil();
    // if tc:IsRelateToEffect(e) and Duel.Destroy(tc,REASON_EFFECT)>0 then
    if card::is_relate_to_effect(d, tc, a.e) && duel::destroy(d, CardSet::from_card(tc), REASON_EFFECT, None, None) > 0 {
        // Duel.Damage(1-tp,1600,REASON_EFFECT)
        duel::damage(d, 1 - tp, 1600, REASON_EFFECT, false, None);
    }
}
/// `function(e,tp,...,chk) if chk==0 then return Duel.CheckPendulumZones(tp) end end`
fn e5_target(d: &mut Duel, a: &EffArgs) -> bool {
    if a.chk == Some(0) {
        return duel::check_pendulum_zones(d, a.tp);
    }
    true
}
/// `s.plop(e,tp,...)`
fn plop(d: &mut Duel, a: &EffArgs) {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if c:IsRelateToEffect(e) then
    if card::is_relate_to_effect(d, c, a.e) {
        // Duel.MoveToField(c,tp,tp,LOCATION_PZONE,POS_FACEUP,true)
        duel::move_to_field(d, c, a.tp, a.tp, LOCATION_PZONE as u16, POS_FACEUP as u8, true, None);
    }
}
