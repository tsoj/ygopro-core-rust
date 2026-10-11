//! Arms Regeneration
use super::prelude::*;

pub const ID: u32 = 32549749;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Make a monster gain 800 ATK or Set or Equip a card from the GY
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    // e1:SetProperty(EFFECT_FLAG_CARD_TARGET+EFFECT_FLAG_DAMAGE_STEP)
    effect::set_property(d, e1, EFFECT_FLAG_CARD_TARGET + EFFECT_FLAG_DAMAGE_STEP, 0);
    // e1:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    // e1:SetHintTiming(TIMING_DAMAGE_STEP,TIMING_DAMAGE_STEP|TIMING_END_PHASE)
    effect::set_hint_timing(d, e1, TIMING_DAMAGE_STEP, Some(TIMING_DAMAGE_STEP | TIMING_END_PHASE));
    // e1:SetCountLimit(1,id,EFFECT_COUNT_CODE_OATH)
    effect::set_count_limit(d, e1, 1, ID, 0, EFFECT_COUNT_CODE_OATH as u8);
    // e1:SetTarget(s.target)
    effect::set_target(d, e1, Some(cost_target(target)));
    // e1:SetOperation(s.operation)
    effect::set_operation(d, e1, Some(operation(op)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
// function s.eqcfilter(c,tp)
fn eqcfilter(d: &mut Duel, c: CardId, tp: u8) -> bool {
    // return c:IsEquipSpell() and (c:IsSSetable()
    //     or Duel.IsExistingMatchingCard(s.eqtfilter,tp,LOCATION_MZONE,0,1,nil,c,tp))
    card::is_equip_spell(d, c)
        && (card::is_s_setable(d, c, false, None)
            || duel::is_existing_matching_card(d, Some(&|d, x| eqtfilter(d, x, c, tp)), tp, LOCATION_MZONE, 0, 1, NO_EX))
}
// function s.eqtfilter(c,ec,tp)
fn eqtfilter(d: &mut Duel, c: CardId, ec: CardId, tp: u8) -> bool {
    // return c:IsFaceup() and ec:CheckEquipTarget(c) and ec:CheckUniqueOnField(tp)
    card::is_faceup(d, c) && card::check_equip_target(d, ec, c) && card::check_unique_on_field(d, ec, tp, None, None)
}
// function s.target(e,tp,eg,ep,ev,re,r,rp,chk)
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // local phase=Duel.GetCurrentPhase()
    let phase = duel::get_current_phase(d);
    // local b1=Duel.IsExistingTarget(Card.IsFaceup,tp,LOCATION_MZONE,0,1,nil) and aux.StatChangeDamageStepCondition()
    let b1 = duel::is_existing_target(d, Some(&|d, c| card::is_faceup(d, c)), tp, LOCATION_MZONE, 0, 1, NO_EX) && aux::stat_change_damage_step_condition(d);
    // local ft=Duel.GetLocationCount(tp,LOCATION_SZONE)
    let mut ft = duel::get_location_count(d, tp, LOCATION_SZONE as u8, None, None, None).0;
    // if e:GetHandler():IsLocation(LOCATION_HAND) then ft=ft-1 end
    let h = effect::get_handler(d, a.e).nil();
    if card::is_location(d, h, LOCATION_HAND) {
        ft -= 1;
    }
    // local b2=ft>0 and Duel.IsExistingTarget(s.eqcfilter,tp,LOCATION_GRAVE,LOCATION_GRAVE,1,nil,tp) and phase~=PHASE_DAMAGE
    let b2 = ft > 0
        && duel::is_existing_target(d, Some(&|d, c| eqcfilter(d, c, tp)), tp, LOCATION_GRAVE, LOCATION_GRAVE, 1, NO_EX)
        && phase != PHASE_DAMAGE;
    // if chk==0 then return b1 or b2 end
    if a.chk == Some(0) {
        return b1 || b2;
    }
    // local op=Duel.SelectEffect(tp,
    //     {b1,aux.Stringid(id,1)},
    //     {b2,aux.Stringid(id,2)})
    let op = duel::select_effect(d, tp, &[(b1, aux::stringid(ID, 1)), (b2, aux::stringid(ID, 2))]);
    // e:SetLabel(op)
    effect::set_label(d, a.e, &[op.map_or(0, |v| v as i64)]);
    // if op==1 then
    if op == Some(1) {
        // e:SetCategory(CATEGORY_ATKCHANGE)
        effect::set_category(d, a.e, CATEGORY_ATKCHANGE);
        // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_ATKDEF)
        duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_ATKDEF as u64);
        // local tc=Duel.SelectTarget(tp,Card.IsFaceup,tp,LOCATION_MZONE,0,1,1,nil)
        let tc = duel::select_target(d, tp, Some(&|d, c| card::is_faceup(d, c)), tp, LOCATION_MZONE, 0, 1, 1, false, NO_EX);
        // Duel.SetOperationInfo(0,CATEGORY_ATKCHANGE,tc,1,tp,800)
        duel::set_operation_info(d, 0, CATEGORY_ATKCHANGE, tc, 1, tp, 800);
    // else
    } else {
        // e:SetCategory(CATEGORY_EQUIP+CATEGORY_SET)
        effect::set_category(d, a.e, CATEGORY_EQUIP + CATEGORY_SET);
        // Duel.Hint(HINT_SELECTMSG,tp,aux.Stringid(id,3))
        duel::hint(d, HINT_SELECTMSG as u8, tp, aux::stringid(ID, 3));
        // local tc=Duel.SelectTarget(tp,s.eqcfilter,tp,LOCATION_GRAVE,LOCATION_GRAVE,1,1,nil,tp):GetFirst()
        let sel = duel::select_target(d, tp, Some(&|d, c| eqcfilter(d, c, tp)), tp, LOCATION_GRAVE, LOCATION_GRAVE, 1, 1, false, NO_EX).nil();
        let tc = group::get_first(d, &sel).nil();
        // Duel.SetOperationInfo(0,CATEGORY_LEAVE_GRAVE,tc,1,tc:GetControler(),0)
        let tc_controler = card::get_controler(d, tc);
        duel::set_operation_info(d, 0, CATEGORY_LEAVE_GRAVE, Some(CardSet::from_card(tc)), 1, tc_controler, 0);
        // Duel.SetPossibleOperationInfo(0,CATEGORY_EQUIP,tc,1,tp,0)
        duel::set_possible_operation_info(d, 0, CATEGORY_EQUIP, Some(CardSet::from_card(tc)), 1, tp, 0);
    }
    true
}
// function s.operation(e,tp,eg,ep,ev,re,r,rp)
fn op(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // if e:GetLabel()==1 then
    if effect::get_label(d, a.e) == 1 {
        // local tc=Duel.GetFirstTarget()
        let tc = duel::first_target(d).nil();
        // if tc:IsFaceup() and tc:IsRelateToEffect(e) then
        if card::is_faceup(d, tc) && card::is_relate_to_effect(d, tc, a.e) {
            // tc:UpdateAttack(800,RESETS_STANDARD_PHASE_END,e:GetHandler())
            let h = effect::get_handler(d, a.e);
            card::update_attack(d, tc, 800, Some(RESETS_STANDARD_PHASE_END), h, None);
        }
    // else
    } else {
        // if Duel.GetLocationCount(tp,LOCATION_SZONE)==0 then return end
        if duel::get_location_count(d, tp, LOCATION_SZONE as u8, None, None, None).0 == 0 {
            return;
        }
        // local eqpc=Duel.GetFirstTarget()
        let eqpc = duel::first_target(d).nil();
        // if not eqpc:IsRelateToEffect(e) then return end
        if !card::is_relate_to_effect(d, eqpc, a.e) {
            return;
        }
        // local b1=eqpc:IsSSetable()
        let b1 = card::is_s_setable(d, eqpc, false, None);
        // local b2=Duel.IsExistingMatchingCard(s.eqtfilter,tp,LOCATION_MZONE,0,1,nil,eqpc,tp)
        let b2 = duel::is_existing_matching_card(d, Some(&|d, x| eqtfilter(d, x, eqpc, tp)), tp, LOCATION_MZONE, 0, 1, NO_EX);
        // if not (b1 or b2) then return end
        if !(b1 || b2) {
            return;
        }
        // local op=Duel.SelectEffect(tp,
        //     {b1,aux.Stringid(id,4)},
        //     {b2,aux.Stringid(id,5)})
        let sel = duel::select_effect(d, tp, &[(b1, aux::stringid(ID, 4)), (b2, aux::stringid(ID, 5))]);
        // if op==1 then
        if sel == Some(1) {
            // Duel.SSet(tp,eqpc)
            duel::s_set(d, tp, CardSet::from_card(eqpc), None, true);
        } else {
            // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_FACEUP)
            duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_FACEUP as u64);
            // local eqptg=Duel.SelectMatchingCard(tp,s.eqtfilter,tp,LOCATION_MZONE,0,1,1,nil,eqpc,tp):GetFirst()
            let sel2 = duel::select_matching_card(d, tp, Some(&|d, x| eqtfilter(d, x, eqpc, tp)), tp, LOCATION_MZONE, 0, 1, 1, false, NO_EX).nil();
            let eqptg = group::get_first(d, &sel2);
            // if not eqptg then return end
            let Some(eqptg) = eqptg else {
                return;
            };
            // Duel.HintSelection(eqptg,true)
            duel::hint_selection(d, CardSet::from_card(eqptg), true);
            // Duel.Equip(tp,eqpc,eqptg)
            duel::equip(d, tp, eqpc, eqptg, true, false);
        }
    }
}
