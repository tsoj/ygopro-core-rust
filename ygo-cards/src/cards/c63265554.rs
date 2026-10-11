//! Power Tool Braver Dragon
use super::prelude::*;

pub const ID: u32 = 63265554;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Synchro Summon procedure
    // Synchro.AddProcedure(c,nil,1,1,Synchro.NonTuner(nil),1,99)
    synchro::add_procedure(d, c, None, 1, 1, Some(synchro::non_tuner(None)), 1, 99, None, None, None, None, None);
    // c:EnableReviveLimit()
    card::enable_revive_limit(d, c);
    // Equip up to 3 Equip Spells
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_EQUIP)
    effect::set_category(d, e1, CATEGORY_EQUIP);
    // e1:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_O);
    // e1:SetCode(EVENT_SPSUMMON_SUCCESS)
    effect::set_code(d, e1, EVENT_SPSUMMON_SUCCESS);
    // e1:SetProperty(EFFECT_FLAG_DELAY)
    effect::set_property(d, e1, EFFECT_FLAG_DELAY, 0);
    // e1:SetCountLimit(1,id)
    effect::set_count_limit(d, e1, 1, ID, 0, 0);
    // e1:SetTarget(s.eqtg)
    effect::set_target(d, e1, Some(cost_target(eqtg)));
    // e1:SetOperation(s.eqop)
    effect::set_operation(d, e1, Some(operation(eqop)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // Change position or negate effects
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e2, aux::stringid(ID, 1));
    // e2:SetCategory(CATEGORY_DISABLE+CATEGORY_POSITION)
    effect::set_category(d, e2, CATEGORY_DISABLE + CATEGORY_POSITION);
    // e2:SetType(EFFECT_TYPE_QUICK_O)
    effect::set_type(d, e2, EFFECT_TYPE_QUICK_O);
    // e2:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e2, EVENT_FREE_CHAIN);
    // e2:SetProperty(EFFECT_FLAG_CARD_TARGET)
    effect::set_property(d, e2, EFFECT_FLAG_CARD_TARGET, 0);
    // e2:SetRange(LOCATION_MZONE)
    effect::set_range(d, e2, LOCATION_MZONE);
    // e2:SetHintTiming(0,TIMING_MAIN_END|TIMINGS_CHECK_MONSTER)
    effect::set_hint_timing(d, e2, 0, Some(TIMING_MAIN_END | TIMINGS_CHECK_MONSTER));
    // e2:SetCountLimit(1,{id,1})
    effect::set_count_limit(d, e2, 1, ID, 1, 0);
    // e2:SetCondition(function() return Duel.IsMainPhase() end)
    effect::set_condition(d, e2, Some(condition(main_phase_cond)));
    // e2:SetCost(s.discost)
    effect::set_cost(d, e2, Some(cost_target(discost)));
    // e2:SetTarget(s.distg)
    effect::set_target(d, e2, Some(cost_target(distg)));
    // e2:SetOperation(s.disop)
    effect::set_operation(d, e2, Some(operation(disop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
}
/// `function() return Duel.IsMainPhase() end`
fn main_phase_cond(d: &mut Duel, _a: &EffArgs) -> bool {
    duel::is_main_phase(d, None)
}
/// `s.eqsfilter(c,tp,ec)`
fn eqsfilter(d: &mut Duel, c: CardId, tp: u8, ec: CardId) -> bool {
    // return c:IsType(TYPE_EQUIP) and c:CheckEquipTarget(ec) and c:CheckUniqueOnField(tp)
    card::is_type(d, c, TYPE_EQUIP) && card::check_equip_target(d, c, ec) && card::check_unique_on_field(d, c, tp, None, None)
}
/// `s.eqtg(e,tp,eg,ep,ev,re,r,rp,chk,chkc)`
fn eqtg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return Duel.GetLocationCount(tp,LOCATION_SZONE)>0
    //     and Duel.IsExistingMatchingCard(s.eqsfilter,tp,LOCATION_DECK|LOCATION_GRAVE,0,1,nil,tp,e:GetHandler()) end
    if a.chk == Some(0) {
        let h = effect::get_handler(d, a.e).nil();
        return duel::get_location_count(d, a.tp, LOCATION_SZONE as u8, None, None, None).0 > 0
            && duel::is_existing_matching_card(d, Some(&|d, c| eqsfilter(d, c, a.tp, h)), a.tp, LOCATION_DECK | LOCATION_GRAVE, 0, 1, NO_EX);
    }
    // Duel.SetOperationInfo(0,CATEGORY_EQUIP,nil,1,0,LOCATION_DECK|LOCATION_GRAVE)
    duel::set_operation_info(d, 0, CATEGORY_EQUIP, None, 1, 0, (LOCATION_DECK | LOCATION_GRAVE) as i32);
    true
}
/// `s.eqop(e,tp,eg,ep,ev,re,r,rp)`
fn eqop(d: &mut Duel, a: &EffArgs) {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if c:IsFacedown() or not c:IsRelateToEffect(e) then return end
    if card::is_facedown(d, c) || !card::is_relate_to_effect(d, c, a.e) {
        return;
    }
    // local g=Duel.GetMatchingGroup(aux.NecroValleyFilter(s.eqsfilter),tp,LOCATION_DECK|LOCATION_GRAVE,0,nil,tp,c)
    let g = duel::get_matching_group(
        d,
        Some(&aux::necro_valley_filter(move |d, x| eqsfilter(d, x, a.tp, c))),
        a.tp,
        LOCATION_DECK | LOCATION_GRAVE,
        0,
        NO_EX,
    );
    // if #g==0 then return end
    if g.is_empty() {
        return;
    }
    // local ft=math.min(Duel.GetLocationCount(tp,LOCATION_SZONE),3)
    let ft = duel::get_location_count(d, a.tp, LOCATION_SZONE as u8, None, None, None).0.min(3);
    // local sg=aux.SelectUnselectGroup(g,e,tp,1,ft,aux.dncheck,1,tp,HINTMSG_EQUIP)
    let sg = aux::select_unselect_group(
        d,
        &g,
        Some(a.e),
        a.tp,
        Some(1),
        Some(ft.max(0) as usize),
        Some(&aux::dncheck()),
        1,
        a.tp,
        Some(HINTMSG_EQUIP as u64),
        None,
        None,
        false,
    )
    .unwrap_or_default();
    // if #sg>0 then
    if !sg.is_empty() {
        // for tc in sg:Iter() do
        for tc in group::iter(d, &sg) {
            // Duel.Equip(tp,tc,c,true,true)
            duel::equip(d, a.tp, tc, c, true, true);
        }
        // Duel.EquipComplete()
        duel::equip_complete(d);
    }
}
/// `s.tgfilter(c,tc)`
fn tgfilter(d: &mut Duel, c: CardId, tc: CardId) -> bool {
    // return c:IsType(TYPE_EQUIP) and c:IsSpell() and c:IsAbleToGraveAsCost() and c:GetEquipTarget()==tc
    card::is_type(d, c, TYPE_EQUIP) && card::is_spell(d, c) && card::is_able_to_grave_as_cost(d, c) && card::get_equip_target(d, c) == Some(tc)
}
/// `s.discost(e,tp,eg,ep,ev,re,r,rp,chk)`
fn discost(d: &mut Duel, a: &EffArgs) -> bool {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if chk==0 then return Duel.IsExistingMatchingCard(s.tgfilter,tp,LOCATION_SZONE,0,1,nil,c) end
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, Some(&|d, x| tgfilter(d, x, c)), a.tp, LOCATION_SZONE, 0, 1, NO_EX);
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_TOGRAVE)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_TOGRAVE as u64);
    // local g=Duel.SelectMatchingCard(tp,s.tgfilter,tp,LOCATION_SZONE,0,1,1,nil,c)
    let g = duel::select_matching_card(d, a.tp, Some(&|d, x| tgfilter(d, x, c)), a.tp, LOCATION_SZONE, 0, 1, 1, false, NO_EX).nil();
    // Duel.SendtoGrave(g,REASON_COST)
    duel::send_to_grave(d, g, REASON_COST, None, None);
    true
}
/// `s.disfilter(c)`
fn disfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsFaceup() and c:IsType(TYPE_EFFECT) and (c:IsNegatableMonster() or c:IsCanChangePosition())
    card::is_faceup(d, c) && card::is_type(d, c, TYPE_EFFECT) && (card::is_negatable_monster(d, c) || card::is_can_change_position(d, c))
}
/// `s.distg(e,tp,eg,ep,ev,re,r,rp,chk,chkc)`
fn distg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chkc then return chkc:IsLocation(LOCATION_MZONE) and s.disfilter(chkc) end
    if let Some(chkc) = a.chkc() {
        return card::is_location(d, chkc, LOCATION_MZONE) && disfilter(d, chkc);
    }
    // if chk==0 then return Duel.IsExistingTarget(s.disfilter,tp,LOCATION_MZONE,LOCATION_MZONE,1,nil) end
    if a.chk == Some(0) {
        return duel::is_existing_target(d, Some(&disfilter), a.tp, LOCATION_MZONE, LOCATION_MZONE, 1, NO_EX);
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_SELECT)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_SELECT as u64);
    // Duel.SelectTarget(tp,s.disfilter,tp,LOCATION_MZONE,LOCATION_MZONE,1,1,nil)
    let _ = duel::select_target(d, a.tp, Some(&disfilter), a.tp, LOCATION_MZONE, LOCATION_MZONE, 1, 1, false, NO_EX);
    true
}
/// `s.disop(e,tp,eg,ep,ev,re,r,rp)`
fn disop(d: &mut Duel, a: &EffArgs) {
    // local tc=Duel.GetFirstTarget()
    let tc = duel::first_target(d).nil();
    // if not tc:IsRelateToEffect(e) then return end
    if !card::is_relate_to_effect(d, tc, a.e) {
        return;
    }
    // local b1=tc:IsCanChangePosition()
    let b1 = card::is_can_change_position(d, tc);
    // local b2=tc:IsNegatableMonster()
    let b2 = card::is_negatable_monster(d, tc);
    // local op=Duel.SelectEffect(tp,{b1,aux.Stringid(id,2)},{b2,aux.Stringid(id,3)})
    let op = duel::select_effect(d, a.tp, &[(b1, aux::stringid(ID, 2)), (b2, aux::stringid(ID, 3))]);
    // if op==1 then
    if op == Some(1) {
        // Change its battle position
        // Duel.ChangePosition(tc,POS_FACEUP_DEFENSE,0,POS_FACEUP_ATTACK,0)
        duel::change_position(d, tc, POS_FACEUP_DEFENSE as u8, Some(0), Some(POS_FACEUP_ATTACK as u8), Some(0), false);
    } else if op == Some(2) {
        // local c=e:GetHandler()
        let c = effect::get_handler(d, a.e).nil();
        // Negate its effects
        // Duel.NegateRelatedChain(tc,RESET_TURN_SET)
        duel::negate_related_chain(d, tc, RESET_TURN_SET);
        // local e1=Effect.CreateEffect(c)
        let e1 = effect::create_effect(d, c);
        // e1:SetType(EFFECT_TYPE_SINGLE)
        effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
        // e1:SetCode(EFFECT_DISABLE)
        effect::set_code(d, e1, EFFECT_DISABLE);
        // e1:SetProperty(EFFECT_FLAG_CANNOT_DISABLE)
        effect::set_property(d, e1, EFFECT_FLAG_CANNOT_DISABLE, 0);
        // e1:SetReset(RESETS_STANDARD_PHASE_END)
        effect::set_reset(d, e1, RESETS_STANDARD_PHASE_END, 1);
        // tc:RegisterEffect(e1)
        card::register_effect(d, tc, e1, false);
        // local e2=Effect.CreateEffect(c)
        let e2 = effect::create_effect(d, c);
        // e2:SetType(EFFECT_TYPE_SINGLE)
        effect::set_type(d, e2, EFFECT_TYPE_SINGLE);
        // e2:SetCode(EFFECT_DISABLE_EFFECT)
        effect::set_code(d, e2, EFFECT_DISABLE_EFFECT);
        // e2:SetProperty(EFFECT_FLAG_CANNOT_DISABLE)
        effect::set_property(d, e2, EFFECT_FLAG_CANNOT_DISABLE, 0);
        // e2:SetValue(RESET_TURN_SET)
        effect::set_value(d, e2, RESET_TURN_SET as i64);
        // e2:SetReset(RESETS_STANDARD_PHASE_END)
        effect::set_reset(d, e2, RESETS_STANDARD_PHASE_END, 1);
        // tc:RegisterEffect(e2)
        card::register_effect(d, tc, e2, false);
    }
}
