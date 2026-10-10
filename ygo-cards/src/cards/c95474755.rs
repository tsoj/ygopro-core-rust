//! No.89: Diablosis the Mind Hacker
use super::prelude::*;

pub const ID: u32 = 95474755;

pub const INFO: ScriptInfo = ScriptInfo { xyz_number: Some(89), ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Xyz Summon
    // Xyz.AddProcedure(c,nil,7,2)
    xyz::add_procedure(d, c, None, Some(7), 2, None, None, None, None, false, None);
    // c:EnableReviveLimit()
    card::enable_revive_limit(d, c);
    // Banish 1 card from the opponent's Extra Deck
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_REMOVE)
    effect::set_category(d, e1, CATEGORY_REMOVE);
    // e1:SetType(EFFECT_TYPE_IGNITION)
    effect::set_type(d, e1, EFFECT_TYPE_IGNITION);
    // e1:SetCountLimit(1)
    effect::set_count_limit(d, e1, 1, 0, 0, 0);
    // e1:SetRange(LOCATION_MZONE)
    effect::set_range(d, e1, LOCATION_MZONE);
    // e1:SetCost(Cost.DetachFromSelf(1,1,nil))
    effect::set_cost(d, e1, Some(cost::detach_from_self(cost::Count::N(1), Some(cost::Count::N(1)), None)));
    // e1:SetTarget(s.extg)
    effect::set_target(d, e1, Some(cost_target(extg)));
    // e1:SetOperation(s.exop)
    effect::set_operation(d, e1, Some(operation(exop)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // Banish 1 card from the opponent's GY
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e2, aux::stringid(ID, 1));
    // e2:SetCategory(CATEGORY_REMOVE)
    effect::set_category(d, e2, CATEGORY_REMOVE);
    // e2:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e2, EFFECT_TYPE_FIELD + EFFECT_TYPE_TRIGGER_O);
    // e2:SetProperty(EFFECT_FLAG_CARD_TARGET)
    effect::set_property(d, e2, EFFECT_FLAG_CARD_TARGET, 0);
    // e2:SetCode(EVENT_PHASE|PHASE_BATTLE)
    effect::set_code(d, e2, EVENT_PHASE | PHASE_BATTLE);
    // e2:SetRange(LOCATION_MZONE)
    effect::set_range(d, e2, LOCATION_MZONE);
    // e2:SetCountLimit(1)
    effect::set_count_limit(d, e2, 1, 0, 0, 0);
    // e2:SetCondition(function(e) return e:GetHandler():HasFlagEffect(id) end)
    effect::set_condition(d, e2, Some(condition(grcond)));
    // e2:SetTarget(s.grtg)
    effect::set_target(d, e2, Some(cost_target(grtg)));
    // e2:SetOperation(s.grop)
    effect::set_operation(d, e2, Some(operation(grop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
    // local e3=Effect.CreateEffect(c)
    let e3 = effect::create_effect(d, c);
    // e3:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_CONTINUOUS)
    effect::set_type(d, e3, EFFECT_TYPE_SINGLE + EFFECT_TYPE_CONTINUOUS);
    // e3:SetProperty(EFFECT_FLAG_CANNOT_DISABLE)
    effect::set_property(d, e3, EFFECT_FLAG_CANNOT_DISABLE, 0);
    // e3:SetCode(EVENT_BATTLE_DESTROYING)
    effect::set_code(d, e3, EVENT_BATTLE_DESTROYING);
    // e3:SetOperation(s.regop)
    effect::set_operation(d, e3, Some(operation(regop)));
    // c:RegisterEffect(e3)
    card::register_effect(d, c, e3, false);
    // Banish cards from the opponent's deck
    // local e4=Effect.CreateEffect(c)
    let e4 = effect::create_effect(d, c);
    // e4:SetDescription(aux.Stringid(id,2))
    effect::set_description(d, e4, aux::stringid(ID, 2));
    // e4:SetCategory(CATEGORY_REMOVE)
    effect::set_category(d, e4, CATEGORY_REMOVE);
    // e4:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e4, EFFECT_TYPE_FIELD + EFFECT_TYPE_TRIGGER_O);
    // e4:SetCode(EVENT_REMOVE)
    effect::set_code(d, e4, EVENT_REMOVE);
    // e4:SetRange(LOCATION_MZONE)
    effect::set_range(d, e4, LOCATION_MZONE);
    // e4:SetProperty(EFFECT_FLAG_DELAY)
    effect::set_property(d, e4, EFFECT_FLAG_DELAY, 0);
    // e4:SetCountLimit(1,id)
    effect::set_count_limit(d, e4, 1, ID, 0, 0);
    // e4:SetCondition(s.dkcon)
    effect::set_condition(d, e4, Some(condition(dkcon)));
    // e4:SetTarget(s.dktg)
    effect::set_target(d, e4, Some(cost_target(dktg)));
    // e4:SetOperation(s.dkop)
    effect::set_operation(d, e4, Some(operation(dkop)));
    // c:RegisterEffect(e4)
    card::register_effect(d, c, e4, false);
}
/// `s.extg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn extg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return Duel.IsExistingMatchingCard(Card.IsAbleToRemove,tp,0,LOCATION_EXTRA,1,nil,tp,POS_FACEDOWN) end
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(
            d,
            Some(&|d, c| card::is_able_to_remove(d, c, Some(a.tp), Some(POS_FACEDOWN as u8), None)),
            a.tp,
            0,
            LOCATION_EXTRA,
            1,
            NO_EX,
        );
    }
    // Duel.SetOperationInfo(0,CATEGORY_REMOVE,nil,1,1-tp,LOCATION_EXTRA)
    duel::set_operation_info(d, 0, CATEGORY_REMOVE, None, 1, 1 - a.tp, LOCATION_EXTRA as i32);
    true
}
/// `s.exop(e,tp,eg,ep,ev,re,r,rp)`
fn exop(d: &mut Duel, a: &EffArgs) {
    // local g=Duel.GetFieldGroup(tp,0,LOCATION_EXTRA)
    let g = duel::get_field_group(d, a.tp, 0, LOCATION_EXTRA);
    // Duel.ConfirmCards(tp,g)
    duel::confirm_cards(d, a.tp, g.clone());
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_REMOVE)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_REMOVE as u64);
    // local sg=g:FilterSelect(tp,Card.IsAbleToRemove,1,1,nil,tp,POS_FACEDOWN)
    let sg = group::filter_select(
        d,
        &g,
        a.tp,
        &|d, c| card::is_able_to_remove(d, c, Some(a.tp), Some(POS_FACEDOWN as u8), None),
        1,
        1,
        false,
        NO_EX,
    )
    .nil();
    // Duel.Remove(sg,POS_FACEDOWN,REASON_EFFECT)
    duel::remove(d, sg, POS_FACEDOWN as u8, REASON_EFFECT, None, None);
    // Duel.ShuffleExtra(1-tp)
    duel::shuffle_extra(d, 1 - a.tp);
}
/// `s.regop(e,tp,eg,ep,ev,re,r,rp)`
fn regop(d: &mut Duel, a: &EffArgs) {
    // e:GetHandler():RegisterFlagEffect(id,RESET_EVENT|RESETS_STANDARD|RESET_PHASE|PHASE_BATTLE,0,1)
    let h = effect::get_handler(d, a.e).nil();
    card::register_flag_effect(d, h, ID, RESET_EVENT | RESETS_STANDARD | RESET_PHASE | PHASE_BATTLE, 0, 1, 0, 0);
}
/// `s.rmfilter(c,tp)`
fn rmfilter(d: &mut Duel, c: CardId, tp: u8) -> bool {
    // return c:IsAbleToRemove(tp,POS_FACEDOWN) and aux.SpElimFilter(c)
    card::is_able_to_remove(d, c, Some(tp), Some(POS_FACEDOWN as u8), None) && aux::sp_elim_filter(d, c, false, false)
}
/// `s.grtg(e,tp,eg,ep,ev,re,r,rp,chk,chkc)`
fn grtg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chkc then return chkc:IsControler(1-tp) and chkc:IsLocation(LOCATION_MZONE|LOCATION_GRAVE) and s.rmfilter(chkc) end
    if let Some(chkc) = a.chkc() {
        return card::is_controler(d, chkc, 1 - a.tp) && card::is_location(d, chkc, LOCATION_MZONE | LOCATION_GRAVE) && rmfilter(d, chkc, a.tp);
    }
    // if chk==0 then return Duel.IsExistingTarget(s.rmfilter,tp,0,LOCATION_MZONE|LOCATION_GRAVE,1,nil,tp) end
    if a.chk == Some(0) {
        return duel::is_existing_target(d, Some(&|d, c| rmfilter(d, c, a.tp)), a.tp, 0, LOCATION_MZONE | LOCATION_GRAVE, 1, NO_EX);
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_REMOVE)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_REMOVE as u64);
    // local g=Duel.SelectTarget(tp,s.rmfilter,tp,0,LOCATION_MZONE|LOCATION_GRAVE,1,1,nil,tp)
    let g = duel::select_target(d, a.tp, Some(&|d, c| rmfilter(d, c, a.tp)), a.tp, 0, LOCATION_MZONE | LOCATION_GRAVE, 1, 1, false, NO_EX);
    // Duel.SetOperationInfo(0,CATEGORY_REMOVE,g,1,0,0)
    duel::set_operation_info(d, 0, CATEGORY_REMOVE, g, 1, 0, 0);
    true
}
/// `s.grop(e,tp,eg,ep,ev,re,r,rp)`
fn grop(d: &mut Duel, a: &EffArgs) {
    // local tc=Duel.GetFirstTarget()
    let tc = duel::first_target(d).nil();
    // if tc:IsRelateToEffect(e) then
    if card::is_relate_to_effect(d, tc, a.e) {
        // Duel.Remove(tc,POS_FACEDOWN,REASON_EFFECT)
        duel::remove(d, tc, POS_FACEDOWN as u8, REASON_EFFECT, None, None);
    }
}
/// `s.dkfilter(c,p)`
fn dkfilter(d: &mut Duel, c: CardId, p: u8) -> bool {
    // return c:IsFacedown() and c:IsControler(p)
    card::is_facedown(d, c) && card::is_controler(d, c, p)
}
/// `function(e) return e:GetHandler():HasFlagEffect(id) end` (condition of e2)
fn grcond(d: &mut Duel, a: &EffArgs) -> bool {
    let h = effect::get_handler(d, a.e).nil();
    card::has_flag_effect(d, h, ID, None)
}
/// `s.dkcon(e,tp,eg,ep,ev,re,r,rp)`
fn dkcon(d: &mut Duel, a: &EffArgs) -> bool {
    // return eg:IsExists(s.dkfilter,1,nil,1-tp)
    let eg = a.eg.clone().nil();
    group::is_exists(d, &eg, &|d, c| dkfilter(d, c, 1 - a.tp), 1, NO_EX)
}
/// `s.dktg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn dktg(d: &mut Duel, a: &EffArgs) -> bool {
    // local ct=Duel.GetMatchingGroupCount(s.dkfilter,tp,0,LOCATION_REMOVED,nil,1-tp)
    let ct = duel::get_matching_group_count(d, Some(&|d, c| dkfilter(d, c, 1 - a.tp)), a.tp, 0, LOCATION_REMOVED, NO_EX);
    // local tg=Duel.GetDecktopGroup(1-tp,ct)
    let tg = duel::get_decktop_group(d, 1 - a.tp, ct as u32);
    // if chk==0 then return ct>0
    //     and tg:FilterCount(Card.IsAbleToRemove,nil,tp,POS_FACEDOWN)==ct end
    if a.chk == Some(0) {
        return ct > 0 && group::filter_count(d, &tg, &|d, c| card::is_able_to_remove(d, c, Some(a.tp), Some(POS_FACEDOWN as u8), None), NO_EX) == ct;
    }
    // Duel.SetOperationInfo(0,CATEGORY_REMOVE,nil,ct,1-tp,LOCATION_DECK)
    duel::set_operation_info(d, 0, CATEGORY_REMOVE, None, ct as u8, 1 - a.tp, LOCATION_DECK as i32);
    true
}
/// `s.dkop(e,tp,eg,ep,ev,re,r,rp)`
fn dkop(d: &mut Duel, a: &EffArgs) {
    // local ct=Duel.GetMatchingGroupCount(s.dkfilter,tp,0,LOCATION_REMOVED,nil,1-tp)
    let ct = duel::get_matching_group_count(d, Some(&|d, c| dkfilter(d, c, 1 - a.tp)), a.tp, 0, LOCATION_REMOVED, NO_EX);
    // if ct==0 then return end
    if ct == 0 {
        return;
    }
    // local tg=Duel.GetDecktopGroup(1-tp,ct)
    let tg = duel::get_decktop_group(d, 1 - a.tp, ct as u32);
    // Duel.DisableShuffleCheck()
    duel::disable_shuffle_check(d, true);
    // Duel.Remove(tg,POS_FACEDOWN,REASON_EFFECT)
    duel::remove(d, tg, POS_FACEDOWN as u8, REASON_EFFECT, None, None);
}
