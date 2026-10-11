//! Melffy Pinny
//! scripted by Naim
use super::prelude::*;

pub const ID: u32 = 34800281;

pub const INFO: ScriptInfo = ScriptInfo {
    // s.listed_names={id}
    listed_names: &[ID],
    // s.listed_series={SET_MELFFY}
    listed_series: &[SET_MELFFY],
    ..ScriptInfo::DEFAULT
};

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // If a face-up Beast monster you control was returned to the hand this turn, except "Melffy Pinny" (Quick Effect): You can Special Summon this card from your hand, then, immediately after this effect resolves, you can Synchro Summon 1 Synchro Monster using only this card you control and "Melffy" monsters in your hand
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_SPECIAL_SUMMON)
    effect::set_category(d, e1, CATEGORY_SPECIAL_SUMMON);
    // e1:SetType(EFFECT_TYPE_QUICK_O)
    effect::set_type(d, e1, EFFECT_TYPE_QUICK_O);
    // e1:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    // e1:SetRange(LOCATION_HAND)
    effect::set_range(d, e1, LOCATION_HAND);
    // e1:SetCountLimit(1,{id,0})
    effect::set_count_limit(d, e1, 1, ID, 0, 0);
    // e1:SetCondition(function(e,tp) return Duel.HasFlagEffect(tp,id) end)
    effect::set_condition(d, e1, Some(condition(|d, a| duel::has_flag_effect(d, a.tp, ID, None))));
    // e1:SetTarget(s.sptg)
    effect::set_target(d, e1, Some(cost_target(sptg)));
    // e1:SetOperation(s.spop)
    effect::set_operation(d, e1, Some(operation(spop)));
    // e1:SetHintTiming(0,TIMING_STANDBY_PHASE|TIMING_MAIN_END|TIMINGS_CHECK_MONSTER_E)
    effect::set_hint_timing(d, e1, 0, Some(TIMING_STANDBY_PHASE | TIMING_MAIN_END | TIMINGS_CHECK_MONSTER_E));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // During your End Phase, if this card is in your GY: You can target 1 Beast Xyz Monster you control; attach this card to it as material
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e2, aux::stringid(ID, 1));
    // e2:SetCategory(CATEGORY_LEAVE_GRAVE)
    effect::set_category(d, e2, CATEGORY_LEAVE_GRAVE);
    // e2:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e2, EFFECT_TYPE_FIELD + EFFECT_TYPE_TRIGGER_O);
    // e2:SetProperty(EFFECT_FLAG_CARD_TARGET)
    effect::set_property(d, e2, EFFECT_FLAG_CARD_TARGET, 0);
    // e2:SetCode(EVENT_PHASE+PHASE_END)
    effect::set_code(d, e2, EVENT_PHASE + PHASE_END);
    // e2:SetRange(LOCATION_GRAVE)
    effect::set_range(d, e2, LOCATION_GRAVE);
    // e2:SetCountLimit(1,{id,1})
    effect::set_count_limit(d, e2, 1, ID, 1, 0);
    // e2:SetCondition(function(e,tp) return Duel.IsTurnPlayer(tp) end)
    effect::set_condition(d, e2, Some(condition(|d, a| duel::is_turn_player(d, a.tp))));
    // e2:SetTarget(s.attachtg)
    effect::set_target(d, e2, Some(cost_target(attachtg)));
    // e2:SetOperation(s.attachop)
    effect::set_operation(d, e2, Some(operation(attachop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
    // Keep track of Beast monsters returning from the field to the hand
    // aux.GlobalCheck(s,function()
    aux::global_check(d, ID, |d| {
        // local ge1=Effect.CreateEffect(c)
        let ge1 = effect::create_effect(d, c);
        // ge1:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_CONTINUOUS)
        effect::set_type(d, ge1, EFFECT_TYPE_FIELD + EFFECT_TYPE_CONTINUOUS);
        // ge1:SetCode(EVENT_TO_HAND)
        effect::set_code(d, ge1, EVENT_TO_HAND);
        // ge1:SetOperation(function(e,tp,eg,ep,ev,re,r,rp)
        effect::set_operation(d, ge1, Some(operation(|d, a| {
            // for ec in eg:Iter() do
            let eg = a.eg.clone().nil();
            for ec in group::iter(d, &eg) {
                // if ec:IsPreviousLocation(LOCATION_MZONE) and ec:IsPreviousPosition(POS_FACEUP) and ec:IsPreviousRaceOnField(RACE_BEAST)
                //     and not ec:IsPreviousCodeOnField(id) and ec:IsRace(RACE_BEAST) then
                if card::is_previous_location(d, ec, LOCATION_MZONE)
                    && card::is_previous_position(d, ec, POS_FACEUP)
                    && card::is_previous_race_on_field(d, ec, RACE_BEAST)
                    && !card::is_previous_code_on_field(d, ec, &[ID])
                    && card::is_race(d, ec, RACE_BEAST)
                {
                    // Duel.RegisterFlagEffect(ec:GetPreviousControler(),id,RESET_PHASE|PHASE_END,0,1)
                    let p = card::get_previous_controler(d, ec);
                    duel::register_flag_effect(d, p, ID, RESET_PHASE | PHASE_END, 0, 1, 0);
                }
            }
        })));
        // Duel.RegisterEffect(ge1,0)
        duel::register_effect(d, ge1, 0);
    });
}
/// `s.sptg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn sptg(d: &mut Duel, a: &EffArgs) -> bool {
    let (e, tp) = (a.e, a.tp);
    let c = effect::get_handler(d, e).nil();
    // if chk==0 then return Duel.GetLocationCount(tp,LOCATION_MZONE)>0
    //     and c:IsCanBeSpecialSummoned(e,0,tp,false,false) end
    if a.chk == Some(0) {
        return duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 > 0
            && card::is_can_be_special_summoned(d, c, e, 0, tp, false, false, None, None, None);
    }
    // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,c,1,tp,0)
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, Some(CardSet::from_card(c)), 1, tp, 0);
    // Duel.SetPossibleOperationInfo(0,CATEGORY_SPECIAL_SUMMON,nil,1,tp,LOCATION_EXTRA)
    duel::set_possible_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, None, 1, tp, LOCATION_EXTRA as i32);
    true
}
/// `s.spop(e,tp,eg,ep,ev,re,r,rp)`
fn spop(d: &mut Duel, a: &EffArgs) {
    let (e, tp) = (a.e, a.tp);
    // local c=e:GetHandler()
    let c = effect::get_handler(d, e).nil();
    // if not c:IsRelateToEffect(e) or Duel.SpecialSummon(c,0,tp,tp,false,false,POS_FACEUP)==0 then return end
    if !card::is_relate_to_effect(d, c, e) || duel::special_summon(d, c, 0, tp, tp, false, false, POS_FACEUP as u8, None) == 0 {
        return;
    }
    // local mg=Duel.GetMatchingGroup(Card.IsSetCard,tp,LOCATION_HAND,0,nil,SET_MELFFY)
    let mg = duel::get_matching_group(d, Some(&|d, x| card::is_set_card(d, x, &[SET_MELFFY as u16])), tp, LOCATION_HAND, 0, NO_EX);
    // if #mg==0 then return end
    if mg.is_empty() {
        return;
    }
    // local mgc=mg+c
    let mut mgc = mg.clone();
    group::add_card(&mut mgc, &CardSet::from_card(c));
    // local sg=Duel.GetMatchingGroup(Card.IsSynchroSummonable,tp,LOCATION_EXTRA,0,nil,c,mg+c)
    let sg = duel::get_matching_group(d, Some(&|d, x| card::is_synchro_summonable(d, x, Some(CardSet::from_card(c)), Some(mgc.clone()), 0, 0)), tp, LOCATION_EXTRA, 0, NO_EX);
    // if #sg==0 or not Duel.SelectYesNo(tp,aux.Stringid(id,2)) then return end
    if sg.is_empty() || !duel::select_yes_no(d, tp, aux::stringid(ID, 2)) {
        return;
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_SPSUMMON)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_SPSUMMON as u64);
    // local sc=sg:Select(tp,1,1,nil):GetFirst()
    let sel = group::select(d, &sg, tp, 1, 1, false, NO_EX).nil();
    let sc = group::get_first(d, &sel);
    // if sc then
    if let Some(sc) = sc {
        // Duel.BreakEffect()
        duel::break_effect(d);
        // Duel.SynchroSummon(tp,sc,c,mg+c)
        duel::synchro_summon(d, tp, sc, Some(CardSet::from_card(c)), Some(mgc), 0, 0);
    }
}
/// `s.xyzfilter(c,ac,tp)`
fn xyzfilter(d: &mut Duel, c: CardId, ac: CardId, tp: u8) -> bool {
    // return c:IsRace(RACE_BEAST) and c:IsXyzMonster() and c:IsFaceup() and ac:IsCanBeXyzMaterial(c,tp,REASON_EFFECT)
    card::is_race(d, c, RACE_BEAST)
        && card::is_xyz_monster(d, c)
        && card::is_faceup(d, c)
        && card::is_can_be_xyz_material(d, ac, Some(c), Some(tp), Some(REASON_EFFECT as u32))
}
/// `s.attachtg(e,tp,eg,ep,ev,re,r,rp,chk,chkc)`
fn attachtg(d: &mut Duel, a: &EffArgs) -> bool {
    let (e, tp) = (a.e, a.tp);
    // local c=e:GetHandler()
    let c = effect::get_handler(d, e).nil();
    // if chkc then return chkc:IsControler(tp) and chkc:IsLocation(LOCATION_MZONE) and s.xyzfilter(chkc,c,tp) end
    if let Some(chkc) = a.chkc() {
        return card::is_controler(d, chkc, tp) && card::is_location(d, chkc, LOCATION_MZONE) && xyzfilter(d, chkc, c, tp);
    }
    // if chk==0 then return Duel.IsExistingTarget(s.xyzfilter,tp,LOCATION_MZONE,0,1,nil,c,tp) end
    if a.chk == Some(0) {
        return duel::is_existing_target(d, Some(&|d, x| xyzfilter(d, x, c, tp)), tp, LOCATION_MZONE, 0, 1, NO_EX);
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_TARGET)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_TARGET as u64);
    // Duel.SelectTarget(tp,s.xyzfilter,tp,LOCATION_MZONE,0,1,1,nil,c,tp)
    duel::select_target(d, tp, Some(&|d, x| xyzfilter(d, x, c, tp)), tp, LOCATION_MZONE, 0, 1, 1, false, NO_EX);
    // Duel.SetOperationInfo(0,CATEGORY_LEAVE_GRAVE,c,1,tp,0)
    duel::set_operation_info(d, 0, CATEGORY_LEAVE_GRAVE, Some(CardSet::from_card(c)), 1, tp, 0);
    true
}
/// `s.attachop(e,tp,eg,ep,ev,re,r,rp)`
fn attachop(d: &mut Duel, a: &EffArgs) {
    let (e, tp) = (a.e, a.tp);
    // local c=e:GetHandler()
    let c = effect::get_handler(d, e).nil();
    // local tc=Duel.GetFirstTarget()
    let tc = duel::first_target(d);
    // if c:IsRelateToEffect(e) and tc:IsRelateToEffect(e) and not tc:IsImmuneToEffect(e)
    //     and c:IsCanBeXyzMaterial(tc,tp,REASON_EFFECT) then
    if card::is_relate_to_effect(d, c, e)
        && card::is_relate_to_effect(d, tc.nil(), e)
        && !card::is_immune_to_effect(d, tc.nil(), e)
        && card::is_can_be_xyz_material(d, c, tc, Some(tp), Some(REASON_EFFECT as u32))
    {
        // Duel.Overlay(tc,c)
        duel::overlay(d, tc.nil(), c, false);
    }
}
