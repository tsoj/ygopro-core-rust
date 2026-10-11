//! ペンデュラム・ペンダント (Pendulum Pendant)
use super::prelude::*;

pub const ID: u32 = 65914127;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Place 1 Pendulum monster in the Pendulum Zone
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    // e1:SetProperty(EFFECT_FLAG_LIMIT_ZONE)
    effect::set_property(d, e1, EFFECT_FLAG_LIMIT_ZONE, 0);
    // e1:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    // e1:SetCountLimit(1,id,EFFECT_COUNT_CODE_OATH)
    effect::set_count_limit(d, e1, 1, ID, 0, EFFECT_COUNT_CODE_OATH as u8);
    // e1:SetValue(s.zones)
    effect::set_value_fn(d, e1, lua_fn(zones));
    // e1:SetCost(s.cost)
    effect::set_cost(d, e1, Some(cost_target(cost_fn)));
    // e1:SetTarget(s.target)
    effect::set_target(d, e1, Some(cost_target(target)));
    // e1:SetOperation(s.activate)
    effect::set_operation(d, e1, Some(operation(activate)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // Reduce a Pendulum Scale by 1
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e2, aux::stringid(ID, 1));
    // e2:SetType(EFFECT_TYPE_IGNITION)
    effect::set_type(d, e2, EFFECT_TYPE_IGNITION);
    // e2:SetProperty(EFFECT_FLAG_CARD_TARGET)
    effect::set_property(d, e2, EFFECT_FLAG_CARD_TARGET, 0);
    // e2:SetRange(LOCATION_GRAVE)
    effect::set_range(d, e2, LOCATION_GRAVE);
    // e2:SetCost(Cost.SelfBanish)
    effect::set_cost(d, e2, Some(cost::self_banish()));
    // e2:SetTarget(s.scltg)
    effect::set_target(d, e2, Some(cost_target(scltg)));
    // e2:SetOperation(s.sclop)
    effect::set_operation(d, e2, Some(operation(sclop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
}
// function s.zones(e,tp,eg,ep,ev,re,r,rp)
fn zones(d: &mut Duel, a: Args) -> i64 {
    let tp = a.p(1);
    // local zone=0xff --all Spell/Trap zones
    let mut zone: i64 = 0xff;
    // if Duel.IsDuelType(DUEL_SEPARATE_PZONE) then return zone end
    if duel::is_duel_type(d, DUEL_SEPARATE_PZONE) {
        return zone;
    }
    // local p0=Duel.CheckLocation(tp,LOCATION_PZONE,0)
    let p0 = duel::check_location(d, tp, LOCATION_PZONE, 0);
    // local p1=Duel.CheckLocation(tp,LOCATION_PZONE,1)
    let p1 = duel::check_location(d, tp, LOCATION_PZONE, 1);
    // if p0==p1 then return zone end
    if p0 == p1 {
        return zone;
    }
    // if p0 then zone=zone-0x1 end --remove the left most S/T zone
    if p0 {
        zone -= 0x1;
    }
    // if p1 then zone=zone-0x10 end --remove the right most S/T zone
    if p1 {
        zone -= 0x10;
    }
    // return zone
    zone
}
// function s.costfilter(c)
fn costfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsFaceup() and c:IsType(TYPE_PENDULUM) and c:IsAbleToRemoveAsCost()
    card::is_faceup(d, c) && card::is_type(d, c, TYPE_PENDULUM) && card::is_able_to_remove_as_cost(d, c, None)
}
// function s.pendfilter(c)
fn pendfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsType(TYPE_PENDULUM) and (c:IsFaceup() or c:IsLocation(LOCATION_DECK)) and not c:IsForbidden()
    card::is_type(d, c, TYPE_PENDULUM)
        && (card::is_faceup(d, c) || card::is_location(d, c, LOCATION_DECK))
        && !card::is_forbidden(d, c)
}
// function s.rescon(pg)
// return function(sg,e,tp,mg) return #sg==5 and pg:IsExists(aux.TRUE,1,sg) end end
fn rescon(pg: &CardSet) -> impl Fn(&mut Duel, &CardSet, Option<EffectId>, u8, &CardSet, Option<CardId>) -> (bool, bool) + '_ {
    move |d, sg, _e, _tp, _mg, _c| {
        (sg.len() == 5 && group::is_exists(d, pg, &|_d: &mut Duel, _c: CardId| true, 1, sg.into()), false)
    }
}
// function s.cost(e,tp,eg,ep,ev,re,r,rp,chk)
fn cost_fn(d: &mut Duel, a: &EffArgs) -> bool {
    // local g=Duel.GetMatchingGroup(s.costfilter,tp,LOCATION_EXTRA,0,nil)
    let g = duel::get_matching_group(d, Some(&costfilter), a.tp, LOCATION_EXTRA, 0, NO_EX);
    // local pg=Duel.GetMatchingGroup(s.pendfilter,tp,LOCATION_EXTRA|LOCATION_DECK,0,nil)
    let pg = duel::get_matching_group(d, Some(&pendfilter), a.tp, LOCATION_EXTRA | LOCATION_DECK, 0, NO_EX);
    // if chk==0 then return #g>=5 and aux.SelectUnselectGroup(g,e,tp,5,5,s.rescon(pg),0) end
    if a.chk == Some(0) {
        let rc = rescon(&pg);
        return g.len() >= 5
            && matches!(
                aux::select_unselect_group(d, &g, Some(a.e), a.tp, Some(5), Some(5), Some(&rc), 0, a.tp, None, None, None, false),
                Err(true)
            );
    }
    // local rg=aux.SelectUnselectGroup(g,e,tp,5,5,s.rescon(pg),1,tp,HINTMSG_REMOVE)
    let rc = rescon(&pg);
    let rg = aux::select_unselect_group(d, &g, Some(a.e), a.tp, Some(5), Some(5), Some(&rc), 1, a.tp, Some(HINTMSG_REMOVE as u64), None, None, false)
        .unwrap_or_default();
    // Duel.Remove(rg,POS_FACEUP,REASON_COST)
    duel::remove(d, rg, POS_FACEUP as u8, REASON_COST, None, None);
    true
}
// function s.target(e,tp,eg,ep,ev,re,r,rp,chk)
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return Duel.CheckPendulumZones(tp)
    //     and Duel.IsExistingMatchingCard(s.pendfilter,tp,LOCATION_EXTRA|LOCATION_DECK,0,1,nil) end
    if a.chk == Some(0) {
        return duel::check_pendulum_zones(d, a.tp)
            && duel::is_existing_matching_card(d, Some(&pendfilter), a.tp, LOCATION_EXTRA | LOCATION_DECK, 0, 1, NO_EX);
    }
    true
}
// function s.activate(e,tp,eg,ep,ev,re,r,rp)
fn activate(d: &mut Duel, a: &EffArgs) {
    // if not Duel.CheckPendulumZones(tp) then return end
    if !duel::check_pendulum_zones(d, a.tp) {
        return;
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_TOFIELD)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_TOFIELD as u64);
    // local tc=Duel.SelectMatchingCard(tp,s.pendfilter,tp,LOCATION_EXTRA|LOCATION_DECK,0,1,1,nil):GetFirst()
    let sel = duel::select_matching_card(
        d,
        a.tp,
        Some(&pendfilter),
        a.tp,
        LOCATION_EXTRA | LOCATION_DECK,
        0,
        1,
        1,
        false,
        NO_EX,
    )
    .nil();
    let tc = group::get_first(d, &sel);
    // if tc then
    if let Some(tc) = tc {
        // Duel.MoveToField(tc,tp,tp,LOCATION_PZONE,POS_FACEUP,true)
        duel::move_to_field(d, tc, a.tp, a.tp, LOCATION_PZONE as u16, POS_FACEUP as u8, true, None);
    }
}
// function s.sclfilter(c)
fn sclfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:GetScale()>0
    card::get_scale(d, c) > 0
}
// function s.scltg(e,tp,eg,ep,ev,re,r,rp,chk,chkc)
fn scltg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chkc then return chkc:IsLocation(LOCATION_PZONE) end
    if let Some(chkc) = a.chkc() {
        return card::is_location(d, chkc, LOCATION_PZONE);
    }
    // if chk==0 then return Duel.IsExistingTarget(s.sclfilter,tp,LOCATION_PZONE,LOCATION_PZONE,1,nil) end
    if a.chk == Some(0) {
        return duel::is_existing_target(d, Some(&sclfilter), a.tp, LOCATION_PZONE, LOCATION_PZONE, 1, NO_EX);
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_FACEUP)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_FACEUP as u64);
    // Duel.SelectTarget(tp,s.sclfilter,tp,LOCATION_PZONE,LOCATION_PZONE,1,1,nil)
    duel::select_target(d, a.tp, Some(&sclfilter), a.tp, LOCATION_PZONE, LOCATION_PZONE, 1, 1, false, NO_EX);
    true
}
// function s.sclop(e,tp,eg,ep,ev,re,r,rp)
fn sclop(d: &mut Duel, a: &EffArgs) {
    // local tc=Duel.GetFirstTarget()
    let tc = duel::first_target(d).nil();
    // if tc:IsRelateToEffect(e) and tc:GetScale()>0 then
    if card::is_relate_to_effect(d, tc, a.e) && card::get_scale(d, tc) > 0 {
        // Reduce its scale by 1
        // local e1=Effect.CreateEffect(e:GetHandler())
        let handler = effect::get_handler(d, a.e).nil();
        let e1 = effect::create_effect(d, handler);
        // e1:SetType(EFFECT_TYPE_SINGLE)
        effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
        // e1:SetProperty(EFFECT_FLAG_CANNOT_DISABLE)
        effect::set_property(d, e1, EFFECT_FLAG_CANNOT_DISABLE, 0);
        // e1:SetCode(EFFECT_UPDATE_LSCALE)
        effect::set_code(d, e1, EFFECT_UPDATE_LSCALE);
        // e1:SetValue(-1)
        effect::set_value(d, e1, -1);
        // e1:SetReset(RESET_EVENT|RESETS_STANDARD)
        effect::set_reset(d, e1, RESET_EVENT | RESETS_STANDARD, 1);
        // tc:RegisterEffect(e1)
        card::register_effect(d, tc, e1, false);
        // local e2=e1:Clone()
        let e2 = effect::clone(d, e1);
        // e2:SetCode(EFFECT_UPDATE_RSCALE)
        effect::set_code(d, e2, EFFECT_UPDATE_RSCALE);
        // tc:RegisterEffect(e2)
        card::register_effect(d, tc, e2, false);
    }
}
