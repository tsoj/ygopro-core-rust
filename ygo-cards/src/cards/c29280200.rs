//! Vanquish Soul Dr. Mad Love
//! Scripted by Hatter
use super::prelude::*;

pub const ID: u32 = 29280200;

pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_VANQUISH_SOUL], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Search 1 "Vanquish Soul" Spell/Trap
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
    // e1:SetCode(EVENT_SUMMON_SUCCESS)
    effect::set_code(d, e1, EVENT_SUMMON_SUCCESS);
    // e1:SetCountLimit(1,id)
    effect::set_count_limit(d, e1, 1, ID, 0, 0);
    // e1:SetCost(s.opccost)
    effect::set_cost(d, e1, Some(cost_target(opccost)));
    // e1:SetTarget(s.thtg)
    effect::set_target(d, e1, Some(cost_target(thtg)));
    // e1:SetOperation(s.thop)
    effect::set_operation(d, e1, Some(operation(thop)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // local e2=e1:Clone()
    let e2 = effect::clone(d, e1);
    // e2:SetCode(EVENT_SPSUMMON_SUCCESS)
    effect::set_code(d, e2, EVENT_SPSUMMON_SUCCESS);
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
    // Reduce 1 opponent's monster's ATK/DEF, OR send 1 face-up monster on the field to the hand
    // local e3=Effect.CreateEffect(c)
    let e3 = effect::create_effect(d, c);
    // e3:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e3, aux::stringid(ID, 1));
    // e3:SetType(EFFECT_TYPE_QUICK_O)
    effect::set_type(d, e3, EFFECT_TYPE_QUICK_O);
    // e3:SetProperty(EFFECT_FLAG_DAMAGE_STEP)
    effect::set_property(d, e3, EFFECT_FLAG_DAMAGE_STEP, 0);
    // e3:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e3, EVENT_FREE_CHAIN);
    // e3:SetRange(LOCATION_MZONE)
    effect::set_range(d, e3, LOCATION_MZONE);
    // e3:SetHintTiming(TIMING_DAMAGE_STEP,TIMING_MAIN_END|TIMINGS_CHECK_MONSTER_E|TIMING_DAMAGE_STEP)
    effect::set_hint_timing(d, e3, TIMING_DAMAGE_STEP, Some(TIMING_MAIN_END | TIMINGS_CHECK_MONSTER_E | TIMING_DAMAGE_STEP));
    // e3:SetCountLimit(1,{id,1})
    effect::set_count_limit(d, e3, 1, ID, 1, 0);
    // e3:SetCost(s.opccost)
    effect::set_cost(d, e3, Some(cost_target(opccost)));
    // e3:SetTarget(s.vstg)
    effect::set_target(d, e3, Some(cost_target(vstg)));
    // e3:SetOperation(s.vsop)
    effect::set_operation(d, e3, Some(operation(vsop)));
    // c:RegisterEffect(e3)
    card::register_effect(d, c, e3, false);
}
/// `s.opccost(e,tp,eg,ep,ev,re,r,rp,chk)`
fn opccost(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return Duel.GetFlagEffect(tp,id)==0 end
    if a.chk == Some(0) {
        return duel::get_flag_effect(d, a.tp, ID) == 0;
    }
    // Duel.RegisterFlagEffect(tp,id,RESET_CHAIN,0,1)
    duel::register_flag_effect(d, a.tp, ID, RESET_CHAIN, 0, 1, 0);
    true
}
/// `s.thfilter(c)`
fn thfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsSetCard(SET_VANQUISH_SOUL) and c:IsSpellTrap() and c:IsAbleToHand()
    card::is_set_card(d, c, &[SET_VANQUISH_SOUL as u16]) && card::is_spell_trap(d, c) && card::is_able_to_hand(d, c, None)
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
/// `s.vscostfilter(c,att)`
fn vscostfilter(d: &mut Duel, c: CardId, att: u32) -> bool {
    // return c:IsAttribute(att) and not c:IsPublic()
    card::is_attribute(d, c, att) && !card::is_public(d, c)
}
/// `s.vsrescon(sg)`
fn vsrescon(d: &mut Duel, sg: &CardSet, _e: Option<EffectId>, _tp: u8, _mg: &CardSet, _c: Option<CardId>) -> (bool, bool) {
    // return sg:FilterCount(Card.IsAttribute,nil,ATTRIBUTE_DARK)>0
    //     and sg:FilterCount(Card.IsAttribute,nil,ATTRIBUTE_EARTH)>0
    let res = group::filter_count(d, sg, &|d, c| card::is_attribute(d, c, ATTRIBUTE_DARK), NO_EX) > 0
        && group::filter_count(d, sg, &|d, c| card::is_attribute(d, c, ATTRIBUTE_EARTH), NO_EX) > 0;
    (res, false)
}
/// `s.vstg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn vstg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // local not_dmg_step=Duel.GetCurrentPhase()~=PHASE_DAMAGE
    let not_dmg_step = duel::get_current_phase(d) != PHASE_DAMAGE;
    // local cg1=Duel.GetMatchingGroup(s.vscostfilter,tp,LOCATION_HAND,0,nil,ATTRIBUTE_DARK)
    let cg1 = duel::get_matching_group(d, Some(&|d, c| vscostfilter(d, c, ATTRIBUTE_DARK)), tp, LOCATION_HAND, 0, NO_EX);
    // local fg1=Duel.GetMatchingGroup(Card.IsFaceup,tp,0,LOCATION_MZONE,nil)
    let fg1 = duel::get_matching_group(d, Some(&|d, c| card::is_faceup(d, c)), tp, 0, LOCATION_MZONE, NO_EX);
    // local b1=#cg1>0 and #fg1>0 and (not_dmg_step or not Duel.IsDamageCalculated())
    let b1 = cg1.len() > 0 && fg1.len() > 0 && (not_dmg_step || !duel::is_damage_calculated(d));
    // local cg2=cg1+Duel.GetMatchingGroup(s.vscostfilter,tp,LOCATION_HAND,0,nil,ATTRIBUTE_EARTH)
    let mut cg2 = cg1.clone();
    let cg_e = duel::get_matching_group(d, Some(&|d, c| vscostfilter(d, c, ATTRIBUTE_EARTH)), tp, LOCATION_HAND, 0, NO_EX);
    group::add_card(&mut cg2, &cg_e);
    // local fg2=Duel.GetMatchingGroup(aux.FaceupFilter(Card.IsAbleToHand),tp,LOCATION_MZONE,LOCATION_MZONE,nil)
    let fg2 = duel::get_matching_group(
        d,
        Some(&aux::faceup_filter(|d, c| card::is_able_to_hand(d, c, None))),
        tp,
        LOCATION_MZONE,
        LOCATION_MZONE,
        NO_EX,
    );
    // local b2=#fg2>0 and aux.SelectUnselectGroup(cg2,e,tp,1,2,s.vsrescon,0) and not_dmg_step
    let rs: aux::Rescon = &vsrescon;
    let b2 = fg2.len() > 0
        && matches!(aux::select_unselect_group(d, &cg2, Some(a.e), tp, Some(1), Some(2), Some(rs), 0, tp, None, None, None, false), Err(true))
        && not_dmg_step;
    // if chk==0 then return b1 or b2 end
    if a.chk == Some(0) {
        return b1 || b2;
    }
    // local op=Duel.SelectEffect(tp, {b1,aux.Stringid(id,2)}, {b2,aux.Stringid(id,3)})
    let op = duel::select_effect(d, tp, &[(b1, aux::stringid(ID, 2)), (b2, aux::stringid(ID, 3))]);
    // e:SetLabel(op)
    effect::set_label(d, a.e, &[op.map_or(0, |v| v as i64)]);
    // if op==1 then
    if op == Some(1) {
        // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_CONFIRM)
        duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_CONFIRM as u64);
        // local g=cg1:Select(tp,1,1,nil)
        let g = group::select(d, &cg1, tp, 1, 1, false, NO_EX).nil();
        // Duel.ConfirmCards(1-tp,g)
        duel::confirm_cards(d, 1 - tp, g);
        // Duel.ShuffleHand(tp)
        duel::shuffle_hand(d, tp);
        // e:SetCategory(CATEGORY_ATKCHANGE+CATEGORY_DEFCHANGE)
        effect::set_category(d, a.e, CATEGORY_ATKCHANGE + CATEGORY_DEFCHANGE);
    // elseif op==2 then
    } else if op == Some(2) {
        // local g=aux.SelectUnselectGroup(cg2,e,tp,1,2,s.vsrescon,1,tp,HINTMSG_CONFIRM,s.vsrescon)
        let g = aux::select_unselect_group(
            d,
            &cg2,
            Some(a.e),
            tp,
            Some(1),
            Some(2),
            Some(rs),
            1,
            tp,
            Some(HINTMSG_CONFIRM as u64),
            Some(rs),
            None,
            false,
        )
        .unwrap_or_default();
        // Duel.ConfirmCards(1-tp,g)
        duel::confirm_cards(d, 1 - tp, g);
        // Duel.ShuffleHand(tp)
        duel::shuffle_hand(d, tp);
        // e:SetCategory(CATEGORY_TOHAND)
        effect::set_category(d, a.e, CATEGORY_TOHAND);
        // Duel.SetOperationInfo(0,CATEGORY_TOHAND,nil,1,PLAYER_EITHER,LOCATION_MZONE)
        duel::set_operation_info(d, 0, CATEGORY_TOHAND, None, 1, PLAYER_EITHER as u8, LOCATION_MZONE as i32);
    }
    true
}
/// `s.vsop(e,tp,eg,ep,ev,re,r,rp)`
fn vsop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // local op=e:GetLabel()
    let op = effect::get_label(d, a.e);
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if op==1 then
    if op == 1 {
        // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_ATKDEF)
        duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_ATKDEF as u64);
        // local tc=Duel.SelectMatchingCard(tp,Card.IsFaceup,tp,0,LOCATION_MZONE,1,1,nil):GetFirst()
        let sel = duel::select_matching_card(d, tp, Some(&|d, c| card::is_faceup(d, c)), tp, 0, LOCATION_MZONE, 1, 1, false, NO_EX).nil();
        // if not tc then return end
        let Some(tc) = group::get_first(d, &sel) else {
            return;
        };
        // Duel.HintSelection(tc,true)
        duel::hint_selection(d, CardSet::from_card(tc), true);
        // tc:UpdateAttack(-500,RESET_EVENT|RESETS_STANDARD,c)
        card::update_attack(d, tc, -500, Some(RESET_EVENT + RESETS_STANDARD), Some(c), None);
        // tc:UpdateDefense(-500,RESET_EVENT|RESETS_STANDARD,c)
        card::update_defense(d, tc, -500, Some(RESET_EVENT + RESETS_STANDARD), Some(c), None);
    // elseif op==2 then
    } else if op == 2 {
        // local g2=Duel.GetMatchingGroup(aux.FaceupFilter(Card.IsDefenseAbove,0),tp,LOCATION_MZONE,LOCATION_MZONE,nil):GetMinGroup(Card.GetDefense)
        let all = duel::get_matching_group(
            d,
            Some(&aux::faceup_filter(|d, x| card::is_defense_above(d, x, 0))),
            tp,
            LOCATION_MZONE,
            LOCATION_MZONE,
            NO_EX,
        );
        let mut g2 = group::get_min_group(d, &all, &|d, x| card::get_defense(d, x) as i64).map(|(g, _)| g).unwrap_or_default();
        // if not g2 or #g2==0 then return end
        if g2.is_empty() {
            return;
        }
        // if #g2>1 then
        if g2.len() > 1 {
            // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_RTOHAND)
            duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_RTOHAND as u64);
            // g2=g2:FilterSelect(tp,Card.IsAbleToHand,1,1,nil)
            g2 = group::filter_select(d, &g2, tp, &|d, x| card::is_able_to_hand(d, x, None), 1, 1, false, NO_EX).nil();
        }
        // if #g2==0 then return end
        if g2.is_empty() {
            return;
        }
        // Duel.HintSelection(g2,true)
        duel::hint_selection(d, g2.clone(), true);
        // Duel.SendtoHand(g2,nil,REASON_EFFECT)
        duel::send_to_hand(d, g2, None, REASON_EFFECT, None);
    }
}
