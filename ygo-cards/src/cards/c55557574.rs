//! Thunderspeed Summon
use super::prelude::*;

pub const ID: u32 = 55557574;

pub const INFO: ScriptInfo = ScriptInfo {
    listed_names: &[ID, CARD_JACK_KNIGHT, CARD_KING_KNIGHT, CARD_QUEEN_KNIGHT],
    ..ScriptInfo::DEFAULT
};

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Normal Summon 1 Level 10 monster
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_TOHAND+CATEGORY_SEARCH+CATEGORY_SUMMON)
    effect::set_category(d, e1, CATEGORY_TOHAND + CATEGORY_SEARCH + CATEGORY_SUMMON);
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    // e1:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    // e1:SetHintTiming(0,TIMING_MAIN_END|TIMING_BATTLE_START)
    effect::set_hint_timing(d, e1, 0, Some(TIMING_MAIN_END | TIMING_BATTLE_START));
    // e1:SetCountLimit(1,id,EFFECT_COUNT_CODE_OATH)
    effect::set_count_limit(d, e1, 1, ID, 0, EFFECT_COUNT_CODE_OATH as u8);
    // e1:SetCondition(function() return Duel.IsMainPhase() or Duel.IsBattlePhase() end)
    effect::set_condition(d, e1, Some(condition(|d, _a| duel::is_main_phase(d, None) || duel::is_battle_phase(d, None))));
    // e1:SetTarget(s.target)
    effect::set_target(d, e1, Some(cost_target(target)));
    // e1:SetOperation(s.activate)
    effect::set_operation(d, e1, Some(operation(activate)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
/// `s.thfilter(c)`
fn thfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsMonster() and c:GetTextAttack()==-2 and c:IsLevel(10)
    //     and c:IsAttributeExcept(ATTRIBUTE_DARK) and c:IsAbleToHand()
    card::is_monster(d, c) && card::get_text_attack(d, c) == -2 && card::is_level(d, c, &[10]) && card::is_attribute_except(d, c, ATTRIBUTE_DARK, None, 0, None) && card::is_able_to_hand(d, c, None)
}
/// `s.mfilter(c,code)`
fn mfilter(d: &mut Duel, c: CardId, code: u32) -> bool {
    // return c:IsFaceup() and c:IsCode(code)
    card::is_faceup(d, c) && card::is_code(d, c, &[code])
}
/// `s.sfilter(c)`
fn sfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsSummonable(true,nil) and c:IsLevel(10)
    card::is_summonable(d, c, true, None, 0, None) && card::is_level(d, c, &[10])
}
/// The second half of the `a` / `b` conditions: `b` is the Jack, King and Queen's Knights on the
/// field and a "Thunderspeed" target in the Deck.
fn has_b(d: &mut Duel, tp: u8) -> bool {
    // Duel.IsExistingMatchingCard(s.mfilter,tp,LOCATION_MZONE,0,1,nil,CARD_JACK_KNIGHT)
    //     and Duel.IsExistingMatchingCard(s.mfilter,tp,LOCATION_MZONE,0,1,nil,CARD_KING_KNIGHT)
    //     and Duel.IsExistingMatchingCard(s.mfilter,tp,LOCATION_MZONE,0,1,nil,CARD_QUEEN_KNIGHT)
    //     and Duel.IsExistingMatchingCard(s.thfilter,tp,LOCATION_DECK,0,1,nil)
    duel::is_existing_matching_card(d, Some(&|d, c| mfilter(d, c, CARD_JACK_KNIGHT)), tp, LOCATION_MZONE, 0, 1, NO_EX)
        && duel::is_existing_matching_card(d, Some(&|d, c| mfilter(d, c, CARD_KING_KNIGHT)), tp, LOCATION_MZONE, 0, 1, NO_EX)
        && duel::is_existing_matching_card(d, Some(&|d, c| mfilter(d, c, CARD_QUEEN_KNIGHT)), tp, LOCATION_MZONE, 0, 1, NO_EX)
        && duel::is_existing_matching_card(d, Some(&thfilter), tp, LOCATION_DECK, 0, 1, NO_EX)
}
/// `s.target(e,tp,eg,ep,ev,re,r,rp,chk)`
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    // local a=Duel.IsExistingMatchingCard(s.sfilter,tp,LOCATION_HAND|LOCATION_MZONE,0,1,nil)
    let av = duel::is_existing_matching_card(d, Some(&sfilter), a.tp, LOCATION_HAND | LOCATION_MZONE, 0, 1, NO_EX);
    // local b=...
    let b = has_b(d, a.tp);
    // if chk==0 then return a or b end
    if a.chk == Some(0) {
        return av || b;
    }
    // Duel.SetOperationInfo(0,CATEGORY_TOHAND,nil,1,tp,LOCATION_DECK)
    duel::set_operation_info(d, 0, CATEGORY_TOHAND, None, 1, a.tp, LOCATION_DECK as i32);
    // Duel.SetOperationInfo(0,CATEGORY_SUMMON,nil,1,0,0)
    duel::set_operation_info(d, 0, CATEGORY_SUMMON, None, 1, 0, 0);
    true
}
/// `s.activate(e,tp,eg,ep,ev,re,r,rp)`
fn activate(d: &mut Duel, a: &EffArgs) {
    // local a=Duel.IsExistingMatchingCard(s.sfilter,tp,LOCATION_HAND|LOCATION_MZONE,0,1,nil)
    let av = duel::is_existing_matching_card(d, Some(&sfilter), a.tp, LOCATION_HAND | LOCATION_MZONE, 0, 1, NO_EX);
    // local b=...
    let b = has_b(d, a.tp);
    // if not (a or b) then return end
    if !(av || b) {
        return;
    }
    // if (not a and b) or (b and Duel.SelectYesNo(tp,aux.Stringid(id,1))) then
    if (!av && b) || (b && duel::select_yes_no(d, a.tp, aux::stringid(ID, 1))) {
        // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_ATOHAND)
        duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_ATOHAND as u64);
        // local g=Duel.SelectMatchingCard(tp,s.thfilter,tp,LOCATION_DECK,0,1,1,nil,tp)
        let g = duel::select_matching_card(d, a.tp, Some(&thfilter), a.tp, LOCATION_DECK, 0, 1, 1, false, NO_EX).nil();
        // if #g>0 and Duel.SendtoHand(g,nil,REASON_EFFECT)>0 then
        if !g.is_empty() && duel::send_to_hand(d, g.clone(), None, REASON_EFFECT, None) > 0 {
            // Duel.ConfirmCards(1-tp,g)
            duel::confirm_cards(d, 1 - a.tp, g);
            // if Duel.IsExistingMatchingCard(s.sfilter,tp,LOCATION_HAND,0,1,nil) and Duel.SelectYesNo(tp,aux.Stringid(id,2)) then
            if duel::is_existing_matching_card(d, Some(&sfilter), a.tp, LOCATION_HAND, 0, 1, NO_EX) && duel::select_yes_no(d, a.tp, aux::stringid(ID, 2)) {
                // Duel.BreakEffect()
                duel::break_effect(d);
                // Duel.ShuffleHand(tp)
                duel::shuffle_hand(d, a.tp);
                // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_SUMMON)
                duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_SUMMON as u64);
                // local g2=Duel.SelectMatchingCard(tp,s.sfilter,tp,LOCATION_HAND|LOCATION_MZONE,0,1,1,nil)
                let g2 = duel::select_matching_card(d, a.tp, Some(&sfilter), a.tp, LOCATION_HAND | LOCATION_MZONE, 0, 1, 1, false, NO_EX).nil();
                // local tc=g2:GetFirst()
                let tc = group::get_first(d, &g2);
                // if tc then Duel.Summon(tp,tc,true,nil) end
                if let Some(tc) = tc {
                    duel::summon(d, a.tp, tc, true, None, 0, None);
                }
            }
        }
    } else if av {
        // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_SUMMON)
        duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_SUMMON as u64);
        // local g=Duel.SelectMatchingCard(tp,s.sfilter,tp,LOCATION_HAND|LOCATION_MZONE,0,1,1,nil)
        let g = duel::select_matching_card(d, a.tp, Some(&sfilter), a.tp, LOCATION_HAND | LOCATION_MZONE, 0, 1, 1, false, NO_EX).nil();
        // local tc=g:GetFirst()
        let tc = group::get_first(d, &g);
        // if tc then Duel.Summon(tp,tc,true,nil) end
        if let Some(tc) = tc {
            duel::summon(d, a.tp, tc, true, None, 0, None);
        }
    }
}
