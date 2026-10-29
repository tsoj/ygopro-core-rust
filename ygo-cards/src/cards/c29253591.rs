//! Dragunity Couse
//! scripted by Naim
use super::prelude::*;

pub const ID: u32 = 29253591;

pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_DRAGUNITY], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // synchro limit
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
    // e1:SetCode(EFFECT_CANNOT_BE_SYNCHRO_MATERIAL)
    effect::set_code(d, e1, EFFECT_CANNOT_BE_SYNCHRO_MATERIAL);
    // e1:SetProperty(EFFECT_FLAG_CANNOT_DISABLE+EFFECT_FLAG_UNCOPYABLE)
    effect::set_property(d, e1, EFFECT_FLAG_CANNOT_DISABLE + EFFECT_FLAG_UNCOPYABLE, 0);
    // e1:SetValue(s.synlimit)
    effect::set_value_fn(d, e1, lua_fn(synlimit));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // Can be treated as level 4 for a Synchro Summon
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e2, EFFECT_TYPE_SINGLE);
    // e2:SetRange(LOCATION_MZONE)
    effect::set_range(d, e2, LOCATION_MZONE);
    // e2:SetCode(EFFECT_SYNCHRO_LEVEL)
    effect::set_code(d, e2, EFFECT_SYNCHRO_LEVEL);
    // e2:SetValue(s.slevel)
    effect::set_value_fn(d, e2, effect_value(slevel));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
    // Special Summon itself while it is equipped to a monster
    // local e3=Effect.CreateEffect(c)
    let e3 = effect::create_effect(d, c);
    // e3:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e3, aux::stringid(ID, 0));
    // e3:SetCategory(CATEGORY_SPECIAL_SUMMON)
    effect::set_category(d, e3, CATEGORY_SPECIAL_SUMMON);
    // e3:SetType(EFFECT_TYPE_IGNITION)
    effect::set_type(d, e3, EFFECT_TYPE_IGNITION);
    // e3:SetRange(LOCATION_SZONE)
    effect::set_range(d, e3, LOCATION_SZONE);
    // e3:SetCondition(function(e) return e:GetHandler():GetEquipTarget() end)
    effect::set_condition(d, e3, Some(lua_fn(|d, a: Args| {
        let h = effect::get_handler(d, a.e(0)).nil();
        card::get_equip_target(d, h).is_some()
    })));
    // e3:SetTarget(s.sptg)
    effect::set_target(d, e3, Some(cost_target(sptg)));
    // e3:SetOperation(s.spop)
    effect::set_operation(d, e3, Some(operation(spop)));
    // c:RegisterEffect(e3)
    card::register_effect(d, c, e3, false);
}
/// `s.synlimit(e,c)`
fn synlimit(d: &mut Duel, a: Args) -> bool {
    // if not c then return false end
    let Some(c) = a.card(1) else {
        return false;
    };
    // return not c:IsSetCard(SET_DRAGUNITY)
    !card::is_set_card(d, c, &[SET_DRAGUNITY as u16])
}
/// `s.slevel(e,c)`
fn slevel(d: &mut Duel, e: EffectId) -> i64 {
    // return 4<<16|e:GetHandler():GetLevel()
    let h = effect::get_handler(d, e).nil();
    (4i64 << 16) | card::get_level(d, h) as i64
}
/// `s.sptg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn sptg(d: &mut Duel, a: &EffArgs) -> bool {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if chk==0 then return Duel.GetLocationCount(tp,LOCATION_MZONE)>0 and c:IsCanBeSpecialSummoned(e,0,tp,false,false) end
    if a.chk == Some(0) {
        return duel::get_location_count(d, a.tp, LOCATION_MZONE as u8, None, None, None).0 > 0
            && card::is_can_be_special_summoned(d, c, a.e, 0, a.tp, false, false, None, None, None);
    }
    // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,c,1,tp,0)
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, Some(CardSet::from_card(c)), 1, a.tp, 0);
    true
}
/// `s.spop(e,tp,eg,ep,ev,re,r,rp)`
fn spop(d: &mut Duel, a: &EffArgs) {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if not c:IsRelateToEffect(e) then return end
    if !card::is_relate_to_effect(d, c, a.e) {
        return;
    }
    // Duel.SpecialSummon(c,0,tp,tp,false,false,POS_FACEUP)
    duel::special_summon(d, c, 0, a.tp, a.tp, false, false, POS_FACEUP as u8, None);
}
