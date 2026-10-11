//! Axon Kicker Oracle
use super::prelude::*;

pub const ID: u32 = 33171768;

pub const INFO: ScriptInfo = ScriptInfo {
    // s.listed_series={SET_TELEPORT}
    listed_series: &[SET_TELEPORT as u32],
    // s.miracle_synchro_fusion=true
    miracle_synchro_fusion: true,
    ..ScriptInfo::DEFAULT
};

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // c:EnableReviveLimit()
    card::enable_revive_limit(d, c);
    // Fusion Materials: 1 Psychic monster + 1 Synchro, Xyz, or Link Monster
    // Fusion.AddProcMix(c,true,true,aux.FilterBoolFunctionEx(Card.IsRace,RACE_PSYCHIC),aux.FilterBoolFunctionEx(Card.IsType,TYPE_SYNCHRO|TYPE_XYZ|TYPE_LINK))
    fusion::add_proc_mix(
        d,
        c,
        true,
        true,
        vec![
            fusion::Mat::F(fusion::fus_mat(|d, c, fc, st, tp| card::is_race_ctx(d, c, RACE_PSYCHIC, Some(fc), st, Some(tp)))),
            fusion::Mat::F(fusion::fus_mat(|d, c, fc, st, tp| {
                card::is_type_ctx(d, c, TYPE_SYNCHRO | TYPE_XYZ | TYPE_LINK, Some(fc), st, Some(tp))
            })),
        ],
    );
    // After this card is Fusion Summoned, it gains 1000 ATK for each Synchro, Xyz, and Link Monster used as it material until the end of the next turn
    // local e0=Effect.CreateEffect(c)
    let e0 = effect::create_effect(d, c);
    // e0:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e0, EFFECT_TYPE_SINGLE);
    // e0:SetCode(EFFECT_MATERIAL_CHECK)
    effect::set_code(d, e0, EFFECT_MATERIAL_CHECK);
    // e0:SetValue(function(e,c) e:SetLabel(c:GetMaterial():FilterCount(Card.IsType,nil,TYPE_SYNCHRO|TYPE_XYZ|TYPE_LINK)) end)
    effect::set_value_fn(d, e0, effect_card_value(material_check));
    // c:RegisterEffect(e0)
    card::register_effect(d, c, e0, false);
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_CONTINUOUS)
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE + EFFECT_TYPE_CONTINUOUS);
    // e1:SetCode(EVENT_SPSUMMON_SUCCESS)
    effect::set_code(d, e1, EVENT_SPSUMMON_SUCCESS);
    // e1:SetCondition(function(e) return e:GetHandler():IsFusionSummoned() end)
    effect::set_condition(d, e1, Some(condition(fusion_summoned_cond)));
    // e1:SetOperation(function(e) e:GetHandler():UpdateAttack(e0:GetLabel()*1000,RESETS_STANDARD_DISABLE_PHASE_END,nil,2) end)
    effect::set_operation(
        d,
        e1,
        Some(operation(move |d, a| {
            let h = effect::get_handler(d, a.e).nil();
            let lv = effect::get_label(d, e0);
            card::update_attack(d, h, lv * 1000, Some(RESETS_STANDARD_DISABLE_PHASE_END), None, Some(2));
        })),
    );
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // Negate an opponent's Special Summon and banish that monster(s)
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e2, aux::stringid(ID, 0));
    // e2:SetCategory(CATEGORY_DISABLE_SUMMON+CATEGORY_REMOVE)
    effect::set_category(d, e2, CATEGORY_DISABLE_SUMMON + CATEGORY_REMOVE);
    // e2:SetType(EFFECT_TYPE_QUICK_O)
    effect::set_type(d, e2, EFFECT_TYPE_QUICK_O);
    // e2:SetCode(EVENT_SPSUMMON)
    effect::set_code(d, e2, EVENT_SPSUMMON);
    // e2:SetRange(LOCATION_MZONE)
    effect::set_range(d, e2, LOCATION_MZONE);
    // e2:SetCountLimit(1,id)
    effect::set_count_limit(d, e2, 1, ID, 0, 0);
    // e2:SetCondition(function(e,tp,eg,ep) return ep==1-tp and Duel.GetCurrentChain()==0 end)
    effect::set_condition(d, e2, Some(condition(negsum_cond)));
    // e2:SetTarget(s.negsumtg)
    effect::set_target(d, e2, Some(cost_target(negsumtg)));
    // e2:SetOperation(s.negsumop)
    effect::set_operation(d, e2, Some(operation(negsumop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
    // Special Summon this card
    // local e3=Effect.CreateEffect(c)
    let e3 = effect::create_effect(d, c);
    // e3:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e3, aux::stringid(ID, 1));
    // e3:SetCategory(CATEGORY_SPECIAL_SUMMON)
    effect::set_category(d, e3, CATEGORY_SPECIAL_SUMMON);
    // e3:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e3, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_O);
    // e3:SetProperty(EFFECT_FLAG_DELAY)
    effect::set_property(d, e3, EFFECT_FLAG_DELAY, 0);
    // e3:SetCode(EVENT_TO_GRAVE)
    effect::set_code(d, e3, EVENT_TO_GRAVE);
    // e3:SetCountLimit(1,{id,1})
    effect::set_count_limit(d, e3, 1, ID, 1, 0);
    // e3:SetCondition(function(e) local c=e:GetHandler() return c:IsFusionSummoned() and c:IsPreviousLocation(LOCATION_MZONE) end)
    effect::set_condition(d, e3, Some(condition(revive_cond)));
    // e3:SetCost(s.spcost)
    effect::set_cost(d, e3, Some(cost_target(spcost)));
    // e3:SetTarget(s.sptg)
    effect::set_target(d, e3, Some(cost_target(sptg)));
    // e3:SetOperation(s.spop)
    effect::set_operation(d, e3, Some(operation(spop)));
    // c:RegisterEffect(e3)
    card::register_effect(d, c, e3, false);
}
// function(e,c) e:SetLabel(c:GetMaterial():FilterCount(...)) end (a function without a return value is 0)
fn material_check(d: &mut Duel, e: EffectId, c: CardId) -> i64 {
    let mat = card::get_material(d, c);
    let n = group::filter_count(d, &mat, &|d, x| card::is_type(d, x, TYPE_SYNCHRO | TYPE_XYZ | TYPE_LINK), NO_EX);
    effect::set_label(d, e, &[n as i64]);
    0
}
fn fusion_summoned_cond(d: &mut Duel, a: &EffArgs) -> bool {
    // function(e) return e:GetHandler():IsFusionSummoned() end
    let c = effect::get_handler(d, a.e).nil();
    card::is_fusion_summoned(d, c)
}
fn negsum_cond(d: &mut Duel, a: &EffArgs) -> bool {
    // function(e,tp,eg,ep) return ep==1-tp and Duel.GetCurrentChain()==0 end
    a.ep == 1 - a.tp && duel::get_current_chain(d, false) == 0
}
fn revive_cond(d: &mut Duel, a: &EffArgs) -> bool {
    // local c=e:GetHandler() return c:IsFusionSummoned() and c:IsPreviousLocation(LOCATION_MZONE)
    let c = effect::get_handler(d, a.e).nil();
    card::is_fusion_summoned(d, c) && card::is_previous_location(d, c, LOCATION_MZONE)
}
fn negsumtg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return eg:IsExists(Card.IsAbleToRemove,1,nil) end
    let eg = a.eg.clone().nil();
    if a.chk == Some(0) {
        return group::is_exists(d, &eg, &|d, c| card::is_able_to_remove(d, c, None, None, None), 1, NO_EX);
    }
    // Duel.SetOperationInfo(0,CATEGORY_DISABLE_SUMMON,eg,#eg,tp,0)
    let n = eg.len() as u8;
    duel::set_operation_info(d, 0, CATEGORY_DISABLE_SUMMON, Some(eg.clone()), n, a.tp, 0);
    // Duel.SetOperationInfo(0,CATEGORY_REMOVE,eg,#eg,tp,0)
    duel::set_operation_info(d, 0, CATEGORY_REMOVE, Some(eg), n, a.tp, 0);
    true
}
fn negsumop(d: &mut Duel, a: &EffArgs) {
    // Duel.NegateSummon(eg)
    let eg = a.eg();
    duel::negate_summon(d, eg.clone());
    // Duel.Remove(eg,POS_FACEUP,REASON_EFFECT)
    duel::remove(d, eg, POS_FACEUP as u8, REASON_EFFECT, None, None);
}
fn spcostfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsSetCard(SET_TELEPORT) and (c:IsNormalSpell() or c:IsQuickPlaySpell()) and c:IsAbleToRemoveAsCost()
    card::is_set_card(d, c, &[SET_TELEPORT as u16])
        && (card::is_normal_spell(d, c) || card::is_quick_play_spell(d, c))
        && card::is_able_to_remove_as_cost(d, c, None)
}
fn spcost(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return Duel.IsExistingMatchingCard(s.spcostfilter,tp,LOCATION_GRAVE,0,1,nil) end
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, Some(&spcostfilter), a.tp, LOCATION_GRAVE, 0, 1, NO_EX);
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_REMOVE)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_REMOVE as u64);
    // local g=Duel.SelectMatchingCard(tp,s.spcostfilter,tp,LOCATION_GRAVE,0,1,1,nil)
    let g = duel::select_matching_card(d, a.tp, Some(&spcostfilter), a.tp, LOCATION_GRAVE, 0, 1, 1, false, NO_EX).nil();
    // Duel.Remove(g,POS_FACEUP,REASON_COST)
    duel::remove(d, g, POS_FACEUP as u8, REASON_COST, None, None);
    true
}
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
fn spop(d: &mut Duel, a: &EffArgs) {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if c:IsRelateToEffect(e) then
    if card::is_relate_to_effect(d, c, a.e) {
        // Duel.SpecialSummon(c,0,tp,tp,false,false,POS_FACEUP)
        duel::special_summon(d, c, 0, a.tp, a.tp, false, false, POS_FACEUP as u8, None);
    }
}
