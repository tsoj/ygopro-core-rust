//! Sinful Spoils of Betrayal - Silvera
use super::prelude::*;

pub const ID: u32 = 38511382;

pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_DIABELLSTAR, SET_SINFUL_SPOILS], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Negate the effects of 1 face-up card on the field
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_DISABLE)
    effect::set_category(d, e1, CATEGORY_DISABLE);
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    // e1:SetProperty(EFFECT_FLAG_CARD_TARGET)
    effect::set_property(d, e1, EFFECT_FLAG_CARD_TARGET, 0);
    // e1:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    // e1:SetHintTiming(0,TIMINGS_CHECK_MONSTER_E)
    effect::set_hint_timing(d, e1, 0, Some(TIMINGS_CHECK_MONSTER_E));
    // e1:SetCountLimit(1,id)
    effect::set_count_limit(d, e1, 1, ID, 0, 0);
    // e1:SetCost(s.discost)
    effect::set_cost(d, e1, Some(cost_target(discost)));
    // e1:SetTarget(s.distg)
    effect::set_target(d, e1, Some(cost_target(distg)));
    // e1:SetOperation(s.disop)
    effect::set_operation(d, e1, Some(operation(disop)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // Negate the opponent's effect
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e2, aux::stringid(ID, 1));
    // e2:SetCategory(CATEGORY_DISABLE)
    effect::set_category(d, e2, CATEGORY_DISABLE);
    // e2:SetType(EFFECT_TYPE_QUICK_O)
    effect::set_type(d, e2, EFFECT_TYPE_QUICK_O);
    // e2:SetCode(EVENT_CHAINING)
    effect::set_code(d, e2, EVENT_CHAINING);
    // e2:SetRange(LOCATION_GRAVE)
    effect::set_range(d, e2, LOCATION_GRAVE);
    // e2:SetCountLimit(1,id)
    effect::set_count_limit(d, e2, 1, ID, 0, 0);
    // e2:SetCondition(s.gydiscon)
    effect::set_condition(d, e2, Some(condition(gydiscon)));
    // e2:SetCost(Cost.SelfBanish)
    effect::set_cost(d, e2, Some(cost::self_banish()));
    // e2:SetTarget(s.gydistg)
    effect::set_target(d, e2, Some(cost_target(gydistg)));
    // e2:SetOperation(function(e,tp,eg,ep,ev,re,r,rp) Duel.NegateEffect(ev) end)
    effect::set_operation(d, e2, Some(operation(gydisop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
}
// function s.tgfilter(c,cc)
fn tgfilter(d: &mut Duel, c: CardId, cc: CardId) -> bool {
    // return c:IsMonster() and c:IsSetCard(SET_DIABELLSTAR) and (c:IsLocation(LOCATION_HAND) or c:IsFaceup()) and c:IsAbleToGraveAsCost()
    //     and Duel.IsExistingTarget(Card.IsNegatable,0,LOCATION_ONFIELD,LOCATION_ONFIELD,1,Group.FromCards(c,cc)+c:GetEquipGroup())
    if !(card::is_monster(d, c)
        && card::is_set_card(d, c, &[SET_DIABELLSTAR as u16])
        && (card::is_location(d, c, LOCATION_HAND) || card::is_faceup(d, c))
        && card::is_able_to_grave_as_cost(d, c))
    {
        return false;
    }
    // Group.FromCards(c,cc)+c:GetEquipGroup()
    let mut ex = CardSet::from_card(c);
    group::add_card(&mut ex, &CardSet::from_card(cc));
    let eq = card::get_equip_group(d, c);
    group::add_card(&mut ex, &eq);
    duel::is_existing_target(d, Some(&|d, x| card::is_negatable(d, x)), 0, LOCATION_ONFIELD, LOCATION_ONFIELD, 1, (&ex).into())
}
// function s.discost(e,tp,eg,ep,ev,re,r,rp,chk)
fn discost(d: &mut Duel, a: &EffArgs) -> bool {
    let (e, tp) = (a.e, a.tp);
    // local c=e:GetHandler()
    let c = effect::get_handler(d, e).nil();
    // if chk==0 then return Duel.IsExistingMatchingCard(s.tgfilter,tp,LOCATION_HAND|LOCATION_MZONE,0,1,nil,c) end
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, Some(&|d, x| tgfilter(d, x, c)), tp, LOCATION_HAND | LOCATION_MZONE, 0, 1, NO_EX);
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_TOGRAVE)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_TOGRAVE as u64);
    // local g=Duel.SelectMatchingCard(tp,s.tgfilter,tp,LOCATION_HAND|LOCATION_MZONE,0,1,1,nil,c)
    let g = duel::select_matching_card(d, tp, Some(&|d, x| tgfilter(d, x, c)), tp, LOCATION_HAND | LOCATION_MZONE, 0, 1, 1, false, NO_EX).nil();
    // Duel.SendtoGrave(g,REASON_COST)
    duel::send_to_grave(d, g, REASON_COST, None, None);
    true
}
// function s.distg(e,tp,eg,ep,ev,re,r,rp,chk,chkc)
fn distg(d: &mut Duel, a: &EffArgs) -> bool {
    let (e, tp) = (a.e, a.tp);
    // local c=e:GetHandler()
    let c = effect::get_handler(d, e).nil();
    // if chkc then return chkc:IsOnField() and chkc:IsNegatable() and chkc~=c end
    if let Some(chkc) = a.chkc() {
        return card::is_on_field(d, chkc) && card::is_negatable(d, chkc) && chkc != c;
    }
    // if chk==0 then return Duel.IsExistingTarget(Card.IsNegatable,tp,LOCATION_ONFIELD,LOCATION_ONFIELD,1,c) end
    if a.chk == Some(0) {
        return duel::is_existing_target(d, Some(&|d, x| card::is_negatable(d, x)), tp, LOCATION_ONFIELD, LOCATION_ONFIELD, 1, c.into());
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_NEGATE)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_NEGATE as u64);
    // local g=Duel.SelectTarget(tp,Card.IsNegatable,tp,LOCATION_ONFIELD,LOCATION_ONFIELD,1,1,c)
    let g = duel::select_target(d, tp, Some(&|d, x| card::is_negatable(d, x)), tp, LOCATION_ONFIELD, LOCATION_ONFIELD, 1, 1, false, c.into());
    // Duel.SetOperationInfo(0,CATEGORY_DISABLE,g,1,0,0)
    duel::set_operation_info(d, 0, CATEGORY_DISABLE, g, 1, 0, 0);
    true
}
// function s.disop(e,tp,eg,ep,ev,re,r,rp,chk)
fn disop(d: &mut Duel, a: &EffArgs) {
    // local tc=Duel.GetFirstTarget()
    let tc = duel::first_target(d).nil();
    // if tc:IsNegatable() and tc:IsRelateToEffect(e) then
    if card::is_negatable(d, tc) && card::is_relate_to_effect(d, tc, a.e) {
        // tc:NegateEffects(e:GetHandler(),nil,true)
        let c = effect::get_handler(d, a.e).nil();
        card::negate_effects(d, tc, c, None, true, None);
    }
}
// function s.gydiscon(e,tp,eg,ep,ev,re,r,rp)
fn gydiscon(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // local ch=ev-1
    let ch = a.ev as i64 - 1;
    // if ch==0 or not (ep==1-tp and Duel.IsChainDisablable(ev)) or re:GetHandler():IsDisabled() then return false end
    if ch == 0 || !(a.ep == 1 - tp && duel::is_chain_disablable(d, a.ev as u8)) {
        return false;
    }
    let rh = effect::get_handler(d, a.re.nil()).nil();
    if card::is_disabled(d, rh) {
        return false;
    }
    // local ch_player,ch_eff=Duel.GetChainInfo(ch,CHAININFO_TRIGGERING_PLAYER,CHAININFO_TRIGGERING_EFFECT)
    let info = duel::get_chain_info(d, ch as u8, &[CHAININFO_TRIGGERING_PLAYER, CHAININFO_TRIGGERING_EFFECT]);
    let ch_player = info[0].int();
    let ch_eff = info[1].as_effect().nil();
    // local ch_c=ch_eff:GetHandler()
    let ch_c = effect::get_handler(d, ch_eff).nil();
    // return ch_player==tp and ((ch_c:IsSetCard(SET_DIABELLSTAR) and ch_eff:IsMonsterEffect())
    //     or (ch_c:IsSetCard(SET_SINFUL_SPOILS) and ch_eff:IsSpellTrapEffect()))
    ch_player == tp as i64
        && ((card::is_set_card(d, ch_c, &[SET_DIABELLSTAR as u16]) && effect::is_monster_effect(d, ch_eff))
            || (card::is_set_card(d, ch_c, &[SET_SINFUL_SPOILS as u16]) && effect::is_spell_trap_effect(d, ch_eff)))
}
// function s.gydistg(e,tp,eg,ep,ev,re,r,rp,chk)
fn gydistg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return true end
    if a.chk == Some(0) {
        return true;
    }
    // Duel.SetOperationInfo(0,CATEGORY_DISABLE,eg,1,0,0)
    duel::set_operation_info(d, 0, CATEGORY_DISABLE, a.eg.clone(), 1, 0, 0);
    true
}
// function(e,tp,eg,ep,ev,re,r,rp) Duel.NegateEffect(ev) end
fn gydisop(d: &mut Duel, a: &EffArgs) {
    duel::negate_effect(d, a.ev as u8);
}
