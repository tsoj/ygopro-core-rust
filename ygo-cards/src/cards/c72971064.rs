//! Lyrilusc - Ensemblue Robin
//! Scripted by The Razgriz
use super::prelude::*;

pub const ID: u32 = 72971064;

pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_LYRILUSC], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // c:EnableReviveLimit()
    card::enable_revive_limit(d, c);
    // Xyz Procedure: 2+ Level 1 monsters
    // Xyz.AddProcedure(c,nil,1,2,nil,nil,Xyz.InfiniteMats)
    xyz::add_procedure(d, c, None, Some(1), 2, None, None, Some(xyz::INFINITE_MATS), None, false, None);
    // This card gains 500 ATK for each material attached to it
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
    // e1:SetCode(EFFECT_UPDATE_ATTACK)
    effect::set_code(d, e1, EFFECT_UPDATE_ATTACK);
    // e1:SetProperty(EFFECT_FLAG_SINGLE_RANGE)
    effect::set_property(d, e1, EFFECT_FLAG_SINGLE_RANGE, 0);
    // e1:SetRange(LOCATION_MZONE)
    effect::set_range(d, e1, LOCATION_MZONE);
    // e1:SetValue(function(e,c) return c:GetOverlayCount()*500 end)
    effect::set_value_fn(d, e1, effect_card_value(|d, _e, c| card::get_overlay_count(d, c) as i64 * 500));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // Return 1 of your opponent's Special Summoned monsters to your opponent's hand
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e2, aux::stringid(ID, 0));
    // e2:SetCategory(CATEGORY_TOHAND)
    effect::set_category(d, e2, CATEGORY_TOHAND);
    // e2:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e2, EFFECT_TYPE_FIELD + EFFECT_TYPE_TRIGGER_O);
    // e2:SetCode(EVENT_SPSUMMON_SUCCESS)
    effect::set_code(d, e2, EVENT_SPSUMMON_SUCCESS);
    // e2:SetProperty(EFFECT_FLAG_CARD_TARGET+EFFECT_FLAG_DELAY)
    effect::set_property(d, e2, EFFECT_FLAG_CARD_TARGET + EFFECT_FLAG_DELAY, 0);
    // e2:SetRange(LOCATION_MZONE)
    effect::set_range(d, e2, LOCATION_MZONE);
    // e2:SetCondition(s.retopthcon)
    effect::set_condition(d, e2, Some(condition(retopthcon)));
    // e2:SetCost(Cost.DetachFromSelf(1,1,nil))
    effect::set_cost(d, e2, Some(cost::detach_from_self(cost::Count::N(1), Some(cost::Count::N(1)), None)));
    // e2:SetTarget(s.retopthtg)
    effect::set_target(d, e2, Some(cost_target(retopthtg)));
    // e2:SetOperation(s.retopthop)
    effect::set_operation(d, e2, Some(operation(retopthop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
    // Add 1 "Lyrilusc" monster from your GY to your hand, except this card
    // local e3=Effect.CreateEffect(c)
    let e3 = effect::create_effect(d, c);
    // e3:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e3, aux::stringid(ID, 1));
    // e3:SetCategory(CATEGORY_TOHAND+CATEGORY_LEAVE_GRAVE)
    effect::set_category(d, e3, CATEGORY_TOHAND + CATEGORY_LEAVE_GRAVE);
    // e3:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e3, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_O);
    // e3:SetCode(EVENT_TO_GRAVE)
    effect::set_code(d, e3, EVENT_TO_GRAVE);
    // e3:SetProperty(EFFECT_FLAG_CARD_TARGET+EFFECT_FLAG_DELAY+EFFECT_FLAG_DAMAGE_STEP)
    effect::set_property(d, e3, EFFECT_FLAG_CARD_TARGET + EFFECT_FLAG_DELAY + EFFECT_FLAG_DAMAGE_STEP, 0);
    // e3:SetCondition(s.llthcon)
    effect::set_condition(d, e3, Some(condition(llthcon)));
    // e3:SetTarget(s.llthtg)
    effect::set_target(d, e3, Some(cost_target(llthtg)));
    // e3:SetOperation(s.llthop)
    effect::set_operation(d, e3, Some(operation(llthop)));
    // c:RegisterEffect(e3)
    card::register_effect(d, c, e3, false);
}
/// `s.retopthfilter(c,e,tp)`
fn retopthfilter(d: &mut Duel, c: CardId, e: EffectId, tp: u8) -> bool {
    // return c:IsSummonPlayer(1-tp) and c:IsAbleToHand() and c:IsCanBeEffectTarget(e) and c:IsLocation(LOCATION_MZONE)
    card::is_summon_player(d, c, 1 - tp) && card::is_able_to_hand(d, c, None) && card::is_can_be_effect_target(d, c, Some(e)) && card::is_location(d, c, LOCATION_MZONE)
}
/// `s.retopthcon(e,tp,eg,ep,ev,re,r,rp)`
fn retopthcon(d: &mut Duel, a: &EffArgs) -> bool {
    // return eg:IsExists(s.retopthfilter,1,nil,e,tp)
    let eg = a.eg.clone().nil();
    group::is_exists(d, &eg, &|d, c| retopthfilter(d, c, a.e, a.tp), 1, NO_EX)
}
/// `s.retopthtg(e,tp,eg,ep,ev,re,r,rp,chk,chkc)`
fn retopthtg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chkc then return eg:IsContains(chkc) and s.retopthfilter(chkc,e,tp) end
    if let Some(chkc) = a.chkc() {
        let eg = a.eg.clone().nil();
        return group::is_contains(&eg, chkc) && retopthfilter(d, chkc, a.e, a.tp);
    }
    // if chk==0 then return eg:IsExists(s.retopthfilter,1,nil,e,tp) end
    if a.chk == Some(0) {
        let eg = a.eg.clone().nil();
        return group::is_exists(d, &eg, &|d, c| retopthfilter(d, c, a.e, a.tp), 1, NO_EX);
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_RTOHAND)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_RTOHAND as u64);
    // local tc=nil
    // if #eg==1 then
    let eg = a.eg.clone().nil();
    let tc: CardSet = if eg.len() == 1 {
        // tc=eg:GetFirst()
        let tc = group::get_first(d, &eg).nil();
        // Duel.SetTargetCard(tc)
        duel::set_target_card(d, tc);
        CardSet::from_card(tc)
    } else {
        // tc=eg:FilterSelect(tp,s.retopthfilter,1,1,nil,e,tp)
        let tc = group::filter_select(d, &eg, a.tp, &|d, c| retopthfilter(d, c, a.e, a.tp), 1, 1, false, NO_EX).nil();
        // Duel.SetTargetCard(tc)
        duel::set_target_card(d, tc.clone());
        tc
    };
    // Duel.SetOperationInfo(0,CATEGORY_TOHAND,tc,1,0,0)
    duel::set_operation_info(d, 0, CATEGORY_TOHAND, Some(tc), 1, 0, 0);
    true
}
/// `s.retopthop(e,tp,eg,ep,ev,re,r,rp)`
fn retopthop(d: &mut Duel, a: &EffArgs) {
    // local tc=Duel.GetFirstTarget()
    // if tc and tc:IsRelateToEffect(e) then
    if let Some(tc) = duel::first_target(d) {
        if card::is_relate_to_effect(d, tc, a.e) {
            // Duel.SendtoHand(tc,nil,REASON_EFFECT)
            duel::send_to_hand(d, tc, None, REASON_EFFECT, None);
        }
    }
}
/// `s.llthcon(e,tp,eg,ep,ev,re,r,rp)`
fn llthcon(d: &mut Duel, a: &EffArgs) -> bool {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // return rp==1-tp and c:GetPreviousControler()==c:GetOwner()
    a.rp == 1 - a.tp && card::get_previous_controler(d, c) == card::get_owner(d, c)
}
/// `s.llthfilter(c)`
fn llthfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsSetCard(SET_LYRILUSC) and c:IsMonster() and c:IsAbleToHand()
    card::is_set_card(d, c, &[SET_LYRILUSC as u16]) && card::is_monster(d, c) && card::is_able_to_hand(d, c, None)
}
/// `s.llthtg(e,tp,eg,ep,ev,re,r,rp,chk,chkc)`
fn llthtg(d: &mut Duel, a: &EffArgs) -> bool {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if chkc then return chkc:IsControler(tp) and chkc:IsLocation(LOCATION_GRAVE) and s.llthfilter(chkc) and chkc~=c end
    if let Some(chkc) = a.chkc() {
        return card::is_controler(d, chkc, a.tp) && card::is_location(d, chkc, LOCATION_GRAVE) && llthfilter(d, chkc) && chkc != c;
    }
    // if chk==0 then return Duel.IsExistingTarget(s.llthfilter,tp,LOCATION_GRAVE,0,1,c) end
    if a.chk == Some(0) {
        return duel::is_existing_target(d, Some(&llthfilter), a.tp, LOCATION_GRAVE, 0, 1, c.into());
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_RTOHAND)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_RTOHAND as u64);
    // local tc=Duel.SelectTarget(tp,s.llthfilter,tp,LOCATION_GRAVE,0,1,1,c)
    let tc = duel::select_target(d, a.tp, Some(&llthfilter), a.tp, LOCATION_GRAVE, 0, 1, 1, false, c.into()).nil();
    // Duel.SetOperationInfo(0,CATEGORY_TOHAND,tc,1,0,0)
    duel::set_operation_info(d, 0, CATEGORY_TOHAND, Some(tc), 1, 0, 0);
    true
}
/// `s.llthop(e,tp,eg,ep,ev,re,r,rp)`
fn llthop(d: &mut Duel, a: &EffArgs) {
    // local tc=Duel.GetFirstTarget()
    let tc = duel::first_target(d).nil();
    // if tc:IsRelateToEffect(e) then
    if card::is_relate_to_effect(d, tc, a.e) {
        // Duel.SendtoHand(tc,tp,REASON_EFFECT)
        duel::send_to_hand(d, tc, Some(a.tp), REASON_EFFECT, None);
        // Duel.ConfirmCards(1-tp,tc)
        duel::confirm_cards(d, 1 - a.tp, tc);
    }
}
