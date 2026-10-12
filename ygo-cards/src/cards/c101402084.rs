//! Kumenyo, the Nine-Hued Deer
//! JP Name
use super::prelude::*;

pub const ID: u32 = 101402084;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // c:EnableReviveLimit()
    card::enable_revive_limit(d, c);
    // Xyz.AddProcedure(c,aux.FilterBoolFunctionEx(Card.IsAttribute,ATTRIBUTE_LIGHT|ATTRIBUTE_DARK),4,2,nil,nil,Xyz.InfiniteMats)
    xyz::add_procedure(
        d,
        c,
        Some(mat_filter(|d, c, sc, st, tp| card::is_attribute_ctx(d, c, ATTRIBUTE_LIGHT | ATTRIBUTE_DARK, sc, st, Some(tp)))),
        Some(4),
        2,
        None,
        None,
        Some(xyz::INFINITE_MATS),
        None,
        false,
        None,
    );
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_LEAVE_GRAVE)
    effect::set_category(d, e1, CATEGORY_LEAVE_GRAVE);
    // e1:SetType(EFFECT_TYPE_QUICK_O)
    effect::set_type(d, e1, EFFECT_TYPE_QUICK_O);
    // e1:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    // e1:SetRange(LOCATION_MZONE)
    effect::set_range(d, e1, LOCATION_MZONE);
    // e1:SetCountLimit(1,id)
    effect::set_count_limit(d, e1, 1, ID, 0, 0);
    // e1:SetCondition(function(e,tp,eg,ep,ev,re,r,rp) return Duel.IsMainPhase() end)
    effect::set_condition(d, e1, Some(condition(|d, _a| duel::is_main_phase(d, None))));
    // e1:SetCost(Cost.DetachFromSelf(1,1,function(e,og) og:GetFirst():CreateEffectRelation(e) end))
    effect::set_cost(
        d,
        e1,
        Some(cost::detach_from_self(
            cost::Count::N(1),
            Some(cost::Count::N(1)),
            Some(std::sync::Arc::new(|d: &mut Duel, e: EffectId, og: &CardSet| {
                let tc = group::get_first(d, og).nil();
                card::create_effect_relation(d, tc, e);
            })),
        )),
    );
    // e1:SetTarget(s.attachtg)
    effect::set_target(d, e1, Some(cost_target(attachtg)));
    // e1:SetOperation(s.attachop)
    effect::set_operation(d, e1, Some(operation(attachop)));
    // e1:SetHintTiming(0,TIMING_MAIN_END|TIMINGS_CHECK_MONSTER_E)
    effect::set_hint_timing(d, e1, 0, Some(TIMING_MAIN_END | TIMINGS_CHECK_MONSTER_E));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e2, EFFECT_TYPE_SINGLE);
    // e2:SetProperty(EFFECT_FLAG_SINGLE_RANGE)
    effect::set_property(d, e2, EFFECT_FLAG_SINGLE_RANGE, 0);
    // e2:SetCode(EFFECT_INDESTRUCTABLE_EFFECT)
    effect::set_code(d, e2, EFFECT_INDESTRUCTABLE_EFFECT);
    // e2:SetRange(LOCATION_MZONE)
    effect::set_range(d, e2, LOCATION_MZONE);
    // e2:SetCondition(function(e) return e:GetHandler():GetOverlayGroup():GetBinClassCount(Card.GetRace)>=2 end)
    effect::set_condition(d, e2, Some(lua_fn(|d, a: Args| bin_race_count(d, a.e(0)) >= 2)));
    // e2:SetValue(1)
    effect::set_value(d, e2, 1);
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
    // local e3=Effect.CreateEffect(c)
    let e3 = effect::create_effect(d, c);
    // e3:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e3, EFFECT_TYPE_FIELD);
    // e3:SetProperty(EFFECT_FLAG_PLAYER_TARGET)
    effect::set_property(d, e3, EFFECT_FLAG_PLAYER_TARGET, 0);
    // e3:SetCode(EFFECT_DRAW_COUNT)
    effect::set_code(d, e3, EFFECT_DRAW_COUNT);
    // e3:SetRange(LOCATION_MZONE)
    effect::set_range(d, e3, LOCATION_MZONE);
    // e3:SetTargetRange(1,0)
    effect::set_target_range(d, e3, 1, 0);
    // e3:SetCondition(function(e) return e:GetHandler():GetOverlayGroup():GetBinClassCount(Card.GetRace)>=3 end)
    effect::set_condition(d, e3, Some(lua_fn(|d, a: Args| bin_race_count(d, a.e(0)) >= 3)));
    // e3:SetValue(2)
    effect::set_value(d, e3, 2);
    // c:RegisterEffect(e3)
    card::register_effect(d, c, e3, false);
}
// e:GetHandler():GetOverlayGroup():GetBinClassCount(Card.GetRace)
fn bin_race_count(d: &mut Duel, e: EffectId) -> u32 {
    let h = effect::get_handler(d, e).nil();
    let og = card::get_overlay_group(d, h);
    group::get_bin_class_count(d, &og, &|d, c| card::get_race(d, c) as i64)
}
// s.attachtg(e,tp,eg,ep,ev,re,r,rp,chk)
fn attachtg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then
    if a.chk == Some(0) {
        // local c=e:GetHandler()
        let c = effect::get_handler(d, a.e).nil();
        // return c:IsXyzMonster() and Duel.IsExistingMatchingCard(Card.IsCanBeXyzMaterial,tp,LOCATION_HAND|LOCATION_GRAVE|LOCATION_REMOVED,0,1,nil,c,tp,REASON_EFFECT)
        return card::is_xyz_monster(d, c)
            && duel::is_existing_matching_card(
                d,
                Some(&|d, x| card::is_can_be_xyz_material(d, x, Some(c), Some(a.tp), Some(REASON_EFFECT))),
                a.tp,
                LOCATION_HAND | LOCATION_GRAVE | LOCATION_REMOVED,
                0,
                1,
                NO_EX,
            );
    }
    // Duel.SetPossibleOperationInfo(0,CATEGORY_LEAVE_GRAVE,nil,1,tp,0)
    duel::set_possible_operation_info(d, 0, CATEGORY_LEAVE_GRAVE, None, 1, a.tp, 0);
    true
}
// s.attachop(e,tp,eg,ep,ev,re,r,rp)
fn attachop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if not c:IsRelateToEffect(e) then return end
    if !card::is_relate_to_effect(d, c, a.e) {
        return;
    }
    // local detached_material=e:GetChainData().cost_detached_materials:GetFirst()
    let detached_materials = chain::effect_data(d, a.e, None, chain::field::COST_DETACHED_MATERIALS).as_group().cloned().nil();
    let detached_material = group::get_first(d, &detached_materials).nil();
    // local except=detached_material:IsRelateToEffect(e) and detached_material or nil
    let except: Option<CardSet> = if card::is_relate_to_effect(d, detached_material, a.e) { Some(CardSet::from_card(detached_material)) } else { None };
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_XMATERIAL)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_XMATERIAL as u64);
    let ex = match &except {
        Some(g) => g.into(),
        None => NO_EX,
    };
    // local sc=Duel.SelectMatchingCard(tp,aux.NecroValleyFilter(Card.IsCanBeXyzMaterial),tp,LOCATION_HAND|LOCATION_GRAVE|LOCATION_REMOVED,0,1,1,except,c,tp,REASON_EFFECT):GetFirst()
    let sel = duel::select_matching_card(
        d,
        tp,
        Some(&aux::necro_valley_filter(|d, x| card::is_can_be_xyz_material(d, x, Some(c), Some(tp), Some(REASON_EFFECT)))),
        tp,
        LOCATION_HAND | LOCATION_GRAVE | LOCATION_REMOVED,
        0,
        1,
        1,
        false,
        ex,
    )
    .nil();
    // if sc then
    if let Some(sc) = group::get_first(d, &sel) {
        // if sc:IsLocation(LOCATION_HAND) then Duel.ConfirmCards(1-tp,sc) else Duel.HintSelection(sc) end
        if card::is_location(d, sc, LOCATION_HAND) {
            duel::confirm_cards(d, 1 - tp, sc);
        } else {
            duel::hint_selection(d, sc, true);
        }
        // Duel.Overlay(c,sc)
        duel::overlay(d, c, sc, false);
    }
}
