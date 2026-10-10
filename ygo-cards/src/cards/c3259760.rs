//! Spellbound
use super::prelude::*;

pub const ID: u32 = 3259760;

/// `local sumtypes={SUMMON_TYPE_FUSION,SUMMON_TYPE_SYNCHRO,SUMMON_TYPE_XYZ,SUMMON_TYPE_LINK}`
const SUMTYPES: [u32; 4] = [SUMMON_TYPE_FUSION, SUMMON_TYPE_SYNCHRO, SUMMON_TYPE_XYZ, SUMMON_TYPE_LINK];

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Prevent the opponent's monster from being Tributed or used as material
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    // e1:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    // e1:SetHintTiming(0,TIMINGS_CHECK_MONSTER)
    effect::set_hint_timing(d, e1, 0, Some(TIMINGS_CHECK_MONSTER));
    // e1:SetTarget(s.target)
    effect::set_target(d, e1, Some(cost_target(target)));
    // e1:SetOperation(s.activate)
    effect::set_operation(d, e1, Some(operation(activate)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
/// `s.filter(c)`
fn filter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsFaceup() and c:GetFlagEffect(id)==0 and s.CanBeTributeOrMaterial(c)
    card::is_faceup(d, c) && card::get_flag_effect(d, c, ID) == 0 && can_be_tribute_or_material(d, c)
}
/// `s.target(e,tp,eg,ep,ev,re,r,rp,chk)`
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return Duel.IsExistingMatchingCard(s.filter,tp,0,LOCATION_MZONE,1,nil) end
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, Some(&filter), a.tp, 0, LOCATION_MZONE, 1, NO_EX);
    }
    true
}
/// `s.activate(e,tp,eg,ep,ev,re,r,rp)`
fn activate(d: &mut Duel, a: &EffArgs) {
    // local sg=Duel.GetMatchingGroup(s.filter,tp,0,LOCATION_MZONE,nil)
    let sg = duel::get_matching_group(d, Some(&filter), a.tp, 0, LOCATION_MZONE, NO_EX);
    // if #sg==0 then return end
    if sg.is_empty() {
        return;
    }
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // for tc in sg:Iter() do
    for tc in group::iter(d, &sg) {
        // Affected monster cannot be tributed
        // local e1=Effect.CreateEffect(c)
        let e1 = effect::create_effect(d, c);
        // e1:SetType(EFFECT_TYPE_SINGLE)
        effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
        // e1:SetCode(EFFECT_UNRELEASABLE_SUM)
        effect::set_code(d, e1, EFFECT_UNRELEASABLE_SUM);
        // e1:SetReset(RESETS_STANDARD_PHASE_END)
        effect::set_reset(d, e1, RESETS_STANDARD_PHASE_END, 1);
        // e1:SetValue(1)
        effect::set_value(d, e1, 1);
        // tc:RegisterEffect(e1)
        card::register_effect(d, tc, e1, false);
        // local e2=e1:Clone()
        let e2 = effect::clone(d, e1);
        // e2:SetCode(EFFECT_UNRELEASABLE_NONSUM)
        effect::set_code(d, e2, EFFECT_UNRELEASABLE_NONSUM);
        // tc:RegisterEffect(e2)
        card::register_effect(d, tc, e2, false);
        // Affected monster cannot be use as material
        // local e3=Effect.CreateEffect(c)
        let e3 = effect::create_effect(d, c);
        // e3:SetDescription(aux.Stringid(id,1))
        effect::set_description(d, e3, aux::stringid(ID, 1));
        // e3:SetType(EFFECT_TYPE_SINGLE)
        effect::set_type(d, e3, EFFECT_TYPE_SINGLE);
        // e3:SetProperty(EFFECT_FLAG_CLIENT_HINT)
        effect::set_property(d, e3, EFFECT_FLAG_CLIENT_HINT, 0);
        // e3:SetCode(EFFECT_CANNOT_BE_MATERIAL)
        effect::set_code(d, e3, EFFECT_CANNOT_BE_MATERIAL);
        // e3:SetReset(RESETS_STANDARD_PHASE_END)
        effect::set_reset(d, e3, RESETS_STANDARD_PHASE_END, 1);
        // e3:SetValue(aux.cannotmatfilter(SUMMON_TYPE_FUSION,SUMMON_TYPE_SYNCHRO,SUMMON_TYPE_XYZ,SUMMON_TYPE_LINK))
        effect::set_value_fn(d, e3, aux::cannotmatfilter(&SUMTYPES));
        // tc:RegisterEffect(e3)
        card::register_effect(d, tc, e3, false);
        // tc:RegisterFlagEffect(id,RESETS_STANDARD_PHASE_END,0,1)
        card::register_flag_effect(d, tc, ID, RESETS_STANDARD_PHASE_END, 0, 1, 0, 0);
    }
}
/// `s.CanBeTributeOrMaterial(c)`
fn can_be_tribute_or_material(d: &mut Duel, c: CardId) -> bool {
    // for _,sumtyp in ipairs(sumtypes) do if c:IsCanBeMaterial(sumtyp) then return true end end
    for sumtyp in SUMTYPES {
        if card::is_can_be_material(d, c, sumtyp as u64, None, None) {
            return true;
        }
    }
    // if not (c:IsHasEffect(EFFECT_UNRELEASABLE_SUM) or c:IsHasEffect(EFFECT_UNRELEASABLE_NONSUM)) then return true end
    let has_sum = !card::is_has_effect(d, c, EFFECT_UNRELEASABLE_SUM, None).is_empty();
    let has_nonsum = !card::is_has_effect(d, c, EFFECT_UNRELEASABLE_NONSUM, None).is_empty();
    if !(has_sum || has_nonsum) {
        return true;
    }
    // return false
    false
}
