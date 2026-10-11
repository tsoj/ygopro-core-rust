//! Phalaenop Sisters
use super::prelude::*;

pub const ID: u32 = 59901153;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Add 1 Level 7 or higher Insect or Plant monster from your Deck to your hand
    let e1 = effect::create_effect(d, c);
    effect::set_description(d, e1, aux::stringid(ID, 0));
    effect::set_category(d, e1, CATEGORY_TOHAND | CATEGORY_SEARCH | CATEGORY_DAMAGE);
    effect::set_type(d, e1, EFFECT_TYPE_IGNITION);
    effect::set_range(d, e1, LOCATION_MZONE);
    effect::set_count_limit(d, e1, 1, ID, 0, 0);
    effect::set_condition(d, e1, Some(condition(thcon)));
    effect::set_cost(d, e1, Some(cost::self_tribute()));
    effect::set_target(d, e1, Some(cost_target(thtg)));
    effect::set_operation(d, e1, Some(operation(thop)));
    card::register_effect(d, c, e1, false);
}
/// `s.thcon(e,tp,eg,ep,ev,re,r,rp)`
fn thcon(d: &mut Duel, a: &EffArgs) -> bool {
    // local g1=Duel.GetMatchingGroup(Card.IsFaceup,tp,LOCATION_MZONE,0,nil)
    let g1 = duel::get_matching_group(d, Some(&|d, c| card::is_faceup(d, c)), a.tp, LOCATION_MZONE, 0, NO_EX);
    // local g2=Duel.GetMatchingGroup(Card.IsFaceup,tp,0,LOCATION_MZONE,nil)
    let g2 = duel::get_matching_group(d, Some(&|d, c| card::is_faceup(d, c)), a.tp, 0, LOCATION_MZONE, NO_EX);
    // return g1:GetSum(Card.GetLevel)<=g2:GetSum(Card.GetLevel)
    let s1 = group::get_sum(d, &g1, &|d, c| card::get_level(d, c) as i64);
    let s2 = group::get_sum(d, &g2, &|d, c| card::get_level(d, c) as i64);
    s1 <= s2
}
/// `s.thfilter(c)`
fn thfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsLevelAbove(7) and c:IsRace(RACE_INSECT|RACE_PLANT) and c:IsAbleToHand()
    card::is_level_above(d, c, 7) && card::is_race(d, c, RACE_INSECT | RACE_PLANT) && card::is_able_to_hand(d, c, None)
}
/// `s.thtg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn thtg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return Duel.IsExistingMatchingCard(s.thfilter,tp,LOCATION_DECK,0,1,nil) end
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, Some(&thfilter), a.tp, LOCATION_DECK, 0, 1, NO_EX);
    }
    // Duel.SetOperationInfo(0,CATEGORY_TOHAND,nil,1,tp,LOCATION_DECK)
    duel::set_operation_info(d, 0, CATEGORY_TOHAND, None, 1, a.tp, LOCATION_DECK as i32);
    // Duel.SetPossibleOperationInfo(0,CATEGORY_DAMAGE,nil,0,tp,2700)
    duel::set_possible_operation_info(d, 0, CATEGORY_DAMAGE, None, 0, a.tp, 2700);
    true
}
/// `s.thop(e,tp,eg,ep,ev,re,r,rp)`
fn thop(d: &mut Duel, a: &EffArgs) {
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_ATOHAND)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_ATOHAND as u64);
    // local sc=Duel.SelectMatchingCard(tp,s.thfilter,tp,LOCATION_DECK,0,1,1,nil):GetFirst()
    let g = duel::select_matching_card(d, a.tp, Some(&thfilter), a.tp, LOCATION_DECK, 0, 1, 1, false, NO_EX).nil();
    let sc = group::get_first(d, &g);
    // if sc then
    if let Some(sc) = sc {
        // Duel.SendtoHand(sc,nil,REASON_EFFECT)
        duel::send_to_hand(d, sc, None, REASON_EFFECT, None);
        // Duel.ConfirmCards(1-tp,sc)
        duel::confirm_cards(d, 1 - a.tp, sc);
        // local c=e:GetHandler()
        let c = effect::get_handler(d, a.e).nil();
        // local code=sc:GetOriginalCodeRule()
        let code = card::get_original_code_rule(d, sc).first().copied().unwrap_or(0);
        // Register if the player Normal or Special Summons the added monster or a card with the same original name
        // local e1=Effect.CreateEffect(c)
        let e1 = effect::create_effect(d, c);
        // e1:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_CONTINUOUS)
        effect::set_type(d, e1, EFFECT_TYPE_FIELD + EFFECT_TYPE_CONTINUOUS);
        // e1:SetCode(EVENT_SUMMON_SUCCESS)
        effect::set_code(d, e1, EVENT_SUMMON_SUCCESS);
        // e1:SetCountLimit(1)
        effect::set_count_limit(d, e1, 1, 0, 0, 0);
        // e1:SetCondition(function(e,tp,eg,ep) return ep==tp and ((eg:IsContains(sc) and sc:IsFaceup()) or eg:IsExists(aux.FaceupFilter(Card.IsOriginalCodeRule,code),1,nil)) end)
        effect::set_condition(
            d,
            e1,
            Some(lua_fn(move |d, a: Args| -> bool {
                let eg = a.g(2);
                if a.p(3) != a.p(1) {
                    return false;
                }
                (group::is_contains(&eg, sc) && card::is_faceup(d, sc))
                    || group::is_exists(d, &eg, &|d, c| card::is_faceup(d, c) && card::is_original_code_rule(d, c, &[code]), 1, NO_EX)
            })),
        );
        // e1:SetOperation(function(e) e:SetLabel(1) end)
        effect::set_operation(
            d,
            e1,
            Some(lua_fn(|d, a: Args| {
                effect::set_label(d, a.e(0), &[1]);
            })),
        );
        // e1:SetReset(RESET_PHASE|PHASE_END)
        effect::set_reset(d, e1, RESET_PHASE | PHASE_END, 1);
        // Duel.RegisterEffect(e1,tp)
        duel::register_effect(d, e1, a.tp);
        // local e2=e1:Clone()
        let e2 = effect::clone(d, e1);
        // e2:SetCode(EVENT_SPSUMMON_SUCCESS)
        effect::set_code(d, e2, EVENT_SPSUMMON_SUCCESS);
        // Duel.RegisterEffect(e2,tp)
        duel::register_effect(d, e2, a.tp);
        // During the End Phase of this turn, take 2700 damage if you did not Normal or Special Summon the added monster, or a card with the same original name
        // local e3=e1:Clone()
        let e3 = effect::clone(d, e1);
        // e3:SetCode(EVENT_PHASE+PHASE_END)
        effect::set_code(d, e3, EVENT_PHASE + PHASE_END);
        // e3:SetCondition(function() return e1:GetLabel()==0 and e2:GetLabel()==0 end)
        effect::set_condition(
            d,
            e3,
            Some(lua_fn(move |d, _a: Args| -> bool {
                let l1 = effect::get_label(d, e1);
                let l2 = effect::get_label(d, e2);
                l1 == 0 && l2 == 0
            })),
        );
        // e3:SetOperation(function(e,tp) Duel.Damage(tp,2700,REASON_EFFECT) end)
        effect::set_operation(
            d,
            e3,
            Some(lua_fn(|d, a: Args| {
                duel::damage(d, a.p(1), 2700, REASON_EFFECT, false, None);
            })),
        );
        // Duel.RegisterEffect(e3,tp)
        duel::register_effect(d, e3, a.tp);
        // aux.RegisterClientHint(c,0,tp,1,0,aux.Stringid(id,1))
        aux::register_client_hint(d, c, 0, a.tp, 1, 0, Some(aux::stringid(ID, 1)), 0, None);
    }
}
