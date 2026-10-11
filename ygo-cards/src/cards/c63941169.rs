//! Abyss-steam
//! --アビスチーム
//! scripted by pyrQ
use super::prelude::*;

pub const ID: u32 = 63941169;

pub const INFO: ScriptInfo = ScriptInfo {
    listed_series: &[SET_MERMAIL],
    ..ScriptInfo::DEFAULT
};

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_DRAW)
    effect::set_category(d, e1, CATEGORY_DRAW);
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    // e1:SetProperty(EFFECT_FLAG_PLAYER_TARGET)
    effect::set_property(d, e1, EFFECT_FLAG_PLAYER_TARGET, 0);
    // e1:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    // e1:SetCountLimit(1,id)
    effect::set_count_limit(d, e1, 1, ID, 0, 0);
    // e1:SetTarget(s.target)
    effect::set_target(d, e1, Some(cost_target(target)));
    // e1:SetOperation(s.activate)
    effect::set_operation(d, e1, Some(operation(activate)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // Add 1 "Mermail" monster from your GY or banishment to your hand
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e2, aux::stringid(ID, 1));
    // e2:SetCategory(CATEGORY_TOHAND)
    effect::set_category(d, e2, CATEGORY_TOHAND);
    // e2:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e2, EFFECT_TYPE_FIELD + EFFECT_TYPE_TRIGGER_O);
    // e2:SetProperty(EFFECT_FLAG_DAMAGE_STEP+EFFECT_FLAG_DELAY,EFFECT_FLAG2_CHECK_SIMULTANEOUS)
    effect::set_property(d, e2, EFFECT_FLAG_DAMAGE_STEP + EFFECT_FLAG_DELAY, EFFECT_FLAG2_CHECK_SIMULTANEOUS);
    // e2:SetCode(EVENT_DESTROYED)
    effect::set_code(d, e2, EVENT_DESTROYED);
    // e2:SetRange(LOCATION_GRAVE)
    effect::set_range(d, e2, LOCATION_GRAVE);
    // e2:SetCountLimit(1,{id,1})
    effect::set_count_limit(d, e2, 1, ID, 1, 0);
    // e2:SetCondition(s.thcon)
    effect::set_condition(d, e2, Some(condition(thcon)));
    // e2:SetCost(Cost.SelfBanish)
    effect::set_cost(d, e2, Some(cost::self_banish()));
    // e2:SetTarget(s.thtg)
    effect::set_target(d, e2, Some(cost_target(thtg)));
    // e2:SetOperation(s.thop)
    effect::set_operation(d, e2, Some(operation(thop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
}
fn faceup_water_race_filter(d: &mut Duel, c: CardId) -> bool {
    // aux.FaceupFilter(Card.IsRace,RACE_FISH|RACE_SEASERPENT|RACE_AQUA)
    card::is_faceup(d, c) && card::is_race(d, c, RACE_FISH | RACE_SEASERPENT | RACE_AQUA)
}
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    // local g=Duel.GetMatchingGroup(aux.FaceupFilter(Card.IsRace,RACE_FISH|RACE_SEASERPENT|RACE_AQUA),tp,LOCATION_MZONE,0,nil)
    let g = duel::get_matching_group(d, Some(&faceup_water_race_filter), a.tp, LOCATION_MZONE, 0, NO_EX);
    // local ct=g:GetClassCount(Card.GetRace)
    let ct = group::get_class_count(d, &g, &|d, c| card::get_race(d, c) as i64);
    // if chk==0 then return ct>0 and Duel.IsPlayerCanDraw(tp,ct) end
    if a.chk == Some(0) {
        return ct > 0 && duel::is_player_can_draw(d, a.tp, ct as u32);
    }
    // Duel.SetTargetPlayer(tp)
    duel::set_target_player(d, a.tp);
    // Duel.SetOperationInfo(0,CATEGORY_DRAW,nil,0,tp,ct)
    duel::set_operation_info(d, 0, CATEGORY_DRAW, None, 0, a.tp, ct as i32);
    true
}
fn activate(d: &mut Duel, a: &EffArgs) {
    // local g=Duel.GetMatchingGroup(aux.FaceupFilter(Card.IsRace,RACE_FISH|RACE_SEASERPENT|RACE_AQUA),tp,LOCATION_MZONE,0,nil)
    let g = duel::get_matching_group(d, Some(&faceup_water_race_filter), a.tp, LOCATION_MZONE, 0, NO_EX);
    // local ct=g:GetClassCount(Card.GetRace)
    let ct = group::get_class_count(d, &g, &|d, c| card::get_race(d, c) as i64);
    // if ct>0 then
    if ct > 0 {
        // local p=Duel.GetChainInfo(0,CHAININFO_TARGET_PLAYER)
        let info = duel::get_chain_info(d, 0, &[CHAININFO_TARGET_PLAYER]);
        let p = info[0].int() as u8;
        // Duel.Draw(p,ct,REASON_EFFECT)
        duel::draw(d, p, ct as u32, REASON_EFFECT);
    }
    // if not e:IsHasType(EFFECT_TYPE_ACTIVATE) then return end
    if !effect::is_has_type(d, a.e, EFFECT_TYPE_ACTIVATE) {
        return;
    }
    // local reset_ct=Duel.IsTurnPlayer(tp) and 2 or 1
    let reset_ct: u32 = if duel::is_turn_player(d, a.tp) { 2 } else { 1 };
    // You cannot Special Summon from the Extra Deck until the end of your next turn after this card resolves, except WATER monsters
    // local e1=Effect.CreateEffect(e:GetHandler())
    let c = effect::get_handler(d, a.e).nil();
    let e1 = effect::create_effect(d, c);
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
    // e1:SetTarget(function(e,c) return c:IsLocation(LOCATION_EXTRA) and not c:IsAttribute(ATTRIBUTE_WATER) end)
    effect::set_target(d, e1, Some(effect_card_fn(|d, _e, c| {
        card::is_location(d, c, LOCATION_EXTRA) && !(card::is_attribute(d, c, ATTRIBUTE_WATER))
    })));
    // e1:SetReset(RESET_PHASE|PHASE_END|RESET_SELF_TURN,reset_ct)
    effect::set_reset(d, e1, RESET_PHASE | PHASE_END | RESET_SELF_TURN, reset_ct as u16);
    // Duel.RegisterEffect(e1,tp)
    duel::register_effect(d, e1, a.tp);
}
fn thconfilter(d: &mut Duel, c: CardId, tp: u8) -> bool {
    // return c:IsPreviousAttributeOnField(ATTRIBUTE_WATER) and c:IsPreviousLocation(LOCATION_MZONE) and c:IsPreviousControler(tp)
    //     and c:IsPreviousPosition(POS_FACEUP) and c:IsReason(REASON_BATTLE|REASON_EFFECT)
    card::is_previous_attribute_on_field(d, c, ATTRIBUTE_WATER)
        && card::is_previous_location(d, c, LOCATION_MZONE)
        && card::is_previous_controler(d, c, tp)
        && card::is_previous_position(d, c, POS_FACEUP)
        && card::is_reason(d, c, REASON_BATTLE | REASON_EFFECT)
}
fn thcon(d: &mut Duel, a: &EffArgs) -> bool {
    // return not eg:IsContains(e:GetHandler()) and eg:IsExists(s.thconfilter,1,nil,tp)
    let eg = a.eg.clone().nil();
    let c = effect::get_handler(d, a.e).nil();
    !group::is_contains(&eg, c) && group::is_exists(d, &eg, &|d, x| thconfilter(d, x, a.tp), 1, NO_EX)
}
fn thfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsSetCard(SET_MERMAIL) and c:IsMonster() and c:IsAbleToHand()
    card::is_set_card(d, c, &[SET_MERMAIL as u16]) && card::is_monster(d, c) && card::is_able_to_hand(d, c, None)
}
fn thtg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return Duel.IsExistingMatchingCard(s.thfilter,tp,LOCATION_GRAVE|LOCATION_REMOVED,0,1,nil) end
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, Some(&thfilter), a.tp, LOCATION_GRAVE | LOCATION_REMOVED, 0, 1, NO_EX);
    }
    // Duel.SetOperationInfo(0,CATEGORY_TOHAND,nil,1,tp,LOCATION_GRAVE|LOCATION_REMOVED)
    duel::set_operation_info(d, 0, CATEGORY_TOHAND, None, 1, a.tp, (LOCATION_GRAVE | LOCATION_REMOVED) as i32);
    true
}
fn thop(d: &mut Duel, a: &EffArgs) {
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_ATOHAND)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_ATOHAND as u64);
    // local g=Duel.SelectMatchingCard(tp,aux.NecroValleyFilter(s.thfilter),tp,LOCATION_GRAVE|LOCATION_REMOVED,0,1,1,nil)
    let g = duel::select_matching_card(
        d,
        a.tp,
        Some(&aux::necro_valley_filter(thfilter)),
        a.tp,
        LOCATION_GRAVE | LOCATION_REMOVED,
        0,
        1,
        1,
        false,
        NO_EX,
    )
    .nil();
    // if #g>0 then
    if !g.is_empty() {
        // Duel.SendtoHand(g,nil,REASON_EFFECT)
        duel::send_to_hand(d, g.clone(), None, REASON_EFFECT, None);
        // Duel.ConfirmCards(1-tp,g)
        duel::confirm_cards(d, 1 - a.tp, g);
    }
}
