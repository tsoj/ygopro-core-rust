//! Noble Knight Borz
use super::prelude::*;

pub const ID: u32 = 47120245;
pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_NOBLE_ARMS], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Normal monster
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
    // e1:SetProperty(EFFECT_FLAG_SINGLE_RANGE)
    effect::set_property(d, e1, EFFECT_FLAG_SINGLE_RANGE, 0);
    // e1:SetCode(EFFECT_ADD_TYPE)
    effect::set_code(d, e1, EFFECT_ADD_TYPE);
    // e1:SetRange(LOCATION_MZONE)
    effect::set_range(d, e1, LOCATION_MZONE);
    // e1:SetCondition(s.eqcon1)
    effect::set_condition(d, e1, Some(lua_fn(eqcon1)));
    // e1:SetValue(TYPE_NORMAL)
    effect::set_value(d, e1, TYPE_NORMAL as i64);
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // local e2=e1:Clone()
    let e2 = effect::clone(d, e1);
    // e2:SetCode(EFFECT_REMOVE_TYPE)
    effect::set_code(d, e2, EFFECT_REMOVE_TYPE);
    // e2:SetValue(TYPE_EFFECT)
    effect::set_value(d, e2, TYPE_EFFECT as i64);
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
    // Attribute Dark
    // local e3=Effect.CreateEffect(c)
    let e3 = effect::create_effect(d, c);
    // e3:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e3, EFFECT_TYPE_SINGLE);
    // e3:SetProperty(EFFECT_FLAG_SINGLE_RANGE)
    effect::set_property(d, e3, EFFECT_FLAG_SINGLE_RANGE, 0);
    // e3:SetCode(EFFECT_CHANGE_ATTRIBUTE)
    effect::set_code(d, e3, EFFECT_CHANGE_ATTRIBUTE);
    // e3:SetRange(LOCATION_MZONE)
    effect::set_range(d, e3, LOCATION_MZONE);
    // e3:SetCondition(s.eqcon2)
    effect::set_condition(d, e3, Some(lua_fn(eqcon2)));
    // e3:SetValue(ATTRIBUTE_DARK)
    effect::set_value(d, e3, ATTRIBUTE_DARK as i64);
    // c:RegisterEffect(e3)
    card::register_effect(d, c, e3, false);
    // local e4=e3:Clone()
    let e4 = effect::clone(d, e3);
    // e4:SetCode(EFFECT_UPDATE_LEVEL)
    effect::set_code(d, e4, EFFECT_UPDATE_LEVEL);
    // e4:SetValue(1)
    effect::set_value(d, e4, 1);
    // c:RegisterEffect(e4)
    card::register_effect(d, c, e4, false);
    // search
    // local e5=Effect.CreateEffect(c)
    let e5 = effect::create_effect(d, c);
    // e5:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e5, aux::stringid(ID, 0));
    // e5:SetCategory(CATEGORY_TOHAND+CATEGORY_SEARCH)
    effect::set_category(d, e5, CATEGORY_TOHAND + CATEGORY_SEARCH);
    // e5:SetType(EFFECT_TYPE_IGNITION)
    effect::set_type(d, e5, EFFECT_TYPE_IGNITION);
    // e5:SetRange(LOCATION_MZONE)
    effect::set_range(d, e5, LOCATION_MZONE);
    // e5:SetCountLimit(1,id)
    effect::set_count_limit(d, e5, 1, ID, 0, 0);
    // e5:SetCondition(s.thcon)
    effect::set_condition(d, e5, Some(condition(thcon)));
    // e5:SetTarget(s.thtg)
    effect::set_target(d, e5, Some(cost_target(thtg)));
    // e5:SetOperation(s.thop)
    effect::set_operation(d, e5, Some(operation(thop)));
    // c:RegisterEffect(e5)
    card::register_effect(d, c, e5, false);
}
// e:GetHandler():GetEquipGroup():IsExists(Card.IsSetCard,1,nil,SET_NOBLE_ARMS)
fn has_noble_arms(d: &mut Duel, e: EffectId) -> bool {
    let h = effect::get_handler(d, e).nil();
    let eq = card::get_equip_group(d, h);
    group::is_exists(d, &eq, &|d, c| card::is_set_card(d, c, &[SET_NOBLE_ARMS as u16]), 1, NO_EX)
}
// function s.eqcon1(e) return not e:GetHandler():GetEquipGroup():IsExists(...) end
fn eqcon1(d: &mut Duel, a: Args) -> bool {
    !has_noble_arms(d, a.e(0))
}
// function s.eqcon2(e) return e:GetHandler():GetEquipGroup():IsExists(...) end
fn eqcon2(d: &mut Duel, a: Args) -> bool {
    has_noble_arms(d, a.e(0))
}
// function s.thcon(e,tp,eg,ep,ev,re,r,rp) return s.eqcon2(e) end
fn thcon(d: &mut Duel, a: &EffArgs) -> bool {
    has_noble_arms(d, a.e)
}
// function s.thfilter(c)
fn thfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsSetCard(SET_NOBLE_ARMS) and c:IsAbleToHand()
    card::is_set_card(d, c, &[SET_NOBLE_ARMS as u16]) && card::is_able_to_hand(d, c, None)
}
// function s.thtg(e,tp,eg,ep,ev,re,r,rp,chk)
fn thtg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return Duel.IsExistingMatchingCard(s.thfilter,tp,LOCATION_DECK,0,3,nil) end
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, Some(&thfilter), a.tp, LOCATION_DECK, 0, 3, NO_EX);
    }
    // Duel.SetOperationInfo(0,CATEGORY_TOHAND,nil,1,0,LOCATION_DECK)
    duel::set_operation_info(d, 0, CATEGORY_TOHAND, None, 1, 0, LOCATION_DECK as i32);
    true
}
// function s.thop(e,tp,eg,ep,ev,re,r,rp)
fn thop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // local g=Duel.GetMatchingGroup(Card.IsSetCard,tp,LOCATION_DECK,0,nil,SET_NOBLE_ARMS)
    let g = duel::get_matching_group(d, Some(&|d, c| card::is_set_card(d, c, &[SET_NOBLE_ARMS as u16])), tp, LOCATION_DECK, 0, NO_EX);
    // if #g>=3 then
    if g.len() >= 3 {
        // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_ATOHAND)
        duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_ATOHAND as u64);
        // local sg=g:Select(tp,3,3,nil)
        let mut sg = group::select(d, &g, tp, 3, 3, false, NO_EX).nil();
        // Duel.ConfirmCards(1-tp,sg)
        duel::confirm_cards(d, 1 - tp, &sg);
        // Duel.ShuffleDeck(tp)
        duel::shuffle_deck(d, tp);
        // local tg=sg:RandomSelect(1-tp,1)
        let tg = group::random_select(d, &sg, 1 - tp, 1);
        // local tc=tg:GetFirst()
        let tc = group::get_first(d, &tg).nil();
        // if tc:IsAbleToHand() then
        if card::is_able_to_hand(d, tc, None) {
            // Duel.SendtoHand(tc,nil,REASON_EFFECT)
            duel::send_to_hand(d, tc, None, REASON_EFFECT, None);
            // sg:RemoveCard(tc)
            group::remove_card(&mut sg, &CardSet::from_card(tc));
        }
        // Duel.SendtoGrave(sg,REASON_EFFECT)
        duel::send_to_grave(d, &sg, REASON_EFFECT, None, None);
    }
}
