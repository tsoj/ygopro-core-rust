//! Elemental HERO Grandmerge
use super::prelude::*;

pub const ID: u32 = 36841733;

pub const INFO: ScriptInfo = ScriptInfo {
    listed_series: &[SET_ELEMENTAL_HERO],
    material_setcode: &[SET_HERO],
    ..ScriptInfo::DEFAULT
};

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // fusion material
    // c:EnableReviveLimit()
    card::enable_revive_limit(d, c);
    // Fusion.AddProcMixN(c,true,true,s.ffilter,2)
    fusion::add_proc_mix_n(
        d,
        c,
        true,
        true,
        vec![(fusion::Mat::F(fusion::fus_mat(|d, x, fc, st, tp| ffilter(d, x, fc, st, tp))), 2)],
    );
    // spsummon condition
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
    // e1:SetProperty(EFFECT_FLAG_CANNOT_DISABLE+EFFECT_FLAG_UNCOPYABLE)
    effect::set_property(d, e1, EFFECT_FLAG_CANNOT_DISABLE + EFFECT_FLAG_UNCOPYABLE, 0);
    // e1:SetCode(EFFECT_SPSUMMON_CONDITION)
    effect::set_code(d, e1, EFFECT_SPSUMMON_CONDITION);
    // e1:SetValue(aux.fuslimit)
    effect::set_value_fn(d, e1, aux::fuslimit());
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // atk
    // local e0=Effect.CreateEffect(c)
    let e0 = effect::create_effect(d, c);
    // e0:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e0, EFFECT_TYPE_SINGLE);
    // e0:SetCode(EFFECT_MATERIAL_CHECK)
    effect::set_code(d, e0, EFFECT_MATERIAL_CHECK);
    // e0:SetValue(s.valcheck)
    effect::set_value_fn(d, e0, effect_card_value(|d, e, c| {
        valcheck(d, e, c);
        0
    }));
    // c:RegisterEffect(e0)
    card::register_effect(d, c, e0, false);
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_CONTINUOUS)
    effect::set_type(d, e2, EFFECT_TYPE_SINGLE + EFFECT_TYPE_CONTINUOUS);
    // e2:SetCode(EVENT_SPSUMMON_SUCCESS)
    effect::set_code(d, e2, EVENT_SPSUMMON_SUCCESS);
    // e2:SetCondition(s.atkcon)
    effect::set_condition(d, e2, Some(condition(atkcon)));
    // e2:SetOperation(s.atkop)
    effect::set_operation(d, e2, Some(operation(atkop)));
    // e2:SetLabelObject(e0)
    effect::set_label_object(d, e2, Value::Effect(e0));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
    // special summon
    // local e3=Effect.CreateEffect(c)
    let e3 = effect::create_effect(d, c);
    // e3:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e3, aux::stringid(ID, 0));
    // e3:SetCategory(CATEGORY_SPECIAL_SUMMON)
    effect::set_category(d, e3, CATEGORY_SPECIAL_SUMMON);
    // e3:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e3, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_O);
    // e3:SetCode(EVENT_BATTLE_DESTROYING)
    effect::set_code(d, e3, EVENT_BATTLE_DESTROYING);
    // e3:SetCondition(s.spcon)
    effect::set_condition(d, e3, Some(condition(spcon)));
    // e3:SetCost(Cost.SelfTribute)
    effect::set_cost(d, e3, Some(cost::self_tribute()));
    // e3:SetTarget(s.sptg)
    effect::set_target(d, e3, Some(cost_target(sptg)));
    // e3:SetOperation(s.spop)
    effect::set_operation(d, e3, Some(operation(spop)));
    // c:RegisterEffect(e3)
    card::register_effect(d, c, e3, false);
}
/// `s.ffilter(c,fc,sumtype,tp)`
fn ffilter(d: &mut Duel, c: CardId, fc: CardId, sumtype: u64, tp: u8) -> bool {
    // return c:IsType(TYPE_NORMAL,fc,sumtype,tp) and c:IsSetCard(SET_HERO,fc,sumtype,tp)
    card::is_type_ctx(d, c, TYPE_NORMAL, Some(fc), sumtype, Some(tp)) && card::is_set_card_ctx(d, c, &[SET_HERO as u16], Some(fc), sumtype, Some(tp))
}
/// `s.valcheck(e,c)`
fn valcheck(d: &mut Duel, e: EffectId, c: CardId) {
    // local g=c:GetMaterial()
    let g = card::get_material(d, c);
    // local atk=0
    let mut atk: i64 = 0;
    // for tc in aux.Next(g) do
    for tc in group::iter(d, &g) {
        // atk=atk+tc:GetOriginalLevel()
        atk += card::get_original_level(d, tc) as i64;
    }
    // e:SetLabel(atk)
    effect::set_label(d, e, &[atk]);
}
/// `s.atkcon(e,tp,eg,ep,ev,re,r,rp)`
fn atkcon(d: &mut Duel, a: &EffArgs) -> bool {
    // return e:GetHandler():IsFusionSummoned()
    let h = effect::get_handler(d, a.e).nil();
    card::is_fusion_summoned(d, h)
}
/// `s.atkop(e,tp,eg,ep,ev,re,r,rp)`
fn atkop(d: &mut Duel, a: &EffArgs) {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // local atk=e:GetLabelObject():GetLabel()*300
    let lo = effect::get_label_object(d, a.e).as_effect().nil();
    let atk = effect::get_label(d, lo) * 300;
    // if atk>0 then
    if atk > 0 {
        // local e1=Effect.CreateEffect(c)
        let e1 = effect::create_effect(d, c);
        // e1:SetType(EFFECT_TYPE_SINGLE)
        effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
        // e1:SetCode(EFFECT_UPDATE_ATTACK)
        effect::set_code(d, e1, EFFECT_UPDATE_ATTACK);
        // e1:SetValue(atk)
        effect::set_value(d, e1, atk);
        // e1:SetReset(RESET_EVENT|RESETS_STANDARD_DISABLE)
        effect::set_reset(d, e1, RESET_EVENT | RESETS_STANDARD_DISABLE, 1);
        // c:RegisterEffect(e1)
        card::register_effect(d, c, e1, false);
        // local e2=e1:Clone()
        let e2 = effect::clone(d, e1);
        // e2:SetCode(EFFECT_UPDATE_DEFENSE)
        effect::set_code(d, e2, EFFECT_UPDATE_DEFENSE);
        // c:RegisterEffect(e2)
        card::register_effect(d, c, e2, false);
    }
}
/// `s.spcon(e,tp,eg,ep,ev,re,r,rp)`
fn spcon(d: &mut Duel, a: &EffArgs) -> bool {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // local bc=c:GetBattleTarget()
    let bc = card::get_battle_target(d, c);
    // return c:IsRelateToBattle() and bc and bc:IsMonster()
    if !card::is_relate_to_battle(d, c) {
        return false;
    }
    match bc {
        Some(bc) => card::is_monster(d, bc),
        None => false,
    }
}
/// `s.spfilter(c,e,tp)`
fn spfilter(d: &mut Duel, c: CardId, e: EffectId, tp: u8) -> bool {
    // return c:IsType(TYPE_FUSION) and c:IsSetCard(SET_ELEMENTAL_HERO) and Duel.GetLocationCountFromEx(tp,tp,e:GetHandler(),c)>0
    //     and c:IsCanBeSpecialSummoned(e,0,tp,true,false)
    if !(card::is_type(d, c, TYPE_FUSION) && card::is_set_card(d, c, &[SET_ELEMENTAL_HERO as u16])) {
        return false;
    }
    let h = effect::get_handler(d, e).nil();
    duel::get_location_count_from_ex(d, tp, Some(tp), h.into(), duel::ExtraCard::Card(c), None).0 > 0
        && card::is_can_be_special_summoned(d, c, e, 0, tp, true, false, None, None, None)
}
/// `s.sptg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn sptg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return Duel.IsExistingMatchingCard(s.spfilter,tp,LOCATION_EXTRA,0,1,nil,e,tp) end
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, Some(&|d, c| spfilter(d, c, a.e, a.tp)), a.tp, LOCATION_EXTRA, 0, 1, NO_EX);
    }
    // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,nil,1,tp,LOCATION_EXTRA)
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, None, 1, a.tp, LOCATION_EXTRA as i32);
    true
}
/// `bttg(e,c)`: the battle target restriction of the summoned monster
fn bttg(d: &mut Duel, e: EffectId, c: CardId) -> bool {
    // return c:IsFaceup() and c:IsLevelBelow(e:GetHandler():GetLevel())
    let h = effect::get_handler(d, e).nil();
    let lv = card::get_level(d, h);
    card::is_faceup(d, c) && card::is_level_below(d, c, lv)
}
/// `s.spop(e,tp,eg,ep,ev,re,r,rp)`
fn spop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_SPSUMMON)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_SPSUMMON as u64);
    // local g=Duel.SelectMatchingCard(tp,s.spfilter,tp,LOCATION_EXTRA,0,1,1,nil,e,tp)
    let g = duel::select_matching_card(d, tp, Some(&|d, c| spfilter(d, c, a.e, tp)), tp, LOCATION_EXTRA, 0, 1, 1, false, NO_EX).nil();
    // local tc=g:GetFirst()
    let tc = group::get_first(d, &g);
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if tc and Duel.SpecialSummonStep(tc,0,tp,tp,true,false,POS_FACEUP) then
    if let Some(tc) = tc {
        if duel::special_summon_step(d, tc, 0, tp, tp, true, false, POS_FACEUP as u8, None) {
            // atklimit
            // local e1=Effect.CreateEffect(c)
            let e1 = effect::create_effect(d, c);
            // e1:SetType(EFFECT_TYPE_SINGLE)
            effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
            // e1:SetCode(EFFECT_CANNOT_SELECT_BATTLE_TARGET)
            effect::set_code(d, e1, EFFECT_CANNOT_SELECT_BATTLE_TARGET);
            // e1:SetProperty(EFFECT_FLAG_CANNOT_DISABLE+EFFECT_FLAG_IGNORE_IMMUNE)
            effect::set_property(d, e1, EFFECT_FLAG_CANNOT_DISABLE + EFFECT_FLAG_IGNORE_IMMUNE, 0);
            // e1:SetValue(s.bttg)
            effect::set_value_fn(d, e1, effect_card_fn(bttg));
            // e1:SetReset(RESET_EVENT|RESETS_STANDARD)
            effect::set_reset(d, e1, RESET_EVENT | RESETS_STANDARD, 1);
            // tc:RegisterEffect(e1,true)
            card::register_effect(d, tc, e1, true);
        }
    }
    // Duel.SpecialSummonComplete()
    duel::special_summon_complete(d);
}
