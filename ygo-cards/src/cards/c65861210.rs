//! Fallen Paradise of the Sacred Beasts
//! Scripted by Eerie Code
use super::prelude::*;

pub const ID: u32 = 65861210;

pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_SACRED_BEAST], ..ScriptInfo::DEFAULT };

/// `LOCATIONS_HAND_DECK_GRAVE_REMOVED`
const LOCATIONS_HAND_DECK_GRAVE_REMOVED: u32 = LOCATION_HAND | LOCATION_DECK | LOCATION_GRAVE | LOCATION_REMOVED;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate
    // local e0=Effect.CreateEffect(c)
    let e0 = effect::create_effect(d, c);
    // e0:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e0, EFFECT_TYPE_ACTIVATE);
    // e0:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e0, EVENT_FREE_CHAIN);
    // c:RegisterEffect(e0)
    card::register_effect(d, c, e0, false);
    // Thrice per turn: You can send 3 other cards of the same type (Monster, Spell, or Trap) from your hand and/or face-up field to the GY, then you can Special Summon 1 "Sacred Beast" monster from your hand, Deck, GY, or banishment, and if you do, it is unaffected by your opponent's activated Spell/Trap effects
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_TOGRAVE+CATEGORY_SPECIAL_SUMMON)
    effect::set_category(d, e1, CATEGORY_TOGRAVE + CATEGORY_SPECIAL_SUMMON);
    // e1:SetType(EFFECT_TYPE_IGNITION)
    effect::set_type(d, e1, EFFECT_TYPE_IGNITION);
    // e1:SetRange(LOCATION_FZONE)
    effect::set_range(d, e1, LOCATION_FZONE);
    // e1:SetCountLimit(3)
    effect::set_count_limit(d, e1, 3, 0, 0, 0);
    // e1:SetTarget(s.gytg)
    effect::set_target(d, e1, Some(cost_target(gytg)));
    // e1:SetOperation(s.gyop)
    effect::set_operation(d, e1, Some(operation(gyop)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // If you control a "Sacred Beast" monster whose original Level is 10: You can draw 2 cards
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e2, aux::stringid(ID, 1));
    // e2:SetCategory(CATEGORY_DRAW)
    effect::set_category(d, e2, CATEGORY_DRAW);
    // e2:SetType(EFFECT_TYPE_IGNITION)
    effect::set_type(d, e2, EFFECT_TYPE_IGNITION);
    // e2:SetProperty(EFFECT_FLAG_PLAYER_TARGET)
    effect::set_property(d, e2, EFFECT_FLAG_PLAYER_TARGET, 0);
    // e2:SetRange(LOCATION_FZONE)
    effect::set_range(d, e2, LOCATION_FZONE);
    // e2:SetCountLimit(1,id)
    effect::set_count_limit(d, e2, 1, ID, 0, 0);
    // e2:SetCondition(s.drcon)
    effect::set_condition(d, e2, Some(condition(drcon)));
    // e2:SetTarget(s.drtg)
    effect::set_target(d, e2, Some(cost_target(drtg)));
    // e2:SetOperation(s.drop)
    effect::set_operation(d, e2, Some(operation(drop_op)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
}
/// `s.gyfilter(c)`
fn gyfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsAbleToGrave() and (c:IsFaceup() or not c:IsOnField())
    card::is_able_to_grave(d, c, None) && (card::is_faceup(d, c) || !card::is_on_field(d, c))
}
/// `s.rescon(sg,e,tp,mg)`: `return sg:GetClassCount(Card.GetMainCardType)==1`
fn rescon(d: &mut Duel, sg: &CardSet, _e: Option<EffectId>, _tp: u8, _mg: &CardSet, _c: Option<CardId>) -> (bool, bool) {
    (group::get_class_count(d, sg, &|d, c| card::get_main_card_type(d, c) as i64) == 1, false)
}
/// `s.gytg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn gytg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // if chk==0 then
    if a.chk == Some(0) {
        // local g=Duel.GetMatchingGroup(s.gyfilter,tp,LOCATION_HAND|LOCATION_ONFIELD,0,e:GetHandler())
        let h = effect::get_handler(d, a.e).nil();
        let g = duel::get_matching_group(d, Some(&gyfilter), tp, LOCATION_HAND | LOCATION_ONFIELD, 0, h.into());
        // return aux.SelectUnselectGroup(g,e,tp,3,3,s.rescon,0)
        return matches!(
            aux::select_unselect_group(d, &g, Some(a.e), tp, Some(3), Some(3), Some(&rescon), 0, tp, None, None, None, false),
            Err(true)
        );
    }
    // Duel.Hint(HINT_OPSELECTED,1-tp,e:GetDescription())
    duel::hint(d, HINT_OPSELECTED as u8, 1 - tp, effect::get_description(d, a.e) as u64);
    // Duel.SetOperationInfo(0,CATEGORY_TOGRAVE,nil,3,tp,LOCATION_HAND|LOCATION_ONFIELD)
    duel::set_operation_info(d, 0, CATEGORY_TOGRAVE, None, 3, tp, (LOCATION_HAND | LOCATION_ONFIELD) as i32);
    // Duel.SetPossibleOperationInfo(0,CATEGORY_SPECIAL_SUMMON,nil,1,tp,LOCATIONS_HAND_DECK_GRAVE_REMOVED)
    duel::set_possible_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, None, 1, tp, LOCATIONS_HAND_DECK_GRAVE_REMOVED as i32);
    true
}
/// `s.spfilter(c,e,tp)`
fn spfilter(d: &mut Duel, c: CardId, e: EffectId, tp: u8) -> bool {
    // return c:IsSetCard(SET_SACRED_BEAST) and (c:IsFaceup() or not c:IsLocation(LOCATION_REMOVED))
    //   and c:IsCanBeSpecialSummoned(e,0,tp,false,true)
    card::is_set_card(d, c, &[SET_SACRED_BEAST as u16])
        && (card::is_faceup(d, c) || !card::is_location(d, c, LOCATION_REMOVED))
        && card::is_can_be_special_summoned(d, c, e, 0, tp, false, true, None, None, None)
}
/// `s.gyop(e,tp,eg,ep,ev,re,r,rp)`
fn gyop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // local g=Duel.GetMatchingGroup(s.gyfilter,tp,LOCATION_HAND|LOCATION_ONFIELD,0,c)
    let g = duel::get_matching_group(d, Some(&gyfilter), tp, LOCATION_HAND | LOCATION_ONFIELD, 0, c.into());
    // if #g<3 then return end
    if g.len() < 3 {
        return;
    }
    // local sg=aux.SelectUnselectGroup(g,e,tp,3,3,s.rescon,1,tp,HINTMSG_TOGRAVE)
    let sg = aux::select_unselect_group(d, &g, Some(a.e), tp, Some(3), Some(3), Some(&rescon), 1, tp, Some(HINTMSG_TOGRAVE as u64), None, None, false).unwrap_or_default();
    // if #sg==3 and Duel.SendtoGrave(sg,REASON_EFFECT)==3 and sg:IsExists(Card.IsLocation,1,nil,LOCATION_GRAVE)
    //   and Duel.GetLocationCount(tp,LOCATION_MZONE)>0
    //   and Duel.IsExistingMatchingCard(aux.NecroValleyFilter(s.spfilter),tp,LOCATIONS_HAND_DECK_GRAVE_REMOVED,0,1,nil,e,tp)
    //   and Duel.SelectYesNo(tp,aux.Stringid(id,2)) then
    if sg.len() == 3
        && duel::send_to_grave(d, sg.clone(), REASON_EFFECT, None, None) == 3
        && group::is_exists(d, &sg, &|d, c| card::is_location(d, c, LOCATION_GRAVE), 1, NO_EX)
        && duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 > 0
        && duel::is_existing_matching_card(d, Some(&aux::necro_valley_filter(|d, c| spfilter(d, c, a.e, tp))), tp, LOCATIONS_HAND_DECK_GRAVE_REMOVED, 0, 1, NO_EX)
        && duel::select_yes_no(d, tp, aux::stringid(ID, 2))
    {
        // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_SPSUMMON)
        duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_SPSUMMON as u64);
        // local sc=Duel.SelectMatchingCard(tp,s.spfilter,tp,LOCATIONS_HAND_DECK_GRAVE_REMOVED,0,1,1,nil,e,tp):GetFirst()
        let g = duel::select_matching_card(d, tp, Some(&|d, c| spfilter(d, c, a.e, tp)), tp, LOCATIONS_HAND_DECK_GRAVE_REMOVED, 0, 1, 1, false, NO_EX).nil();
        // if not sc then return end
        let Some(sc) = group::get_first(d, &g) else { return };
        // Duel.BreakEffect()
        duel::break_effect(d);
        // if Duel.SpecialSummonStep(sc,0,tp,tp,false,true,POS_FACEUP) then
        if duel::special_summon_step(d, sc, 0, tp, tp, false, true, POS_FACEUP as u8, None) {
            // It is unaffected by your opponent's activated Spell/Trap effects
            // local e1=Effect.CreateEffect(c)
            let e1 = effect::create_effect(d, c);
            // e1:SetDescription(aux.Stringid(id,3))
            effect::set_description(d, e1, aux::stringid(ID, 3));
            // e1:SetType(EFFECT_TYPE_SINGLE)
            effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
            // e1:SetProperty(EFFECT_FLAG_CANNOT_DISABLE+EFFECT_FLAG_CLIENT_HINT)
            effect::set_property(d, e1, EFFECT_FLAG_CANNOT_DISABLE + EFFECT_FLAG_CLIENT_HINT, 0);
            // e1:SetCode(EFFECT_IMMUNE_EFFECT)
            effect::set_code(d, e1, EFFECT_IMMUNE_EFFECT);
            // e1:SetValue(function(e,te) return te:GetOwnerPlayer()~=e:GetHandlerPlayer() and te:IsActivated() and te:IsSpellTrapEffect() end)
            effect::set_value_fn(d, e1, lua_fn(immune_value));
            // e1:SetReset(RESET_EVENT|RESETS_STANDARD)
            effect::set_reset(d, e1, RESET_EVENT | RESETS_STANDARD, 1);
            // sc:RegisterEffect(e1)
            card::register_effect(d, sc, e1, false);
        }
        // Duel.SpecialSummonComplete()
        duel::special_summon_complete(d);
    }
}
/// `function(e,te) return te:GetOwnerPlayer()~=e:GetHandlerPlayer() and te:IsActivated() and te:IsSpellTrapEffect() end`
fn immune_value(d: &mut Duel, a: Args) -> bool {
    let te = a.e(1);
    let e = a.e(0);
    effect::get_owner_player(d, te) != effect::get_handler_player(d, e) && effect::is_activated(d, te) && effect::is_spell_trap_effect(d, te)
}
/// `s.drconfilter(c)`
fn drconfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsSetCard(SET_SACRED_BEAST) and c:IsOriginalLevel(10) and c:IsFaceup()
    card::is_set_card(d, c, &[SET_SACRED_BEAST as u16]) && card::is_original_level(d, c, 10) && card::is_faceup(d, c)
}
/// `s.drcon(e,tp,eg,ep,ev,re,r,rp)`
fn drcon(d: &mut Duel, a: &EffArgs) -> bool {
    // return Duel.IsExistingMatchingCard(s.drconfilter,tp,LOCATION_MZONE,0,1,nil)
    duel::is_existing_matching_card(d, Some(&drconfilter), a.tp, LOCATION_MZONE, 0, 1, NO_EX)
}
/// `s.drtg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn drtg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // if chk==0 then return Duel.IsPlayerCanDraw(tp,2) end
    if a.chk == Some(0) {
        return duel::is_player_can_draw(d, tp, 2);
    }
    // Duel.Hint(HINT_OPSELECTED,1-tp,e:GetDescription())
    duel::hint(d, HINT_OPSELECTED as u8, 1 - tp, effect::get_description(d, a.e) as u64);
    // Duel.SetTargetPlayer(tp)
    duel::set_target_player(d, tp);
    // Duel.SetTargetParam(2)
    duel::set_target_param(d, 2);
    // Duel.SetOperationInfo(0,CATEGORY_DRAW,nil,0,tp,2)
    duel::set_operation_info(d, 0, CATEGORY_DRAW, None, 0, tp, 2);
    true
}
/// `s.drop(e,tp,eg,ep,ev,re,r,rp)`
fn drop_op(d: &mut Duel, _a: &EffArgs) {
    // local p,d=Duel.GetChainInfo(0,CHAININFO_TARGET_PLAYER,CHAININFO_TARGET_PARAM)
    let info = duel::get_chain_info(d, 0, &[CHAININFO_TARGET_PLAYER, CHAININFO_TARGET_PARAM]);
    // Duel.Draw(p,d,REASON_EFFECT)
    duel::draw(d, info[0].int() as u8, info[1].int() as u32, REASON_EFFECT);
}
