//! Reptilianne Recoil
//! Scripted by Hatter
use super::prelude::*;

pub const ID: u32 = 17000165;

pub const INFO: ScriptInfo = ScriptInfo {
    listed_names: &[TOKEN_REPTILIANNE],
    listed_series: &[SET_REPTILIANNE],
    ..ScriptInfo::DEFAULT
};

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // --Activate
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    // e1:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // --Destroy and Special Summon
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e2, aux::stringid(ID, 1));
    // e2:SetCategory(CATEGORY_DESTROY+CATEGORY_SPECIAL_SUMMON)
    effect::set_category(d, e2, CATEGORY_DESTROY + CATEGORY_SPECIAL_SUMMON);
    // e2:SetType(EFFECT_TYPE_IGNITION)
    effect::set_type(d, e2, EFFECT_TYPE_IGNITION);
    // e2:SetProperty(EFFECT_FLAG_CARD_TARGET)
    effect::set_property(d, e2, EFFECT_FLAG_CARD_TARGET, 0);
    // e2:SetRange(LOCATION_FZONE)
    effect::set_range(d, e2, LOCATION_FZONE);
    // e2:SetCountLimit(1,id)
    effect::set_count_limit(d, e2, 1, ID, 0, 0);
    // e2:SetTarget(s.sptg)
    effect::set_target(d, e2, Some(cost_target(sptg)));
    // e2:SetOperation(s.spop)
    effect::set_operation(d, e2, Some(operation(spop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
    // --Take control of opponent monster and Special Summon token
    // local e3=Effect.CreateEffect(c)
    let e3 = effect::create_effect(d, c);
    // e3:SetDescription(aux.Stringid(id,2))
    effect::set_description(d, e3, aux::stringid(ID, 2));
    // e3:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e3, EFFECT_TYPE_FIELD + EFFECT_TYPE_TRIGGER_O);
    // e3:SetCategory(CATEGORY_CONTROL+CATEGORY_SPECIAL_SUMMON+CATEGORY_TOKEN)
    effect::set_category(d, e3, CATEGORY_CONTROL + CATEGORY_SPECIAL_SUMMON + CATEGORY_TOKEN);
    // e3:SetProperty(EFFECT_FLAG_DELAY+EFFECT_FLAG_CARD_TARGET)
    effect::set_property(d, e3, EFFECT_FLAG_DELAY + EFFECT_FLAG_CARD_TARGET, 0);
    // e3:SetCode(EVENT_CHAINING)
    effect::set_code(d, e3, EVENT_CHAINING);
    // e3:SetRange(LOCATION_FZONE)
    effect::set_range(d, e3, LOCATION_FZONE);
    // e3:SetCountLimit(1,{id,1})
    effect::set_count_limit(d, e3, 1, ID, 1, 0);
    // e3:SetCondition(s.ctlcon)
    effect::set_condition(d, e3, Some(condition(ctlcon)));
    // e3:SetTarget(s.ctltg)
    effect::set_target(d, e3, Some(cost_target(ctltg)));
    // e3:SetOperation(s.ctlop)
    effect::set_operation(d, e3, Some(operation(ctlop)));
    // c:RegisterEffect(e3)
    card::register_effect(d, c, e3, false);
}
// function s.desfilter(c,tp)
fn desfilter(d: &mut Duel, c: CardId, tp: u8) -> bool {
    // return c:IsFaceup() and c:IsAttack(0) and Duel.GetMZoneCount(tp,c)>0
    card::is_faceup(d, c) && card::is_attack(d, c, &[0]) && duel::get_m_zone_count(d, tp, c.into(), None, None, None).0 > 0
}
// function s.spfilter(c,e,tp)
fn spfilter(d: &mut Duel, c: CardId, e: EffectId, tp: u8) -> bool {
    // return c:IsAttribute(ATTRIBUTE_DARK) and c:IsRace(RACE_REPTILE) and c:IsCanBeSpecialSummoned(e,0,tp,false,false)
    card::is_attribute(d, c, ATTRIBUTE_DARK)
        && card::is_race(d, c, RACE_REPTILE)
        && card::is_can_be_special_summoned(d, c, e, 0, tp, false, false, None, None, None)
}
// function s.sptg(e,tp,eg,ep,ev,re,r,rp,chk,chkc)
fn sptg(d: &mut Duel, a: &EffArgs) -> bool {
    let (e, tp) = (a.e, a.tp);
    // if chkc then return false end
    if a.chkc().is_some() {
        return false;
    }
    // if chk==0 then return Duel.IsExistingTarget(s.desfilter,tp,LOCATION_MZONE,0,1,nil,tp)
    //     and Duel.IsExistingTarget(s.spfilter,tp,LOCATION_GRAVE,0,1,nil,e,tp)
    if a.chk == Some(0) {
        return duel::is_existing_target(d, Some(&|d, c| desfilter(d, c, tp)), tp, LOCATION_MZONE, 0, 1, NO_EX)
            && duel::is_existing_target(d, Some(&|d, c| spfilter(d, c, e, tp)), tp, LOCATION_GRAVE, 0, 1, NO_EX);
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_DESTROY)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_DESTROY as u64);
    // local g1=Duel.SelectTarget(tp,s.desfilter,tp,LOCATION_MZONE,0,1,1,nil,tp)
    let g1 = duel::select_target(d, tp, Some(&|d, c| desfilter(d, c, tp)), tp, LOCATION_MZONE, 0, 1, 1, false, NO_EX).nil();
    // e:SetLabelObject(g1:GetFirst())
    let first = match group::get_first(d, &g1) {
        Some(c) => Value::Card(c),
        None => Value::Nil,
    };
    effect::set_label_object(d, e, first);
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_SPSUMMON)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_SPSUMMON as u64);
    // local g2=Duel.SelectTarget(tp,s.spfilter,tp,LOCATION_GRAVE,0,1,1,nil,e,tp)
    let g2 = duel::select_target(d, tp, Some(&|d, c| spfilter(d, c, e, tp)), tp, LOCATION_GRAVE, 0, 1, 1, false, NO_EX);
    // Duel.SetOperationInfo(0,CATEGORY_DESTROY,g1,1,0,0)
    duel::set_operation_info(d, 0, CATEGORY_DESTROY, Some(g1), 1, 0, 0);
    // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,g2,1,0,0)
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, g2, 1, 0, 0);
    true
}
// function s.spop(e,tp,eg,ep,ev,re,r,rp)
fn spop(d: &mut Duel, a: &EffArgs) {
    let (e, tp) = (a.e, a.tp);
    // local tc1,tc2=Duel.GetFirstTarget()
    let targets = duel::get_first_target(d);
    let mut tc1 = targets.first().copied();
    let mut tc2 = targets.get(1).copied();
    // if tc1~=e:GetLabelObject() then tc1,tc2=tc2,tc1 end
    let label_obj = effect::get_label_object(d, e).as_card();
    if tc1 != label_obj {
        std::mem::swap(&mut tc1, &mut tc2);
    }
    // if tc1:IsRelateToEffect(e) and Duel.Destroy(tc1,REASON_EFFECT)>0
    let tc1 = tc1.nil();
    if !card::is_relate_to_effect(d, tc1, e) || duel::destroy(d, tc1, REASON_EFFECT, None, None) <= 0 {
        return;
    }
    // and tc2:IsRelateToEffect(e) and Duel.GetLocationCount(tp,LOCATION_MZONE)>0 then
    let tc2 = tc2.nil();
    if !card::is_relate_to_effect(d, tc2, e) || duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 <= 0 {
        return;
    }
    // Duel.SpecialSummon(tc2,0,tp,tp,false,false,POS_FACEUP)
    duel::special_summon(d, tc2, 0, tp, tp, false, false, POS_FACEUP as u8, None);
}
// function s.ctlcon(e,tp,eg,ep,ev,re,r,rp)
fn ctlcon(d: &mut Duel, a: &EffArgs) -> bool {
    // return rp==1-tp and re:IsMonsterEffect()
    a.rp == 1 - a.tp && effect::is_monster_effect(d, a.re.nil())
}
// function s.ctlfilter(c,tp)
fn ctlfilter(d: &mut Duel, c: CardId, tp: u8) -> bool {
    // return c:IsFaceup() and c:IsControlerCanBeChanged() and c:IsAttack(0) and Duel.GetMZoneCount(1-tp,c,tp)>0
    card::is_faceup(d, c)
        && card::is_controler_can_be_changed(d, c, false, None)
        && card::is_attack(d, c, &[0])
        && duel::get_m_zone_count(d, 1 - tp, c.into(), Some(tp), None, None).0 > 0
}
/// `Duel.IsPlayerCanSpecialSummonMonster(tp,TOKEN_REPTILIANNE,0,TYPES_TOKEN,0,0,1,RACE_REPTILE,ATTRIBUTE_EARTH,POS_FACEUP,1-tp)`
fn can_summon_token(d: &mut Duel, tp: u8) -> bool {
    duel::is_player_can_special_summon_monster(
        d,
        tp,
        TOKEN_REPTILIANNE,
        duel::MonsterData {
            setcodes: Some(vec![0]),
            type_: Some(TYPES_TOKEN),
            attack: Some(0),
            defense: Some(0),
            level: Some(1),
            race: Some(RACE_REPTILE),
            attribute: Some(ATTRIBUTE_EARTH),
        },
        Some(POS_FACEUP as u8),
        Some(1 - tp),
        0,
    )
}
// function s.ctltg(e,tp,eg,ep,ev,re,r,rp,chk,chkc)
fn ctltg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // if chkc then return chkc:IsLocation(LOCATION_MZONE) and chkc:IsControler(1-tp) and s.ctlfilter(chkc,tp) end
    if let Some(chkc) = a.chkc() {
        return card::is_location(d, chkc, LOCATION_MZONE) && card::is_controler(d, chkc, 1 - tp) && ctlfilter(d, chkc, tp);
    }
    // if chk==0 then return Duel.IsExistingTarget(s.ctlfilter,tp,0,LOCATION_MZONE,1,nil,tp)
    //     and Duel.IsPlayerCanSpecialSummonMonster(tp,TOKEN_REPTILIANNE,0,TYPES_TOKEN,0,0,1,RACE_REPTILE,ATTRIBUTE_EARTH,POS_FACEUP,1-tp)
    // end
    if a.chk == Some(0) {
        return duel::is_existing_target(d, Some(&|d, c| ctlfilter(d, c, tp)), tp, 0, LOCATION_MZONE, 1, NO_EX) && can_summon_token(d, tp);
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_CONTROL)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_CONTROL as u64);
    // local g=Duel.SelectTarget(tp,s.ctlfilter,tp,0,LOCATION_MZONE,1,1,nil,tp)
    let g = duel::select_target(d, tp, Some(&|d, c| ctlfilter(d, c, tp)), tp, 0, LOCATION_MZONE, 1, 1, false, NO_EX);
    // Duel.SetOperationInfo(0,CATEGORY_CONTROL,g,1,0,0)
    duel::set_operation_info(d, 0, CATEGORY_CONTROL, g, 1, 0, 0);
    // Duel.SetOperationInfo(0,CATEGORY_TOKEN,nil,1,0,0)
    duel::set_operation_info(d, 0, CATEGORY_TOKEN, None, 1, 0, 0);
    // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,nil,1,tp,0)
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, None, 1, tp, 0);
    true
}
// function s.ctlop(e,tp,eg,ep,ev,re,r,rp)
fn ctlop(d: &mut Duel, a: &EffArgs) {
    let (e, tp) = (a.e, a.tp);
    // local tc=Duel.GetFirstTarget()
    let tc = duel::first_target(d).nil();
    // if not tc:IsRelateToEffect(e) then return end
    if !card::is_relate_to_effect(d, tc, e) {
        return;
    }
    // if Duel.GetControl(tc,tp) and Duel.GetLocationCount(1-tp,LOCATION_MZONE)>0
    //     and Duel.IsPlayerCanSpecialSummonMonster(tp,TOKEN_REPTILIANNE,0,TYPES_TOKEN,0,0,1,RACE_REPTILE,ATTRIBUTE_EARTH,POS_FACEUP,1-tp) then
    if duel::get_control(d, tc, tp, 0, 0, None, None) != 0
        && duel::get_location_count(d, 1 - tp, LOCATION_MZONE as u8, None, None, None).0 > 0
        && can_summon_token(d, tp)
    {
        // Duel.BreakEffect()
        duel::break_effect(d);
        // local token=Duel.CreateToken(tp,TOKEN_REPTILIANNE)
        let token = duel::create_token(d, tp, TOKEN_REPTILIANNE).nil();
        // Duel.SpecialSummon(token,0,tp,1-tp,false,false,POS_FACEUP)
        duel::special_summon(d, token, 0, tp, 1 - tp, false, false, POS_FACEUP as u8, None);
    }
}
