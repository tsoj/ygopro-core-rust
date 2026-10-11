//! Dinomist Rex
use super::prelude::*;

pub const ID: u32 = 63251695;

pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_DINOMIST], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Pendulum Summon
    // Pendulum.AddProcedure(c)
    pendulum::add_procedure(d, c, None, None);
    // Negate an activated effect that targets another "Dinomist" card(s) you control
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_CONTINUOUS)
    effect::set_type(d, e1, EFFECT_TYPE_FIELD + EFFECT_TYPE_CONTINUOUS);
    // e1:SetCode(EVENT_CHAIN_SOLVING)
    effect::set_code(d, e1, EVENT_CHAIN_SOLVING);
    // e1:SetRange(LOCATION_PZONE)
    effect::set_range(d, e1, LOCATION_PZONE);
    // e1:SetCondition(s.negcon)
    effect::set_condition(d, e1, Some(condition(negcon)));
    // e1:SetOperation(s.negop)
    effect::set_operation(d, e1, Some(operation(negop)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // Activate 1 of these effects
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e2, aux::stringid(ID, 0));
    // e2:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e2, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_O);
    // e2:SetCode(EVENT_DAMAGE_STEP_END)
    effect::set_code(d, e2, EVENT_DAMAGE_STEP_END);
    // e2:SetCondition(s.effcon)
    effect::set_condition(d, e2, Some(condition(effcon)));
    // e2:SetCost(s.effcost)
    effect::set_cost(d, e2, Some(cost_target(effcost)));
    // e2:SetTarget(s.efftg)
    effect::set_target(d, e2, Some(cost_target(efftg)));
    // e2:SetOperation(s.effop)
    effect::set_operation(d, e2, Some(operation(effop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
}
/// `s.negconfilter(c,tp)`
fn negconfilter(d: &mut Duel, c: CardId, tp: u8) -> bool {
    // return c:IsFaceup() and c:IsSetCard(SET_DINOMIST) and c:IsControler(tp) and c:IsOnField()
    card::is_faceup(d, c) && card::is_set_card(d, c, &[SET_DINOMIST as u16]) && card::is_controler(d, c, tp) && card::is_on_field(d, c)
}
/// `s.negcon(e,tp,eg,ep,ev,re,r,rp)`
fn negcon(d: &mut Duel, a: &EffArgs) -> bool {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if not (re:IsHasProperty(EFFECT_FLAG_CARD_TARGET) and not c:HasFlagEffect(id)) then return false end
    let re = a.re.nil();
    if !(effect::is_has_property(d, re, EFFECT_FLAG_CARD_TARGET, 0) && !card::has_flag_effect(d, c, ID, None)) {
        return false;
    }
    // local g=Duel.GetChainInfo(ev,CHAININFO_TARGET_CARDS)
    let g = duel::get_chain_info(d, a.ev as u8, &[CHAININFO_TARGET_CARDS]).first().and_then(|v| v.as_group().cloned());
    // return g and g:IsExists(s.negconfilter,1,c,tp) and Duel.IsChainDisablable(ev)
    match g {
        None => false,
        Some(g) => group::is_exists(d, &g, &|d, x| negconfilter(d, x, a.tp), 1, c.into()) && duel::is_chain_disablable(d, a.ev as u8),
    }
}
/// `s.negop(e,tp,eg,ep,ev,re,r,rp)`
fn negop(d: &mut Duel, a: &EffArgs) {
    // if not Duel.SelectYesNo(tp,aux.Stringid(id,1)) then return end
    if !duel::select_yes_no(d, a.tp, aux::stringid(ID, 1)) {
        return;
    }
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // c:RegisterFlagEffect(id,RESET_EVENT|RESETS_STANDARD,0,1)
    card::register_flag_effect(d, c, ID, RESET_EVENT | RESETS_STANDARD, 0, 1, 0, 0);
    // if Duel.NegateEffect(ev) then
    if duel::negate_effect(d, a.ev as u8) {
        // Duel.BreakEffect()
        duel::break_effect(d);
        // Duel.Destroy(c,REASON_EFFECT)
        duel::destroy(d, CardSet::from_card(c), REASON_EFFECT, None, None);
    }
}
/// `s.effcon(e,tp,eg,ep,ev,re,r,rp)`
fn effcon(d: &mut Duel, a: &EffArgs) -> bool {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // return Duel.GetAttacker()==c and c:IsRelateToBattle()
    duel::get_attacker(d) == Some(c) && card::is_relate_to_battle(d, c)
}
/// `s.effcost(e,tp,eg,ep,ev,re,r,rp,chk)`
fn effcost(d: &mut Duel, a: &EffArgs) -> bool {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if chk==0 then return Duel.CheckReleaseGroupCost(tp,Card.IsSetCard,1,false,nil,c,SET_DINOMIST) end
    if a.chk == Some(0) {
        return duel::check_release_group_cost(d, a.tp, Some(&|d, x| card::is_set_card(d, x, &[SET_DINOMIST as u16])), 1, 1, false, None, c.into());
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_RELEASE)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_RELEASE as u64);
    // local g=Duel.SelectReleaseGroupCost(tp,Card.IsSetCard,1,1,false,nil,c,SET_DINOMIST)
    let g = aux::select_release_group_cost(d, a.tp, Some(&|d, x| card::is_set_card(d, x, &[SET_DINOMIST as u16])), 1, 1, false, None, c.into());
    // Duel.Release(g,REASON_COST)
    duel::release(d, g, REASON_COST, None);
    true
}
/// `s.efftg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn efftg(d: &mut Duel, a: &EffArgs) -> bool {
    // local b1=e:GetHandler():CanChainAttack(0,true)
    let h = effect::get_handler(d, a.e).nil();
    let b1 = card::can_chain_attack(d, h, Some(0), true);
    // local b2=Duel.IsExistingMatchingCard(Card.IsAbleToDeck,tp,0,LOCATION_ONFIELD|LOCATION_HAND,1,nil)
    let b2 = duel::is_existing_matching_card(d, Some(&|d, x| card::is_able_to_deck(d, x)), a.tp, 0, LOCATION_ONFIELD | LOCATION_HAND, 1, NO_EX);
    // if chk==0 then return b1 or b2 end
    if a.chk == Some(0) {
        return b1 || b2;
    }
    // local op=Duel.SelectEffect(tp, {b1,aux.Stringid(id,2)}, {b2,aux.Stringid(id,3)})
    let op = duel::select_effect(d, a.tp, &[(b1, aux::stringid(ID, 2)), (b2, aux::stringid(ID, 3))]).nil() as i64;
    // e:SetLabel(op)
    effect::set_label(d, a.e, &[op]);
    // if op==1 then
    if op == 1 {
        // e:SetCategory(0)
        effect::set_category(d, a.e, 0);
    } else {
        // e:SetCategory(CATEGORY_TODECK)
        effect::set_category(d, a.e, CATEGORY_TODECK);
        // Duel.SetOperationInfo(0,CATEGORY_TODECK,nil,1,1-tp,LOCATION_ONFIELD|LOCATION_HAND)
        duel::set_operation_info(d, 0, CATEGORY_TODECK, None, 1, 1 - a.tp, (LOCATION_ONFIELD | LOCATION_HAND) as i32);
    }
    true
}
/// `s.effop(e,tp,eg,ep,ev,re,r,rp)`
fn effop(d: &mut Duel, a: &EffArgs) {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if e:GetLabel()==1 then
    if effect::get_label(d, a.e) == 1 {
        // if not (c:IsFaceup() and c:IsRelateToEffect(e) and c:IsRelateToBattle()) then return end
        if !(card::is_faceup(d, c) && card::is_relate_to_effect(d, c, a.e) && card::is_relate_to_battle(d, c)) {
            return;
        }
        // Duel.ChainAttack()
        duel::chain_attack(d, None);
        // The consecutive attack must be on a monster
        // local e1=Effect.CreateEffect(c)
        let e1 = effect::create_effect(d, c);
        // e1:SetType(EFFECT_TYPE_SINGLE)
        effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
        // e1:SetProperty(EFFECT_FLAG_CANNOT_DISABLE)
        effect::set_property(d, e1, EFFECT_FLAG_CANNOT_DISABLE, 0);
        // e1:SetCode(EFFECT_CANNOT_DIRECT_ATTACK)
        effect::set_code(d, e1, EFFECT_CANNOT_DIRECT_ATTACK);
        // e1:SetReset(RESET_EVENT|RESETS_STANDARD|RESET_PHASE|PHASE_BATTLE|PHASE_DAMAGE_CAL)
        effect::set_reset(d, e1, RESET_EVENT | RESETS_STANDARD | RESET_PHASE | PHASE_BATTLE | PHASE_DAMAGE_CAL, 1);
        // c:RegisterEffect(e1)
        card::register_effect(d, c, e1, false);
        // Inflicts piercing battle damage
        // local e2=e1:Clone()
        let e2 = effect::clone(d, e1);
        // e2:SetCode(EFFECT_PIERCE)
        effect::set_code(d, e2, EFFECT_PIERCE);
        // c:RegisterEffect(e2)
        card::register_effect(d, c, e2, false);
    } else {
        // local hg=Duel.GetMatchingGroup(Card.IsAbleToDeck,tp,0,LOCATION_HAND,nil)
        let hg = duel::get_matching_group(d, Some(&|d, x| card::is_able_to_deck(d, x)), a.tp, 0, LOCATION_HAND, NO_EX);
        // local fg=Duel.GetMatchingGroup(Card.IsAbleToDeck,tp,0,LOCATION_ONFIELD,nil)
        let fg = duel::get_matching_group(d, Some(&|d, x| card::is_able_to_deck(d, x)), a.tp, 0, LOCATION_ONFIELD, NO_EX);
        // local b1=#hg>0
        let b1 = hg.len() > 0;
        // local b2=#fg>0
        let b2 = fg.len() > 0;
        // if not (b1 or b2) then return end
        if !(b1 || b2) {
            return;
        }
        // local op=Duel.SelectEffect(tp, {b1,aux.Stringid(id,4)}, {b2,aux.Stringid(id,5)})
        let op = duel::select_effect(d, a.tp, &[(b1, aux::stringid(ID, 4)), (b2, aux::stringid(ID, 5))]).nil();
        // local sg=nil
        let sg = if op == 1 {
            // sg=hg:RandomSelect(tp,1)
            group::random_select(d, &hg, a.tp, 1)
        } else {
            // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_TODECK)
            duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_TODECK as u64);
            // sg=fg:Select(tp,1,1,nil)
            let sg = group::select(d, &fg, a.tp, 1, 1, false, NO_EX).nil();
            // Duel.HintSelection(sg,true)
            duel::hint_selection(d, sg.clone(), true);
            sg
        };
        // if Duel.SendtoDeck(sg,nil,SEQ_DECKSHUFFLE,REASON_EFFECT)>0 and sg:GetFirst():IsLocation(LOCATION_DECK|LOCATION_EXTRA)
        //     and c:IsRelateToEffect(e) and c:IsFaceup() then
        let sent = duel::send_to_deck(d, sg.clone(), None, SEQ_DECKSHUFFLE as i32, REASON_EFFECT, None);
        if sent > 0 {
            let first = group::get_first(d, &sg).nil();
            if card::is_location(d, first, LOCATION_DECK | LOCATION_EXTRA) && card::is_relate_to_effect(d, c, a.e) && card::is_faceup(d, c) {
                // Duel.BreakEffect()
                duel::break_effect(d);
                // c:UpdateAttack(100)
                card::update_attack(d, c, 100, None, None, None);
            }
        }
    }
}
