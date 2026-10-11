//! Miracle of the Supreme King
//! 覇王龍の奇跡
//! Scripted by Eerie Code
use super::prelude::*;

pub const ID: u32 = 40456412;

pub const INFO: ScriptInfo = ScriptInfo { listed_names: &[CARD_ZARC], listed_series: &[SET_ODD_EYES], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate 1 of these effects
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    // e1:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    // e1:SetHintTiming(0,TIMINGS_CHECK_MONSTER_E)
    effect::set_hint_timing(d, e1, 0, Some(TIMINGS_CHECK_MONSTER_E));
    // e1:SetCondition(function(_,tp) return Duel.IsExistingMatchingCard(aux.FaceupFilter(Card.IsCode,CARD_ZARC),tp,LOCATION_ONFIELD,0,1,nil) end)
    effect::set_condition(d, e1, Some(condition(zarc_cond)));
    // e1:SetTarget(s.target)
    effect::set_target(d, e1, Some(cost_target(target)));
    // e1:SetOperation(s.activate)
    effect::set_operation(d, e1, Some(operation(activate)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
/// `function(_,tp) return Duel.IsExistingMatchingCard(aux.FaceupFilter(Card.IsCode,CARD_ZARC),tp,LOCATION_ONFIELD,0,1,nil) end`
fn zarc_cond(d: &mut Duel, a: &EffArgs) -> bool {
    duel::is_existing_matching_card(d, Some(&|d, c| card::is_faceup(d, c) && card::is_code(d, c, &[CARD_ZARC])), a.tp, LOCATION_ONFIELD, 0, 1, NO_EX)
}
/// `s.desfilter(c,e,tp)`
fn desfilter(d: &mut Duel, c: CardId, e: EffectId, tp: u8) -> bool {
    // return c:IsFaceup() and c:IsCode(CARD_ZARC)
    //     and Duel.IsExistingMatchingCard(s.spfilter,tp,LOCATION_DECK|LOCATION_EXTRA,0,1,nil,e,tp,c)
    if !(card::is_faceup(d, c) && card::is_code(d, c, &[CARD_ZARC])) {
        return false;
    }
    duel::is_existing_matching_card(d, Some(&|d, x| spfilter(d, x, e, tp, Some(c))), tp, LOCATION_DECK | LOCATION_EXTRA, 0, 1, NO_EX)
}
/// `s.spfilter(c,e,tp,ec)`
fn spfilter(d: &mut Duel, c: CardId, e: EffectId, tp: u8, ec: Option<CardId>) -> bool {
    // if not ((c:IsSetCard(SET_ODD_EYES) and c:IsType(TYPE_PENDULUM)) or (c:IsAttribute(ATTRIBUTE_LIGHT) and c:IsCode(CARD_ZARC))) then return false end
    let ok = (card::is_set_card(d, c, &[SET_ODD_EYES as u16]) && card::is_type(d, c, TYPE_PENDULUM))
        || (card::is_attribute(d, c, ATTRIBUTE_LIGHT) && card::is_code(d, c, &[CARD_ZARC]));
    if !ok {
        return false;
    }
    // if not c:IsCanBeSpecialSummoned(e,0,tp,true,false) then return false end
    if !card::is_can_be_special_summoned(d, c, e, 0, tp, true, false, None, None, None) {
        return false;
    }
    let ex = ec.map_or(NO_EX, |x| x.into());
    // if c:IsLocation(LOCATION_DECK) then
    if card::is_location(d, c, LOCATION_DECK) {
        // return Duel.GetMZoneCount(tp,ec)>0
        duel::get_m_zone_count(d, tp, ex, None, None, None).0 > 0
    } else {
        // return Duel.GetLocationCountFromEx(tp,tp,ec,c)>0
        duel::get_location_count_from_ex(d, tp, Some(tp), ex, duel::ExtraCard::Card(c), None).0 > 0
    }
}
/// `s.pcfilter(c)`
fn pcfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsFaceup() and c:IsType(TYPE_PENDULUM) and not c:IsForbidden()
    card::is_faceup(d, c) && card::is_type(d, c, TYPE_PENDULUM) && !card::is_forbidden(d, c)
}
/// `s.setfilter(c)`
fn setfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsQuickPlaySpell() and c:IsSSetable()
    card::is_quick_play_spell(d, c) && card::is_s_setable(d, c, false, None)
}
/// `s.target(e,tp,eg,ep,ev,re,r,rp,chk)`
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    let e = a.e;
    // local b1=not Duel.HasFlagEffect(tp,id)
    //     and Duel.IsExistingMatchingCard(s.desfilter,tp,LOCATION_ONFIELD,0,1,nil,e,tp)
    let b1 = !duel::has_flag_effect(d, tp, ID, None) && duel::is_existing_matching_card(d, Some(&|d, c| desfilter(d, c, e, tp)), tp, LOCATION_ONFIELD, 0, 1, NO_EX);
    // local b2=not Duel.HasFlagEffect(tp,id+1) and Duel.CheckPendulumZones(tp)
    //     and Duel.IsExistingMatchingCard(s.pcfilter,tp,LOCATION_EXTRA,0,1,nil)
    let b2 = !duel::has_flag_effect(d, tp, ID + 1, None)
        && duel::check_pendulum_zones(d, tp)
        && duel::is_existing_matching_card(d, Some(&pcfilter), tp, LOCATION_EXTRA, 0, 1, NO_EX);
    // local b3=not Duel.HasFlagEffect(tp,id+2) and Duel.GetLocationCount(tp,LOCATION_SZONE)>0
    //     and Duel.IsExistingMatchingCard(s.setfilter,tp,LOCATION_DECK,0,1,nil)
    let b3 = !duel::has_flag_effect(d, tp, ID + 2, None)
        && duel::get_location_count(d, tp, LOCATION_SZONE as u8, None, None, None).0 > 0
        && duel::is_existing_matching_card(d, Some(&setfilter), tp, LOCATION_DECK, 0, 1, NO_EX);
    // if chk==0 then return b1 or b2 or b3 end
    if a.chk == Some(0) {
        return b1 || b2 || b3;
    }
    // local op=Duel.SelectEffect(tp,
    //     {b1,aux.Stringid(id,1)},
    //     {b2,aux.Stringid(id,2)},
    //     {b3,aux.Stringid(id,3)})
    let op = duel::select_effect(d, tp, &[(b1, aux::stringid(ID, 1)), (b2, aux::stringid(ID, 2)), (b3, aux::stringid(ID, 3))]).nil();
    // e:SetLabel(op)
    effect::set_label(d, e, &[op as i64]);
    if op == 1 {
        // e:SetCategory(CATEGORY_DESTROY|CATEGORY_SPECIAL_SUMMON)
        effect::set_category(d, e, CATEGORY_DESTROY | CATEGORY_SPECIAL_SUMMON);
        // Duel.RegisterFlagEffect(tp,id,RESET_PHASE|PHASE_END,0,1)
        duel::register_flag_effect(d, tp, ID, RESET_PHASE | PHASE_END, 0, 1, 0);
        // local g=Duel.GetMatchingGroup(aux.FaceupFilter(Card.IsCode,CARD_ZARC),tp,LOCATION_ONFIELD,0,nil)
        let g = duel::get_matching_group(d, Some(&|d, c| card::is_faceup(d, c) && card::is_code(d, c, &[CARD_ZARC])), tp, LOCATION_ONFIELD, 0, NO_EX);
        // Duel.SetOperationInfo(0,CATEGORY_DESTROY,g,1,tp,LOCATION_ONFIELD)
        duel::set_operation_info(d, 0, CATEGORY_DESTROY, Some(g), 1, tp, LOCATION_ONFIELD as i32);
        // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,nil,1,tp,LOCATION_DECK|LOCATION_EXTRA)
        duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, None, 1, tp, (LOCATION_DECK | LOCATION_EXTRA) as i32);
    } else if op == 2 {
        // e:SetCategory(0)
        effect::set_category(d, e, 0);
        // Duel.RegisterFlagEffect(tp,id+1,RESET_PHASE|PHASE_END,0,1)
        duel::register_flag_effect(d, tp, ID + 1, RESET_PHASE | PHASE_END, 0, 1, 0);
    } else if op == 3 {
        // e:SetCategory(CATEGORY_SET)
        effect::set_category(d, e, CATEGORY_SET);
        // Duel.RegisterFlagEffect(tp,id+2,RESET_PHASE|PHASE_END,0,1)
        duel::register_flag_effect(d, tp, ID + 2, RESET_PHASE | PHASE_END, 0, 1, 0);
    }
    true
}
/// `s.activate(e,tp,eg,ep,ev,re,r,rp)`
fn activate(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    let e = a.e;
    // local op=e:GetLabel()
    let op = effect::get_label(d, e);
    if op == 1 {
        // Destroy 1 "Supreme King Z-ARC" you control
        // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_DESTROY)
        duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_DESTROY as u64);
        // local g=Duel.SelectMatchingCard(tp,s.desfilter,tp,LOCATION_ONFIELD,0,1,1,nil,e,tp)
        let g = duel::select_matching_card(d, tp, Some(&|d, c| desfilter(d, c, e, tp)), tp, LOCATION_ONFIELD, 0, 1, 1, false, NO_EX).nil();
        // if #g>0 and Duel.Destroy(g,REASON_EFFECT)>0 then
        if !g.is_empty() && duel::destroy(d, g, REASON_EFFECT, None, None) > 0 {
            // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_SPSUMMON)
            duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_SPSUMMON as u64);
            // local sg=Duel.SelectMatchingCard(tp,s.spfilter,tp,LOCATION_DECK|LOCATION_EXTRA,0,1,1,nil,e,tp)
            let sg = duel::select_matching_card(d, tp, Some(&|d, c| spfilter(d, c, e, tp, None)), tp, LOCATION_DECK | LOCATION_EXTRA, 0, 1, 1, false, NO_EX).nil();
            // if #sg>0 then
            if !sg.is_empty() {
                // Duel.SpecialSummon(sg,0,tp,tp,true,false,POS_FACEUP)
                duel::special_summon(d, sg, 0, tp, tp, true, false, POS_FACEUP as u8, None);
            }
        }
    } else if op == 2 {
        // Place 1 face-up Pendulum Monster from your Extra Deck in your Pendulum Zone
        // if not Duel.CheckPendulumZones(tp) then return end
        if !duel::check_pendulum_zones(d, tp) {
            return;
        }
        // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_TOFIELD)
        duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_TOFIELD as u64);
        // local sc=Duel.SelectMatchingCard(tp,s.pcfilter,tp,LOCATION_EXTRA,0,1,1,nil):GetFirst()
        let g = duel::select_matching_card(d, tp, Some(&pcfilter), tp, LOCATION_EXTRA, 0, 1, 1, false, NO_EX).nil();
        let sc = group::get_first(d, &g);
        // if sc then
        if let Some(sc) = sc {
            // Duel.MoveToField(sc,tp,tp,LOCATION_PZONE,POS_FACEUP,true)
            duel::move_to_field(d, sc, tp, tp, LOCATION_PZONE as u16, POS_FACEUP as u8, true, None);
        }
    } else if op == 3 {
        // Set 1 Quick-Play Spell directly from your Deck
        // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_SET)
        duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_SET as u64);
        // local g=Duel.SelectMatchingCard(tp,s.setfilter,tp,LOCATION_DECK,0,1,1,nil)
        let g = duel::select_matching_card(d, tp, Some(&setfilter), tp, LOCATION_DECK, 0, 1, 1, false, NO_EX).nil();
        // if #g>0 then
        if !g.is_empty() {
            // Duel.SSet(tp,g)
            duel::s_set(d, tp, g, None, true);
        }
    }
}
