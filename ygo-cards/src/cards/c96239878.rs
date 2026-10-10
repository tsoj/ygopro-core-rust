//! Strength in Unity
//! Scripted by Naim
use super::prelude::*;

pub const ID: u32 = 96239878;
pub const INFO: ScriptInfo = ScriptInfo { listed_names: &[CARD_BLUEEYES_W_DRAGON, CARD_DARK_MAGICIAN], ..ScriptInfo::DEFAULT };

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
    // Check if a monster was Ritual or Fusion Summoned using "Blue-Eyes White Dragon" or "Dark Magician"
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e1, EFFECT_TYPE_FIELD);
    // e1:SetProperty(EFFECT_FLAG_CANNOT_DISABLE+EFFECT_FLAG_SET_AVAILABLE+EFFECT_FLAG_IGNORE_RANGE)
    effect::set_property(d, e1, EFFECT_FLAG_CANNOT_DISABLE + EFFECT_FLAG_SET_AVAILABLE + EFFECT_FLAG_IGNORE_RANGE, 0);
    // e1:SetCode(EFFECT_MATERIAL_CHECK)
    effect::set_code(d, e1, EFFECT_MATERIAL_CHECK);
    // e1:SetRange(LOCATION_SZONE)
    effect::set_range(d, e1, LOCATION_SZONE);
    // e1:SetValue(s.valcheck)
    effect::set_value_fn(d, e1, effect_card_value(valcheck));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // Banish 1 card from the opponent's field or GY
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e2, aux::stringid(ID, 0));
    // e2:SetCategory(CATEGORY_REMOVE)
    effect::set_category(d, e2, CATEGORY_REMOVE);
    // e2:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e2, EFFECT_TYPE_FIELD + EFFECT_TYPE_TRIGGER_O);
    // e2:SetProperty(EFFECT_FLAG_CARD_TARGET+EFFECT_FLAG_DELAY)
    effect::set_property(d, e2, EFFECT_FLAG_CARD_TARGET + EFFECT_FLAG_DELAY, 0);
    // e2:SetCode(EVENT_SPSUMMON_SUCCESS)
    effect::set_code(d, e2, EVENT_SPSUMMON_SUCCESS);
    // e2:SetRange(LOCATION_SZONE)
    effect::set_range(d, e2, LOCATION_SZONE);
    // e2:SetCountLimit(1,id)
    effect::set_count_limit(d, e2, 1, ID, 0, 0);
    // e2:SetCondition(s.rmvcond)
    effect::set_condition(d, e2, Some(condition(rmvcond)));
    // e2:SetTarget(s.rmvtg)
    effect::set_target(d, e2, Some(cost_target(rmvtg)));
    // e2:SetOperation(s.rmvop)
    effect::set_operation(d, e2, Some(operation(rmvop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
    // Return 1 Level 7 or higher Normal Monster in your GY to your hand or Deck
    // local e3=Effect.CreateEffect(c)
    let e3 = effect::create_effect(d, c);
    // e3:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e3, aux::stringid(ID, 1));
    // e3:SetCategory(CATEGORY_TOHAND+CATEGORY_TODECK)
    effect::set_category(d, e3, CATEGORY_TOHAND + CATEGORY_TODECK);
    // e3:SetType(EFFECT_TYPE_IGNITION)
    effect::set_type(d, e3, EFFECT_TYPE_IGNITION);
    // e3:SetProperty(EFFECT_FLAG_CARD_TARGET)
    effect::set_property(d, e3, EFFECT_FLAG_CARD_TARGET, 0);
    // e3:SetRange(LOCATION_SZONE)
    effect::set_range(d, e3, LOCATION_SZONE);
    // e3:SetCountLimit(1,{id,1})
    effect::set_count_limit(d, e3, 1, ID, 1, 0);
    // e3:SetCost(Cost.SelfToGrave)
    effect::set_cost(d, e3, Some(cost::self_to_grave()));
    // e3:SetTarget(s.rettg)
    effect::set_target(d, e3, Some(cost_target(rettg)));
    // e3:SetOperation(s.retop)
    effect::set_operation(d, e3, Some(operation(retop)));
    // c:RegisterEffect(e3)
    card::register_effect(d, c, e3, false);
}
/// `s.matfilter(c,sc,sumtype,tp)`
fn matfilter(d: &mut Duel, c: CardId, sc: CardId, sumtype: u64, tp: u8) -> bool {
    // return c:IsSummonCode(sc,sumtype,tp,CARD_BLUEEYES_W_DRAGON,CARD_DARK_MAGICIAN)
    card::is_summon_code(d, c, Some(sc), sumtype, tp, &[CARD_BLUEEYES_W_DRAGON, CARD_DARK_MAGICIAN])
}
/// `s.valcheck(e,c)`
fn valcheck(d: &mut Duel, e: EffectId, c: CardId) -> i64 {
    // local g=c:GetMaterial()
    let g = card::get_material(d, c);
    // if not g or #g==0 then return end
    if g.is_empty() {
        return 0;
    }
    // local tp=e:GetHandlerPlayer()
    let tp = effect::get_handler_player(d, e);
    // local ritual=g:IsExists(s.matfilter,1,nil,c,SUMMON_TYPE_RITUAL,tp)
    let ritual = group::is_exists(d, &g, &|d, mc| matfilter(d, mc, c, SUMMON_TYPE_RITUAL as u64, tp), 1, NO_EX);
    // local fusion=g:IsExists(s.matfilter,1,nil,c,SUMMON_TYPE_FUSION,tp)
    let fusion = group::is_exists(d, &g, &|d, mc| matfilter(d, mc, c, SUMMON_TYPE_FUSION as u64, tp), 1, NO_EX);
    // if ritual or fusion then
    if ritual || fusion {
        // local sumtype=(ritual and SUMMON_TYPE_RITUAL) or 0
        let mut sumtype = if ritual { SUMMON_TYPE_RITUAL } else { 0 };
        // if fusion then sumtype=sumtype|SUMMON_TYPE_FUSION end
        if fusion {
            sumtype |= SUMMON_TYPE_FUSION;
        }
        // c:RegisterFlagEffect(id,RESET_CHAIN,0,1,sumtype)
        card::register_flag_effect(d, c, ID, RESET_CHAIN, 0, 1, sumtype as i64, 0);
    }
    // (the function returns nothing: nil, which the core reads as 0)
    0
}
/// `s.cfilter(c,tp)`
fn cfilter(d: &mut Duel, c: CardId, tp: u8) -> bool {
    // if not (c:HasFlagEffect(id) and c:IsSummonPlayer(tp) and c:IsSummonType({SUMMON_TYPE_RITUAL,SUMMON_TYPE_FUSION})) then return false end
    if !(card::has_flag_effect(d, c, ID, None) && card::is_summon_player(d, c, tp) && card::is_summon_type(d, c, &[SUMMON_TYPE_RITUAL, SUMMON_TYPE_FUSION])) {
        return false;
    }
    // local sumtype=c:GetFlagEffectLabel(id)
    let sumtype = card::get_flag_effect_label(d, c, ID).first().copied().unwrap_or(0);
    // return sumtype==SUMMON_TYPE_RITUAL|SUMMON_TYPE_FUSION or c:IsSummonType(sumtype)
    sumtype == (SUMMON_TYPE_RITUAL | SUMMON_TYPE_FUSION) as i64 || card::is_summon_type(d, c, &[sumtype as u32])
}
/// `s.rmvcond(e,tp,eg,ep,ev,re,r,rp)`
fn rmvcond(d: &mut Duel, a: &EffArgs) -> bool {
    // return eg:IsExists(s.cfilter,1,nil,tp)
    let eg = a.eg.clone().nil();
    group::is_exists(d, &eg, &|d, c| cfilter(d, c, a.tp), 1, NO_EX)
}
/// `s.rmvtg(e,tp,...,chk,chkc)`
fn rmvtg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chkc then return chkc:IsLocation(LOCATION_ONFIELD|LOCATION_GRAVE) and chkc:IsControler(1-tp) and chkc:IsAbleToRemove() end
    if let Some(chkc) = a.chkc() {
        return card::is_location(d, chkc, LOCATION_ONFIELD | LOCATION_GRAVE) && card::is_controler(d, chkc, 1 - a.tp) && card::is_able_to_remove(d, chkc, None, None, None);
    }
    // if chk==0 then return Duel.IsExistingTarget(Card.IsAbleToRemove,tp,0,LOCATION_ONFIELD|LOCATION_GRAVE,1,nil) end
    if a.chk == Some(0) {
        return duel::is_existing_target(d, Some(&|d, c| card::is_able_to_remove(d, c, None, None, None)), a.tp, 0, LOCATION_ONFIELD | LOCATION_GRAVE, 1, NO_EX);
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_REMOVE)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_REMOVE as u64);
    // local g=Duel.SelectTarget(tp,Card.IsAbleToRemove,tp,0,LOCATION_ONFIELD|LOCATION_GRAVE,1,1,nil)
    let g = duel::select_target(d, a.tp, Some(&|d, c| card::is_able_to_remove(d, c, None, None, None)), a.tp, 0, LOCATION_ONFIELD | LOCATION_GRAVE, 1, 1, false, NO_EX);
    // Duel.SetOperationInfo(0,CATEGORY_REMOVE,g,1,tp,0)
    duel::set_operation_info(d, 0, CATEGORY_REMOVE, g, 1, a.tp, 0);
    true
}
/// `s.rmvop(e,tp,...)`
fn rmvop(d: &mut Duel, a: &EffArgs) {
    // local tc=Duel.GetFirstTarget()
    let tc = duel::first_target(d).nil();
    // if tc:IsRelateToEffect(e) then
    if card::is_relate_to_effect(d, tc, a.e) {
        // Duel.Remove(tc,POS_FACEUP,REASON_EFFECT)
        duel::remove(d, tc, POS_FACEUP as u8, REASON_EFFECT, None, None);
    }
}
/// `s.retfilter(c)`
fn retfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsType(TYPE_NORMAL) and c:IsLevelAbove(7) and (c:IsAbleToHand() or c:IsAbleToDeck())
    card::is_type(d, c, TYPE_NORMAL) && card::is_level_above(d, c, 7) && (card::is_able_to_hand(d, c, None) || card::is_able_to_deck(d, c))
}
/// `s.rettg(e,tp,...,chk,chkc)`
fn rettg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chkc then return chkc:IsLocation(LOCATION_GRAVE) and chkc:IsControler(tp) and s.retfilter(chkc) end
    if let Some(chkc) = a.chkc() {
        return card::is_location(d, chkc, LOCATION_GRAVE) && card::is_controler(d, chkc, a.tp) && retfilter(d, chkc);
    }
    // if chk==0 then return Duel.IsExistingTarget(s.retfilter,tp,LOCATION_GRAVE,0,1,nil) end
    if a.chk == Some(0) {
        return duel::is_existing_target(d, Some(&retfilter), a.tp, LOCATION_GRAVE, 0, 1, NO_EX);
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_TARGET)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_TARGET as u64);
    // local g=Duel.SelectTarget(tp,s.retfilter,tp,LOCATION_GRAVE,0,1,1,nil)
    let g = duel::select_target(d, a.tp, Some(&retfilter), a.tp, LOCATION_GRAVE, 0, 1, 1, false, NO_EX);
    // Duel.SetPossibleOperationInfo(0,CATEGORY_TOHAND,g,1,tp,0)
    duel::set_possible_operation_info(d, 0, CATEGORY_TOHAND, g.clone(), 1, a.tp, 0);
    // Duel.SetPossibleOperationInfo(0,CATEGORY_TODECK,g,1,tp,0)
    duel::set_possible_operation_info(d, 0, CATEGORY_TODECK, g, 1, a.tp, 0);
    true
}
/// `s.retop(e,tp,...)`
fn retop(d: &mut Duel, a: &EffArgs) {
    // local tc=Duel.GetFirstTarget()
    let tc = duel::first_target(d).nil();
    // if not tc:IsRelateToEffect(e) then return end
    if !card::is_relate_to_effect(d, tc, a.e) {
        return;
    }
    // aux.ToHandOrElse(tc,tp,
    //     function(c) return tc:IsAbleToDeck() end,
    //     function(c) Duel.SendtoDeck(tc,nil,SEQ_DECKSHUFFLE,REASON_EFFECT) end,
    //     aux.Stringid(id,2))
    aux::to_hand_or_else(
        d,
        tc,
        a.tp,
        Some(&|d, _c| card::is_able_to_deck(d, tc)),
        Some(&|d, _g| {
            duel::send_to_deck(d, tc, None, SEQ_DECKSHUFFLE as i32, REASON_EFFECT, None);
            0
        }),
        Some(aux::stringid(ID, 2)),
    );
}
