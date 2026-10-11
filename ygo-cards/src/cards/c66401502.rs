//! VS Pantera
//! Scripted by Hatter
use super::prelude::*;

pub const ID: u32 = 66401502;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Special Summon this card from your hand
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_SPECIAL_SUMMON)
    effect::set_category(d, e1, CATEGORY_SPECIAL_SUMMON);
    // e1:SetType(EFFECT_TYPE_IGNITION)
    effect::set_type(d, e1, EFFECT_TYPE_IGNITION);
    // e1:SetRange(LOCATION_HAND)
    effect::set_range(d, e1, LOCATION_HAND);
    // e1:SetCountLimit(1,id)
    effect::set_count_limit(d, e1, 1, ID, 0, 0);
    // e1:SetCondition(function(_,tp) return Duel.GetFieldGroupCount(tp,LOCATION_MMZONE,0)==0 end)
    effect::set_condition(d, e1, Some(condition(|d, a| duel::get_field_group_count(d, a.tp, LOCATION_MMZONE, 0) == 0)));
    // e1:SetCost(Cost.HardOncePerChain(id))
    effect::set_cost(d, e1, Some(cost::hard_once_per_chain(ID, None)));
    // e1:SetTarget(s.sptg)
    effect::set_target(d, e1, Some(cost_target(sptg)));
    // e1:SetOperation(s.spop)
    effect::set_operation(d, e1, Some(operation(spop)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // Activate 1 of these effects, by revealing monster(s) in your hand with the listed Attribute(s)
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e2, aux::stringid(ID, 1));
    // e2:SetType(EFFECT_TYPE_QUICK_O)
    effect::set_type(d, e2, EFFECT_TYPE_QUICK_O);
    // e2:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e2, EVENT_FREE_CHAIN);
    // e2:SetRange(LOCATION_MZONE)
    effect::set_range(d, e2, LOCATION_MZONE);
    // e2:SetCountLimit(1,{id,1})
    effect::set_count_limit(d, e2, 1, ID, 1, 0);
    // e2:SetHintTiming(0,TIMING_STANDBY_PHASE|TIMING_MAIN_END|TIMINGS_CHECK_MONSTER_E)
    effect::set_hint_timing(d, e2, 0, Some(TIMING_STANDBY_PHASE | TIMING_MAIN_END | TIMINGS_CHECK_MONSTER_E));
    // e2:SetCost(Cost.HardOncePerChain(id))
    effect::set_cost(d, e2, Some(cost::hard_once_per_chain(ID, None)));
    // e2:SetTarget(s.vstg)
    effect::set_target(d, e2, Some(cost_target(vstg)));
    // e2:SetOperation(s.vsop)
    effect::set_operation(d, e2, Some(operation(vsop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
}
/// `s.sptg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn sptg(d: &mut Duel, a: &EffArgs) -> bool {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if chk==0 then return Duel.GetLocationCount(tp,LOCATION_MZONE)>0
    //     and c:IsCanBeSpecialSummoned(e,0,tp,false,false) end
    if a.chk == Some(0) {
        return duel::get_location_count(d, a.tp, LOCATION_MZONE as u8, None, None, None).0 > 0 && card::is_can_be_special_summoned(d, c, a.e, 0, a.tp, false, false, None, None, None);
    }
    // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,c,1,tp,0)
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, Some(CardSet::from_card(c)), 1, a.tp, 0);
    true
}
/// `s.spop(e,tp,eg,ep,ev,re,r,rp)`
fn spop(d: &mut Duel, a: &EffArgs) {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if c:IsRelateToEffect(e) then
    if card::is_relate_to_effect(d, c, a.e) {
        // Duel.SpecialSummon(c,0,tp,tp,false,false,POS_FACEUP)
        duel::special_summon(d, c, 0, a.tp, a.tp, false, false, POS_FACEUP as u8, None);
    }
}
/// `s.vscostfilter(c,att)`
fn vscostfilter(d: &mut Duel, c: CardId, att: u32) -> bool {
    // return c:IsAttribute(att) and not c:IsPublic()
    card::is_attribute(d, c, att) && !card::is_public(d, c)
}
/// `s.vsrescon(sg)`
fn vsrescon(d: &mut Duel, sg: &CardSet, _e: Option<EffectId>, _tp: u8, _mg: &CardSet, _c: Option<CardId>) -> (bool, bool) {
    // return sg:FilterCount(Card.IsAttribute,nil,ATTRIBUTE_EARTH)>0
    //     and sg:FilterCount(Card.IsAttribute,nil,ATTRIBUTE_FIRE)>0
    let earth = group::filter_count(d, sg, &|d, c| card::is_attribute(d, c, ATTRIBUTE_EARTH), NO_EX) > 0;
    let fire = group::filter_count(d, sg, &|d, c| card::is_attribute(d, c, ATTRIBUTE_FIRE), NO_EX) > 0;
    (earth && fire, false)
}
/// `s.vstg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn vstg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // local cg1=Duel.GetMatchingGroup(s.vscostfilter,tp,LOCATION_HAND,0,nil,ATTRIBUTE_EARTH)
    let cg1 = duel::get_matching_group(d, Some(&|d, c| vscostfilter(d, c, ATTRIBUTE_EARTH)), tp, LOCATION_HAND, 0, NO_EX);
    // local b1=#cg1>0 and (Duel.IsAbleToEnterBP() or Duel.IsBattlePhase())
    let b1 = !cg1.is_empty() && (duel::is_able_to_enter_bp(d) || duel::is_battle_phase(d, None));
    // local cg2=cg1+Duel.GetMatchingGroup(s.vscostfilter,tp,LOCATION_HAND,0,nil,ATTRIBUTE_FIRE)
    let fire = duel::get_matching_group(d, Some(&|d, c| vscostfilter(d, c, ATTRIBUTE_FIRE)), tp, LOCATION_HAND, 0, NO_EX);
    let mut cg2 = cg1.clone();
    group::add_card(&mut cg2, &fire);
    // local colg=e:GetHandler():GetColumnGroup():Match(Card.IsSpellTrap,nil)
    let h = effect::get_handler(d, a.e).nil();
    let mut colg = card::get_column_group(d, h, 0, 0);
    group::match_(d, &mut colg, &|d, c| card::is_spell_trap(d, c), NO_EX);
    // local b2=#colg>0 and aux.SelectUnselectGroup(cg2,e,tp,1,2,s.vsrescon,0)
    let b2 = !colg.is_empty()
        && matches!(
            aux::select_unselect_group(d, &cg2, Some(a.e), tp, Some(1), Some(2), Some(&vsrescon), 0, tp, None, None, None, false),
            Err(true)
        );
    // if chk==0 then return b1 or b2 end
    if a.chk == Some(0) {
        return b1 || b2;
    }
    // local op=Duel.SelectEffect(tp,
    //     {b1,aux.Stringid(id,2)},
    //     {b2,aux.Stringid(id,3)})
    let op = duel::select_effect(d, tp, &[(b1, aux::stringid(ID, 2)), (b2, aux::stringid(ID, 3))]).nil();
    // e:SetLabel(op)
    effect::set_label(d, a.e, &[op as i64]);
    if op == 1 {
        // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_CONFIRM)
        duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_CONFIRM as u64);
        // local g=cg1:Select(tp,1,1,nil)
        let g = group::select(d, &cg1, tp, 1, 1, false, NO_EX).nil();
        // Duel.ConfirmCards(1-tp,g)
        duel::confirm_cards(d, 1 - tp, g);
        // Duel.ShuffleHand(tp)
        duel::shuffle_hand(d, tp);
        // e:SetCategory(0)
        effect::set_category(d, a.e, 0);
    } else if op == 2 {
        // local g=aux.SelectUnselectGroup(cg2,e,tp,1,2,s.vsrescon,1,tp,HINTMSG_CONFIRM,s.vsrescon)
        let g = aux::select_unselect_group(d, &cg2, Some(a.e), tp, Some(1), Some(2), Some(&vsrescon), 1, tp, Some(HINTMSG_CONFIRM as u64), Some(&vsrescon), None, false).unwrap_or_default();
        // Duel.ConfirmCards(1-tp,g)
        duel::confirm_cards(d, 1 - tp, g);
        // Duel.ShuffleHand(tp)
        duel::shuffle_hand(d, tp);
        // e:SetCategory(CATEGORY_DESTROY)
        effect::set_category(d, a.e, CATEGORY_DESTROY);
        // Duel.SetOperationInfo(0,CATEGORY_DESTROY,colg,#colg,tp,0)
        let n = colg.len() as u8;
        duel::set_operation_info(d, 0, CATEGORY_DESTROY, Some(colg), n, tp, 0);
    }
    true
}
/// `s.vsop(e,tp,eg,ep,ev,re,r,rp)`
fn vsop(d: &mut Duel, a: &EffArgs) {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if not c:IsRelateToEffect(e) then return end
    if !card::is_relate_to_effect(d, c, a.e) {
        return;
    }
    // local op=e:GetLabel()
    let op = effect::get_label(d, a.e);
    if op == 1 {
        // --This card cannot be destroyed by battle this turn
        // local e1=Effect.CreateEffect(c)
        let e1 = effect::create_effect(d, c);
        // e1:SetDescription(3000)
        effect::set_description(d, e1, 3000);
        // e1:SetType(EFFECT_TYPE_SINGLE)
        effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
        // e1:SetProperty(EFFECT_FLAG_CANNOT_DISABLE+EFFECT_FLAG_CLIENT_HINT)
        effect::set_property(d, e1, EFFECT_FLAG_CANNOT_DISABLE + EFFECT_FLAG_CLIENT_HINT, 0);
        // e1:SetCode(EFFECT_INDESTRUCTABLE_BATTLE)
        effect::set_code(d, e1, EFFECT_INDESTRUCTABLE_BATTLE);
        // e1:SetValue(1)
        effect::set_value(d, e1, 1);
        // e1:SetReset(RESETS_STANDARD_PHASE_END)
        effect::set_reset(d, e1, RESETS_STANDARD_PHASE_END, 1);
        // c:RegisterEffect(e1)
        card::register_effect(d, c, e1, false);
    } else if op == 2 {
        // --Destroy all Spells/Traps in this card's column
        // local colg=c:GetColumnGroup():Match(Card.IsSpellTrap,nil)
        let mut colg = card::get_column_group(d, c, 0, 0);
        group::match_(d, &mut colg, &|d, x| card::is_spell_trap(d, x), NO_EX);
        // if c:IsSpellTrap() then colg:AddCard(c) end
        if card::is_spell_trap(d, c) {
            group::add_card(&mut colg, &CardSet::from_card(c));
        }
        // if #colg>0 then
        if !colg.is_empty() {
            // Duel.Destroy(colg,REASON_EFFECT)
            duel::destroy(d, colg, REASON_EFFECT, None, None);
        }
    }
}
