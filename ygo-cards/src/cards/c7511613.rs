//! Ryzeal Duo Drive
//! scripted by Naim
use super::prelude::*;

pub const ID: u32 = 7511613;

// s.listed_series={SET_RYZEAL}
pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_RYZEAL], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // c:EnableReviveLimit()
    card::enable_revive_limit(d, c);
    // Xyz Summon procedure: 2+ Level 4 monsters
    // Xyz.AddProcedure(c,nil,4,2,nil,nil,Xyz.InfiniteMats)
    xyz::add_procedure(d, c, None, Some(4), 2, None, None, Some(xyz::INFINITE_MATS), None, false, None);
    // For each material attached to this card, monsters you control gain 100 ATK and monsters your opponent controls lose 100 ATK
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e1, EFFECT_TYPE_FIELD);
    // e1:SetCode(EFFECT_UPDATE_ATTACK)
    effect::set_code(d, e1, EFFECT_UPDATE_ATTACK);
    // e1:SetRange(LOCATION_MZONE)
    effect::set_range(d, e1, LOCATION_MZONE);
    // e1:SetTargetRange(LOCATION_MZONE,LOCATION_MZONE)
    effect::set_target_range(d, e1, LOCATION_MZONE, LOCATION_MZONE);
    // e1:SetValue(function(e,c) return (c:IsControler(e:GetHandlerPlayer()) and 100 or -100)*e:GetHandler():GetOverlayCount() end)
    effect::set_value_fn(d, e1, effect_card_value(atk_value));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // Attach 1 "Ryzeal" monster from your GY to this card as material
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e2, aux::stringid(ID, 0));
    // e2:SetCategory(CATEGORY_LEAVE_GRAVE)
    effect::set_category(d, e2, CATEGORY_LEAVE_GRAVE);
    // e2:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e2, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_O);
    // e2:SetProperty(EFFECT_FLAG_DELAY)
    effect::set_property(d, e2, EFFECT_FLAG_DELAY, 0);
    // e2:SetCode(EVENT_SPSUMMON_SUCCESS)
    effect::set_code(d, e2, EVENT_SPSUMMON_SUCCESS);
    // e2:SetCountLimit(1,id)
    effect::set_count_limit(d, e2, 1, ID, 0, 0);
    // e2:SetTarget(s.attachtg)
    effect::set_target(d, e2, Some(cost_target(attachtg)));
    // e2:SetOperation(s.attachop)
    effect::set_operation(d, e2, Some(operation(attachop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
    // Detach 2 materials from monster(s) you control, and if you do, add 2 "Ryzeal" cards with different names from your Deck to your hand
    // local e3=Effect.CreateEffect(c)
    let e3 = effect::create_effect(d, c);
    // e3:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e3, aux::stringid(ID, 1));
    // e3:SetCategory(CATEGORY_TOHAND+CATEGORY_SEARCH)
    effect::set_category(d, e3, CATEGORY_TOHAND + CATEGORY_SEARCH);
    // e3:SetType(EFFECT_TYPE_IGNITION)
    effect::set_type(d, e3, EFFECT_TYPE_IGNITION);
    // e3:SetRange(LOCATION_MZONE)
    effect::set_range(d, e3, LOCATION_MZONE);
    // e3:SetCountLimit(1,{id,1})
    effect::set_count_limit(d, e3, 1, ID, 1, 0);
    // e3:SetTarget(s.thtg)
    effect::set_target(d, e3, Some(cost_target(thtg)));
    // e3:SetOperation(s.thop)
    effect::set_operation(d, e3, Some(operation(thop)));
    // c:RegisterEffect(e3)
    card::register_effect(d, c, e3, false);
}
/// `function(e,c) return (c:IsControler(e:GetHandlerPlayer()) and 100 or -100)*e:GetHandler():GetOverlayCount() end`
fn atk_value(d: &mut Duel, e: EffectId, c: CardId) -> i64 {
    let p = effect::get_handler_player(d, e);
    let sign: i64 = if card::is_controler(d, c, p) { 100 } else { -100 };
    let h = effect::get_handler(d, e).nil();
    sign * card::get_overlay_count(d, h) as i64
}
/// `s.attachfilter(c,xyzc,tp)`
fn attachfilter(d: &mut Duel, c: CardId, xyzc: CardId, tp: u8) -> bool {
    // return c:IsSetCard(SET_RYZEAL) and c:IsMonster() and c:IsCanBeXyzMaterial(xyzc,tp,REASON_EFFECT)
    card::is_set_card(d, c, &[SET_RYZEAL as u16]) && card::is_monster(d, c) && card::is_can_be_xyz_material(d, c, Some(xyzc), Some(tp), Some(REASON_EFFECT))
}
/// `s.attachtg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn attachtg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // if chk==0 then return Duel.IsExistingMatchingCard(s.attachfilter,tp,LOCATION_GRAVE,0,1,nil,e:GetHandler(),tp) end
    if a.chk == Some(0) {
        let h = effect::get_handler(d, a.e).nil();
        return duel::is_existing_matching_card(d, Some(&|d, x| attachfilter(d, x, h, tp)), tp, LOCATION_GRAVE, 0, 1, NO_EX);
    }
    // Duel.SetOperationInfo(0,CATEGORY_LEAVE_GRAVE,nil,1,tp,0)
    duel::set_operation_info(d, 0, CATEGORY_LEAVE_GRAVE, None, 1, tp, 0);
    true
}
/// `s.attachop(e,tp,eg,ep,ev,re,r,rp)`
fn attachop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if not c:IsRelateToEffect(e) then return end
    if !card::is_relate_to_effect(d, c, a.e) {
        return;
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_XMATERIAL)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_XMATERIAL as u64);
    // local tc=Duel.SelectMatchingCard(tp,s.attachfilter,tp,LOCATION_GRAVE,0,1,1,nil,c,tp):GetFirst()
    let sel = duel::select_matching_card(d, tp, Some(&|d, x| attachfilter(d, x, c, tp)), tp, LOCATION_GRAVE, 0, 1, 1, false, NO_EX).nil();
    let tc = group::get_first(d, &sel);
    // if tc then
    if let Some(tc) = tc {
        // Duel.HintSelection(tc)
        duel::hint_selection(d, tc, true);
        // Duel.Overlay(c,tc)
        duel::overlay(d, c, tc, false);
    }
}
/// `s.thfilter(c)`
fn thfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsSetCard(SET_RYZEAL) and c:IsAbleToHand()
    card::is_set_card(d, c, &[SET_RYZEAL as u16]) && card::is_able_to_hand(d, c, None)
}
/// `s.thtg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn thtg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    let e = a.e;
    // if chk==0 then
    if a.chk == Some(0) {
        // local g=Duel.GetMatchingGroup(s.thfilter,tp,LOCATION_DECK,0,nil)
        let g = duel::get_matching_group(d, Some(&thfilter), tp, LOCATION_DECK, 0, NO_EX);
        // return Duel.CheckRemoveOverlayCard(tp,1,0,2,REASON_EFFECT)
        //   and #g>=2 and aux.SelectUnselectGroup(g,e,tp,2,2,aux.dncheck,0)
        if !duel::check_remove_overlay_card(d, tp, 1, 0, 2, REASON_EFFECT, None) {
            return false;
        }
        if g.len() < 2 {
            return false;
        }
        let dc = aux::dncheck();
        let rs: aux::Rescon = &dc;
        return matches!(aux::select_unselect_group(d, &g, Some(e), tp, Some(2), Some(2), Some(rs), 0, tp, None, None, None, false), Err(true));
    }
    // Duel.SetOperationInfo(0,CATEGORY_TOHAND,nil,2,tp,LOCATION_DECK)
    duel::set_operation_info(d, 0, CATEGORY_TOHAND, None, 2, tp, LOCATION_DECK as i32);
    true
}
/// `s.thop(e,tp,eg,ep,ev,re,r,rp)`
fn thop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    let e = a.e;
    // if Duel.RemoveOverlayCard(tp,1,0,2,2,REASON_EFFECT)==2 then
    if duel::remove_overlay_card(d, tp, 1, 0, 2, 2, REASON_EFFECT, None) == 2 {
        // local g=Duel.GetMatchingGroup(s.thfilter,tp,LOCATION_DECK,0,nil)
        let g = duel::get_matching_group(d, Some(&thfilter), tp, LOCATION_DECK, 0, NO_EX);
        // if #g<2 then return end
        if g.len() < 2 {
            return;
        }
        // local thg=aux.SelectUnselectGroup(g,e,tp,2,2,aux.dncheck,1,tp,HINTMSG_ATOHAND)
        let dc = aux::dncheck();
        let rs: aux::Rescon = &dc;
        let thg = aux::select_unselect_group(d, &g, Some(e), tp, Some(2), Some(2), Some(rs), 1, tp, Some(HINTMSG_ATOHAND as u64), None, None, false).unwrap_or_default();
        // if #thg>0 then
        if !thg.is_empty() {
            // Duel.SendtoHand(thg,nil,REASON_EFFECT)
            duel::send_to_hand(d, thg.clone(), None, REASON_EFFECT, None);
            // Duel.ConfirmCards(1-tp,thg)
            duel::confirm_cards(d, 1 - tp, thg);
        }
    }
}
