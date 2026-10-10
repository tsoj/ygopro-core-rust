//! Wightbaking
use super::prelude::*;

pub const ID: u32 = 6128460;
pub const INFO: ScriptInfo = ScriptInfo { listed_names: &[CARD_SKULL_SERVANT, ID], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Name becomes "Skull Servant" while in GY
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
    // e1:SetProperty(EFFECT_FLAG_SINGLE_RANGE)
    effect::set_property(d, e1, EFFECT_FLAG_SINGLE_RANGE, 0);
    // e1:SetCode(EFFECT_CHANGE_CODE)
    effect::set_code(d, e1, EFFECT_CHANGE_CODE);
    // e1:SetRange(LOCATION_GRAVE)
    effect::set_range(d, e1, LOCATION_GRAVE);
    // e1:SetValue(CARD_SKULL_SERVANT)
    effect::set_value(d, e1, CARD_SKULL_SERVANT as i64);
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // Substitute destruction for a level 3 or lower zombie monster(s)
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_CONTINUOUS)
    effect::set_type(d, e2, EFFECT_TYPE_FIELD + EFFECT_TYPE_CONTINUOUS);
    // e2:SetCode(EFFECT_DESTROY_REPLACE)
    effect::set_code(d, e2, EFFECT_DESTROY_REPLACE);
    // e2:SetRange(LOCATION_HAND)
    effect::set_range(d, e2, LOCATION_HAND);
    // e2:SetTarget(s.reptg)
    effect::set_target(d, e2, Some(cost_target(reptg)));
    // e2:SetValue(s.repval)
    effect::set_value_fn(d, e2, effect_card_fn(repval));
    // e2:SetOperation(s.repop)
    effect::set_operation(d, e2, Some(operation(repop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
    // Add 2 "Skull Servant"/monsters that lists "Skull Servant" with different names from the Deck, then discard 1
    // local e3=Effect.CreateEffect(c)
    let e3 = effect::create_effect(d, c);
    // e3:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e3, aux::stringid(ID, 0));
    // e3:SetCategory(CATEGORY_TOHAND+CATEGORY_SEARCH+CATEGORY_HANDES)
    effect::set_category(d, e3, CATEGORY_TOHAND + CATEGORY_SEARCH + CATEGORY_HANDES);
    // e3:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e3, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_O);
    // e3:SetCode(EVENT_TO_GRAVE)
    effect::set_code(d, e3, EVENT_TO_GRAVE);
    // e3:SetProperty(EFFECT_FLAG_DAMAGE_STEP+EFFECT_FLAG_DELAY)
    effect::set_property(d, e3, EFFECT_FLAG_DAMAGE_STEP + EFFECT_FLAG_DELAY, 0);
    // e3:SetCountLimit(1,{id,1})
    effect::set_count_limit(d, e3, 1, ID, 1, 0);
    // e3:SetTarget(s.thtg)
    effect::set_target(d, e3, Some(cost_target(thtg)));
    // e3:SetOperation(s.thop)
    effect::set_operation(d, e3, Some(operation(thop)));
    // c:RegisterEffect(e3)
    card::register_effect(d, c, e3, false);
}
/// `s.repfilter(c,tp)`: Check for level 3 or lower zombie monsters
fn repfilter(d: &mut Duel, c: CardId, tp: u8) -> bool {
    // return c:IsFaceup() and c:IsRace(RACE_ZOMBIE) and c:IsLevelBelow(3) and c:IsControler(tp)
    //     and not c:IsReason(REASON_REPLACE) and c:IsReason(REASON_EFFECT|REASON_BATTLE)
    card::is_faceup(d, c)
        && card::is_race(d, c, RACE_ZOMBIE)
        && card::is_level_below(d, c, 3)
        && card::is_controler(d, c, tp)
        && !card::is_reason(d, c, REASON_REPLACE)
        && card::is_reason(d, c, REASON_EFFECT | REASON_BATTLE)
}
/// `s.reptg(e,tp,eg,ep,ev,re,r,rp,chk)`: Activation legality
fn reptg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return e:GetHandler():IsDiscardable() and eg:IsExists(s.repfilter,1,nil,tp) end
    if a.chk == Some(0) {
        let h = effect::get_handler(d, a.e).nil();
        return card::is_discardable(d, h, None, None) && group::is_exists(d, &a.eg.clone().nil(), &|d, c| repfilter(d, c, a.tp), 1, NO_EX);
    }
    // return Duel.SelectEffectYesNo(tp,e:GetHandler(),96)
    let h = effect::get_handler(d, a.e).nil();
    duel::select_effect_yes_no(d, a.tp, h, Some(96))
}
/// `s.repval(e,c)`
fn repval(d: &mut Duel, e: EffectId, c: CardId) -> bool {
    // return s.repfilter(c,e:GetHandlerPlayer())
    let p = effect::get_handler_player(d, e);
    repfilter(d, c, p)
}
/// `s.repop(e,tp,eg,ep,ev,re,r,rp)`: Discard itself as substitute for a level 3 or lower zombie monster's destruction
fn repop(d: &mut Duel, a: &EffArgs) {
    // Duel.SendtoGrave(e:GetHandler(),REASON_EFFECT+REASON_DISCARD+REASON_REPLACE)
    let h = effect::get_handler(d, a.e).nil();
    duel::send_to_grave(d, CardSet::from_card(h), REASON_EFFECT | REASON_DISCARD | REASON_REPLACE, None, None);
}
/// `s.thfilter(c)`: Check for "Skull Servant" or a monster that lists "Skull Servant"
fn thfilter(d: &mut Duel, c: CardId) -> bool {
    // return not c:IsCode(id) and c:IsAbleToHand()
    //     and (c:IsCode(CARD_SKULL_SERVANT) or (c:IsMonster() and c:ListsCode(CARD_SKULL_SERVANT)))
    !card::is_code(d, c, &[ID])
        && card::is_able_to_hand(d, c, None)
        && (card::is_code(d, c, &[CARD_SKULL_SERVANT]) || (card::is_monster(d, c) && card::lists_code(d, c, &[CARD_SKULL_SERVANT])))
}
/// `s.thtg(e,tp,eg,ep,ev,re,r,rp,chk)`: Activation legality
fn thtg(d: &mut Duel, a: &EffArgs) -> bool {
    // local g=Duel.GetMatchingGroup(s.thfilter,tp,LOCATION_DECK,0,nil)
    let g = duel::get_matching_group(d, Some(&thfilter), a.tp, LOCATION_DECK, 0, NO_EX);
    // if chk==0 then return aux.SelectUnselectGroup(g,e,tp,2,2,aux.dncheck,0) end
    if a.chk == Some(0) {
        return matches!(
            aux::select_unselect_group(d, &g, Some(a.e), a.tp, Some(2), Some(2), Some(&aux::dncheck()), 0, a.tp, None, None, None, false),
            Err(true)
        );
    }
    // Duel.SetOperationInfo(0,CATEGORY_TOHAND,nil,2,tp,LOCATION_DECK)
    duel::set_operation_info(d, 0, CATEGORY_TOHAND, None, 2, a.tp, LOCATION_DECK as i32);
    // Duel.SetOperationInfo(0,CATEGORY_HANDES,nil,0,tp,1)
    duel::set_operation_info(d, 0, CATEGORY_HANDES, None, 0, a.tp, 1);
    true
}
/// `s.thop(e,tp,eg,ep,ev,re,r,rp)`: Add up to 2 "Skull Servant"/monsters that lists "Skull Servant" from deck, then discard 1
fn thop(d: &mut Duel, a: &EffArgs) {
    // local g=Duel.GetMatchingGroup(s.thfilter,tp,LOCATION_DECK,0,nil)
    let g = duel::get_matching_group(d, Some(&thfilter), a.tp, LOCATION_DECK, 0, NO_EX);
    // if #g>0 then
    if !g.is_empty() {
        // local sg=aux.SelectUnselectGroup(g,e,tp,2,2,aux.dncheck,1,tp,HINTMSG_ATOHAND)
        let sg = aux::select_unselect_group(d, &g, Some(a.e), a.tp, Some(2), Some(2), Some(&aux::dncheck()), 1, a.tp, Some(HINTMSG_ATOHAND as u64), None, None, false)
            .unwrap_or_default();
        // if #sg>0 and Duel.SendtoHand(sg,nil,REASON_EFFECT)~=0 then
        if !sg.is_empty() && duel::send_to_hand(d, sg.clone(), None, REASON_EFFECT, None) != 0 {
            // Duel.ConfirmCards(1-tp,sg)
            duel::confirm_cards(d, 1 - a.tp, sg);
            // Duel.ShuffleHand(tp)
            duel::shuffle_hand(d, a.tp);
            // Duel.BreakEffect()
            duel::break_effect(d);
            // Duel.DiscardHand(tp,aux.TRUE,1,1,REASON_EFFECT|REASON_DISCARD)
            duel::discard_hand(d, a.tp, None, 1, 1, REASON_EFFECT | REASON_DISCARD, NO_EX);
        }
    }
}
