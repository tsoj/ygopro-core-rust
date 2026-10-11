//! First Penguin
use super::prelude::*;

pub const ID: u32 = 12975671;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // If you control no face-up monsters, you can Special Summon this card (from your hand), and if you do, it is treated as a Tuner
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e1, EFFECT_TYPE_FIELD);
    // e1:SetProperty(EFFECT_FLAG_UNCOPYABLE)
    effect::set_property(d, e1, EFFECT_FLAG_UNCOPYABLE, 0);
    // e1:SetCode(EFFECT_SPSUMMON_PROC)
    effect::set_code(d, e1, EFFECT_SPSUMMON_PROC);
    // e1:SetRange(LOCATION_HAND)
    effect::set_range(d, e1, LOCATION_HAND);
    // e1:SetCondition(s.selfspcon)
    effect::set_condition(d, e1, Some(lua_fn(selfspcon)));
    // e1:SetOperation(s.selfspop)
    effect::set_operation(d, e1, Some(lua_fn(selfspop)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // Add 1 WATER monster with the same Type as the revealed monster, but 1 Level lower, from your Deck to your hand, then you can change this card to face-down Defense Position, also you cannot Special Summon from the Extra Deck for the rest of this turn, except WATER monsters
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e2, aux::stringid(ID, 1));
    // e2:SetCategory(CATEGORY_TOHAND+CATEGORY_SEARCH+CATEGORY_POSITION+CATEGORY_SET)
    effect::set_category(d, e2, CATEGORY_TOHAND + CATEGORY_SEARCH + CATEGORY_POSITION + CATEGORY_SET);
    // e2:SetType(EFFECT_TYPE_IGNITION)
    effect::set_type(d, e2, EFFECT_TYPE_IGNITION);
    // e2:SetRange(LOCATION_MZONE)
    effect::set_range(d, e2, LOCATION_MZONE);
    // e2:SetCountLimit(1,id)
    effect::set_count_limit(d, e2, 1, ID, 0, 0);
    // e2:SetCost(Cost.AND(Cost.Discard(),Cost.Reveal(s.revealfilter,false,1,1,function(e,tp,g) local sc=g:GetFirst() e:SetLabel(sc:GetRace(),sc:GetLevel()-1) end,LOCATION_EXTRA)))
    effect::set_cost(
        d,
        e2,
        Some(cost::and(vec![
            cost::discard(None, false, None, None, None),
            cost::reveal(
                Some(std::sync::Arc::new(|d, c, e, tp| revealfilter(d, c, e, tp))),
                false,
                Some(cost::Count::N(1)),
                Some(cost::Count::N(1)),
                Some(std::sync::Arc::new(|d, e, _tp, g| {
                    // local sc=g:GetFirst() e:SetLabel(sc:GetRace(),sc:GetLevel()-1)
                    let sc = group::get_first(d, g).nil();
                    let race = card::get_race(d, sc) as i64;
                    let lv = card::get_level(d, sc) as i64 - 1;
                    effect::set_label(d, e, &[race, lv]);
                })),
                Some(LOCATION_EXTRA),
            ),
        ])),
    );
    // e2:SetTarget(s.thtg)
    effect::set_target(d, e2, Some(cost_target(thtg)));
    // e2:SetOperation(s.thop)
    effect::set_operation(d, e2, Some(operation(thop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
}
/// `s.selfspcon(e,c)`
fn selfspcon(d: &mut Duel, a: Args) -> bool {
    // if c==nil then return true end
    if a.card(1).is_none() {
        return true;
    }
    // local tp=e:GetHandlerPlayer()
    let tp = effect::get_handler_player(d, a.e(0));
    // return Duel.GetLocationCount(tp,LOCATION_MZONE)>0 and not Duel.IsExistingMatchingCard(Card.IsFaceup,tp,LOCATION_MZONE,0,1,nil)
    duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 > 0
        && !duel::is_existing_matching_card(d, Some(&|d, c| card::is_faceup(d, c)), tp, LOCATION_MZONE, 0, 1, NO_EX)
}
/// `s.selfspop(e,tp,eg,ep,ev,re,r,rp,c)`
fn selfspop(d: &mut Duel, a: Args) {
    // local c=... (the 9th argument)
    let c = a.c(8);
    // It is treated as a Tuner
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
    // e1:SetProperty(EFFECT_FLAG_CANNOT_DISABLE)
    effect::set_property(d, e1, EFFECT_FLAG_CANNOT_DISABLE, 0);
    // e1:SetCode(EFFECT_ADD_TYPE)
    effect::set_code(d, e1, EFFECT_ADD_TYPE);
    // e1:SetValue(TYPE_TUNER)
    effect::set_value(d, e1, TYPE_TUNER as i64);
    // e1:SetReset(RESET_EVENT|(RESETS_STANDARD&~RESET_TOFIELD))
    effect::set_reset(d, e1, RESET_EVENT | (RESETS_STANDARD & !RESET_TOFIELD), 1);
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
/// `s.revealfilter(c,e,tp)`
fn revealfilter(d: &mut Duel, c: CardId, e: EffectId, tp: u8) -> bool {
    // return c:IsAttribute(ATTRIBUTE_WATER) and c:IsSynchroMonster() and not c:IsLevel(1) and not c:IsPublic()
    //     and Duel.IsExistingMatchingCard(s.thfilter,tp,LOCATION_DECK,0,1,nil,c:GetRace(),c:GetLevel()-1)
    let _ = e;
    if !(card::is_attribute(d, c, ATTRIBUTE_WATER) && card::is_synchro_monster(d, c) && !card::is_level(d, c, &[1]) && !card::is_public(d, c)) {
        return false;
    }
    let race = card::get_race(d, c);
    let lv = card::get_level(d, c) - 1;
    duel::is_existing_matching_card(d, Some(&|d, x| thfilter(d, x, race, lv)), tp, LOCATION_DECK, 0, 1, NO_EX)
}
/// `s.thfilter(c,race,lv)`
fn thfilter(d: &mut Duel, c: CardId, race: u64, lv: u32) -> bool {
    // return c:IsAttribute(ATTRIBUTE_WATER) and c:IsRace(race) and c:IsLevel(lv) and c:IsAbleToHand()
    card::is_attribute(d, c, ATTRIBUTE_WATER) && card::is_race(d, c, race) && card::is_level(d, c, &[lv]) && card::is_able_to_hand(d, c, None)
}
/// `s.thtg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn thtg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return true end
    if a.chk == Some(0) {
        return true;
    }
    // Duel.SetOperationInfo(0,CATEGORY_TOHAND,nil,1,tp,LOCATION_DECK)
    duel::set_operation_info(d, 0, CATEGORY_TOHAND, None, 1, a.tp, LOCATION_DECK as i32);
    // Duel.SetPossibleOperationInfo(0,CATEGORY_POSITION,e:GetHandler(),1,tp,POS_FACEDOWN_DEFENSE)
    let h = effect::get_handler(d, a.e).nil();
    duel::set_possible_operation_info(d, 0, CATEGORY_POSITION, Some(CardSet::from_card(h)), 1, a.tp, POS_FACEDOWN_DEFENSE as i32);
    true
}
/// `s.thop(e,tp,eg,ep,ev,re,r,rp)`
fn thop(d: &mut Duel, a: &EffArgs) {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // local race,lv=e:GetLabel()
    let labels = effect::get_labels(d, a.e);
    let race = labels.first().copied().unwrap_or(0) as u64;
    let lv = labels.get(1).copied().unwrap_or(0) as u32;
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_ATOHAND)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_ATOHAND as u64);
    // local sc=Duel.SelectMatchingCard(tp,s.thfilter,tp,LOCATION_DECK,0,1,1,nil,race,lv):GetFirst()
    let g = duel::select_matching_card(d, a.tp, Some(&|d, x| thfilter(d, x, race, lv)), a.tp, LOCATION_DECK, 0, 1, 1, false, NO_EX).nil();
    let sc = group::get_first(d, &g);
    // if sc and Duel.SendtoHand(sc,nil,REASON_EFFECT)>0 and sc:IsLocation(LOCATION_HAND) then
    if let Some(sc) = sc {
        if duel::send_to_hand(d, CardSet::from_card(sc), None, REASON_EFFECT, None) > 0 && card::is_location(d, sc, LOCATION_HAND) {
            // Duel.ConfirmCards(1-tp,sc)
            duel::confirm_cards(d, 1 - a.tp, CardSet::from_card(sc));
            // Duel.ShuffleHand(tp)
            duel::shuffle_hand(d, a.tp);
            // if c:IsRelateToEffect(e) and c:IsCanTurnSet() and Duel.SelectYesNo(tp,aux.Stringid(id,2)) then
            if card::is_relate_to_effect(d, c, a.e) && card::is_can_turn_set(d, c) && duel::select_yes_no(d, a.tp, aux::stringid(ID, 2)) {
                // Duel.BreakEffect()
                duel::break_effect(d);
                // Duel.ChangePosition(c,POS_FACEDOWN_DEFENSE)
                duel::change_position(d, c, POS_FACEDOWN_DEFENSE as u8, None, None, None, false);
            }
        }
    }
    // You cannot Special Summon from the Extra Deck for the rest of this turn, except WATER monsters
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,3))
    effect::set_description(d, e1, aux::stringid(ID, 3));
    // e1:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e1, EFFECT_TYPE_FIELD);
    // e1:SetProperty(EFFECT_FLAG_PLAYER_TARGET+EFFECT_FLAG_CLIENT_HINT)
    effect::set_property(d, e1, EFFECT_FLAG_PLAYER_TARGET + EFFECT_FLAG_CLIENT_HINT, 0);
    // e1:SetCode(EFFECT_CANNOT_SPECIAL_SUMMON)
    effect::set_code(d, e1, EFFECT_CANNOT_SPECIAL_SUMMON);
    // e1:SetTargetRange(1,0)
    effect::set_target_range(d, e1, 1, 0);
    // e1:SetTarget(function(e,c) return c:IsLocation(LOCATION_EXTRA) and not c:IsAttribute(ATTRIBUTE_WATER) end)
    effect::set_target(d, e1, Some(effect_card_fn(|d, _e, c| card::is_location(d, c, LOCATION_EXTRA) && !card::is_attribute(d, c, ATTRIBUTE_WATER))));
    // e1:SetReset(RESET_PHASE|PHASE_END)
    effect::set_reset(d, e1, RESET_PHASE | PHASE_END, 1);
    // Duel.RegisterEffect(e1,tp)
    duel::register_effect(d, e1, a.tp);
    // "Clock Lizard" check
    // aux.addTempLizardCheck(c,tp,function(e,c) return not c:IsOriginalAttribute(ATTRIBUTE_WATER) end)
    aux::add_temp_lizard_check(
        d,
        c,
        a.tp,
        Some(effect_card_fn(|d, _e, c| !card::is_original_attribute(d, c, ATTRIBUTE_WATER))),
        None,
        None,
        None,
        None,
    );
}
