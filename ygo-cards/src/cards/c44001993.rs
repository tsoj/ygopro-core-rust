//! Magician of Dark Chaos - Black Chaos
use super::prelude::*;

pub const ID: u32 = 44001993;
pub const INFO: ScriptInfo = ScriptInfo { listed_names: &[CARD_LIGHT_AND_DARKNESS_RITUAL], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // c:EnableReviveLimit()
    card::enable_revive_limit(d, c);
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
    // e1:SetProperty(EFFECT_FLAG_SINGLE_RANGE)
    effect::set_property(d, e1, EFFECT_FLAG_SINGLE_RANGE, 0);
    // e1:SetCode(EFFECT_IMMUNE_EFFECT)
    effect::set_code(d, e1, EFFECT_IMMUNE_EFFECT);
    // e1:SetRange(LOCATION_MZONE)
    effect::set_range(d, e1, LOCATION_MZONE);
    // e1:SetValue(s.immval)
    effect::set_value_fn(d, e1, lua_fn(immval));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // local e2a=Effect.CreateEffect(c)
    let e2a = effect::create_effect(d, c);
    // e2a:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e2a, EFFECT_TYPE_FIELD);
    // e2a:SetProperty(EFFECT_FLAG_SET_AVAILABLE)
    effect::set_property(d, e2a, EFFECT_FLAG_SET_AVAILABLE, 0);
    // e2a:SetCode(EFFECT_INDESTRUCTABLE_EFFECT)
    effect::set_code(d, e2a, EFFECT_INDESTRUCTABLE_EFFECT);
    // e2a:SetRange(LOCATION_MZONE)
    effect::set_range(d, e2a, LOCATION_MZONE);
    // e2a:SetTargetRange(LOCATION_ONFIELD,0)
    effect::set_target_range(d, e2a, LOCATION_ONFIELD, 0);
    // e2a:SetTarget(function(e,c) return c:IsSpellTrap() end)
    effect::set_target(d, e2a, Some(effect_card_fn(|d, _e, c| card::is_spell_trap(d, c))));
    // e2a:SetValue(aux.indoval)
    effect::set_value_fn(d, e2a, aux::indoval());
    // c:RegisterEffect(e2a)
    card::register_effect(d, c, e2a, false);
    // local e2b=e2a:Clone()
    let e2b = effect::clone(d, e2a);
    // e2b:SetProperty(EFFECT_FLAG_PLAYER_TARGET)
    effect::set_property(d, e2b, EFFECT_FLAG_PLAYER_TARGET, 0);
    // e2b:SetCode(EFFECT_CANNOT_REMOVE)
    effect::set_code(d, e2b, EFFECT_CANNOT_REMOVE);
    // e2b:SetTargetRange(0,1)
    effect::set_target_range(d, e2b, 0, 1);
    // e2b:SetTarget(function(e,c,tp,r) return c:IsSpellTrap() and c:IsControler(e:GetHandlerPlayer()) and c:IsOnField() and r==REASON_EFFECT end)
    effect::set_target(d, e2b, Some(lua_fn(e2b_target)));
    // e2b:SetValue(1)
    effect::set_value(d, e2b, 1);
    // c:RegisterEffect(e2b)
    card::register_effect(d, c, e2b, false);
    // local e3=Effect.CreateEffect(c)
    let e3 = effect::create_effect(d, c);
    // e3:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e3, aux::stringid(ID, 0));
    // e3:SetCategory(CATEGORY_TOHAND)
    effect::set_category(d, e3, CATEGORY_TOHAND);
    // e3:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e3, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_O);
    // e3:SetProperty(EFFECT_FLAG_DELAY)
    effect::set_property(d, e3, EFFECT_FLAG_DELAY, 0);
    // e3:SetCode(EVENT_SPSUMMON_SUCCESS)
    effect::set_code(d, e3, EVENT_SPSUMMON_SUCCESS);
    // e3:SetCountLimit(1,{id,0})
    effect::set_count_limit(d, e3, 1, ID, 0, 0);
    // e3:SetTarget(s.thtg)
    effect::set_target(d, e3, Some(cost_target(thtg)));
    // e3:SetOperation(s.thop)
    effect::set_operation(d, e3, Some(operation(thop)));
    // c:RegisterEffect(e3)
    card::register_effect(d, c, e3, false);
    // local e4=Effect.CreateEffect(c)
    let e4 = effect::create_effect(d, c);
    // e4:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e4, aux::stringid(ID, 1));
    // e4:SetCategory(CATEGORY_REMOVE)
    effect::set_category(d, e4, CATEGORY_REMOVE);
    // e4:SetType(EFFECT_TYPE_IGNITION)
    effect::set_type(d, e4, EFFECT_TYPE_IGNITION);
    // e4:SetProperty(EFFECT_FLAG_CARD_TARGET)
    effect::set_property(d, e4, EFFECT_FLAG_CARD_TARGET, 0);
    // e4:SetRange(LOCATION_MZONE)
    effect::set_range(d, e4, LOCATION_MZONE);
    // e4:SetCountLimit(1,{id,1})
    effect::set_count_limit(d, e4, 1, ID, 1, 0);
    // e4:SetTarget(s.bantg)
    effect::set_target(d, e4, Some(cost_target(bantg)));
    // e4:SetOperation(s.banop)
    effect::set_operation(d, e4, Some(operation(banop)));
    // c:RegisterEffect(e4)
    card::register_effect(d, c, e4, false);
}
/// `s.immval(e,re)`
fn immval(d: &mut Duel, a: Args) -> bool {
    let e = a.e(0);
    let re = a.v(1).as_effect().nil();
    // if not (re:IsActivated() and e:GetOwnerPlayer()==1-re:GetOwnerPlayer()) then return false end
    if !(effect::is_activated(d, re) && effect::get_owner_player(d, e) == 1 - effect::get_owner_player(d, re)) {
        return false;
    }
    // if not re:IsHasProperty(EFFECT_FLAG_CARD_TARGET) then return true end
    if !effect::is_has_property(d, re, EFFECT_FLAG_CARD_TARGET, 0) {
        return true;
    }
    // local tg=Duel.GetChainInfo(0,CHAININFO_TARGET_CARDS)
    let tg = duel::get_chain_info(d, 0, &[CHAININFO_TARGET_CARDS]);
    // return not (tg and tg:IsContains(e:GetHandler()))
    let h = effect::get_handler(d, e).nil();
    !tg.first().and_then(|v| v.as_group().cloned()).is_some_and(|g| g.contains(h))
}
/// `function(e,c,tp,r) return c:IsSpellTrap() and c:IsControler(e:GetHandlerPlayer()) and c:IsOnField() and r==REASON_EFFECT end`
fn e2b_target(d: &mut Duel, a: Args) -> bool {
    let c = a.c(1);
    let r = a.int(3);
    card::is_spell_trap(d, c)
        && card::is_controler(d, c, effect::get_handler_player(d, a.e(0)))
        && card::is_on_field(d, c)
        && r == REASON_EFFECT as i64
}
/// `s.thfilter(c)`
fn thfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsSpell() and c:IsAbleToHand()
    card::is_spell(d, c) && card::is_able_to_hand(d, c, None)
}
/// `s.thtg(e,tp,...,chk)`
fn thtg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return Duel.IsExistingMatchingCard(s.thfilter,tp,LOCATION_GRAVE,0,1,nil) end
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, Some(&thfilter), a.tp, LOCATION_GRAVE, 0, 1, NO_EX);
    }
    // Duel.SetOperationInfo(0,CATEGORY_TOHAND,nil,1,tp,LOCATION_GRAVE)
    duel::set_operation_info(d, 0, CATEGORY_TOHAND, None, 1, a.tp, LOCATION_GRAVE as i32);
    true
}
/// `s.thop(e,tp,...)`
fn thop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_ATOHAND)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_ATOHAND as u64);
    // local g=Duel.SelectMatchingCard(tp,s.thfilter,tp,LOCATION_GRAVE,0,1,1,nil)
    let g = duel::select_matching_card(d, tp, Some(&thfilter), tp, LOCATION_GRAVE, 0, 1, 1, false, NO_EX).nil();
    // if #g>0 then
    if !g.is_empty() {
        // Duel.HintSelection(g)
        duel::hint_selection(d, g.clone(), true);
        // Duel.SendtoHand(g,nil,REASON_EFFECT)
        duel::send_to_hand(d, g, None, REASON_EFFECT, None);
    }
}
/// `s.bantg(e,tp,...,chk,chkc)`
fn bantg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // if chkc then return chkc:IsControler(1-tp) and chkc:IsOnField() and chkc:IsAbleToRemove(tp,POS_FACEDOWN) end
    if let Some(chkc) = a.chkc() {
        return card::is_controler(d, chkc, 1 - tp)
            && card::is_on_field(d, chkc)
            && card::is_able_to_remove(d, chkc, Some(tp), Some(POS_FACEDOWN as u8), None);
    }
    // if chk==0 then return Duel.IsExistingTarget(Card.IsAbleToRemove,tp,0,LOCATION_ONFIELD,1,nil,tp,POS_FACEDOWN) end
    if a.chk == Some(0) {
        return duel::is_existing_target(
            d,
            Some(&|d, c| card::is_able_to_remove(d, c, Some(tp), Some(POS_FACEDOWN as u8), None)),
            tp,
            0,
            LOCATION_ONFIELD,
            1,
            NO_EX,
        );
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_REMOVE)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_REMOVE as u64);
    // local g=Duel.SelectTarget(tp,Card.IsAbleToRemove,tp,0,LOCATION_ONFIELD,1,1,nil,tp,POS_FACEDOWN)
    let g = duel::select_target(
        d,
        tp,
        Some(&|d, c| card::is_able_to_remove(d, c, Some(tp), Some(POS_FACEDOWN as u8), None)),
        tp,
        0,
        LOCATION_ONFIELD,
        1,
        1,
        false,
        NO_EX,
    );
    // Duel.SetOperationInfo(0,CATEGORY_REMOVE,g,1,tp,0)
    duel::set_operation_info(d, 0, CATEGORY_REMOVE, g, 1, tp, 0);
    true
}
/// `s.banop(e,tp,...)`
fn banop(d: &mut Duel, a: &EffArgs) {
    // local tc=Duel.GetFirstTarget()
    let tc = duel::first_target(d).nil();
    // if tc:IsRelateToEffect(e) then
    if card::is_relate_to_effect(d, tc, a.e) {
        // Duel.Remove(tc,POS_FACEDOWN,REASON_EFFECT)
        duel::remove(d, CardSet::from_card(tc), POS_FACEDOWN as u8, REASON_EFFECT, None, None);
    }
}
