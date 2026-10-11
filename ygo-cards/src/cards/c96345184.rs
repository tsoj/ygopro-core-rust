//! Infinity of the Sacred Beasts - Raviel, Lord of Phantasms
//! 無窮の三幻魔－幻魔皇ラビエル
//! scripted by Naim
use super::prelude::*;

pub const ID: u32 = 96345184;

pub const INFO: ScriptInfo = ScriptInfo {
    listed_names: &[ID],
    listed_series: &[SET_SACRED_BEAST],
    ..ScriptInfo::DEFAULT
};

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // c:EnableReviveLimit()
    card::enable_revive_limit(d, c);
    // Must be Special Summoned with a "Sacred Beast" card
    // local e0=Effect.CreateEffect(c)
    let e0 = effect::create_effect(d, c);
    // e0:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e0, EFFECT_TYPE_SINGLE);
    // e0:SetProperty(EFFECT_FLAG_CANNOT_DISABLE+EFFECT_FLAG_UNCOPYABLE)
    effect::set_property(d, e0, EFFECT_FLAG_CANNOT_DISABLE + EFFECT_FLAG_UNCOPYABLE, 0);
    // e0:SetCode(EFFECT_SPSUMMON_CONDITION)
    effect::set_code(d, e0, EFFECT_SPSUMMON_CONDITION);
    // e0:SetValue(function(e,sum_eff) return sum_eff:GetHandler():IsSetCard(SET_SACRED_BEAST) end)
    effect::set_value_fn(d, e0, lua_fn(sp_value));
    // c:RegisterEffect(e0)
    card::register_effect(d, c, e0, false);
    // You can reveal this card in your hand; add 1 "Sacred Beast" monster from your Deck to your hand, except "Raviel, Lord of Phantasms - Sacred Beast of Endless Eternity", then discard 1 card
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_TOHAND+CATEGORY_SEARCH+CATEGORY_HANDES)
    effect::set_category(d, e1, CATEGORY_TOHAND + CATEGORY_SEARCH + CATEGORY_HANDES);
    // e1:SetType(EFFECT_TYPE_IGNITION)
    effect::set_type(d, e1, EFFECT_TYPE_IGNITION);
    // e1:SetRange(LOCATION_HAND)
    effect::set_range(d, e1, LOCATION_HAND);
    // e1:SetCountLimit(1,id)
    effect::set_count_limit(d, e1, 1, ID, 0, 0);
    // e1:SetCost(Cost.SelfReveal)
    effect::set_cost(d, e1, Some(cost::self_reveal()));
    // e1:SetTarget(s.thtg)
    effect::set_target(d, e1, Some(cost_target(thtg)));
    // e1:SetOperation(s.thop)
    effect::set_operation(d, e1, Some(operation(thop)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // Once per turn (Quick Effect): You can Tribute 2 other "Sacred Beast" monsters; destroy as many monsters your opponent controls as possible, and if you do, this card gains 1000 ATK for each card destroyed by this effect
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e2, aux::stringid(ID, 1));
    // e2:SetCategory(CATEGORY_DESTROY+CATEGORY_ATKCHANGE)
    effect::set_category(d, e2, CATEGORY_DESTROY + CATEGORY_ATKCHANGE);
    // e2:SetType(EFFECT_TYPE_QUICK_O)
    effect::set_type(d, e2, EFFECT_TYPE_QUICK_O);
    // e2:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e2, EVENT_FREE_CHAIN);
    // e2:SetRange(LOCATION_MZONE)
    effect::set_range(d, e2, LOCATION_MZONE);
    // e2:SetCountLimit(1)
    effect::set_count_limit(d, e2, 1, 0, 0, 0);
    // e2:SetCost(s.descost)
    effect::set_cost(d, e2, Some(cost_target(descost)));
    // e2:SetTarget(s.destg)
    effect::set_target(d, e2, Some(cost_target(destg)));
    // e2:SetOperation(s.desop)
    effect::set_operation(d, e2, Some(operation(desop)));
    // e2:SetHintTiming(0,TIMING_STANDBY_PHASE|TIMING_MAIN_END|TIMINGS_CHECK_MONSTER_E)
    effect::set_hint_timing(d, e2, 0, Some(TIMING_STANDBY_PHASE | TIMING_MAIN_END | TIMINGS_CHECK_MONSTER_E));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
}
// function s.thfilter(c)
fn thfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsSetCard(SET_SACRED_BEAST) and c:IsMonster() and c:IsAbleToHand() and not c:IsCode(id)
    card::is_set_card(d, c, &[SET_SACRED_BEAST as u16])
        && card::is_monster(d, c)
        && card::is_able_to_hand(d, c, None)
        && !card::is_code(d, c, &[ID])
}
// function s.thtg(e,tp,eg,ep,ev,re,r,rp,chk)
fn thtg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return Duel.IsExistingMatchingCard(s.thfilter,tp,LOCATION_DECK,0,1,nil) end
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, Some(&thfilter), a.tp, LOCATION_DECK, 0, 1, NO_EX);
    }
    // Duel.SetOperationInfo(0,CATEGORY_TOHAND,nil,1,tp,LOCATION_DECK)
    duel::set_operation_info(d, 0, CATEGORY_TOHAND, None, 1, a.tp, LOCATION_DECK as i32);
    // Duel.SetOperationInfo(0,CATEGORY_HANDES,nil,0,tp,1)
    duel::set_operation_info(d, 0, CATEGORY_HANDES, None, 0, a.tp, 1);
    true
}
// function s.thop(e,tp,eg,ep,ev,re,r,rp)
fn thop(d: &mut Duel, a: &EffArgs) {
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_ATOHAND)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_ATOHAND as u64);
    // local g=Duel.SelectMatchingCard(tp,s.thfilter,tp,LOCATION_DECK,0,1,1,nil)
    let g = duel::select_matching_card(d, a.tp, Some(&thfilter), a.tp, LOCATION_DECK, 0, 1, 1, false, NO_EX).nil();
    // if #g>0 and Duel.SendtoHand(g,nil,REASON_EFFECT)>0 then
    if !g.is_empty() && duel::send_to_hand(d, g.clone(), None, REASON_EFFECT, None) > 0 {
        // Duel.ConfirmCards(1-tp,g)
        duel::confirm_cards(d, 1 - a.tp, g);
        // Duel.ShuffleHand(tp)
        duel::shuffle_hand(d, a.tp);
        // Duel.BreakEffect()
        duel::break_effect(d);
        // Duel.DiscardHand(tp,nil,1,1,REASON_EFFECT|REASON_DISCARD,nil)
        duel::discard_hand(d, a.tp, None, 1, 1, REASON_EFFECT | REASON_DISCARD, NO_EX);
    }
}
// function s.descostfilter(c)
fn descostfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsSetCard(SET_SACRED_BEAST)
    card::is_set_card(d, c, &[SET_SACRED_BEAST as u16])
}
// function s.descostchk(sg,tp,exg)
fn descostchk(d: &mut Duel, sg: &CardSet, tp: u8) -> bool {
    // return Duel.IsExistingMatchingCard(nil,tp,0,LOCATION_MZONE,1,sg)
    duel::is_existing_matching_card(d, None, tp, 0, LOCATION_MZONE, 1, sg.into())
}
// function s.descost(e,tp,eg,ep,ev,re,r,rp,chk)
fn descost(d: &mut Duel, a: &EffArgs) -> bool {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if chk==0 then return Duel.CheckReleaseGroupCost(tp,s.descostfilter,2,false,s.descostchk,c) end
    if a.chk == Some(0) {
        return duel::check_release_group_cost(
            d,
            a.tp,
            Some(&descostfilter),
            2,
            2,
            false,
            Some(&|d, sg, tp, _exg| (descostchk(d, sg, tp), false)),
            c.into(),
        );
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_RELEASE)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_RELEASE as u64);
    // local g=Duel.SelectReleaseGroupCost(tp,s.descostfilter,2,2,false,s.descostchk,c)
    let g = duel::select_release_group_cost(
        d,
        a.tp,
        Some(&descostfilter),
        2,
        2,
        false,
        Some(&|d, sg, tp, _exg| (descostchk(d, sg, tp), false)),
        c.into(),
    );
    // Duel.Release(g,REASON_COST)
    duel::release(d, g, REASON_COST, None);
    true
}
// function s.destg(e,tp,eg,ep,ev,re,r,rp,chk)
fn destg(d: &mut Duel, a: &EffArgs) -> bool {
    // local g=Duel.GetFieldGroup(tp,0,LOCATION_MZONE)
    let g = duel::get_field_group(d, a.tp, 0, LOCATION_MZONE);
    // if chk==0 then return #g>0 end
    if a.chk == Some(0) {
        return g.len() > 0;
    }
    // Duel.SetOperationInfo(0,CATEGORY_DESTROY,g,#g,tp,0)
    let n = g.len() as u8;
    duel::set_operation_info(d, 0, CATEGORY_DESTROY, Some(g), n, a.tp, 0);
    // Duel.SetOperationInfo(0,CATEGORY_ATKCHANGE,e:GetHandler(),0,tp,#g*1000)
    let h = effect::get_handler(d, a.e).nil();
    duel::set_operation_info(d, 0, CATEGORY_ATKCHANGE, Some(CardSet::from_card(h)), 0, a.tp, n as i32 * 1000);
    true
}
// function s.desop(e,tp,eg,ep,ev,re,r,rp)
fn desop(d: &mut Duel, a: &EffArgs) {
    // local g=Duel.GetFieldGroup(tp,0,LOCATION_MZONE)
    let g = duel::get_field_group(d, a.tp, 0, LOCATION_MZONE);
    // if #g==0 then return end
    if g.is_empty() {
        return;
    }
    // local ct=Duel.Destroy(g,REASON_EFFECT)
    let ct = duel::destroy(d, g, REASON_EFFECT, None, None);
    // if ct==0 then return end
    if ct == 0 {
        return;
    }
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if c:IsRelateToEffect(e) and c:IsFaceup() then
    if card::is_relate_to_effect(d, c, a.e) && card::is_faceup(d, c) {
        // --This card gains 1000 ATK for each card destroyed by this effect
        // local e1=Effect.CreateEffect(c)
        let e1 = effect::create_effect(d, c);
        // e1:SetType(EFFECT_TYPE_SINGLE)
        effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
        // e1:SetCode(EFFECT_UPDATE_ATTACK)
        effect::set_code(d, e1, EFFECT_UPDATE_ATTACK);
        // e1:SetValue(ct*1000)
        effect::set_value(d, e1, ct as i64 * 1000);
        // e1:SetReset(RESET_EVENT|RESETS_STANDARD_DISABLE)
        effect::set_reset(d, e1, RESET_EVENT | RESETS_STANDARD_DISABLE, 1);
        // c:RegisterEffect(e1)
        card::register_effect(d, c, e1, false);
    }
}
// Value function: function(e,sum_eff) return sum_eff:GetHandler():IsSetCard(SET_SACRED_BEAST) end
fn sp_value(d: &mut Duel, a: Args) -> bool {
    let sum_eff = a.e(1);
    let h = effect::get_handler(d, sum_eff).nil();
    card::is_set_card(d, h, &[SET_SACRED_BEAST as u16])
}
