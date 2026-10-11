//! Number 99: Utopia Dragonar
//! Scripted by Larry126
use super::prelude::*;

pub const ID: u32 = 95134948;
pub const INFO: ScriptInfo = ScriptInfo { xyz_number: Some(99), listed_series: &[SET_NUMBER], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // c:EnableReviveLimit()
    card::enable_revive_limit(d, c);
    // Xyz Summon procedure: 3+ Level 12 monsters
    // Xyz.AddProcedure(c,nil,12,3,nil,nil,Xyz.InfiniteMats)
    xyz::add_procedure(d, c, None, Some(12), 3, None, None, Some(xyz::INFINITE_MATS), None, false, None);
    // Special Summon from your Extra Deck, 1 "Number" monster that has a number between "1" and "100" in its name (this is treated as an Xyz Summon)
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
    // e1:SetRange(LOCATION_MZONE)
    effect::set_range(d, e1, LOCATION_MZONE);
    // e1:SetCountLimit(1,id)
    effect::set_count_limit(d, e1, 1, ID, 0, 0);
    // e1:SetHintTiming(0,TIMING_STANDBY_PHASE|TIMING_MAIN_END|TIMINGS_CHECK_MONSTER_E)
    effect::set_hint_timing(d, e1, 0, Some(TIMING_STANDBY_PHASE | TIMING_MAIN_END | TIMINGS_CHECK_MONSTER_E));
    // e1:SetCost(Cost.AND(Cost.DetachFromSelf(2),s.spcost))
    effect::set_cost(
        d,
        e1,
        Some(cost::and(vec![cost::detach_from_self(cost::Count::N(2), None, None), cost_target(spcost)])),
    );
    // e1:SetTarget(s.sptg)
    effect::set_target(d, e1, Some(cost_target(sptg)));
    // e1:SetOperation(s.spop)
    effect::set_operation(d, e1, Some(operation(spop)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // Change the ATK of the opponent's monster to 0
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e2, aux::stringid(ID, 1));
    // e2:SetCategory(CATEGORY_ATKCHANGE)
    effect::set_category(d, e2, CATEGORY_ATKCHANGE);
    // e2:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e2, EFFECT_TYPE_FIELD + EFFECT_TYPE_TRIGGER_O);
    // e2:SetCode(EVENT_ATTACK_ANNOUNCE)
    effect::set_code(d, e2, EVENT_ATTACK_ANNOUNCE);
    // e2:SetRange(LOCATION_MZONE)
    effect::set_range(d, e2, LOCATION_MZONE);
    // e2:SetCountLimit(1,{id,1})
    effect::set_count_limit(d, e2, 1, ID, 1, 0);
    // e2:SetCondition(function(e,tp) return Duel.GetAttacker():IsControler(1-tp) end)
    effect::set_condition(
        d,
        e2,
        Some(condition(|d, a| {
            let at = duel::get_attacker(d).nil();
            card::is_controler(d, at, 1 - a.tp)
        })),
    );
    // e2:SetTarget(s.atktg)
    effect::set_target(d, e2, Some(cost_target(atktg)));
    // e2:SetOperation(s.atkop)
    effect::set_operation(d, e2, Some(operation(atkop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
    // Keep track of Special Summons and direct attacks
    // Duel.AddCustomActivityCounter(id,ACTIVITY_SPSUMMON,function(c) return not c:IsSummonLocation(LOCATION_EXTRA) or c:IsType(TYPE_XYZ) end)
    duel::add_custom_activity_counter(
        d,
        ID,
        ACTIVITY_SPSUMMON,
        lua_fn(|d, a: Args| {
            let c = a.c(0);
            !card::is_summon_location(d, c, LOCATION_EXTRA) || card::is_type(d, c, TYPE_XYZ)
        }),
    );
    // aux.GlobalCheck(s,function()
    //     local ge1=Effect.CreateEffect(c)
    //     ...
    // end)
    aux::global_check(d, ID, |d| {
        // local ge1=Effect.CreateEffect(c)
        let ge1 = effect::create_effect(d, c);
        // ge1:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_CONTINUOUS)
        effect::set_type(d, ge1, EFFECT_TYPE_FIELD + EFFECT_TYPE_CONTINUOUS);
        // ge1:SetCode(EVENT_ATTACK_ANNOUNCE)
        effect::set_code(d, ge1, EVENT_ATTACK_ANNOUNCE);
        // ge1:SetOperation(s.checkop)
        effect::set_operation(d, ge1, Some(operation(checkop)));
        // Duel.RegisterEffect(ge1,0)
        duel::register_effect(d, ge1, 0);
    });
}
/// `s.checkop(e,tp,eg,ep,ev,re,r,rp)`
fn checkop(d: &mut Duel, a: &EffArgs) {
    // if not Duel.GetAttackTarget() then
    if duel::get_attack_target(d).is_none() {
        // Duel.RegisterFlagEffect(ep,id,RESET_PHASE|PHASE_END,0,1)
        duel::register_flag_effect(d, a.ep, ID, RESET_PHASE | PHASE_END, 0, 1, 0);
        // eg:GetFirst():RegisterFlagEffect(id,RESETS_STANDARD_PHASE_END,0,1)
        let eg = a.eg.clone().nil();
        let first = group::get_first(d, &eg).nil();
        card::register_flag_effect(d, first, ID, RESETS_STANDARD_PHASE_END, 0, 1, 0, 0);
    }
}
/// `s.spcost(e,tp,eg,ep,ev,re,r,rp,chk)`
fn spcost(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if chk==0 then return Duel.GetCustomActivityCount(id,tp,ACTIVITY_SPSUMMON)==0
    //     and (not Duel.HasFlagEffect(tp,id) or Duel.GetFlagEffect(tp,id)==c:GetFlagEffect(id)) end
    if a.chk == Some(0) {
        return duel::get_custom_activity_count(d, ID, tp, ACTIVITY_SPSUMMON) == 0
            && (!duel::has_flag_effect(d, tp, ID, None) || duel::get_flag_effect(d, tp, ID) == card::get_flag_effect(d, c, ID));
    }
    // local fid=c:GetFieldID()
    let fid = card::get_field_id(d, c);
    // The turn you activate this effect, other monsters you control cannot attack directly
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e1, EFFECT_TYPE_FIELD);
    // e1:SetProperty(EFFECT_FLAG_OATH+EFFECT_FLAG_IGNORE_IMMUNE)
    effect::set_property(d, e1, EFFECT_FLAG_OATH + EFFECT_FLAG_IGNORE_IMMUNE, 0);
    // e1:SetCode(EFFECT_CANNOT_DIRECT_ATTACK)
    effect::set_code(d, e1, EFFECT_CANNOT_DIRECT_ATTACK);
    // e1:SetTargetRange(LOCATION_MZONE,0)
    effect::set_target_range(d, e1, LOCATION_MZONE, 0);
    // e1:SetTarget(function(e,c) return c:GetFieldID()~=fid end)
    effect::set_target(d, e1, Some(effect_card_fn(move |d, _e, x| card::get_field_id(d, x) != fid)));
    // e1:SetReset(RESET_PHASE|PHASE_END)
    effect::set_reset(d, e1, RESET_PHASE | PHASE_END, 1);
    // Duel.RegisterEffect(e1,tp)
    duel::register_effect(d, e1, tp);
    // Also you cannot Special Summon from the Extra Deck, except Xyz Monsters
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,2))
    effect::set_description(d, e2, aux::stringid(ID, 2));
    // e2:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e2, EFFECT_TYPE_FIELD);
    // e2:SetProperty(EFFECT_FLAG_PLAYER_TARGET+EFFECT_FLAG_OATH+EFFECT_FLAG_CLIENT_HINT)
    effect::set_property(d, e2, EFFECT_FLAG_PLAYER_TARGET + EFFECT_FLAG_OATH + EFFECT_FLAG_CLIENT_HINT, 0);
    // e2:SetCode(EFFECT_CANNOT_SPECIAL_SUMMON)
    effect::set_code(d, e2, EFFECT_CANNOT_SPECIAL_SUMMON);
    // e2:SetTargetRange(1,0)
    effect::set_target_range(d, e2, 1, 0);
    // e2:SetTarget(function(e,c) return c:IsLocation(LOCATION_EXTRA) and not c:IsType(TYPE_XYZ) end)
    effect::set_target(
        d,
        e2,
        Some(effect_card_fn(|d, _e, x| card::is_location(d, x, LOCATION_EXTRA) && !card::is_type(d, x, TYPE_XYZ))),
    );
    // e2:SetReset(RESET_PHASE|PHASE_END)
    effect::set_reset(d, e2, RESET_PHASE | PHASE_END, 1);
    // Duel.RegisterEffect(e2,tp)
    duel::register_effect(d, e2, tp);
    // "Clock Lizard" check
    // aux.addTempLizardCheck(c,tp,function(e,c) return not c:IsOriginalType(TYPE_XYZ) end)
    aux::add_temp_lizard_check(d, c, tp, Some(effect_card_fn(|d, _e, x| !card::is_original_type(d, x, TYPE_XYZ))), None, None, None, None);
    true
}
/// `s.spfilter(c,e,tp)`
fn spfilter(d: &mut Duel, c: CardId, e: EffectId, tp: u8) -> bool {
    // return c:IsSetCard(SET_NUMBER) and c.xyz_number and c.xyz_number>=1 and c.xyz_number<=100
    //     and Duel.GetLocationCountFromEx(tp,tp,nil,c)>0 and c:IsCanBeSpecialSummoned(e,SUMMON_TYPE_XYZ,tp,false,false)
    card::is_set_card(d, c, &[SET_NUMBER as u16])
        && matches!(crate::card_info(d, c).xyz_number, Some(n) if (1..=100).contains(&n))
        && duel::get_location_count_from_ex(d, tp, Some(tp), NO_EX, duel::ExtraCard::Card(c), None).0 > 0
        && card::is_can_be_special_summoned(d, c, e, SUMMON_TYPE_XYZ, tp, false, false, None, None, None)
}
/// `s.sptg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn sptg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // if chk==0 then return Duel.IsExistingMatchingCard(s.spfilter,tp,LOCATION_EXTRA,0,1,nil,e,tp) end
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, Some(&|d, c| spfilter(d, c, a.e, tp)), tp, LOCATION_EXTRA, 0, 1, NO_EX);
    }
    // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,nil,1,tp,LOCATION_EXTRA)
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, None, 1, tp, LOCATION_EXTRA as i32);
    true
}
/// `s.spop(e,tp,eg,ep,ev,re,r,rp)`
fn spop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_SPSUMMON)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_SPSUMMON as u64);
    // local sc=Duel.SelectMatchingCard(tp,s.spfilter,tp,LOCATION_EXTRA,0,1,1,nil,e,tp):GetFirst()
    let sel = duel::select_matching_card(d, tp, Some(&|d, c| spfilter(d, c, a.e, tp)), tp, LOCATION_EXTRA, 0, 1, 1, false, NO_EX).nil();
    let sc = group::get_first(d, &sel);
    // if not sc then return end
    let Some(sc) = sc else { return };
    // sc:SetMaterial(nil)
    card::set_material(d, sc, CardSet::new());
    // if Duel.SpecialSummon(sc,SUMMON_TYPE_XYZ,tp,tp,false,false,POS_FACEUP)>0 then
    if duel::special_summon(d, sc, SUMMON_TYPE_XYZ, tp, tp, false, false, POS_FACEUP as u8, None) > 0 {
        // sc:CompleteProcedure()
        card::complete_procedure(d, sc);
    }
}
/// `s.atktg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn atktg(d: &mut Duel, a: &EffArgs) -> bool {
    // local bc=Duel.GetAttacker()
    let bc = duel::get_attacker(d).nil();
    // if chk==0 then return bc:IsOnField() and bc:HasNonZeroAttack() end
    if a.chk == Some(0) {
        return card::is_on_field(d, bc) && card::has_non_zero_attack(d, bc);
    }
    // e:SetLabelObject(bc)
    effect::set_label_object(d, a.e, Value::Card(bc));
    // bc:CreateEffectRelation(e)
    card::create_effect_relation(d, bc, a.e);
    true
}
/// `s.atkop(e,tp,eg,ep,ev,re,r,rp)`
fn atkop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // local bc=e:GetLabelObject()
    let bc = effect::get_label_object(d, a.e).as_card().nil();
    // if bc:IsRelateToEffect(e) and bc:IsFaceup() and bc:IsControler(1-tp) then
    if card::is_relate_to_effect(d, bc, a.e) && card::is_faceup(d, bc) && card::is_controler(d, bc, 1 - tp) {
        // The ATK of that opponent's monster becomes 0
        // local e1=Effect.CreateEffect(e:GetHandler())
        let h = effect::get_handler(d, a.e).nil();
        let e1 = effect::create_effect(d, h);
        // e1:SetType(EFFECT_TYPE_SINGLE)
        effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
        // e1:SetProperty(EFFECT_FLAG_CANNOT_DISABLE)
        effect::set_property(d, e1, EFFECT_FLAG_CANNOT_DISABLE, 0);
        // e1:SetCode(EFFECT_SET_ATTACK_FINAL)
        effect::set_code(d, e1, EFFECT_SET_ATTACK_FINAL);
        // e1:SetValue(0)
        effect::set_value(d, e1, 0);
        // e1:SetReset(RESET_EVENT|RESETS_STANDARD)
        effect::set_reset(d, e1, RESET_EVENT | RESETS_STANDARD, 1);
        // bc:RegisterEffect(e1)
        card::register_effect(d, bc, e1, false);
    }
}
