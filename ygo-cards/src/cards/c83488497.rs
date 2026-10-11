//! Scareclaw Astra
//! Scripted by Hatter
use super::prelude::*;

pub const ID: u32 = 83488497;

pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_SCARECLAW], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Special Summon procedure
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e1, EFFECT_TYPE_FIELD);
    // e1:SetCode(EFFECT_SPSUMMON_PROC)
    effect::set_code(d, e1, EFFECT_SPSUMMON_PROC);
    // e1:SetProperty(EFFECT_FLAG_UNCOPYABLE)
    effect::set_property(d, e1, EFFECT_FLAG_UNCOPYABLE, 0);
    // e1:SetRange(LOCATION_HAND)
    effect::set_range(d, e1, LOCATION_HAND);
    // e1:SetCountLimit(1,id,EFFECT_COUNT_CODE_OATH)
    effect::set_count_limit(d, e1, 1, ID, 0, EFFECT_COUNT_CODE_OATH as u8);
    // e1:SetValue(s.hspval)
    effect::set_value_fn(d, e1, lua_fn(hspval));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // Multiple attack
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e2, EFFECT_TYPE_FIELD);
    // e2:SetCode(EFFECT_EXTRA_ATTACK)
    effect::set_code(d, e2, EFFECT_EXTRA_ATTACK);
    // e2:SetRange(LOCATION_MZONE)
    effect::set_range(d, e2, LOCATION_MZONE);
    // e2:SetTargetRange(LOCATION_EMZONE,0)
    effect::set_target_range(d, e2, LOCATION_EMZONE, 0);
    // e2:SetCondition(s.atkcon)
    effect::set_condition(d, e2, Some(condition(atkcon)));
    // e2:SetTarget(aux.TargetBoolFunction(s.sclawfilter))
    effect::set_target(d, e2, Some(effect_card_fn(|d, _e, c| sclawfilter(d, c))));
    // e2:SetValue(s.atkval)
    effect::set_value_fn(d, e2, lua_fn(atkval));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
}
fn sclawfilter(d: &mut Duel, c: CardId) -> bool {
    card::is_faceup(d, c) && card::is_set_card(d, c, &[SET_SCARECLAW as u16])
}
fn hspval(d: &mut Duel, a: Args) -> Value {
    // local zone=0
    let mut zone: u32 = 0;
    // local tp=c:GetControler()
    let c = a.c(1);
    let tp = card::get_controler(d, c);
    // local lg=Duel.GetMatchingGroup(s.sclawfilter,tp,LOCATION_MZONE,0,nil)
    let lg = duel::get_matching_group(d, Some(&sclawfilter), tp, LOCATION_MZONE, 0, NO_EX);
    // for tc in lg:Iter() do
    for tc in group::iter(d, &lg) {
        // left_right=tc:IsInMainMZone() and 1 or 0
        let left_right: u8 = if card::is_in_main_m_zone(d, tc, None) { 1 } else { 0 };
        // zone=(zone|tc:GetColumnZone(LOCATION_MZONE,left_right,left_right,tp))
        zone |= card::get_column_zone(d, tc, LOCATION_MZONE, left_right, left_right, Some(tp));
    }
    // return 0,zone&ZONES_MMZ
    Value::Multi(vec![Value::Int(0), Value::Int((zone & ZONES_MMZ) as i64)])
}
fn atkcon(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = effect::get_handler_player(d, a.e);
    duel::is_existing_matching_card(d, Some(&|d, c| sclawfilter(d, c) && card::is_defense_pos(d, c)), tp, LOCATION_MZONE, 0, 1, NO_EX)
}
fn atkval(d: &mut Duel, a: Args) -> i64 {
    let tp = effect::get_handler_player(d, a.e(0));
    let g = duel::get_matching_group(d, Some(&|d, c| sclawfilter(d, c) && card::is_defense_pos(d, c)), tp, LOCATION_MZONE, 0, NO_EX);
    // return g:GetClassCount(Card.GetCode)-1
    group::get_class_count(d, &g, &|d, c| card::get_code(d, c)[0] as i64) as i64 - 1
}
