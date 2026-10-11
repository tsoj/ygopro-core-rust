//! Disaster, Dragon Ruler of All Apocalypses
use super::prelude::*;

pub const ID: u32 = 67359907;

pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_DRAGON_RULER], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // c:EnableReviveLimit()
    card::enable_revive_limit(d, c);
    // --Xyz Summon procedure: 2 Rank 7 Xyz monsters
    // Xyz.AddProcedure(c,aux.FilterBoolFunction(Card.IsRank,7),nil,2)
    xyz::add_procedure(d, c, Some(mat_filter(|d, c, _sc, _st, _tp| card::is_rank(d, c, &[7]))), None, 2, None, None, None, None, false, None);
    // --Attach Level 7 "Dragon Ruler" monsters to this card when it is Xyz Summoned
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_REMOVE)
    effect::set_category(d, e1, CATEGORY_REMOVE);
    // e1:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_O);
    // e1:SetProperty(EFFECT_FLAG_DELAY+EFFECT_FLAG_CARD_TARGET)
    effect::set_property(d, e1, EFFECT_FLAG_DELAY + EFFECT_FLAG_CARD_TARGET, 0);
    // e1:SetCode(EVENT_SPSUMMON_SUCCESS)
    effect::set_code(d, e1, EVENT_SPSUMMON_SUCCESS);
    // e1:SetCountLimit(1,id)
    effect::set_count_limit(d, e1, 1, ID, 0, 0);
    // e1:SetCondition(function(e) return e:GetHandler():IsXyzSummoned() end)
    effect::set_condition(d, e1, Some(lua_fn(xyz_summoned_cond)));
    // e1:SetTarget(s.attachtg)
    effect::set_target(d, e1, Some(cost_target(attachtg)));
    // e1:SetOperation(s.attachop)
    effect::set_operation(d, e1, Some(operation(attachop)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // --If this card has LIGHT, DARK, EARTH, WATER, FIRE, and WIND "Dragon Ruler" monsters as material, it gains 4600 ATK/DEF
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e2, EFFECT_TYPE_SINGLE);
    // e2:SetProperty(EFFECT_FLAG_SINGLE_RANGE)
    effect::set_property(d, e2, EFFECT_FLAG_SINGLE_RANGE, 0);
    // e2:SetCode(EFFECT_UPDATE_ATTACK)
    effect::set_code(d, e2, EFFECT_UPDATE_ATTACK);
    // e2:SetRange(LOCATION_MZONE)
    effect::set_range(d, e2, LOCATION_MZONE);
    // e2:SetCondition(s.attributematcon)
    effect::set_condition(d, e2, Some(lua_fn(attributematcon)));
    // e2:SetValue(4600)
    effect::set_value(d, e2, 4600);
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
    // local e3=e2:Clone()
    let e3 = effect::clone(d, e2);
    // e3:SetCode(EFFECT_UPDATE_DEFENSE)
    effect::set_code(d, e3, EFFECT_UPDATE_DEFENSE);
    // c:RegisterEffect(e3)
    card::register_effect(d, c, e3, false);
    // --If this card has LIGHT, DARK, EARTH, WATER, FIRE, and WIND "Dragon Ruler" monsters as material, it is unaffected by other cards' effects
    // local e4=e2:Clone()
    let e4 = effect::clone(d, e2);
    // e4:SetCode(EFFECT_IMMUNE_EFFECT)
    effect::set_code(d, e4, EFFECT_IMMUNE_EFFECT);
    // e4:SetValue(function(e,te) return te:GetOwner()~=e:GetOwner() end)
    effect::set_value_fn(d, e4, lua_fn(immune_value));
    // c:RegisterEffect(e4)
    card::register_effect(d, c, e4, false);
}
// function(e) return e:GetHandler():IsXyzSummoned() end
fn xyz_summoned_cond(d: &mut Duel, a: Args) -> bool {
    let c = effect::get_handler(d, a.e(0)).nil();
    card::is_xyz_summoned(d, c)
}
// function s.attachfilter(c,xc,tp)
fn attachfilter(d: &mut Duel, c: CardId, xc: CardId, tp: u8) -> bool {
    // return c:IsSetCard(SET_DRAGON_RULER) and c:IsLevel(7) and c:IsCanBeXyzMaterial(xc,tp,REASON_EFFECT) and c:IsFaceup()
    card::is_set_card(d, c, &[SET_DRAGON_RULER as u16])
        && card::is_level(d, c, &[7])
        && card::is_can_be_xyz_material(d, c, Some(xc), Some(tp), Some(REASON_EFFECT))
        && card::is_faceup(d, c)
}
// function s.attachtg(e,tp,eg,ep,ev,re,r,rp,chk)
fn attachtg(d: &mut Duel, a: &EffArgs) -> bool {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // (chkc is not a parameter of this function: it is nil in Lua, so no chkc branch)
    // if chk==0 then return Duel.IsExistingTarget(s.attachfilter,tp,LOCATION_GRAVE|LOCATION_REMOVED,0,1,nil,c,tp) end
    if a.chk == Some(0) {
        return duel::is_existing_target(d, Some(&|d, cc| attachfilter(d, cc, c, a.tp)), a.tp, LOCATION_GRAVE | LOCATION_REMOVED, 0, 1, NO_EX);
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_XMATERIAL)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_XMATERIAL as u64);
    // local g=Duel.SelectTarget(tp,s.attachfilter,tp,LOCATION_GRAVE|LOCATION_REMOVED,0,1,4,nil,c,tp)
    let g = duel::select_target(d, a.tp, Some(&|d, cc| attachfilter(d, cc, c, a.tp)), a.tp, LOCATION_GRAVE | LOCATION_REMOVED, 0, 1, 4, false, NO_EX).nil();
    // if g:IsExists(Card.IsLocation,1,nil,LOCATION_GRAVE) then
    if group::is_exists(d, &g, &|d, cc| card::is_location(d, cc, LOCATION_GRAVE), 1, NO_EX) {
        // local gyg=g:Filter(Card.IsLocation,nil,LOCATION_GRAVE)
        let gyg = group::filter(d, &g, &|d, cc| card::is_location(d, cc, LOCATION_GRAVE), NO_EX);
        // Duel.SetOperationInfo(0,CATEGORY_LEAVE_GRAVE,gyg,#gyg,tp,0)
        let n = gyg.len() as u8;
        duel::set_operation_info(d, 0, CATEGORY_LEAVE_GRAVE, Some(gyg), n, a.tp, 0);
    }
    // Duel.SetPossibleOperationInfo(0,CATEGORY_REMOVE,nil,1,1-tp,LOCATION_MZONE|LOCATION_GRAVE)
    duel::set_possible_operation_info(d, 0, CATEGORY_REMOVE, None, 1, 1 - a.tp, (LOCATION_MZONE | LOCATION_GRAVE) as i32);
    true
}
// function s.rmfilter(c,og)
fn rmfilter(d: &mut Duel, c: CardId, og: &Group) -> bool {
    // return c:IsMonster() and c:IsAbleToRemove() and c:IsFaceup() and og:IsExists(Card.IsAttribute,1,nil,c:GetAttribute())
    if !(card::is_monster(d, c) && card::is_able_to_remove(d, c, None, None, None) && card::is_faceup(d, c)) {
        return false;
    }
    let attr = card::get_attribute(d, c);
    group::is_exists(d, og, &|d, cc| card::is_attribute(d, cc, attr), 1, NO_EX)
}
// function s.attachop(e,tp,eg,ep,ev,re,r,rp)
fn attachop(d: &mut Duel, a: &EffArgs) {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if not c:IsRelateToEffect(e) or c:IsImmuneToEffect(e) then return end
    if !card::is_relate_to_effect(d, c, a.e) || card::is_immune_to_effect(d, c, a.e) {
        return;
    }
    // local tg=Duel.GetTargetCards(e):Filter(s.attachfilter,nil,c,tp):Remove(Card.IsImmuneToEffect,nil,e)
    let targets = duel::get_target_cards(d, a.e).nil();
    let mut tg = group::filter(d, &targets, &|d, cc| attachfilter(d, cc, c, a.tp), NO_EX);
    group::remove(d, &mut tg, &|d, cc| card::is_immune_to_effect(d, cc, a.e), NO_EX);
    // if #tg>0 then
    if !tg.is_empty() {
        // Duel.Overlay(c,tg)
        duel::overlay(d, c, tg, false);
        // local og=c:GetOverlayGroup()
        let og = card::get_overlay_group(d, c);
        // if Duel.IsExistingMatchingCard(s.rmfilter,tp,0,LOCATION_MZONE|LOCATION_GRAVE,1,nil,og) and Duel.SelectYesNo(tp,aux.Stringid(id,1)) then
        if duel::is_existing_matching_card(d, Some(&|d, cc| rmfilter(d, cc, &og)), a.tp, 0, LOCATION_MZONE | LOCATION_GRAVE, 1, NO_EX)
            && duel::select_yes_no(d, a.tp, aux::stringid(ID, 1))
        {
            // local rg=Duel.GetMatchingGroup(s.rmfilter,tp,0,LOCATION_MZONE|LOCATION_GRAVE,nil,og)
            let rg = duel::get_matching_group(d, Some(&|d, cc| rmfilter(d, cc, &og)), a.tp, 0, LOCATION_MZONE | LOCATION_GRAVE, NO_EX);
            // Duel.BreakEffect()
            duel::break_effect(d);
            // Duel.Remove(rg,POS_FACEUP,REASON_EFFECT)
            duel::remove(d, rg, POS_FACEUP as u8, REASON_EFFECT, None, None);
        }
    }
}
// function s.attributematcon(e)
fn attributematcon(d: &mut Duel, a: Args) -> bool {
    // local og=e:GetHandler():GetOverlayGroup():Filter(Card.IsSetCard,nil,SET_DRAGON_RULER)
    let handler = effect::get_handler(d, a.e(0)).nil();
    let og = card::get_overlay_group(d, handler);
    let og = group::filter(d, &og, &|d, cc| card::is_set_card(d, cc, &[SET_DRAGON_RULER as u16]), NO_EX);
    // return og:IsExists(Card.IsAttribute,1,nil,ATTRIBUTE_LIGHT) and og:IsExists(Card.IsAttribute,1,nil,ATTRIBUTE_DARK)
    //   and og:IsExists(Card.IsAttribute,1,nil,ATTRIBUTE_EARTH) and og:IsExists(Card.IsAttribute,1,nil,ATTRIBUTE_WATER)
    //   and og:IsExists(Card.IsAttribute,1,nil,ATTRIBUTE_FIRE) and og:IsExists(Card.IsAttribute,1,nil,ATTRIBUTE_WIND)
    [ATTRIBUTE_LIGHT, ATTRIBUTE_DARK, ATTRIBUTE_EARTH, ATTRIBUTE_WATER, ATTRIBUTE_FIRE, ATTRIBUTE_WIND]
        .iter()
        .all(|&attr| group::is_exists(d, &og, &|d, cc| card::is_attribute(d, cc, attr), 1, NO_EX))
}
// function(e,te) return te:GetOwner()~=e:GetOwner() end
fn immune_value(d: &mut Duel, a: Args) -> bool {
    effect::get_owner(d, a.e(1)) != effect::get_owner(d, a.e(0))
}
