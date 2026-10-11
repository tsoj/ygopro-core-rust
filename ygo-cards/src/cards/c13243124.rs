//! Favorite HERO Flame Wingman
use super::prelude::*;

pub const ID: u32 = 13243124;
pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_ELEMENTAL_HERO], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // c:EnableReviveLimit()
    card::enable_revive_limit(d, c);
    // Fusion Materials: 2 monsters with the same Type but different Attributes
    // Fusion.AddProcMixN(c,true,true,s.matfilter,2)
    fusion::add_proc_mix_n(d, c, true, true, vec![(fusion::Mat::F(fusion::fus_mat_full(matfilter)), 2)]);
    // When an attack is declared involving this card and a monster with 2200 or more ATK: You can make this card gain 1000 ATK, then you can Special Summon 1 Level 4 or lower monster from your hand in Defense Position
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_ATKCHANGE+CATEGORY_SPECIAL_SUMMON)
    effect::set_category(d, e1, CATEGORY_ATKCHANGE + CATEGORY_SPECIAL_SUMMON);
    // e1:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e1, EFFECT_TYPE_FIELD + EFFECT_TYPE_TRIGGER_O);
    // e1:SetCode(EVENT_ATTACK_ANNOUNCE)
    effect::set_code(d, e1, EVENT_ATTACK_ANNOUNCE);
    // e1:SetRange(LOCATION_MZONE)
    effect::set_range(d, e1, LOCATION_MZONE);
    // e1:SetCondition(s.atkspcon)
    effect::set_condition(d, e1, Some(condition(atkspcon)));
    // e1:SetTarget(s.atksptg)
    effect::set_target(d, e1, Some(cost_target(atksptg)));
    // e1:SetOperation(s.atkspop)
    effect::set_operation(d, e1, Some(operation(atkspop)));
    // e1:SetCountLimit(1,{id,0})
    effect::set_count_limit(d, e1, 1, ID, 0, 0);
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // If a monster(s) was destroyed by battle this turn (Quick Effect): You can Fusion Summon 1 Fusion Monster from your Extra Deck, using monsters from your hand or field
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e2, aux::stringid(ID, 1));
    // e2:SetCategory(CATEGORY_SPECIAL_SUMMON+CATEGORY_FUSION_SUMMON)
    effect::set_category(d, e2, CATEGORY_SPECIAL_SUMMON + CATEGORY_FUSION_SUMMON);
    // e2:SetType(EFFECT_TYPE_QUICK_O)
    effect::set_type(d, e2, EFFECT_TYPE_QUICK_O);
    // e2:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e2, EVENT_FREE_CHAIN);
    // e2:SetRange(LOCATION_MZONE)
    effect::set_range(d, e2, LOCATION_MZONE);
    // e2:SetCondition(function() return Duel.HasFlagEffect(0,id) end)
    effect::set_condition(d, e2, Some(condition(flag_cond)));
    // e2:SetTarget(Fusion.SummonEffTG())
    let tg = fusion::summon_eff_tg(d, fusion::SummonEff::default());
    effect::set_target(d, e2, Some(tg));
    // e2:SetOperation(Fusion.SummonEffOP())
    let op = fusion::summon_eff_op(d, fusion::SummonEff::default());
    effect::set_operation(d, e2, Some(op));
    // e2:SetCountLimit(1,{id,1})
    effect::set_count_limit(d, e2, 1, ID, 1, 0);
    // e2:SetHintTiming(0,TIMING_STANDBY_PHASE|TIMING_MAIN_END|TIMING_BATTLE_END|TIMINGS_CHECK_MONSTER_E)
    effect::set_hint_timing(d, e2, 0, Some(TIMING_STANDBY_PHASE | TIMING_MAIN_END | TIMING_BATTLE_END | TIMINGS_CHECK_MONSTER_E));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
    // Keep track of a monster being destroyed by battle
    // aux.GlobalCheck(s,function()
    aux::global_check(d, ID, |d| {
        // local ge1=Effect.CreateEffect(c)
        let ge1 = effect::create_effect(d, c);
        // ge1:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_CONTINUOUS)
        effect::set_type(d, ge1, EFFECT_TYPE_FIELD + EFFECT_TYPE_CONTINUOUS);
        // ge1:SetCode(EVENT_BATTLE_DESTROYED)
        effect::set_code(d, ge1, EVENT_BATTLE_DESTROYED);
        // ge1:SetOperation(function() Duel.RegisterFlagEffect(0,id,RESET_PHASE|PHASE_END,0,1) end)
        effect::set_operation(d, ge1, Some(lua_fn(|d, _a| {
            duel::register_flag_effect(d, 0, ID, RESET_PHASE | PHASE_END, 0, 1, 0);
        })));
        // Duel.RegisterEffect(ge1,0)
        duel::register_effect(d, ge1, 0);
    });
}
/// `s.matfilter(c,fc,sumtype,sp,sub,mg,sg)`: "2 monsters with the same Type but different Attributes"
fn matfilter(d: &mut Duel, c: CardId, a: &fusion::MatArgs) -> bool {
    // return not sg or sg:FilterCount(aux.TRUE,c)==0 or (sg:IsExists(Card.IsRace,1,c,c:GetRace(),fc,sumtype,sp) and not sg:IsExists(Card.IsAttribute,1,c,c:GetAttribute(),fc,sumtype,sp))
    let sg = match a.sg {
        None => return true,
        Some(sg) => sg.clone(),
    };
    if group::filter_count(d, &sg, &|_, _| true, c.into()) == 0 {
        return true;
    }
    let race = card::get_race(d, c);
    let attribute = card::get_attribute(d, c);
    let (fc, sumtype, tp) = (a.fc, a.sumtype, a.tp);
    group::is_exists(d, &sg, &|d, x| card::is_race_ctx(d, x, race, Some(fc), sumtype, Some(tp)), 1, c.into())
        && !group::is_exists(d, &sg, &|d, x| card::is_attribute_ctx(d, x, attribute, Some(fc), sumtype, Some(tp)), 1, c.into())
}
/// `function() return Duel.HasFlagEffect(0,id) end`
fn flag_cond(d: &mut Duel, _a: &EffArgs) -> bool {
    duel::has_flag_effect(d, 0, ID, None)
}
/// `s.atkspcon(e,tp,eg,ep,ev,re,r,rp)`
fn atkspcon(d: &mut Duel, a: &EffArgs) -> bool {
    // local bc0,bc1=Duel.GetBattleMonster(tp)
    let (bc0, bc1) = duel::get_battle_monster(d, a.tp);
    // return bc0 and bc1 and bc0==e:GetHandler() and bc1:IsAttackAbove(2200) and bc1:IsFaceup()
    match (bc0, bc1) {
        (Some(bc0), Some(bc1)) => {
            bc0 == effect::get_handler(d, a.e).nil() && card::is_attack_above(d, bc1, 2200) && card::is_faceup(d, bc1)
        }
        _ => false,
    }
}
/// `s.atksptg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn atksptg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // if chk==0 then return true end
    if a.chk == Some(0) {
        return true;
    }
    // Duel.SetOperationInfo(0,CATEGORY_ATKCHANGE,e:GetHandler(),1,tp,1000)
    let h = effect::get_handler(d, a.e).nil();
    duel::set_operation_info(d, 0, CATEGORY_ATKCHANGE, Some(h.into()), 1, tp, 1000);
    // Duel.SetPossibleOperationInfo(0,CATEGORY_SPECIAL_SUMMON,nil,1,tp,LOCATION_HAND)
    duel::set_possible_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, None, 1, tp, LOCATION_HAND as i32);
    true
}
/// `s.spfilter(c,e,tp)`
fn spfilter(d: &mut Duel, c: CardId, e: EffectId, tp: u8) -> bool {
    // return c:IsLevelBelow(4) and c:IsCanBeSpecialSummoned(e,0,tp,false,false,POS_FACEUP_DEFENSE)
    card::is_level_below(d, c, 4) && card::is_can_be_special_summoned(d, c, e, 0, tp, false, false, Some(POS_FACEUP_DEFENSE as u8), None, None)
}
/// `s.atkspop(e,tp,eg,ep,ev,re,r,rp)`
fn atkspop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if c:IsRelateToEffect(e) and c:IsFaceup() and c:UpdateAttack(1000)>0
    //     and Duel.GetLocationCount(tp,LOCATION_MZONE)>0
    //     and Duel.IsExistingMatchingCard(s.spfilter,tp,LOCATION_HAND,0,1,nil,e,tp)
    //     and Duel.SelectYesNo(tp,aux.Stringid(id,2)) then
    if card::is_relate_to_effect(d, c, a.e)
        && card::is_faceup(d, c)
        && card::update_attack(d, c, 1000, None, None, None) > 0
        && duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 > 0
        && duel::is_existing_matching_card(d, Some(&|d, x| spfilter(d, x, a.e, tp)), tp, LOCATION_HAND, 0, 1, NO_EX)
        && duel::select_yes_no(d, tp, aux::stringid(ID, 2))
    {
        // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_SPSUMMON)
        duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_SPSUMMON as u64);
        // local g=Duel.SelectMatchingCard(tp,s.spfilter,tp,LOCATION_HAND,0,1,1,nil,e,tp)
        let g = duel::select_matching_card(d, tp, Some(&|d, x| spfilter(d, x, a.e, tp)), tp, LOCATION_HAND, 0, 1, 1, false, NO_EX).nil();
        // if #g>0 then
        if g.len() > 0 {
            // Duel.BreakEffect()
            duel::break_effect(d);
            // Duel.SpecialSummon(g,0,tp,tp,false,false,POS_FACEUP_DEFENSE)
            duel::special_summon(d, g, 0, tp, tp, false, false, POS_FACEUP_DEFENSE as u8, None);
        }
    }
}
