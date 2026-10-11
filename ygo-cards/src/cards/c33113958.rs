//! LeVirtue Dragon
use super::prelude::*;

pub const ID: u32 = 33113958;

pub const INFO: ScriptInfo = ScriptInfo { listed_names: &[ID, 80534031], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // c:EnableReviveLimit()
    card::enable_revive_limit(d, c);
    // Xyz.AddProcedure(c,nil,3,2)
    xyz::add_procedure(d, c, None, Some(3), 2, None, None, None, None, false, None);
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_TOHAND+CATEGORY_SEARCH)
    effect::set_category(d, e1, CATEGORY_TOHAND + CATEGORY_SEARCH);
    // e1:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_O);
    // e1:SetProperty(EFFECT_FLAG_DELAY)
    effect::set_property(d, e1, EFFECT_FLAG_DELAY, 0);
    // e1:SetCode(EVENT_SPSUMMON_SUCCESS)
    effect::set_code(d, e1, EVENT_SPSUMMON_SUCCESS);
    // e1:SetCountLimit(1,id)
    effect::set_count_limit(d, e1, 1, ID, 0, 0);
    // e1:SetTarget(s.thtg)
    effect::set_target(d, e1, Some(cost_target(thtg)));
    // e1:SetOperation(s.thop)
    effect::set_operation(d, e1, Some(operation(thop)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // --Activate 1 of these effects
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e2, aux::stringid(ID, 1));
    // e2:SetType(EFFECT_TYPE_IGNITION)
    effect::set_type(d, e2, EFFECT_TYPE_IGNITION);
    // e2:SetRange(LOCATION_MZONE)
    effect::set_range(d, e2, LOCATION_MZONE);
    // e2:SetCountLimit(1,{id,1})
    effect::set_count_limit(d, e2, 1, ID, 1, 0);
    // e2:SetTarget(s.efftg)
    effect::set_target(d, e2, Some(cost_target(efftg)));
    // e2:SetOperation(s.effop)
    effect::set_operation(d, e2, Some(operation(effop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
}
/// `s.thfilter(c)`
fn thfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsCode(80534031) and c:IsAbleToHand()
    card::is_code(d, c, &[80534031]) && card::is_able_to_hand(d, c, None)
}
/// `s.thtg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn thtg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return Duel.IsExistingMatchingCard(s.thfilter,tp,LOCATION_DECK,0,1,nil) end
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, Some(&thfilter), a.tp, LOCATION_DECK, 0, 1, NO_EX);
    }
    // Duel.SetOperationInfo(0,CATEGORY_TOHAND,nil,1,tp,LOCATION_DECK)
    duel::set_operation_info(d, 0, CATEGORY_TOHAND, None, 1, a.tp, LOCATION_DECK as i32);
    true
}
/// `s.thop(e,tp,eg,ep,ev,re,r,rp)`
fn thop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_ATOHAND)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_ATOHAND as u64);
    // local g=Duel.SelectMatchingCard(tp,s.thfilter,tp,LOCATION_DECK,0,1,1,nil)
    let g = duel::select_matching_card(d, tp, Some(&thfilter), tp, LOCATION_DECK, 0, 1, 1, false, NO_EX).nil();
    // if #g>0 then
    if !g.is_empty() {
        // Duel.SendtoHand(g,nil,REASON_EFFECT)
        duel::send_to_hand(d, g.clone(), None, REASON_EFFECT, None);
        // Duel.ConfirmCards(1-tp,g)
        duel::confirm_cards(d, 1 - tp, g);
    }
}
/// `s.thgyfilter(c)`
fn thgyfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsRace(RACE_FISH|RACE_SEASERPENT|RACE_AQUA) and c:IsAbleToHand() and not c:IsCode(id)
    card::is_race(d, c, RACE_FISH | RACE_SEASERPENT | RACE_AQUA)
        && card::is_able_to_hand(d, c, None)
        && !card::is_code(d, c, &[ID])
}
/// `s.xyzfilter(c,e)`
fn xyzfilter(d: &mut Duel, c: CardId, e: EffectId) -> bool {
    // return c:IsType(TYPE_XYZ) and c:IsFaceup() and c:IsCanBeEffectTarget(e)
    card::is_type(d, c, TYPE_XYZ) && card::is_faceup(d, c) && card::is_can_be_effect_target(d, c, Some(e))
}
/// `s.rescon(sg,e,tp,mg)`
fn rescon(d: &mut Duel, sg: &CardSet, _e: Option<EffectId>, _tp: u8, _mg: &CardSet, _c: Option<CardId>) -> (bool, bool) {
    // return sg:IsExists(s.overlayfilter,1,nil)
    (group::is_exists(d, sg, &overlayfilter, 1, NO_EX), false)
}
/// `s.overlayfilter(c)`
fn overlayfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:GetOverlayCount()>0
    card::get_overlay_count(d, c) > 0
}
/// `s.efftg(e,tp,eg,ep,ev,re,r,rp,chk,chkc)`
fn efftg(d: &mut Duel, a: &EffArgs) -> bool {
    let e = a.e;
    let tp = a.tp;
    // local b1=Duel.CheckRemoveOverlayCard(tp,1,0,1,REASON_EFFECT)
    //   and Duel.IsExistingMatchingCard(s.thgyfilter,tp,LOCATION_GRAVE,0,1,nil)
    let b1 = duel::check_remove_overlay_card(d, tp, 1, 0, 1, REASON_EFFECT, None)
        && duel::is_existing_matching_card(d, Some(&thgyfilter), tp, LOCATION_GRAVE, 0, 1, NO_EX);
    // local g=Duel.GetMatchingGroup(s.xyzfilter,tp,LOCATION_MZONE,LOCATION_MZONE,nil,e)
    let g = duel::get_matching_group(d, Some(&|d, c| xyzfilter(d, c, e)), tp, LOCATION_MZONE, LOCATION_MZONE, NO_EX);
    // local b2=#g>=2 and aux.SelectUnselectGroup(g,e,tp,2,2,s.rescon,0)
    let b2 = g.len() >= 2 && {
        let rs: aux::Rescon = &rescon;
        matches!(aux::select_unselect_group(d, &g, Some(e), tp, Some(2), Some(2), Some(rs), 0, tp, None, None, None, false), Err(true))
    };
    // if chk==0 then return b1 or b2 end
    if a.chk == Some(0) {
        return b1 || b2;
    }
    // local op=Duel.SelectEffect(tp,
    //   {b1,aux.Stringid(id,2)},
    //   {b2,aux.Stringid(id,3)})
    let op = duel::select_effect(d, tp, &[(b1, aux::stringid(ID, 2)), (b2, aux::stringid(ID, 3))]).unwrap_or(0) as i64;
    // e:SetLabel(op)
    effect::set_label(d, e, &[op]);
    // if op==1 then
    if op == 1 {
        // e:SetCategory(CATEGORY_TOHAND)
        effect::set_category(d, e, CATEGORY_TOHAND);
        // e:SetProperty(0)
        effect::set_property(d, e, 0, 0);
        // Duel.SetOperationInfo(0,CATEGORY_TOHAND,nil,1,tp,LOCATION_GRAVE)
        duel::set_operation_info(d, 0, CATEGORY_TOHAND, None, 1, tp, LOCATION_GRAVE as i32);
    // elseif op==2 then
    } else if op == 2 {
        // e:SetCategory(0)
        effect::set_category(d, e, 0);
        // e:SetProperty(EFFECT_FLAG_CARD_TARGET)
        effect::set_property(d, e, EFFECT_FLAG_CARD_TARGET, 0);
        // local g=aux.SelectUnselectGroup(g,e,tp,2,2,s.rescon,1,tp,HINTMSG_TARGET)
        let rs: aux::Rescon = &rescon;
        let g = aux::select_unselect_group(d, &g, Some(e), tp, Some(2), Some(2), Some(rs), 1, tp, Some(HINTMSG_TARGET as u64), None, None, false).unwrap_or_default();
        // Duel.SetTargetCard(g)
        duel::set_target_card(d, g);
    }
    true
}
/// `s.effop(e,tp,eg,ep,ev,re,r,rp)`
fn effop(d: &mut Duel, a: &EffArgs) {
    let e = a.e;
    let tp = a.tp;
    // local op=e:GetLabel()
    let op = effect::get_label(d, e);
    // if op==1 then
    if op == 1 {
        // Detach 1 Xyz Material from a monster you control, then add 1 Fish, Sea Serpent, or Aqua monster from your GY to your hand
        // if Duel.RemoveOverlayCard(tp,1,0,1,1,REASON_EFFECT)==0 then return end
        if duel::remove_overlay_card(d, tp, 1, 0, 1, 1, REASON_EFFECT, None) == 0 {
            return;
        }
        // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_ATOHAND)
        duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_ATOHAND as u64);
        // local g=Duel.SelectMatchingCard(tp,s.thgyfilter,tp,LOCATION_GRAVE,0,1,1,nil)
        let g = duel::select_matching_card(d, tp, Some(&thgyfilter), tp, LOCATION_GRAVE, 0, 1, 1, false, NO_EX).nil();
        // if #g>0 then
        if !g.is_empty() {
            // Duel.BreakEffect()
            duel::break_effect(d);
            // Duel.SendtoHand(g,nil,REASON_EFFECT)
            duel::send_to_hand(d, g.clone(), None, REASON_EFFECT, None);
            // Duel.ConfirmCards(1-tp,g)
            duel::confirm_cards(d, 1 - tp, g);
        }
    // elseif op==2 then
    } else if op == 2 {
        // Detach 1 material from 1 of the targeted Xyz Monsters and attach it to the other one
        // local g=Duel.GetTargetCards(e)
        let mut g = duel::get_target_cards(d, e).nil();
        // if #g~=2 then return end
        if g.len() != 2 {
            return;
        }
        // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_DEATTACHFROM)
        duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_DEATTACHFROM as u64);
        // local detachxyz=g:FilterSelect(tp,s.overlayfilter,1,1,nil):GetFirst()
        let sel = group::filter_select(d, &g, tp, &overlayfilter, 1, 1, false, NO_EX).nil();
        let detachxyz = group::get_first(d, &sel).nil();
        // local attachxyz=g:RemoveCard(detachxyz):GetFirst()
        group::remove_card(&mut g, &CardSet::from_card(detachxyz));
        let attachxyz = group::get_first(d, &g).nil();
        // local attach_group=detachxyz:GetOverlayGroup()
        let mut attach_group = card::get_overlay_group(d, detachxyz);
        // if #attach_group>0 then
        if !attach_group.is_empty() {
            // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_ATTACH)
            duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_ATTACH as u64);
            // attach_group=attach_group:Select(tp,1,1,nil)
            attach_group = group::select(d, &attach_group, tp, 1, 1, false, NO_EX).nil();
        }
        // Duel.Overlay(attachxyz,attach_group)
        duel::overlay(d, attachxyz, attach_group, false);
        // Duel.RaiseSingleEvent(detachxyz,EVENT_DETACH_MATERIAL,e,0,0,0,0)
        duel::raise_single_event(d, detachxyz, EVENT_DETACH_MATERIAL, Some(e), 0, 0, 0, 0);
        // Duel.RaiseEvent(detachxyz,EVENT_DETACH_MATERIAL,e,REASON_EFFECT,tp,tp,0)
        duel::raise_event(d, CardSet::from_card(detachxyz), EVENT_DETACH_MATERIAL, Some(e), REASON_EFFECT, tp, tp, 0);
    }
}
