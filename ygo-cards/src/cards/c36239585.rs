//! Ghostrick Fairy
use super::prelude::*;

pub const ID: u32 = 36239585;

pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_GHOSTRICK], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Cannot be normal summoned if player controls no "Ghostrick" monster
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
    // e1:SetCode(EFFECT_CANNOT_SUMMON)
    effect::set_code(d, e1, EFFECT_CANNOT_SUMMON);
    // e1:SetCondition(s.sumcon)
    effect::set_condition(d, e1, Some(lua_fn(sumcon)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // Change itself to face-down defense position
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e2, aux::stringid(ID, 0));
    // e2:SetCategory(CATEGORY_POSITION+CATEGORY_SET)
    effect::set_category(d, e2, CATEGORY_POSITION + CATEGORY_SET);
    // e2:SetType(EFFECT_TYPE_IGNITION)
    effect::set_type(d, e2, EFFECT_TYPE_IGNITION);
    // e2:SetRange(LOCATION_MZONE)
    effect::set_range(d, e2, LOCATION_MZONE);
    // e2:SetTarget(s.postg)
    effect::set_target(d, e2, Some(cost_target(postg)));
    // e2:SetOperation(s.posop)
    effect::set_operation(d, e2, Some(operation(posop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
    // Set 1 "Ghostrick" card from GY
    // local e3=Effect.CreateEffect(c)
    let e3 = effect::create_effect(d, c);
    // e3:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e3, aux::stringid(ID, 1));
    // e3:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e3, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_O);
    // e3:SetProperty(EFFECT_FLAG_CARD_TARGET+EFFECT_FLAG_DAMAGE_STEP)
    effect::set_property(d, e3, EFFECT_FLAG_CARD_TARGET + EFFECT_FLAG_DAMAGE_STEP, 0);
    // e3:SetCode(EVENT_FLIP)
    effect::set_code(d, e3, EVENT_FLIP);
    // e3:SetTarget(s.settg)
    effect::set_target(d, e3, Some(cost_target(settg)));
    // e3:SetOperation(s.setop)
    effect::set_operation(d, e3, Some(operation(setop)));
    // c:RegisterEffect(e3)
    card::register_effect(d, c, e3, false);
}
/// `s.sumcon(e)`
fn sumcon(d: &mut Duel, a: Args) -> bool {
    // return not Duel.IsExistingMatchingCard(aux.FaceupFilter(Card.IsSetCard,SET_GHOSTRICK),e:GetHandlerPlayer(),LOCATION_MZONE,0,1,nil)
    let p = effect::get_handler_player(d, a.e(0));
    let f = aux::faceup_filter(|d, x| card::is_set_card(d, x, &[SET_GHOSTRICK as u16]));
    !duel::is_existing_matching_card(d, Some(&f), p, LOCATION_MZONE, 0, 1, NO_EX)
}
/// `s.postg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn postg(d: &mut Duel, a: &EffArgs) -> bool {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if chk==0 then return c:IsCanTurnSet() and c:GetFlagEffect(id)==0 end
    if a.chk == Some(0) {
        return card::is_can_turn_set(d, c) && card::get_flag_effect(d, c, ID) == 0;
    }
    // c:RegisterFlagEffect(id,RESET_EVENT|(RESETS_STANDARD&~RESET_TURN_SET)|RESET_PHASE|PHASE_END,0,1)
    card::register_flag_effect(d, c, ID, RESET_EVENT | (RESETS_STANDARD & !RESET_TURN_SET) | RESET_PHASE | PHASE_END, 0, 1, 0, 0);
    // Duel.SetOperationInfo(0,CATEGORY_POSITION,c,1,tp,POS_FACEDOWN_DEFENSE)
    duel::set_operation_info(d, 0, CATEGORY_POSITION, Some(CardSet::from_card(c)), 1, a.tp, POS_FACEDOWN_DEFENSE as i32);
    true
}
/// `s.posop(e,tp,eg,ep,ev,re,r,rp)`
fn posop(d: &mut Duel, a: &EffArgs) {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if c:IsRelateToEffect(e) and c:IsFaceup() then
    if card::is_relate_to_effect(d, c, a.e) && card::is_faceup(d, c) {
        // Duel.ChangePosition(c,POS_FACEDOWN_DEFENSE)
        duel::change_position(d, c, POS_FACEDOWN_DEFENSE as u8, None, None, None, false);
    }
}
/// `s.setfilter(c,e,tp)`
fn setfilter(d: &mut Duel, c: CardId, e: EffectId, tp: u8) -> bool {
    // if not c:IsSetCard(SET_GHOSTRICK) then return end
    if !card::is_set_card(d, c, &[SET_GHOSTRICK as u16]) {
        return false;
    }
    // if c:IsMonster() then
    if card::is_monster(d, c) {
        // return Duel.GetLocationCount(tp,LOCATION_MZONE)>0 and c:IsCanBeSpecialSummoned(e,0,tp,false,false,POS_FACEDOWN_DEFENSE)
        duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 > 0
            && card::is_can_be_special_summoned(d, c, e, 0, tp, false, false, Some(POS_FACEDOWN_DEFENSE as u8), None, None)
    // elseif c:IsSpellTrap() then
    } else if card::is_spell_trap(d, c) {
        // return (c:IsFieldSpell() or Duel.GetLocationCount(tp,LOCATION_SZONE)>0) and c:IsSSetable()
        (card::is_field_spell(d, c) || duel::get_location_count(d, tp, LOCATION_SZONE as u8, None, None, None).0 > 0)
            && card::is_s_setable(d, c, false, None)
    } else {
        // return false (end)
        false
    }
}
/// `s.settg(e,tp,eg,ep,ev,re,r,rp,chk,chkc)`
fn settg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chkc then return chkc:IsLocation(LOCATION_GRAVE) and chkc:IsControler(tp) and s.setfilter(chkc,e,tp) end
    if let Some(chkc) = a.chkc() {
        return card::is_location(d, chkc, LOCATION_GRAVE) && card::is_controler(d, chkc, a.tp) && setfilter(d, chkc, a.e, a.tp);
    }
    // if chk==0 then return Duel.IsExistingTarget(s.setfilter,tp,LOCATION_GRAVE,0,1,nil,e,tp) end
    if a.chk == Some(0) {
        return duel::is_existing_target(d, Some(&|d, x| setfilter(d, x, a.e, a.tp)), a.tp, LOCATION_GRAVE, 0, 1, NO_EX);
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_SET)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_SET as u64);
    // local tc=Duel.SelectTarget(tp,s.setfilter,tp,LOCATION_GRAVE,0,1,1,nil,e,tp):GetFirst()
    let sel = duel::select_target(d, a.tp, Some(&|d, x| setfilter(d, x, a.e, a.tp)), a.tp, LOCATION_GRAVE, 0, 1, 1, false, NO_EX).nil();
    let tc = group::get_first(d, &sel).nil();
    // if tc:IsMonster() then
    if card::is_monster(d, tc) {
        // e:SetCategory(CATEGORY_SPECIAL_SUMMON+CATEGORY_SET)
        effect::set_category(d, a.e, CATEGORY_SPECIAL_SUMMON + CATEGORY_SET);
        // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,tc,1,tp,LOCATION_GRAVE)
        duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, Some(CardSet::from_card(tc)), 1, a.tp, LOCATION_GRAVE as i32);
    // elseif tc:IsSpellTrap() then
    } else if card::is_spell_trap(d, tc) {
        // e:SetCategory(CATEGORY_SET)
        effect::set_category(d, a.e, CATEGORY_SET);
        // Duel.SetOperationInfo(0,CATEGORY_LEAVE_GRAVE,tc,1,tp,LOCATION_GRAVE)
        duel::set_operation_info(d, 0, CATEGORY_LEAVE_GRAVE, Some(CardSet::from_card(tc)), 1, a.tp, LOCATION_GRAVE as i32);
    }
    // Duel.SetPossibleOperationInfo(0,CATEGORY_POSITION,nil,1,1-tp,POS_FACEDOWN_DEFENSE)
    duel::set_possible_operation_info(d, 0, CATEGORY_POSITION, None, 1, 1 - a.tp, POS_FACEDOWN_DEFENSE as i32);
    true
}
/// `s.setop(e,tp,eg,ep,ev,re,r,rp)`
fn setop(d: &mut Duel, a: &EffArgs) {
    // local tc=Duel.GetFirstTarget()
    let tc = duel::first_target(d);
    // if not tc or not tc:IsRelateToEffect(e) then return end
    let tc = match tc {
        Some(tc) if card::is_relate_to_effect(d, tc, a.e) => tc,
        _ => return,
    };
    // local andifyoudo=false
    let mut andifyoudo = false;
    // if tc:IsMonster() then
    if card::is_monster(d, tc) {
        // if Duel.GetLocationCount(tp,LOCATION_MZONE)>0 and Duel.SpecialSummon(tc,0,tp,tp,false,false,POS_FACEDOWN_DEFENSE)>0 then
        if duel::get_location_count(d, a.tp, LOCATION_MZONE as u8, None, None, None).0 > 0
            && duel::special_summon(d, tc, 0, a.tp, a.tp, false, false, POS_FACEDOWN_DEFENSE as u8, None) > 0
        {
            // andifyoudo=true
            andifyoudo = true;
        }
    // elseif tc:IsSpellTrap() then
    } else if card::is_spell_trap(d, tc) {
        // if tc:IsFieldSpell() then
        if card::is_field_spell(d, tc) {
            // local fc=Duel.GetFieldCard(tp,LOCATION_FZONE,0)
            let fc = duel::get_field_card(d, a.tp, LOCATION_FZONE, 0);
            // if fc then
            if let Some(fc) = fc {
                // Duel.SendtoGrave(fc,REASON_RULE)
                duel::send_to_grave(d, fc, REASON_RULE, None, None);
                // Duel.BreakEffect()
                duel::break_effect(d);
            }
        }
        // if Duel.GetLocationCount(tp,LOCATION_SZONE)>0 then
        if duel::get_location_count(d, a.tp, LOCATION_SZONE as u8, None, None, None).0 > 0 {
            // Duel.SSet(tp,tc)
            duel::s_set(d, a.tp, tc, None, true);
            // andifyoudo=true
            andifyoudo = true;
        }
    }
    // if andifyoudo then
    if andifyoudo {
        // local e1=Effect.CreateEffect(e:GetHandler())
        let h = effect::get_handler(d, a.e).nil();
        let e1 = effect::create_effect(d, h);
        // e1:SetDescription(3300)
        effect::set_description(d, e1, 3300);
        // e1:SetType(EFFECT_TYPE_SINGLE)
        effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
        // e1:SetCode(EFFECT_LEAVE_FIELD_REDIRECT)
        effect::set_code(d, e1, EFFECT_LEAVE_FIELD_REDIRECT);
        // e1:SetProperty(EFFECT_FLAG_CANNOT_DISABLE+EFFECT_FLAG_CLIENT_HINT)
        effect::set_property(d, e1, EFFECT_FLAG_CANNOT_DISABLE + EFFECT_FLAG_CLIENT_HINT, 0);
        // e1:SetValue(LOCATION_REMOVED)
        effect::set_value(d, e1, LOCATION_REMOVED as i64);
        // e1:SetReset(RESET_EVENT|RESETS_REDIRECT)
        effect::set_reset(d, e1, RESET_EVENT | RESETS_REDIRECT, 1);
        // tc:RegisterEffect(e1)
        card::register_effect(d, tc, e1, false);
        // if Duel.IsExistingMatchingCard(Card.IsCanTurnSet,tp,0,LOCATION_MZONE,1,nil) and Duel.SelectYesNo(tp,aux.Stringid(id,2)) then
        if duel::is_existing_matching_card(d, Some(&|d, x| card::is_can_turn_set(d, x)), a.tp, 0, LOCATION_MZONE, 1, NO_EX)
            && duel::select_yes_no(d, a.tp, aux::stringid(ID, 2))
        {
            // Duel.BreakEffect()
            duel::break_effect(d);
            // local ct=Duel.GetMatchingGroupCount(Card.IsFacedown,tp,LOCATION_ONFIELD,0,nil)
            let ct = duel::get_matching_group_count(d, Some(&|d, x| card::is_facedown(d, x)), a.tp, LOCATION_ONFIELD, 0, NO_EX);
            // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_POSCHANGE)
            duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_POSCHANGE as u64);
            // local g=Duel.SelectMatchingCard(tp,Card.IsCanTurnSet,tp,0,LOCATION_MZONE,1,ct,nil)
            let g = duel::select_matching_card(d, a.tp, Some(&|d, x| card::is_can_turn_set(d, x)), a.tp, 0, LOCATION_MZONE, 1, ct as u16, false, NO_EX).nil();
            // Duel.HintSelection(g)
            duel::hint_selection(d, g.clone(), true);
            // Duel.ChangePosition(g,POS_FACEDOWN_DEFENSE)
            duel::change_position(d, g, POS_FACEDOWN_DEFENSE as u8, None, None, None, false);
        }
    }
}
