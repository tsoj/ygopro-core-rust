//! The Man with the Mark
use super::prelude::*;

pub const ID: u32 = 97522863;

pub const INFO: ScriptInfo = ScriptInfo { listed_names: &[CARD_TEMPLE_OF_THE_KINGS], listed_series: &[SET_APOPHIS], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // --Take 1 "Temple of the Kings", or 1 Spell/Trap that mentions it, from your Deck and either add it to your hand or send it to the GY
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_TOHAND+CATEGORY_SEARCH+CATEGORY_TOGRAVE)
    effect::set_category(d, e1, CATEGORY_TOHAND + CATEGORY_SEARCH + CATEGORY_TOGRAVE);
    // e1:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_O);
    // e1:SetProperty(EFFECT_FLAG_DELAY)
    effect::set_property(d, e1, EFFECT_FLAG_DELAY, 0);
    // e1:SetCode(EVENT_SUMMON_SUCCESS)
    effect::set_code(d, e1, EVENT_SUMMON_SUCCESS);
    // e1:SetCountLimit(1,id)
    effect::set_count_limit(d, e1, 1, ID, 0, 0);
    // e1:SetTarget(s.thtgtg)
    effect::set_target(d, e1, Some(cost_target(thtgtg)));
    // e1:SetOperation(s.thtgop)
    effect::set_operation(d, e1, Some(operation(thtgop)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // local e2=e1:Clone()
    let e2 = effect::clone(d, e1);
    // e2:SetCode(EVENT_SPSUMMON_SUCCESS)
    effect::set_code(d, e2, EVENT_SPSUMMON_SUCCESS);
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
    // --While you control "Temple of the Kings", this card and "Apophis" monsters you control cannot be destroyed by battle or card effects
    // --"Temple of the Kings" you control cannot be destroyed by card effects
    // local e3=Effect.CreateEffect(c)
    let e3 = effect::create_effect(d, c);
    // e3:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e3, EFFECT_TYPE_FIELD);
    // e3:SetCode(EFFECT_INDESTRUCTABLE_BATTLE)
    effect::set_code(d, e3, EFFECT_INDESTRUCTABLE_BATTLE);
    // e3:SetRange(LOCATION_MZONE)
    effect::set_range(d, e3, LOCATION_MZONE);
    // e3:SetTargetRange(LOCATION_MZONE,0)
    effect::set_target_range(d, e3, LOCATION_MZONE, 0);
    // e3:SetCondition(function(e) return Duel.IsExistingMatchingCard(aux.FaceupFilter(Card.IsCode,CARD_TEMPLE_OF_THE_KINGS),e:GetHandlerPlayer(),LOCATION_ONFIELD,0,1,nil) end)
    effect::set_condition(d, e3, Some(lua_fn(temple_on_field)));
    // e3:SetTarget(function(e,c) return c==e:GetHandler() or c:IsSetCard(SET_APOPHIS) end)
    effect::set_target(d, e3, Some(effect_card_fn(|d, e, c| Some(c) == effect::get_handler(d, e) || card::is_set_card(d, c, &[SET_APOPHIS as u16]))));
    // e3:SetValue(1)
    effect::set_value(d, e3, 1);
    // c:RegisterEffect(e3)
    card::register_effect(d, c, e3, false);
    // local e4=e3:Clone()
    let e4 = effect::clone(d, e3);
    // e4:SetCode(EFFECT_INDESTRUCTABLE_EFFECT)
    effect::set_code(d, e4, EFFECT_INDESTRUCTABLE_EFFECT);
    // e4:SetTargetRange(LOCATION_ONFIELD,0)
    effect::set_target_range(d, e4, LOCATION_ONFIELD, 0);
    // e4:SetTarget(function(e,c) return c==e:GetHandler() or (c:IsSetCard(SET_APOPHIS) and c:IsMonster()) or c:IsCode(CARD_TEMPLE_OF_THE_KINGS) end)
    effect::set_target(
        d,
        e4,
        Some(effect_card_fn(|d, e, c| {
            Some(c) == effect::get_handler(d, e) || (card::is_set_card(d, c, &[SET_APOPHIS as u16]) && card::is_monster(d, c)) || card::is_code(d, c, &[CARD_TEMPLE_OF_THE_KINGS])
        })),
    );
    // c:RegisterEffect(e4)
    card::register_effect(d, c, e4, false);
}
// function(e) return Duel.IsExistingMatchingCard(aux.FaceupFilter(Card.IsCode,CARD_TEMPLE_OF_THE_KINGS),e:GetHandlerPlayer(),LOCATION_ONFIELD,0,1,nil) end
fn temple_on_field(d: &mut Duel, a: Args) -> bool {
    let p = effect::get_handler_player(d, a.e(0));
    duel::is_existing_matching_card(d, Some(&aux::faceup_filter(|d, c| card::is_code(d, c, &[CARD_TEMPLE_OF_THE_KINGS]))), p, LOCATION_ONFIELD, 0, 1, NO_EX)
}
// function s.thtgfilter(c)
fn thtgfilter(d: &mut Duel, c: CardId) -> bool {
    // return (c:IsCode(CARD_TEMPLE_OF_THE_KINGS) or (c:IsSpellTrap() and c:ListsCode(CARD_TEMPLE_OF_THE_KINGS))) and (c:IsAbleToHand() or c:IsAbleToGrave())
    (card::is_code(d, c, &[CARD_TEMPLE_OF_THE_KINGS]) || (card::is_spell_trap(d, c) && card::lists_code(d, c, &[CARD_TEMPLE_OF_THE_KINGS])))
        && (card::is_able_to_hand(d, c, None) || card::is_able_to_grave(d, c, None))
}
// function s.thtgtg(e,tp,eg,ep,ev,re,r,rp,chk)
fn thtgtg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return Duel.IsExistingMatchingCard(s.thtgfilter,tp,LOCATION_DECK,0,1,nil) end
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, Some(&thtgfilter), a.tp, LOCATION_DECK, 0, 1, NO_EX);
    }
    // Duel.SetPossibleOperationInfo(0,CATEGORY_TOHAND,nil,1,tp,LOCATION_DECK)
    duel::set_possible_operation_info(d, 0, CATEGORY_TOHAND, None, 1, a.tp, LOCATION_DECK as i32);
    // Duel.SetPossibleOperationInfo(0,CATEGORY_TOGRAVE,nil,1,tp,LOCATION_DECK)
    duel::set_possible_operation_info(d, 0, CATEGORY_TOGRAVE, None, 1, a.tp, LOCATION_DECK as i32);
    true
}
// function s.thtgop(e,tp,eg,ep,ev,re,r,rp)
fn thtgop(d: &mut Duel, a: &EffArgs) {
    // Duel.Hint(HINT_SELECTMSG,tp,aux.Stringid(id,1))
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, aux::stringid(ID, 1));
    // local g=Duel.SelectMatchingCard(tp,s.thtgfilter,tp,LOCATION_DECK,0,1,1,nil)
    let g = duel::select_matching_card(d, a.tp, Some(&thtgfilter), a.tp, LOCATION_DECK, 0, 1, 1, false, NO_EX).nil();
    // if #g>0 then
    if !g.is_empty() {
        // aux.ToHandOrElse(g,tp)
        aux::to_hand_or_else(d, g, a.tp, None, None, None);
    }
}
