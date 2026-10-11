//! Magical Musketeer Max
//! Scripted by AlphaKretin
use super::prelude::*;

pub const ID: u32 = 71791814;

pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_MAGICAL_MUSKET], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // c:EnableReviveLimit()
    card::enable_revive_limit(d, c);
    // Link Summon procedure
    // Link.AddProcedure(c,s.matfilter,1,1)
    link::add_procedure(d, c, Some(mat_filter(|d, c, sc, st, tp| matfilter(d, c, sc, st, tp))), 1, Some(1), None, None, None);
    // Activate 1 effect when it is Link Summoned
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_O);
    // e1:SetProperty(EFFECT_FLAG_DELAY)
    effect::set_property(d, e1, EFFECT_FLAG_DELAY, 0);
    // e1:SetCode(EVENT_SPSUMMON_SUCCESS)
    effect::set_code(d, e1, EVENT_SPSUMMON_SUCCESS);
    // e1:SetCountLimit(1,id)
    effect::set_count_limit(d, e1, 1, ID, 0, 0);
    // e1:SetCondition(function(e) return e:GetHandler():IsLinkSummoned() end)
    effect::set_condition(
        d,
        e1,
        Some(condition(|d, a| {
            let h = effect::get_handler(d, a.e).nil();
            card::is_link_summoned(d, h)
        })),
    );
    // e1:SetTarget(s.target)
    effect::set_target(d, e1, Some(cost_target(target)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // "Magical Musket" Spell/Traps can be activated from the hand
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,2))
    effect::set_description(d, e2, aux::stringid(ID, 2));
    // e2:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e2, EFFECT_TYPE_FIELD);
    // e2:SetCode(EFFECT_QP_ACT_IN_NTPHAND)
    effect::set_code(d, e2, EFFECT_QP_ACT_IN_NTPHAND);
    // e2:SetRange(LOCATION_MZONE)
    effect::set_range(d, e2, LOCATION_MZONE);
    // e2:SetTargetRange(LOCATION_HAND,0)
    effect::set_target_range(d, e2, LOCATION_HAND, 0);
    // e2:SetTarget(aux.TargetBoolFunction(Card.IsSetCard,SET_MAGICAL_MUSKET))
    effect::set_target(d, e2, Some(effect_card_fn(|d, _e, c| card::is_set_card(d, c, &[SET_MAGICAL_MUSKET as u16]))));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
    // local e3=e2:Clone()
    let e3 = effect::clone(d, e2);
    // e3:SetCode(EFFECT_TRAP_ACT_IN_HAND)
    effect::set_code(d, e3, EFFECT_TRAP_ACT_IN_HAND);
    // c:RegisterEffect(e3)
    card::register_effect(d, c, e3, false);
}
/// `s.matfilter(c,lc,sumtype,tp)`
fn matfilter(d: &mut Duel, c: CardId, lc: Option<CardId>, sumtype: u64, tp: u8) -> bool {
    // return c:IsLevelBelow(8) and c:IsSetCard(SET_MAGICAL_MUSKET,lc,sumtype,tp)
    card::is_level_below(d, c, 8) && card::is_set_card_ctx(d, c, &[SET_MAGICAL_MUSKET as u16], lc, sumtype, Some(tp))
}
/// `s.thfilter(c)`
fn thfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsSetCard(SET_MAGICAL_MUSKET) and c:IsSpellTrap() and c:IsAbleToHand()
    card::is_set_card(d, c, &[SET_MAGICAL_MUSKET as u16]) && card::is_spell_trap(d, c) && card::is_able_to_hand(d, c, None)
}
/// `s.spfilter(c,e,tp)`
fn spfilter(d: &mut Duel, c: CardId, e: EffectId, tp: u8) -> bool {
    // return c:IsSetCard(SET_MAGICAL_MUSKET) and c:IsMonster() and c:IsCanBeSpecialSummoned(e,0,tp,false,false)
    card::is_set_card(d, c, &[SET_MAGICAL_MUSKET as u16])
        && card::is_monster(d, c)
        && card::is_can_be_special_summoned(d, c, e, 0, tp, false, false, None, None, None)
}
/// `s.target(e,tp,eg,ep,ev,re,r,rp,chk)`
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // local monct=Duel.GetFieldGroupCount(tp,0,LOCATION_MZONE)
    let monct = duel::get_field_group_count(d, tp, 0, LOCATION_MZONE);
    // local stct=Duel.GetMatchingGroupCount(Card.IsSpellTrap,tp,0,LOCATION_ONFIELD,nil)
    let stct = duel::get_matching_group_count(d, Some(&card::is_spell_trap), tp, 0, LOCATION_ONFIELD, NO_EX);
    // local b1=monct>0 and Duel.IsExistingMatchingCard(s.thfilter,tp,LOCATION_DECK,0,1,nil)
    let b1 = monct > 0 && duel::is_existing_matching_card(d, Some(&thfilter), tp, LOCATION_DECK, 0, 1, NO_EX);
    // local b2=stct>0 and Duel.GetLocationCount(tp,LOCATION_MZONE)>0 and Duel.IsExistingMatchingCard(s.spfilter,tp,LOCATION_DECK,0,1,nil,e,tp)
    let b2 = stct > 0
        && duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 > 0
        && duel::is_existing_matching_card(d, Some(&|d, c| spfilter(d, c, a.e, tp)), tp, LOCATION_DECK, 0, 1, NO_EX);
    // if chk==0 then return b1 or b2 end
    if a.chk == Some(0) {
        return b1 || b2;
    }
    // local op=Duel.SelectEffect(tp,
    //     {b1,aux.Stringid(id,0)},
    //     {b2,aux.Stringid(id,1)})
    let op = duel::select_effect(d, tp, &[(b1, aux::stringid(ID, 0)), (b2, aux::stringid(ID, 1))]);
    // if op==1 then
    if op == Some(1) {
        // e:SetCategory(CATEGORY_TOHAND+CATEGORY_SEARCH)
        effect::set_category(d, a.e, CATEGORY_TOHAND + CATEGORY_SEARCH);
        // e:SetOperation(s.thop)
        effect::set_operation(d, a.e, Some(operation(thop)));
        // Duel.SetOperationInfo(0,CATEGORY_TOHAND,nil,1,tp,LOCATION_DECK)
        duel::set_operation_info(d, 0, CATEGORY_TOHAND, None, 1, tp, LOCATION_DECK as i32);
    // elseif op==2 then
    } else if op == Some(2) {
        // e:SetCategory(CATEGORY_SPECIAL_SUMMON)
        effect::set_category(d, a.e, CATEGORY_SPECIAL_SUMMON);
        // e:SetOperation(s.spop)
        effect::set_operation(d, a.e, Some(operation(spop)));
        // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,nil,1,tp,LOCATION_DECK)
        duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, None, 1, tp, LOCATION_DECK as i32);
    }
    true
}
/// `s.thop(e,tp,eg,ep,ev,re,r,rp)`
fn thop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // local ct=Duel.GetFieldGroupCount(tp,0,LOCATION_MZONE)
    let ct = duel::get_field_group_count(d, tp, 0, LOCATION_MZONE);
    // local g=Duel.GetMatchingGroup(s.thfilter,tp,LOCATION_DECK,0,nil)
    let g = duel::get_matching_group(d, Some(&thfilter), tp, LOCATION_DECK, 0, NO_EX);
    // if not (ct>0 and #g>0) then return end
    if !(ct > 0 && group::get_count(&g) > 0) {
        return;
    }
    // local sg=aux.SelectUnselectGroup(g,e,tp,1,ct,aux.dncheck,1,tp,HINTMSG_ATOHAND)
    let dn = aux::dncheck();
    let rs: aux::Rescon = &dn;
    let sg = aux::select_unselect_group(d, &g, Some(a.e), tp, Some(1), Some(ct as usize), Some(rs), 1, tp, Some(HINTMSG_ATOHAND as u64), None, None, false)
        .unwrap_or_default();
    // if #sg>0 then
    if group::get_count(&sg) > 0 {
        // Duel.SendtoHand(sg,tp,REASON_EFFECT)
        duel::send_to_hand(d, sg.clone(), Some(tp), REASON_EFFECT, None);
        // Duel.ConfirmCards(1-tp,sg)
        duel::confirm_cards(d, 1 - tp, sg);
    }
}
/// `s.spop(e,tp,eg,ep,ev,re,r,rp)`
fn spop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // local ct=Duel.GetMatchingGroupCount(Card.IsSpellTrap,tp,0,LOCATION_ONFIELD,nil)
    let ct = duel::get_matching_group_count(d, Some(&card::is_spell_trap), tp, 0, LOCATION_ONFIELD, NO_EX) as i32;
    // local ft=Duel.GetLocationCount(tp,LOCATION_MZONE)
    let mut ft = duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0;
    // local g=Duel.GetMatchingGroup(s.spfilter,tp,LOCATION_DECK,0,nil,e,tp)
    let g = duel::get_matching_group(d, Some(&|d, c| spfilter(d, c, a.e, tp)), tp, LOCATION_DECK, 0, NO_EX);
    // if not (ct>0 and #g>0 and ft>0) then return end
    if !(ct > 0 && group::get_count(&g) > 0 && ft > 0) {
        return;
    }
    // if Duel.IsPlayerAffectedByEffect(tp,CARD_BLUEEYES_SPIRIT) then ft=1 end
    if duel::is_player_affected_by_effect(d, tp, CARD_BLUEEYES_SPIRIT as u32).is_some() {
        ft = 1;
    }
    // local sg=aux.SelectUnselectGroup(g,e,tp,1,math.min(ct,ft),aux.dncheck,1,tp,HINTMSG_SPSUMMON)
    let dn = aux::dncheck();
    let rs: aux::Rescon = &dn;
    let sg = aux::select_unselect_group(d, &g, Some(a.e), tp, Some(1), Some(ct.min(ft) as usize), Some(rs), 1, tp, Some(HINTMSG_SPSUMMON as u64), None, None, false)
        .unwrap_or_default();
    // if #sg>0 then
    if group::get_count(&sg) > 0 {
        // Duel.SpecialSummon(sg,0,tp,tp,false,false,POS_FACEUP)
        duel::special_summon(d, sg, 0, tp, tp, false, false, POS_FACEUP as u8, None);
    }
}
