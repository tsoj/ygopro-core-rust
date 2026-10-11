//! Number C79: Battlin' Boxer General Kaiser
use super::prelude::*;

pub const ID: u32 = 10300821;
pub const INFO: ScriptInfo = ScriptInfo { xyz_number: Some(79), listed_names: &[71921856], listed_series: &[SET_BATTLIN_BOXER], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // c:EnableReviveLimit()
    card::enable_revive_limit(d, c);
    // Xyz Summon procedure: 3 Level 5 monsters
    // Xyz.AddProcedure(c,nil,5,3)
    xyz::add_procedure(d, c, None, Some(5), 3, None, None, None, None, false, None);
    // Gains 200 ATK for each material attached to it
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
    // e1:SetProperty(EFFECT_FLAG_SINGLE_RANGE)
    effect::set_property(d, e1, EFFECT_FLAG_SINGLE_RANGE, 0);
    // e1:SetCode(EFFECT_UPDATE_ATTACK)
    effect::set_code(d, e1, EFFECT_UPDATE_ATTACK);
    // e1:SetRange(LOCATION_MZONE)
    effect::set_range(d, e1, LOCATION_MZONE);
    // e1:SetValue(function(e,c) return c:GetOverlayCount()*200 end)
    effect::set_value_fn(d, e1, effect_card_value(|d, _e, c| card::get_overlay_count(d, c) as i64 * 200));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // Detach 2 materials from this card, and if you do, negate your opponent's Special Summon, and if you do that, destroy that monster(s)
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e2, aux::stringid(ID, 0));
    // e2:SetCategory(CATEGORY_DISABLE_SUMMON+CATEGORY_DESTROY)
    effect::set_category(d, e2, CATEGORY_DISABLE_SUMMON + CATEGORY_DESTROY);
    // e2:SetType(EFFECT_TYPE_QUICK_O)
    effect::set_type(d, e2, EFFECT_TYPE_QUICK_O);
    // e2:SetCode(EVENT_SPSUMMON)
    effect::set_code(d, e2, EVENT_SPSUMMON);
    // e2:SetRange(LOCATION_MZONE)
    effect::set_range(d, e2, LOCATION_MZONE);
    // e2:SetCountLimit(1)
    effect::set_count_limit(d, e2, 1, 0, 0, 0);
    // e2:SetCondition(function(e,tp,eg,ep,ev,re,r,rp) return tp==1-ep and Duel.GetCurrentChain(true)==0 end)
    effect::set_condition(d, e2, Some(condition(negsum_cond)));
    // e2:SetTarget(s.negsumtg)
    effect::set_target(d, e2, Some(cost_target(negsumtg)));
    // e2:SetOperation(s.negsumop)
    effect::set_operation(d, e2, Some(operation(negsumop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
    // Send 1 "Battlin' Boxer" monster from your hand or Deck to the GY, and if you do, attach that opponent's battling monster to this card as material
    // local e3=Effect.CreateEffect(c)
    let e3 = effect::create_effect(d, c);
    // e3:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e3, aux::stringid(ID, 1));
    // e3:SetCategory(CATEGORY_TOGRAVE)
    effect::set_category(d, e3, CATEGORY_TOGRAVE);
    // e3:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e3, EFFECT_TYPE_FIELD + EFFECT_TYPE_TRIGGER_O);
    // e3:SetCode(EVENT_ATTACK_ANNOUNCE)
    effect::set_code(d, e3, EVENT_ATTACK_ANNOUNCE);
    // e3:SetRange(LOCATION_MZONE)
    effect::set_range(d, e3, LOCATION_MZONE);
    // e3:SetCountLimit(1)
    effect::set_count_limit(d, e3, 1, 0, 0, 0);
    // e3:SetCondition(s.tgattachcon)
    effect::set_condition(d, e3, Some(condition(tgattachcon)));
    // e3:SetTarget(s.tgattachtg)
    effect::set_target(d, e3, Some(cost_target(tgattachtg)));
    // e3:SetOperation(s.tgattachop)
    effect::set_operation(d, e3, Some(operation(tgattachop)));
    // c:RegisterEffect(e3)
    card::register_effect(d, c, e3, false);
}
/// `function(e,tp,eg,ep,ev,re,r,rp) return tp==1-ep and Duel.GetCurrentChain(true)==0 end`
fn negsum_cond(d: &mut Duel, a: &EffArgs) -> bool {
    a.tp as i64 == 1 - a.ep as i64 && duel::get_current_chain(d, true) == 0
}
/// `s.negsumtg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn negsumtg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // if chk==0 then return e:GetHandler():CheckRemoveOverlayCard(tp,2,REASON_EFFECT) end
    let h = effect::get_handler(d, a.e).nil();
    if a.chk == Some(0) {
        return card::check_remove_overlay_card(d, h, tp, 2, REASON_EFFECT);
    }
    // Duel.SetOperationInfo(0,CATEGORY_DISABLE_SUMMON,eg,#eg,tp,0)
    let eg = a.eg();
    let n = eg.len() as u8;
    duel::set_operation_info(d, 0, CATEGORY_DISABLE_SUMMON, Some(eg.clone()), n, tp, 0);
    // Duel.SetOperationInfo(0,CATEGORY_DESTROY,eg,#eg,tp,0)
    duel::set_operation_info(d, 0, CATEGORY_DESTROY, Some(eg), n, tp, 0);
    true
}
/// `s.negsumop(e,tp,eg,ep,ev,re,r,rp)`
fn negsumop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if c:IsRelateToEffect(e) and c:RemoveOverlayCard(tp,2,2,REASON_EFFECT)==2 then
    if card::is_relate_to_effect(d, c, a.e) && card::remove_overlay_card(d, c, tp, 2, 2, REASON_EFFECT) == 2 {
        // Duel.NegateSummon(eg)
        duel::negate_summon(d, a.eg());
        // Duel.Destroy(eg,REASON_EFFECT)
        duel::destroy(d, a.eg(), REASON_EFFECT, None, None);
    }
}
/// `s.tgattachcon(e,tp,eg,ep,ev,re,r,rp)`
fn tgattachcon(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // if not e:GetHandler():GetOverlayGroup():IsExists(Card.IsCode,1,nil,71921856) then return false end
    let h = effect::get_handler(d, a.e).nil();
    let og = card::get_overlay_group(d, h);
    if !group::is_exists(d, &og, &|d, c| card::is_code(d, c, &[71921856]), 1, NO_EX) {
        return false;
    }
    // local a,b=Duel.GetBattleMonster(tp)
    // return a and b and a:IsFaceup() and a:IsSetCard(SET_BATTLIN_BOXER)
    let (a1, b1) = duel::get_battle_monster(d, tp);
    let (Some(a1), Some(_b1)) = (a1, b1) else { return false };
    card::is_faceup(d, a1) && card::is_set_card(d, a1, &[SET_BATTLIN_BOXER as u16])
}
/// `s.tgfilter(c)`
fn tgfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsSetCard(SET_BATTLIN_BOXER) and c:IsMonster() and c:IsAbleToGrave()
    card::is_set_card(d, c, &[SET_BATTLIN_BOXER as u16]) && card::is_monster(d, c) && card::is_able_to_grave(d, c, None)
}
/// `s.tgattachtg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn tgattachtg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // local _,b=Duel.GetBattleMonster(tp)
    let (_, b) = duel::get_battle_monster(d, tp);
    let b = b.nil();
    // if chk==0 then return b:IsAbleToChangeControler() and b:IsCanBeXyzMaterial(e:GetHandler(),tp,REASON_EFFECT)
    //     and Duel.IsExistingMatchingCard(s.tgfilter,tp,LOCATION_HAND|LOCATION_DECK,0,1,nil) end
    if a.chk == Some(0) {
        let h = effect::get_handler(d, a.e).nil();
        return card::is_able_to_change_controler(d, b)
            && card::is_can_be_xyz_material(d, b, Some(h), Some(tp), Some(REASON_EFFECT))
            && duel::is_existing_matching_card(d, Some(&tgfilter), tp, LOCATION_HAND | LOCATION_DECK, 0, 1, NO_EX);
    }
    // Duel.SetTargetCard(b)
    duel::set_target_card(d, b);
    // Duel.SetOperationInfo(0,CATEGORY_TOGRAVE,nil,1,tp,LOCATION_HAND|LOCATION_DECK)
    duel::set_operation_info(d, 0, CATEGORY_TOGRAVE, None, 1, tp, (LOCATION_HAND | LOCATION_DECK) as i32);
    true
}
/// `s.tgattachop(e,tp,eg,ep,ev,re,r,rp)`
fn tgattachop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_TOGRAVE)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_TOGRAVE as u64);
    // local g=Duel.SelectMatchingCard(tp,s.tgfilter,tp,LOCATION_HAND|LOCATION_DECK,0,1,1,nil)
    let g = duel::select_matching_card(d, tp, Some(&tgfilter), tp, LOCATION_HAND | LOCATION_DECK, 0, 1, 1, false, NO_EX).nil();
    // if #g>0 and Duel.SendtoGrave(g,REASON_EFFECT)>0 and g:GetFirst():IsLocation(LOCATION_GRAVE) then
    if g.is_empty() {
        return;
    }
    if duel::send_to_grave(d, g.clone(), REASON_EFFECT, None, None) <= 0 {
        return;
    }
    let first = group::get_first(d, &g).nil();
    if !card::is_location(d, first, LOCATION_GRAVE) {
        return;
    }
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // local tc=Duel.GetFirstTarget()
    let tc = duel::first_target(d).nil();
    // if tc:IsRelateToEffect(e) and tc:IsControler(1-tp) and not tc:IsImmuneToEffect(e)
    //     and c:IsRelateToEffect(e) then
    if card::is_relate_to_effect(d, tc, a.e)
        && card::is_controler(d, tc, 1 - tp)
        && !card::is_immune_to_effect(d, tc, a.e)
        && card::is_relate_to_effect(d, c, a.e)
    {
        // Duel.Overlay(c,tc,true)
        duel::overlay(d, c, tc, true);
    }
}
