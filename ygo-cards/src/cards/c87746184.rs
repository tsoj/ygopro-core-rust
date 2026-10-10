//! Albion the Branded Dragon
use super::prelude::*;

pub const ID: u32 = 87746184;
pub const INFO: ScriptInfo = ScriptInfo { listed_names: &[CARD_ALBAZ, 87746184], listed_series: &[SET_BRANDED as u32], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // c:EnableReviveLimit()
    card::enable_revive_limit(d, c);
    // Fusion.AddProcMix(c,true,true,CARD_ALBAZ,aux.FilterBoolFunctionEx(Card.IsAttribute,ATTRIBUTE_LIGHT))
    fusion::add_proc_mix(
        d,
        c,
        true,
        true,
        vec![
            fusion::Mat::Code(CARD_ALBAZ),
            fusion::Mat::F(fusion::fus_mat(|d, x, fc, st, tp| card::is_attribute_ctx(d, x, ATTRIBUTE_LIGHT, Some(fc), st, Some(tp)))),
        ],
    );
    // local params={fusfilter=s.fusfilter,matfilter=Card.IsAbleToRemove,extrafil=s.fextra,extraop=Fusion.BanishMaterial,extratg=s.extratg}
    let params = fusion::SummonEff {
        fusfilter: Some(fusion::fus_filter(|d, x, _tp| fusfilter(d, x))),
        // Card.IsAbleToRemove as a Fusion matfilter: the C function gets (c, e, tp, chk) by position
        matfilter: Some(std::sync::Arc::new(|d, x, _e, tp, chk| card::is_able_to_remove(d, x, None, Some(tp), Some(chk as u32)))),
        extrafil: Some(std::sync::Arc::new(fextra)),
        extraop: Some(fusion::banish_material()),
        extratg: Some(cost_target(extratg)),
        ..Default::default()
    };
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_SPECIAL_SUMMON+CATEGORY_FUSION_SUMMON)
    effect::set_category(d, e1, CATEGORY_SPECIAL_SUMMON + CATEGORY_FUSION_SUMMON);
    // e1:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_O);
    // e1:SetProperty(EFFECT_FLAG_DELAY)
    effect::set_property(d, e1, EFFECT_FLAG_DELAY, 0);
    // e1:SetCode(EVENT_SPSUMMON_SUCCESS)
    effect::set_code(d, e1, EVENT_SPSUMMON_SUCCESS);
    // e1:SetCountLimit(1,id)
    effect::set_count_limit(d, e1, 1, ID, 0, 0);
    // e1:SetCondition(function(e) return e:GetHandler():IsFusionSummoned() end)
    effect::set_condition(d, e1, Some(lua_fn(fusion_summoned_cond)));
    // e1:SetTarget(Fusion.SummonEffTG(params))
    let tg = fusion::summon_eff_tg(d, params.clone());
    effect::set_target(d, e1, Some(tg));
    // e1:SetOperation(Fusion.SummonEffOP(params))
    let op = fusion::summon_eff_op(d, params);
    effect::set_operation(d, e1, Some(op));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // Add to your hand or Set 1 "Branded" Spell/Trap from your Deck
    let e2 = effect::create_effect(d, c);
    effect::set_description(d, e2, aux::stringid(ID, 1));
    effect::set_category(d, e2, CATEGORY_TOHAND + CATEGORY_SEARCH + CATEGORY_SET);
    effect::set_type(d, e2, EFFECT_TYPE_FIELD + EFFECT_TYPE_TRIGGER_O);
    effect::set_code(d, e2, EVENT_PHASE + PHASE_END);
    effect::set_range(d, e2, LOCATION_GRAVE);
    effect::set_count_limit(d, e2, 1, ID, 1, 0);
    effect::set_condition(d, e2, Some(condition(thsetcon)));
    effect::set_target(d, e2, Some(cost_target(thsettg)));
    effect::set_operation(d, e2, Some(operation(thsetop)));
    card::register_effect(d, c, e2, false);
}
/// `s.fusfilter(c)`
fn fusfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsLevelBelow(8) and not c:IsCode(id)
    card::is_level_below(d, c, 8) && !card::is_code(d, c, &[ID])
}
/// `function(e) return e:GetHandler():IsFusionSummoned() end`
fn fusion_summoned_cond(d: &mut Duel, a: Args) -> bool {
    let h = effect::get_handler(d, a.e(0)).nil();
    card::is_fusion_summoned(d, h)
}
/// `s.fextra(e,tp,mg)`
fn fextra(d: &mut Duel, _e: EffectId, tp: u8, _mg: &CardSet) -> (Option<CardSet>, Option<LuaFn>) {
    // if not Duel.IsPlayerAffectedByEffect(tp,CARD_SPIRIT_ELIMINATION) then
    if duel::is_player_affected_by_effect(d, tp, CARD_SPIRIT_ELIMINATION).is_none() {
        // return Duel.GetMatchingGroup(Fusion.IsMonsterFilter(Card.IsAbleToRemove),tp,LOCATION_GRAVE,0,nil)
        let g = duel::get_matching_group(d, Some(&|d, c| card::is_monster(d, c) && card::is_able_to_remove(d, c, None, None, None)), tp, LOCATION_GRAVE, 0, NO_EX);
        return (Some(g), None);
    }
    // return nil
    (None, None)
}
/// `s.extratg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn extratg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return true end
    if a.chk == Some(0) {
        return true;
    }
    // Duel.SetOperationInfo(0,CATEGORY_REMOVE,nil,0,tp,LOCATION_HAND|LOCATION_ONFIELD|LOCATION_GRAVE)
    duel::set_operation_info(d, 0, CATEGORY_REMOVE, None, 0, a.tp, (LOCATION_HAND | LOCATION_ONFIELD | LOCATION_GRAVE) as i32);
    true
}
/// `s.thsetcon(e,tp,eg,ep,ev,re,r,rp)`
fn thsetcon(d: &mut Duel, a: &EffArgs) -> bool {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // return c:GetTurnID()==Duel.GetTurnCount() and not c:IsReason(REASON_RETURN)
    card::get_turn_id(d, c) as i32 == duel::get_turn_count(d, None) && !card::is_reason(d, c, REASON_RETURN)
}
/// `s.thsetfilter(c)`
fn thsetfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsSetCard(SET_BRANDED) and c:IsSpellTrap() and (c:IsAbleToHand() or c:IsSSetable())
    card::is_set_card(d, c, &[SET_BRANDED as u16]) && card::is_spell_trap(d, c) && (card::is_able_to_hand(d, c, None) || card::is_s_setable(d, c, false, None))
}
/// `s.thsettg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn thsettg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return Duel.IsExistingMatchingCard(s.thsetfilter,tp,LOCATION_DECK,0,1,nil) end
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, Some(&thsetfilter), a.tp, LOCATION_DECK, 0, 1, NO_EX);
    }
    // Duel.SetOperationInfo(0,CATEGORY_TOHAND,nil,1,tp,LOCATION_DECK)
    duel::set_operation_info(d, 0, CATEGORY_TOHAND, None, 1, a.tp, LOCATION_DECK as i32);
    true
}
/// `s.thsetop(e,tp,eg,ep,ev,re,r,rp)`
fn thsetop(d: &mut Duel, a: &EffArgs) {
    // Duel.Hint(HINT_SELECTMSG,tp,aux.Stringid(id,2))
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, aux::stringid(ID, 2));
    // local sc=Duel.SelectMatchingCard(tp,s.thsetfilter,tp,LOCATION_DECK,0,1,1,nil):GetFirst()
    let g = duel::select_matching_card(d, a.tp, Some(&thsetfilter), a.tp, LOCATION_DECK, 0, 1, 1, false, NO_EX).nil();
    let Some(sc) = group::get_first(d, &g) else { return };
    // aux.ToHandOrElse(sc,tp,function(sc) return sc:IsSSetable() end,function(sc) Duel.SSet(tp,sc) end,aux.Stringid(id,3))
    aux::to_hand_or_else(
        d,
        sc,
        a.tp,
        Some(&|d, c| card::is_s_setable(d, c, false, None)),
        Some(&|d, g| {
            duel::s_set(d, a.tp, g.clone(), None, true);
            0
        }),
        Some(aux::stringid(ID, 3)),
    );
}
