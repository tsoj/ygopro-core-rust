//! Vouiburial, the Dragon Undertaker
//! Scripted by Eerie Code
use super::prelude::*;

pub const ID: u32 = 36974120;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Neither monster can be destroyed by battle
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e1, EFFECT_TYPE_FIELD);
    // e1:SetCode(EFFECT_INDESTRUCTABLE_BATTLE)
    effect::set_code(d, e1, EFFECT_INDESTRUCTABLE_BATTLE);
    // e1:SetRange(LOCATION_MZONE)
    effect::set_range(d, e1, LOCATION_MZONE);
    // e1:SetTargetRange(LOCATION_MZONE,LOCATION_MZONE)
    effect::set_target_range(d, e1, LOCATION_MZONE, LOCATION_MZONE);
    // e1:SetTarget(s.indestg)
    effect::set_target(d, e1, Some(effect_card_fn(indestg)));
    // e1:SetValue(1)
    effect::set_value(d, e1, 1);
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // Activate the appropriate effect based on where this card is
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e2, aux::stringid(ID, 0));
    // e2:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e2, EFFECT_TYPE_FIELD + EFFECT_TYPE_TRIGGER_O);
    // e2:SetProperty(EFFECT_FLAG_DELAY)
    effect::set_property(d, e2, EFFECT_FLAG_DELAY, 0);
    // e2:SetCode(EVENT_TO_GRAVE)
    effect::set_code(d, e2, EVENT_TO_GRAVE);
    // e2:SetRange(LOCATION_HAND|LOCATION_MZONE)
    effect::set_range(d, e2, LOCATION_HAND | LOCATION_MZONE);
    // e2:SetCondition(s.effcon)
    effect::set_condition(d, e2, Some(condition(effcon)));
    // e2:SetTarget(s.efftg)
    effect::set_target(d, e2, Some(cost_target(efftg)));
    // e2:SetOperation(s.effop)
    effect::set_operation(d, e2, Some(operation(effop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
}
/// `s.indestg(e,c)`
fn indestg(d: &mut Duel, e: EffectId, c: CardId) -> bool {
    // local handler=e:GetHandler()
    let handler = effect::get_handler(d, e).nil();
    // return c==handler or c==handler:GetBattleTarget()
    c == handler || card::get_battle_target(d, handler) == Some(c)
}
/// `s.effconfilter(c,tp)`
fn effconfilter(d: &mut Duel, c: CardId, tp: u8) -> bool {
    // return c:IsPreviousLocation(LOCATION_MZONE) and c:IsPreviousControler(1-tp)
    card::is_previous_location(d, c, LOCATION_MZONE) && card::is_previous_controler(d, c, 1 - tp)
}
/// `s.effcon(e,tp,eg,ep,ev,re,r,rp)`
fn effcon(d: &mut Duel, a: &EffArgs) -> bool {
    // return Duel.IsMainPhase() and eg:IsExists(s.effconfilter,1,nil,tp)
    let eg = a.eg.clone().nil();
    duel::is_main_phase(d, None) && group::is_exists(d, &eg, &|d, c| effconfilter(d, c, a.tp), 1, NO_EX)
}
/// `s.efftg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn efftg(d: &mut Duel, a: &EffArgs) -> bool {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    let tp = a.tp;
    // local b1=not Duel.HasFlagEffect(tp,id) and c:IsLocation(LOCATION_HAND)
    //     and Duel.GetLocationCount(tp,LOCATION_MZONE)>0
    //     and c:IsCanBeSpecialSummoned(e,0,tp,false,false)
    let b1 = !duel::has_flag_effect(d, tp, ID, None)
        && card::is_location(d, c, LOCATION_HAND)
        && duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 > 0
        && card::is_can_be_special_summoned(d, c, a.e, 0, tp, false, false, None, None, None);
    // local b2=not Duel.HasFlagEffect(tp,id+1) and c:IsLocation(LOCATION_MZONE)
    //     and Duel.IsExistingMatchingCard(aux.FaceupFilter(Card.IsType,TYPE_EFFECT),tp,0,LOCATION_MZONE,1,nil)
    let b2 = !duel::has_flag_effect(d, tp, ID + 1, None)
        && card::is_location(d, c, LOCATION_MZONE)
        && duel::is_existing_matching_card(d, Some(&aux::faceup_filter(|d, c| card::is_type(d, c, TYPE_EFFECT))), tp, 0, LOCATION_MZONE, 1, NO_EX);
    // if chk==0 then return b1 or b2 end
    if a.chk == Some(0) {
        return b1 || b2;
    }
    // local op=b1 and 1 or 2
    let op = if b1 { 1 } else { 2 };
    // e:SetLabel(op)
    effect::set_label(d, a.e, &[op]);
    if op == 1 {
        // e:SetCategory(CATEGORY_SPECIAL_SUMMON)
        effect::set_category(d, a.e, CATEGORY_SPECIAL_SUMMON);
        // Duel.RegisterFlagEffect(tp,id,RESET_PHASE|PHASE_END,0,1)
        duel::register_flag_effect(d, tp, ID, RESET_PHASE | PHASE_END, 0, 1, 0);
        // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,c,1,tp,0)
        duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, Some(CardSet::from_card(c)), 1, tp, 0);
    } else if op == 2 {
        // e:SetCategory(CATEGORY_ATKCHANGE|CATEGORY_DESTROY)
        effect::set_category(d, a.e, CATEGORY_ATKCHANGE | CATEGORY_DESTROY);
        // Duel.RegisterFlagEffect(tp,id+1,RESET_PHASE|PHASE_END,0,1)
        duel::register_flag_effect(d, tp, ID + 1, RESET_PHASE | PHASE_END, 0, 1, 0);
        // Duel.SetPossibleOperationInfo(0,CATEGORY_DESTROY,nil,1,1-tp,LOCATION_MZONE)
        duel::set_possible_operation_info(d, 0, CATEGORY_DESTROY as u64, None, 1, 1 - tp, LOCATION_MZONE as i32);
    }
    true
}
/// `s.effop(e,tp,eg,ep,ev,re,r,rp)`
fn effop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // local op=e:GetLabel()
    let op = effect::get_label(d, a.e);
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    if op == 1 {
        // --● Hand: Special Summon this card
        // if c:IsRelateToEffect(e) then
        if card::is_relate_to_effect(d, c, a.e) {
            // Duel.SpecialSummon(c,0,tp,tp,false,false,POS_FACEUP)
            duel::special_summon(d, c, 0, tp, tp, false, false, POS_FACEUP as u8, None);
        }
    } else if op == 2 {
        // --● Field: Make 1 Effect Monster your opponent controls lose 1500 ATK, then if its ATK becomes 0 by this effect, you can destroy it
        // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_ATKDEF)
        duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_ATKDEF as u64);
        // local tc=Duel.SelectMatchingCard(tp,aux.FaceupFilter(Card.IsType,TYPE_EFFECT),tp,0,LOCATION_MZONE,1,1,nil):GetFirst()
        let g = duel::select_matching_card(d, tp, Some(&aux::faceup_filter(|d, c| card::is_type(d, c, TYPE_EFFECT))), tp, 0, LOCATION_MZONE, 1, 1, false, NO_EX).nil();
        // if not tc then return end
        let Some(tc) = group::get_first(d, &g) else { return };
        // Duel.HintSelection(tc)
        duel::hint_selection(d, CardSet::from_card(tc), true);
        // local atk=tc:GetAttack()
        let atk = card::get_attack(d, tc);
        // --Loses 1500 ATK
        // local e1=Effect.CreateEffect(c)
        let e1 = effect::create_effect(d, c);
        // e1:SetType(EFFECT_TYPE_SINGLE)
        effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
        // e1:SetProperty(EFFECT_FLAG_CANNOT_DISABLE)
        effect::set_property(d, e1, EFFECT_FLAG_CANNOT_DISABLE, 0);
        // e1:SetCode(EFFECT_UPDATE_ATTACK)
        effect::set_code(d, e1, EFFECT_UPDATE_ATTACK);
        // e1:SetValue(-1500)
        effect::set_value(d, e1, -1500);
        // e1:SetReset(RESET_EVENT|RESETS_STANDARD)
        effect::set_reset(d, e1, RESET_EVENT | RESETS_STANDARD, 1);
        // tc:RegisterEffect(e1)
        card::register_effect(d, tc, e1, false);
        // Duel.AdjustInstantly(tc)
        duel::adjust_instantly(d, Some(tc));
        // if atk>0 and tc:IsAttack(0) and Duel.SelectYesNo(tp,aux.Stringid(id,1)) then
        if atk > 0 && card::is_attack(d, tc, &[0]) && duel::select_yes_no(d, tp, aux::stringid(ID, 1)) {
            // Duel.BreakEffect()
            duel::break_effect(d);
            // Duel.Destroy(tc,REASON_EFFECT)
            duel::destroy(d, tc, REASON_EFFECT, None, None);
        }
    }
}
