//! Chevreuil, Hunting Scout of the Deep Forest
//! scripted by pyrQ
use super::prelude::*;

pub const ID: u32 = 13023431;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // c:EnableReviveLimit()
    card::enable_revive_limit(d, c);
    // Fusion Materials: 1 Level 5 or higher EARTH Warrior monster + 1 EARTH Warrior monster
    // Fusion.AddProcMix(c,true,true,s.matfilter1,s.matfilter2)
    fusion::add_proc_mix(
        d,
        c,
        true,
        true,
        vec![fusion::Mat::F(fusion::fus_mat(matfilter1)), fusion::Mat::F(fusion::fus_mat(matfilter2))],
    );
    // Unaffected by your opponent's activated effects, during the Main Phase only
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
    // e1:SetProperty(EFFECT_FLAG_SINGLE_RANGE)
    effect::set_property(d, e1, EFFECT_FLAG_SINGLE_RANGE, 0);
    // e1:SetCode(EFFECT_IMMUNE_EFFECT)
    effect::set_code(d, e1, EFFECT_IMMUNE_EFFECT);
    // e1:SetRange(LOCATION_MZONE)
    effect::set_range(d, e1, LOCATION_MZONE);
    // e1:SetCondition(function() return Duel.IsMainPhase() end)
    effect::set_condition(d, e1, Some(condition(main_phase_cond)));
    // e1:SetValue(function(e,te) return te:GetOwnerPlayer()~=e:GetHandlerPlayer() and te:IsActivated() end)
    effect::set_value_fn(d, e1, lua_fn(immval));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // Your opponent's monsters that can attack must attack the monster you control with the highest ATK (their choice, if tied)
    // local e2a=Effect.CreateEffect(c)
    let e2a = effect::create_effect(d, c);
    // e2a:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e2a, EFFECT_TYPE_FIELD);
    // e2a:SetCode(EFFECT_MUST_ATTACK)
    effect::set_code(d, e2a, EFFECT_MUST_ATTACK);
    // e2a:SetRange(LOCATION_MZONE)
    effect::set_range(d, e2a, LOCATION_MZONE);
    // e2a:SetTargetRange(0,LOCATION_MZONE)
    effect::set_target_range(d, e2a, 0, LOCATION_MZONE);
    // c:RegisterEffect(e2a)
    card::register_effect(d, c, e2a, false);
    // local e2b=e2a:Clone()
    let e2b = effect::clone(d, e2a);
    // e2b:SetCode(EFFECT_MUST_ATTACK_MONSTER)
    effect::set_code(d, e2b, EFFECT_MUST_ATTACK_MONSTER);
    // e2b:SetValue(function(e,c) return Duel.GetMatchingGroup(Card.IsFaceup,e:GetHandlerPlayer(),LOCATION_MZONE,0,nil):GetMaxGroup(Card.GetAttack):IsContains(c) end)
    effect::set_value_fn(d, e2b, effect_card_fn(must_attack_value));
    // c:RegisterEffect(e2b)
    card::register_effect(d, c, e2b, false);
    // Special Summon 1 EARTH Warrior monster from your GY or banishment
    // local e3=Effect.CreateEffect(c)
    let e3 = effect::create_effect(d, c);
    // e3:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e3, aux::stringid(ID, 0));
    // e3:SetCategory(CATEGORY_SPECIAL_SUMMON)
    effect::set_category(d, e3, CATEGORY_SPECIAL_SUMMON);
    // e3:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e3, EFFECT_TYPE_FIELD + EFFECT_TYPE_TRIGGER_O);
    // e3:SetCode(EVENT_BATTLE_DESTROYING)
    effect::set_code(d, e3, EVENT_BATTLE_DESTROYING);
    // e3:SetRange(LOCATION_MZONE)
    effect::set_range(d, e3, LOCATION_MZONE);
    // e3:SetCountLimit(1,id)
    effect::set_count_limit(d, e3, 1, ID, 0, 0);
    // e3:SetCondition(s.spcon)
    effect::set_condition(d, e3, Some(condition(spcon)));
    // e3:SetTarget(s.sptg)
    effect::set_target(d, e3, Some(cost_target(sptg)));
    // e3:SetOperation(s.spop)
    effect::set_operation(d, e3, Some(operation(spop)));
    // c:RegisterEffect(e3)
    card::register_effect(d, c, e3, false);
}
/// `s.matfilter1(c,fc,sumtype,tp)`
fn matfilter1(d: &mut Duel, c: CardId, fc: CardId, sumtype: u64, tp: u8) -> bool {
    // return c:IsLevelAbove(5) and s.matfilter2(c,fc,sumtype,tp)
    card::is_level_above(d, c, 5) && matfilter2(d, c, fc, sumtype, tp)
}
/// `s.matfilter2(c,fc,sumtype,tp)`
fn matfilter2(d: &mut Duel, c: CardId, fc: CardId, sumtype: u64, tp: u8) -> bool {
    // return c:IsAttribute(ATTRIBUTE_EARTH,fc,sumtype,tp) and c:IsRace(RACE_WARRIOR,fc,sumtype,tp)
    card::is_attribute_ctx(d, c, ATTRIBUTE_EARTH, Some(fc), sumtype, Some(tp))
        && card::is_race_ctx(d, c, RACE_WARRIOR, Some(fc), sumtype, Some(tp))
}
/// `function() return Duel.IsMainPhase() end`
fn main_phase_cond(d: &mut Duel, _a: &EffArgs) -> bool {
    duel::is_main_phase(d, None)
}
/// `s.immval(e,te)`: `EFFECT_IMMUNE_EFFECT`'s value
fn immval(d: &mut Duel, a: Args) -> bool {
    // return te:GetOwnerPlayer()~=e:GetHandlerPlayer() and te:IsActivated()
    let e = a.e(0);
    let te = a.e(1);
    effect::get_owner_player(d, te) != effect::get_handler_player(d, e) && effect::is_activated(d, te)
}
/// `function(e,c) return Duel.GetMatchingGroup(Card.IsFaceup,e:GetHandlerPlayer(),LOCATION_MZONE,0,nil):GetMaxGroup(Card.GetAttack):IsContains(c) end`
fn must_attack_value(d: &mut Duel, e: EffectId, c: CardId) -> bool {
    let p = effect::get_handler_player(d, e);
    let g = duel::get_matching_group(d, Some(&|d: &mut Duel, x| card::is_faceup(d, x)), p, LOCATION_MZONE, 0, NO_EX);
    let (max, _) = group::get_max_group(d, &g, &|d: &mut Duel, x| card::get_attack(d, x) as i64).nil();
    group::is_contains(&max, c)
}
/// `s.spconfilter(c,tp)`
fn spconfilter(d: &mut Duel, c: CardId, tp: u8) -> bool {
    // local bc=c:GetBattleTarget()
    let bc = card::get_battle_target(d, c).nil();
    // if not bc:IsPreviousControler(1-tp) then return false end
    if !card::is_previous_controler(d, bc, 1 - tp) {
        return false;
    }
    // if c:IsRelateToBattle() then
    if card::is_relate_to_battle(d, c) {
        // return c:IsAttribute(ATTRIBUTE_EARTH) and c:IsRace(RACE_WARRIOR) and c:IsControler(tp)
        card::is_attribute(d, c, ATTRIBUTE_EARTH) && card::is_race(d, c, RACE_WARRIOR) && card::is_controler(d, c, tp)
    } else {
        // return c:IsPreviousAttributeOnField(ATTRIBUTE_EARTH) and c:IsPreviousRaceOnField(RACE_WARRIOR) and c:IsPreviousControler(tp)
        card::is_previous_attribute_on_field(d, c, ATTRIBUTE_EARTH)
            && card::is_previous_race_on_field(d, c, RACE_WARRIOR)
            && card::is_previous_controler(d, c, tp)
    }
}
/// `s.spcon(e,tp,eg,ep,ev,re,r,rp)`
fn spcon(d: &mut Duel, a: &EffArgs) -> bool {
    // return eg:IsExists(s.spconfilter,1,e:GetHandler(),tp)
    let h = effect::get_handler(d, a.e).nil();
    group::is_exists(d, &a.eg.clone().nil(), &|d: &mut Duel, x| spconfilter(d, x, a.tp), 1, h.into())
}
/// `s.spfilter(c,e,tp)`
fn spfilter(d: &mut Duel, c: CardId, e: EffectId, tp: u8) -> bool {
    // return c:IsAttribute(ATTRIBUTE_EARTH) and c:IsRace(RACE_WARRIOR) and c:IsFaceup()
    //     and c:IsCanBeSpecialSummoned(e,0,tp,false,false)
    card::is_attribute(d, c, ATTRIBUTE_EARTH)
        && card::is_race(d, c, RACE_WARRIOR)
        && card::is_faceup(d, c)
        && card::is_can_be_special_summoned(d, c, e, 0, tp, false, false, None, None, None)
}
/// `s.sptg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn sptg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return Duel.GetMZoneCount(tp)>0
    //     and Duel.IsExistingMatchingCard(s.spfilter,tp,LOCATION_GRAVE|LOCATION_REMOVED,0,1,nil,e,tp) end
    if a.chk == Some(0) {
        return duel::get_m_zone_count(d, a.tp, NO_EX, None, None, None).0 > 0
            && duel::is_existing_matching_card(d, Some(&|d: &mut Duel, x| spfilter(d, x, a.e, a.tp)), a.tp, LOCATION_GRAVE | LOCATION_REMOVED, 0, 1, NO_EX);
    }
    // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,nil,1,tp,LOCATION_GRAVE|LOCATION_REMOVED)
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, None, 1, a.tp, (LOCATION_GRAVE | LOCATION_REMOVED) as i32);
    true
}
/// `s.spop(e,tp,eg,ep,ev,re,r,rp)`
fn spop(d: &mut Duel, a: &EffArgs) {
    // if Duel.GetMZoneCount(tp)<=0 then return end
    if duel::get_m_zone_count(d, a.tp, NO_EX, None, None, None).0 <= 0 {
        return;
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_SPSUMMON)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_SPSUMMON as u64);
    // local g=Duel.SelectMatchingCard(tp,aux.NecroValleyFilter(s.spfilter),tp,LOCATION_GRAVE|LOCATION_REMOVED,0,1,1,nil,e,tp)
    let g = duel::select_matching_card(
        d,
        a.tp,
        Some(&aux::necro_valley_filter(|d: &mut Duel, x| spfilter(d, x, a.e, a.tp))),
        a.tp,
        LOCATION_GRAVE | LOCATION_REMOVED,
        0,
        1,
        1,
        false,
        NO_EX,
    )
    .nil();
    // if #g>0 then
    if !g.is_empty() {
        // Duel.SpecialSummon(g,0,tp,tp,false,false,POS_FACEUP)
        duel::special_summon(d, g, 0, a.tp, a.tp, false, false, POS_FACEUP as u8, None);
    }
}
