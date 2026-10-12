//! Reindsurm Encounter
//! scripted by Naim
use super::prelude::*;

pub const ID: u32 = 101403076;

pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_REINDSURM], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // When a Spell/Trap Card, or monster effect, is activated while you control a "Reindsurm" monster: Negate the activation, then you can add 1 "Reindsurm" monster from your Deck or GY to your hand, and if you do, send 1 Insect monster you control to the GY
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_NEGATE+CATEGORY_TOHAND+CATEGORY_SEARCH+CATEGORY_TOGRAVE)
    effect::set_category(d, e1, CATEGORY_NEGATE + CATEGORY_TOHAND + CATEGORY_SEARCH + CATEGORY_TOGRAVE);
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    // e1:SetCode(EVENT_CHAINING)
    effect::set_code(d, e1, EVENT_CHAINING);
    // e1:SetCountLimit(1,{id,0})
    effect::set_count_limit(d, e1, 1, ID, 0, 0);
    // e1:SetCondition(s.negcon)
    effect::set_condition(d, e1, Some(condition(negcon)));
    // e1:SetTarget(s.negtg)
    effect::set_target(d, e1, Some(cost_target(negtg)));
    // e1:SetOperation(s.negop)
    effect::set_operation(d, e1, Some(operation(negop)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // If a "Reindsurm" monster(s) you control would be destroyed by battle or card effect, you can place this card from your GY on the bottom of the Deck instead
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_CONTINUOUS)
    effect::set_type(d, e2, EFFECT_TYPE_FIELD + EFFECT_TYPE_CONTINUOUS);
    // e2:SetCode(EFFECT_DESTROY_REPLACE)
    effect::set_code(d, e2, EFFECT_DESTROY_REPLACE);
    // e2:SetRange(LOCATION_GRAVE)
    effect::set_range(d, e2, LOCATION_GRAVE);
    // e2:SetCountLimit(1,{id,1})
    effect::set_count_limit(d, e2, 1, ID, 1, 0);
    // e2:SetTarget(s.reptg)
    effect::set_target(d, e2, Some(cost_target(reptg)));
    // e2:SetValue(s.repval)
    effect::set_value_fn(d, e2, effect_card_fn(|d, e, c| {
        let p = effect::get_handler_player(d, e);
        repfilter(d, c, p)
    }));
    // e2:SetOperation(s.repop)
    effect::set_operation(d, e2, Some(operation(repop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
}
/// `s.negcon(e,tp,eg,ep,ev,re,r,rp)`
fn negcon(d: &mut Duel, a: &EffArgs) -> bool {
    // return Duel.IsExistingMatchingCard(aux.FaceupFilter(Card.IsSetCard,SET_REINDSURM),tp,LOCATION_MZONE,0,1,nil)
    //     and (re:IsMonsterEffect() or re:IsHasType(EFFECT_TYPE_ACTIVATE)) and Chain.IsNegatable(ev)
    duel::is_existing_matching_card(d, Some(&|d, c| card::is_faceup(d, c) && card::is_set_card(d, c, &[SET_REINDSURM as u16])), a.tp, LOCATION_MZONE, 0, 1, NO_EX)
        && (effect::is_monster_effect(d, a.re.nil()) || effect::is_has_type(d, a.re.nil(), EFFECT_TYPE_ACTIVATE))
        && duel::is_chain_negatable(d, a.ev as u8)
}
/// `s.negtg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn negtg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return true end
    if a.chk == Some(0) {
        return true;
    }
    // Duel.SetOperationInfo(0,CATEGORY_NEGATE,eg,1,tp,0)
    duel::set_operation_info(d, 0, CATEGORY_NEGATE, a.eg.clone(), 1, a.tp, 0);
    // Duel.SetPossibleOperationInfo(0,CATEGORY_TOHAND,nil,1,tp,LOCATION_DECK|LOCATION_GRAVE)
    duel::set_possible_operation_info(d, 0, CATEGORY_TOHAND, None, 1, a.tp, (LOCATION_DECK | LOCATION_GRAVE) as i32);
    // Duel.SetPossibleOperationInfo(0,CATEGORY_TOGRAVE,nil,1,tp,LOCATION_MZONE)
    duel::set_possible_operation_info(d, 0, CATEGORY_TOGRAVE, None, 1, a.tp, LOCATION_MZONE as i32);
    true
}
/// `s.thfilter(c)`
fn thfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsSetCard(SET_REINDSURM) and c:IsMonster() and c:IsAbleToHand()
    card::is_set_card(d, c, &[SET_REINDSURM as u16]) && card::is_monster(d, c) && card::is_able_to_hand(d, c, None)
}
/// `s.tgfilter(c)`
fn tgfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsRace(RACE_INSECT) and c:IsFaceup() and c:IsAbleToGrave()
    card::is_race(d, c, RACE_INSECT) && card::is_faceup(d, c) && card::is_able_to_grave(d, c, None)
}
/// `s.negop(e,tp,eg,ep,ev,re,r,rp)`
fn negop(d: &mut Duel, a: &EffArgs) {
    // if Duel.NegateActivation(ev) and Duel.IsExistingMatchingCard(aux.NecroValleyFilter(s.thfilter),tp,LOCATION_DECK|LOCATION_GRAVE,0,1,nil)
    //     and Duel.IsExistingMatchingCard(s.tgfilter,tp,LOCATION_MZONE,0,1,nil)
    //     and Duel.SelectYesNo(tp,aux.Stringid(id,1)) then
    if duel::negate_activation(d, a.ev as u8)
        && duel::is_existing_matching_card(d, Some(&aux::necro_valley_filter(|d, c| thfilter(d, c))), a.tp, LOCATION_DECK | LOCATION_GRAVE, 0, 1, NO_EX)
        && duel::is_existing_matching_card(d, Some(&tgfilter), a.tp, LOCATION_MZONE, 0, 1, NO_EX)
        && duel::select_yes_no(d, a.tp, aux::stringid(ID, 1))
    {
        // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_ATOHAND)
        duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_ATOHAND as u64);
        // local thc=Duel.SelectMatchingCard(tp,aux.NecroValleyFilter(s.thfilter),tp,LOCATION_DECK|LOCATION_GRAVE,0,1,1,nil):GetFirst()
        let g = duel::select_matching_card(d, a.tp, Some(&aux::necro_valley_filter(|d, c| thfilter(d, c))), a.tp, LOCATION_DECK | LOCATION_GRAVE, 0, 1, 1, false, NO_EX).nil();
        // if not thc then return end
        let Some(thc) = group::get_first(d, &g) else { return };
        // if thc:IsLocation(LOCATION_GRAVE) then Duel.HintSelection(thc) end
        if card::is_location(d, thc, LOCATION_GRAVE) {
            duel::hint_selection(d, thc, true);
        }
        // Duel.BreakEffect()
        duel::break_effect(d);
        // if Duel.SendtoHand(thc,nil,REASON_EFFECT)>0 and thc:IsLocation(LOCATION_HAND) then
        if duel::send_to_hand(d, thc, None, REASON_EFFECT, None) > 0 && card::is_location(d, thc, LOCATION_HAND) {
            // if thc:IsPreviousLocation(LOCATION_DECK) then Duel.ConfirmCards(1-tp,thc) end
            if card::is_previous_location(d, thc, LOCATION_DECK) {
                duel::confirm_cards(d, 1 - a.tp, thc);
            }
            // Duel.ShuffleHand(tp)
            duel::shuffle_hand(d, a.tp);
            // if thc:IsPreviousLocation(LOCATION_DECK) then Duel.ShuffleDeck(tp) end
            if card::is_previous_location(d, thc, LOCATION_DECK) {
                duel::shuffle_deck(d, a.tp);
            }
            // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_TOGRAVE)
            duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_TOGRAVE as u64);
            // local tg=Duel.SelectMatchingCard(tp,s.tgfilter,tp,LOCATION_MZONE,0,1,1,nil)
            let tg = duel::select_matching_card(d, a.tp, Some(&tgfilter), a.tp, LOCATION_MZONE, 0, 1, 1, false, NO_EX).nil();
            // if #tg>0 then
            if !tg.is_empty() {
                // Duel.HintSelection(tg)
                duel::hint_selection(d, tg.clone(), true);
                // Duel.SendtoGrave(tg,REASON_EFFECT)
                duel::send_to_grave(d, tg, REASON_EFFECT, None, None);
            }
        }
    }
}
/// `s.repfilter(c,tp)`
fn repfilter(d: &mut Duel, c: CardId, tp: u8) -> bool {
    // return c:IsSetCard(SET_REINDSURM) and c:IsLocation(LOCATION_MZONE) and c:IsControler(tp)
    //     and c:IsReason(REASON_BATTLE|REASON_EFFECT) and not c:IsReason(REASON_REPLACE)
    card::is_set_card(d, c, &[SET_REINDSURM as u16])
        && card::is_location(d, c, LOCATION_MZONE)
        && card::is_controler(d, c, tp)
        && card::is_reason(d, c, REASON_BATTLE | REASON_EFFECT)
        && !card::is_reason(d, c, REASON_REPLACE)
}
/// `s.reptg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn reptg(d: &mut Duel, a: &EffArgs) -> bool {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if chk==0 then return eg:IsExists(s.repfilter,1,nil,tp) and c:IsAbleToDeck() and aux.nvfilter(c) end
    if a.chk == Some(0) {
        return group::is_exists(d, &a.eg.clone().nil(), &|d, x| repfilter(d, x, a.tp), 1, NO_EX) && card::is_able_to_deck(d, c) && aux::nvfilter(d, c);
    }
    // return Duel.SelectEffectYesNo(tp,c,96)
    duel::select_effect_yes_no(d, a.tp, c, Some(96))
}
/// `s.repop(e,tp,eg,ep,ev,re,r,rp)`
fn repop(d: &mut Duel, a: &EffArgs) {
    // Duel.Hint(HINT_CARD,0,id)
    duel::hint(d, HINT_CARD as u8, 0, ID as u64);
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // Duel.HintSelection(c)
    duel::hint_selection(d, c, true);
    // Duel.SendtoDeck(c,nil,SEQ_DECKBOTTOM,REASON_EFFECT|REASON_REPLACE)
    duel::send_to_deck(d, c, None, SEQ_DECKBOTTOM as i32, REASON_EFFECT | REASON_REPLACE, None);
}
