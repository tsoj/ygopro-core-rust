//! Elemental HERO Clay Guardian
use super::prelude::*;

pub const ID: u32 = 8593259;

pub const INFO: ScriptInfo = ScriptInfo {
    listed_series: &[SET_HERO, SET_ELEMENTAL_HERO],
    material_setcode: &[SET_HERO, SET_ELEMENTAL_HERO],
    ..ScriptInfo::DEFAULT
};

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // c:EnableReviveLimit()
    card::enable_revive_limit(d, c);
    // Fusion Materials: 1 "Elemental HERO" monster + 1 Warrior monster
    // Fusion.AddProcMix(c,true,true,aux.FilterBoolFunctionEx(Card.IsSetCard,SET_ELEMENTAL_HERO),aux.FilterBoolFunctionEx(Card.IsRace,RACE_WARRIOR))
    fusion::add_proc_mix(
        d,
        c,
        true,
        true,
        vec![
            fusion::Mat::F(fusion::fus_mat(|d, c, fc, st, tp| card::is_set_card_ctx(d, c, &[SET_ELEMENTAL_HERO as u16], Some(fc), st, Some(tp)))),
            fusion::Mat::F(fusion::fus_mat(|d, c, fc, st, tp| card::is_race_ctx(d, c, RACE_WARRIOR as u64, Some(fc), st, Some(tp)))),
        ],
    );
    // If this card is Special Summoned: You can Special Summon 1 "Elemental HERO" monster from your Deck, ...
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_SPECIAL_SUMMON+CATEGORY_DAMAGE)
    effect::set_category(d, e1, CATEGORY_SPECIAL_SUMMON + CATEGORY_DAMAGE);
    // e1:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_O);
    // e1:SetProperty(EFFECT_FLAG_DELAY)
    effect::set_property(d, e1, EFFECT_FLAG_DELAY, 0);
    // e1:SetCode(EVENT_SPSUMMON_SUCCESS)
    effect::set_code(d, e1, EVENT_SPSUMMON_SUCCESS);
    // e1:SetCountLimit(1,id)
    effect::set_count_limit(d, e1, 1, ID, 0, 0);
    // e1:SetTarget(s.sptg)
    effect::set_target(d, e1, Some(cost_target(sptg)));
    // e1:SetOperation(s.spop)
    effect::set_operation(d, e1, Some(operation(spop)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // Other "Elemental HERO" monsters you control cannot be destroyed by battle or card effects
    // local e2a=Effect.CreateEffect(c)
    let e2a = effect::create_effect(d, c);
    // e2a:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e2a, EFFECT_TYPE_FIELD);
    // e2a:SetCode(EFFECT_INDESTRUCTABLE_BATTLE)
    effect::set_code(d, e2a, EFFECT_INDESTRUCTABLE_BATTLE);
    // e2a:SetRange(LOCATION_MZONE)
    effect::set_range(d, e2a, LOCATION_MZONE);
    // e2a:SetTargetRange(LOCATION_MZONE,0)
    effect::set_target_range(d, e2a, LOCATION_MZONE, 0);
    // e2a:SetTarget(function(e,c) return c:IsSetCard(SET_ELEMENTAL_HERO) and c~=e:GetHandler() end)
    effect::set_target(d, e2a, Some(effect_card_fn(|d, e, cc| {
        card::is_set_card(d, cc, &[SET_ELEMENTAL_HERO as u16]) && cc != effect::get_handler(d, e).nil()
    })));
    // e2a:SetValue(1)
    effect::set_value(d, e2a, 1);
    // c:RegisterEffect(e2a)
    card::register_effect(d, c, e2a, false);
    // local e2b=e2a:Clone()
    let e2b = effect::clone(d, e2a);
    // e2b:SetCode(EFFECT_INDESTRUCTABLE_EFFECT)
    effect::set_code(d, e2b, EFFECT_INDESTRUCTABLE_EFFECT);
    // c:RegisterEffect(e2b)
    card::register_effect(d, c, e2b, false);
}
/// `s.spfilter(c,e,tp)`
fn spfilter(d: &mut Duel, c: CardId, e: EffectId, tp: u8) -> bool {
    // return c:IsSetCard(SET_ELEMENTAL_HERO) and c:IsCanBeSpecialSummoned(e,0,tp,false,false)
    card::is_set_card(d, c, &[SET_ELEMENTAL_HERO as u16]) && card::is_can_be_special_summoned(d, c, e, 0, tp, false, false, None, None, None)
}
/// `s.sptg(e,tp,...,chk)`
fn sptg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    let e = a.e;
    // if chk==0 then return Duel.GetLocationCount(tp,LOCATION_MZONE)>0
    //     and Duel.IsExistingMatchingCard(s.spfilter,tp,LOCATION_DECK,0,1,nil,e,tp) end
    if a.chk == Some(0) {
        return duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 > 0
            && duel::is_existing_matching_card(d, Some(&|d, c| spfilter(d, c, e, tp)), tp, LOCATION_DECK, 0, 1, NO_EX);
    }
    // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,nil,1,tp,LOCATION_DECK)
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, None, 1, tp, LOCATION_DECK as i32);
    // Duel.SetPossibleOperationInfo(0,CATEGORY_DAMAGE,nil,0,1-tp,400*Duel.GetFieldGroupCount(tp,0,LOCATION_ONFIELD))
    let n = duel::get_field_group_count(d, tp, 0, LOCATION_ONFIELD);
    duel::set_possible_operation_info(d, 0, CATEGORY_DAMAGE, None, 0, 1 - tp, 400 * n);
    true
}
/// `s.spop(e,tp,...)`
fn spop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    let e = a.e;
    // if Duel.GetLocationCount(tp,LOCATION_MZONE)>0 then
    if duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 > 0 {
        // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_SPSUMMON)
        duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_SPSUMMON as u64);
        // local g=Duel.SelectMatchingCard(tp,s.spfilter,tp,LOCATION_DECK,0,1,1,nil,e,tp)
        let g = duel::select_matching_card(d, tp, Some(&|d, c| spfilter(d, c, e, tp)), tp, LOCATION_DECK, 0, 1, 1, false, NO_EX).nil();
        // if #g>0 and Duel.SpecialSummon(g,0,tp,tp,false,false,POS_FACEUP)>0 and Duel.GetFieldGroupCount(tp,0,LOCATION_ONFIELD)>0
        //     and Duel.SelectYesNo(tp,aux.Stringid(id,1)) then
        if !g.is_empty()
            && duel::special_summon(d, g.clone(), 0, tp, tp, false, false, POS_FACEUP as u8, None) > 0
            && duel::get_field_group_count(d, tp, 0, LOCATION_ONFIELD) > 0
            && duel::select_yes_no(d, tp, aux::stringid(ID, 1))
        {
            // local dam=400*Duel.GetFieldGroupCount(tp,0,LOCATION_ONFIELD)
            let dam = 400 * duel::get_field_group_count(d, tp, 0, LOCATION_ONFIELD);
            // Duel.BreakEffect()
            duel::break_effect(d);
            // Duel.Damage(1-tp,dam,REASON_EFFECT)
            duel::damage(d, 1 - tp, dam as i64, REASON_EFFECT, false, None);
        }
    }
    // You cannot Special Summon from the Extra Deck for the rest of this turn, except "HERO" monsters
    // local e1=Effect.CreateEffect(e:GetHandler())
    let h = effect::get_handler(d, e).nil();
    let e1 = effect::create_effect(d, h);
    // e1:SetDescription(aux.Stringid(id,2))
    effect::set_description(d, e1, aux::stringid(ID, 2));
    // e1:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e1, EFFECT_TYPE_FIELD);
    // e1:SetProperty(EFFECT_FLAG_PLAYER_TARGET+EFFECT_FLAG_CLIENT_HINT)
    effect::set_property(d, e1, EFFECT_FLAG_PLAYER_TARGET + EFFECT_FLAG_CLIENT_HINT, 0);
    // e1:SetCode(EFFECT_CANNOT_SPECIAL_SUMMON)
    effect::set_code(d, e1, EFFECT_CANNOT_SPECIAL_SUMMON);
    // e1:SetTargetRange(1,0)
    effect::set_target_range(d, e1, 1, 0);
    // e1:SetTarget(function(e,c) return c:IsLocation(LOCATION_EXTRA) and not c:IsSetCard(SET_HERO) end)
    effect::set_target(d, e1, Some(effect_card_fn(|d, _e, cc| {
        card::is_location(d, cc, LOCATION_EXTRA) && !card::is_set_card(d, cc, &[SET_HERO as u16])
    })));
    // e1:SetReset(RESET_PHASE|PHASE_END)
    effect::set_reset(d, e1, RESET_PHASE | PHASE_END, 1);
    // Duel.RegisterEffect(e1,tp)
    duel::register_effect(d, e1, tp);
}
