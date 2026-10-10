//! Darkest Diabolos, Lord of the Lair
use super::prelude::*;

pub const ID: u32 = 50383626;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // cannot release
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
    // e1:SetProperty(EFFECT_FLAG_SINGLE_RANGE)
    effect::set_property(d, e1, EFFECT_FLAG_SINGLE_RANGE, 0);
    // e1:SetRange(LOCATION_MZONE)
    effect::set_range(d, e1, LOCATION_MZONE);
    // e1:SetCode(EFFECT_UNRELEASABLE_SUM)
    effect::set_code(d, e1, EFFECT_UNRELEASABLE_SUM);
    // e1:SetValue(s.sumlimit)
    effect::set_value_fn(d, e1, lua_fn(sumlimit));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e2, EFFECT_TYPE_FIELD);
    // e2:SetCode(EFFECT_CANNOT_RELEASE)
    effect::set_code(d, e2, EFFECT_CANNOT_RELEASE);
    // e2:SetProperty(EFFECT_FLAG_PLAYER_TARGET)
    effect::set_property(d, e2, EFFECT_FLAG_PLAYER_TARGET, 0);
    // e2:SetRange(LOCATION_MZONE)
    effect::set_range(d, e2, LOCATION_MZONE);
    // e2:SetTargetRange(0,1)
    effect::set_target_range(d, e2, 0, 1);
    // e2:SetTarget(s.relval)
    effect::set_target(d, e2, Some(effect_card_fn(relval)));
    // e2:SetValue(1)
    effect::set_value(d, e2, 1);
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
    // cannot be target
    // local e3=Effect.CreateEffect(c)
    let e3 = effect::create_effect(d, c);
    // e3:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e3, EFFECT_TYPE_SINGLE);
    // e3:SetCode(EFFECT_CANNOT_BE_EFFECT_TARGET)
    effect::set_code(d, e3, EFFECT_CANNOT_BE_EFFECT_TARGET);
    // e3:SetProperty(EFFECT_FLAG_SINGLE_RANGE)
    effect::set_property(d, e3, EFFECT_FLAG_SINGLE_RANGE, 0);
    // e3:SetRange(LOCATION_MZONE)
    effect::set_range(d, e3, LOCATION_MZONE);
    // e3:SetValue(aux.tgoval)
    effect::set_value_fn(d, e3, aux::tgoval());
    // c:RegisterEffect(e3)
    card::register_effect(d, c, e3, false);
    // spsummon
    // local e4=Effect.CreateEffect(c)
    let e4 = effect::create_effect(d, c);
    // e4:SetCategory(CATEGORY_SPECIAL_SUMMON)
    effect::set_category(d, e4, CATEGORY_SPECIAL_SUMMON);
    // e4:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e4, EFFECT_TYPE_FIELD + EFFECT_TYPE_TRIGGER_O);
    // e4:SetCode(EVENT_RELEASE)
    effect::set_code(d, e4, EVENT_RELEASE);
    // e4:SetProperty(EFFECT_FLAG_DELAY)
    effect::set_property(d, e4, EFFECT_FLAG_DELAY, 0);
    // e4:SetRange(LOCATION_HAND|LOCATION_GRAVE)
    effect::set_range(d, e4, LOCATION_HAND | LOCATION_GRAVE);
    // e4:SetCountLimit(1,id)
    effect::set_count_limit(d, e4, 1, ID, 0, 0);
    // e4:SetCondition(s.spcon)
    effect::set_condition(d, e4, Some(condition(spcon)));
    // e4:SetTarget(s.sptg)
    effect::set_target(d, e4, Some(cost_target(sptg)));
    // e4:SetOperation(s.spop)
    effect::set_operation(d, e4, Some(operation(spop)));
    // c:RegisterEffect(e4)
    card::register_effect(d, c, e4, false);
    // to deck
    // local e5=Effect.CreateEffect(c)
    let e5 = effect::create_effect(d, c);
    // e5:SetType(EFFECT_TYPE_IGNITION)
    effect::set_type(d, e5, EFFECT_TYPE_IGNITION);
    // e5:SetRange(LOCATION_MZONE)
    effect::set_range(d, e5, LOCATION_MZONE);
    // e5:SetCountLimit(1,{id,1})
    effect::set_count_limit(d, e5, 1, ID, 1, 0);
    // e5:SetCost(s.tdcost)
    effect::set_cost(d, e5, Some(cost_target(tdcost)));
    // e5:SetTarget(s.tdtg)
    effect::set_target(d, e5, Some(cost_target(tdtg)));
    // e5:SetOperation(s.tdop)
    effect::set_operation(d, e5, Some(operation(tdop)));
    // c:RegisterEffect(e5)
    card::register_effect(d, c, e5, false);
}
/// `s.sumlimit(e,c)`
fn sumlimit(d: &mut Duel, a: Args) -> bool {
    // if not c then return false end
    let Some(c) = a.card(1) else { return false };
    // return not c:IsControler(e:GetHandlerPlayer())
    let hp = effect::get_handler_player(d, a.e(0));
    !card::is_controler(d, c, hp)
}
/// `s.relval(e,c)`: `return c==e:GetHandler()`
fn relval(d: &mut Duel, e: EffectId, c: CardId) -> bool {
    effect::get_handler(d, e).nil() == c
}
/// `s.spcfilter(c,tp)`
fn spcfilter(d: &mut Duel, c: CardId, tp: u8) -> bool {
    // return c:IsPreviousControler(tp) and c:GetPreviousAttributeOnField()&ATTRIBUTE_DARK==ATTRIBUTE_DARK
    card::is_previous_controler(d, c, tp) && (card::get_previous_attribute_on_field(d, c) & ATTRIBUTE_DARK) == ATTRIBUTE_DARK
}
/// `s.spcon(e,tp,eg,ep,ev,re,r,rp)`
fn spcon(d: &mut Duel, a: &EffArgs) -> bool {
    // return eg:IsExists(s.spcfilter,1,nil,tp)
    group::is_exists(d, &a.eg.clone().nil(), &|d, c| spcfilter(d, c, a.tp), 1, NO_EX)
}
/// `s.sptg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn sptg(d: &mut Duel, a: &EffArgs) -> bool {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if chk==0 then return Duel.GetLocationCount(tp,LOCATION_MZONE)>0
    //     and c:IsCanBeSpecialSummoned(e,0,tp,false,false) and ((c:IsLocation(LOCATION_GRAVE) and not eg:IsContains(c))
    //     or (c:IsLocation(LOCATION_HAND))) end
    if a.chk == Some(0) {
        let grave_ok = card::is_location(d, c, LOCATION_GRAVE) && !group::is_contains(&a.eg.clone().nil(), c);
        return duel::get_location_count(d, a.tp, LOCATION_MZONE as u8, None, None, None).0 > 0
            && card::is_can_be_special_summoned(d, c, a.e, 0, a.tp, false, false, None, None, None)
            && (grave_ok || card::is_location(d, c, LOCATION_HAND));
    }
    // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,c,1,0,0)
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, Some(CardSet::from_card(c)), 1, 0, 0);
    true
}
/// `s.spop(e,tp,eg,ep,ev,re,r,rp)`
fn spop(d: &mut Duel, a: &EffArgs) {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if c:IsRelateToEffect(e) then
    if card::is_relate_to_effect(d, c, a.e) {
        // Duel.SpecialSummon(c,0,tp,tp,false,false,POS_FACEUP)
        duel::special_summon(d, c, 0, a.tp, a.tp, false, false, POS_FACEUP as u8, None);
    }
}
/// `s.tdcost(e,tp,eg,ep,ev,re,r,rp,chk)`
fn tdcost(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return Duel.CheckReleaseGroupCost(tp,Card.IsAttribute,1,false,nil,nil,ATTRIBUTE_DARK) end
    if a.chk == Some(0) {
        return duel::check_release_group_cost(d, a.tp, Some(&|d, x| card::is_attribute(d, x, ATTRIBUTE_DARK)), 1, 1, false, None, NO_EX);
    }
    // local g=Duel.SelectReleaseGroupCost(tp,Card.IsAttribute,1,1,false,nil,nil,ATTRIBUTE_DARK)
    let g = duel::select_release_group_cost(d, a.tp, Some(&|d, x| card::is_attribute(d, x, ATTRIBUTE_DARK)), 1, 1, false, None, NO_EX);
    // Duel.Release(g,REASON_COST)
    duel::release(d, g, REASON_COST, None);
    true
}
/// `s.tdtg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn tdtg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return Duel.IsExistingMatchingCard(Card.IsAbleToDeck,tp,0,LOCATION_HAND,1,nil) end
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, Some(&card::is_able_to_deck), a.tp, 0, LOCATION_HAND, 1, NO_EX);
    }
    true
}
/// `s.tdop(e,tp,eg,ep,ev,re,r,rp)`
fn tdop(d: &mut Duel, a: &EffArgs) {
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_TODECK)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_TODECK as u64);
    // local g=Duel.SelectMatchingCard(1-tp,Card.IsAbleToDeck,1-tp,LOCATION_HAND,0,1,1,nil)
    let g = duel::select_matching_card(d, 1 - a.tp, Some(&card::is_able_to_deck), 1 - a.tp, LOCATION_HAND, 0, 1, 1, false, NO_EX).nil();
    // if #g>0 then
    if group::get_count(&g) > 0 {
        // if Duel.SelectOption(1-tp,aux.Stringid(id,2),aux.Stringid(id,3))==0 then
        if duel::select_option(d, 1 - a.tp, true, &[aux::stringid(ID, 2), aux::stringid(ID, 3)]) == 0 {
            // Duel.SendtoDeck(g,nil,SEQ_DECKTOP,REASON_EFFECT)
            duel::send_to_deck(d, g, None, SEQ_DECKTOP as i32, REASON_EFFECT, None);
        } else {
            // Duel.SendtoDeck(g,nil,SEQ_DECKBOTTOM,REASON_EFFECT)
            duel::send_to_deck(d, g, None, SEQ_DECKBOTTOM as i32, REASON_EFFECT, None);
        }
    }
}
