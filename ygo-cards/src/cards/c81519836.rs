//! Vernusylph of the Misting Seedlings
//! Scripted by Hatter
use super::prelude::*;

pub const ID: u32 = 81519836;

pub const INFO: ScriptInfo = ScriptInfo { listed_names: &[ID], listed_series: &[SET_VERNUSYLPH], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Reduce ATK of non-"Vernalizer" monsters
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e1, EFFECT_TYPE_FIELD);
    // e1:SetCode(EFFECT_UPDATE_ATTACK)
    effect::set_code(d, e1, EFFECT_UPDATE_ATTACK);
    // e1:SetRange(LOCATION_MZONE)
    effect::set_range(d, e1, LOCATION_MZONE);
    // e1:SetTargetRange(LOCATION_MZONE,LOCATION_MZONE)
    effect::set_target_range(d, e1, LOCATION_MZONE, LOCATION_MZONE);
    // e1:SetTarget(function(_,c) return not c:IsSetCard(SET_VERNUSYLPH) end)
    effect::set_target(d, e1, Some(effect_card_fn(|d, _e, c| !card::is_set_card(d, c, &[SET_VERNUSYLPH as u16]))));
    // e1:SetValue(-600)
    effect::set_value(d, e1, -600);
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // Search 1 EARTH Fairy monster
    // Vernusylph.AddSpSummonEffect(c,id,0,CATEGORY_TOHAND+CATEGORY_SEARCH,s.thtg,s.thop)
    aux::vernusylph::add_sp_summon_effect(d, c, ID, 0, CATEGORY_TOHAND + CATEGORY_SEARCH, cost_target(thtg), lua_fn(thop));
}

/// `s.thfilter(c)`: `c:IsAttribute(ATTRIBUTE_EARTH) and c:IsRace(RACE_FAIRY) and not c:IsCode(id) and c:IsAbleToHand()`
fn thfilter(d: &mut Duel, c: CardId) -> bool {
    card::is_attribute(d, c, ATTRIBUTE_EARTH) && card::is_race(d, c, RACE_FAIRY) && !card::is_code(d, c, &[ID]) && card::is_able_to_hand(d, c, None)
}

/// `s.thtg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn thtg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return Duel.IsExistingMatchingCard(s.thfilter,tp,LOCATION_DECK,0,1,nil) end
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, Some(&thfilter), a.tp, LOCATION_DECK, 0, 1, NO_EX);
    }
    // Duel.SetOperationInfo(0,CATEGORY_TOHAND,nil,1,tp,LOCATION_DECK)
    duel::set_operation_info(d, 0, CATEGORY_TOHAND, None, 1, a.tp, LOCATION_DECK as i32);
    true
}

/// `s.thop(e,tp,eg,ep,ev,re,r,rp)`
fn thop(d: &mut Duel, a: Args) -> bool {
    let tp = a.p(1);
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_ATOHAND)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_ATOHAND as u64);
    // local g=Duel.SelectMatchingCard(tp,s.thfilter,tp,LOCATION_DECK,0,1,1,nil)
    let g = duel::select_matching_card(d, tp, Some(&thfilter), tp, LOCATION_DECK, 0, 1, 1, false, NO_EX).nil();
    // if #g>0 and Duel.SendtoHand(g,nil,REASON_EFFECT)>0 and g:GetFirst():IsLocation(LOCATION_HAND) then
    if !g.is_empty()
        && duel::send_to_hand(d, g.clone(), None, REASON_EFFECT, None) > 0
        && card::is_location(d, group::get_first(d, &g).nil(), LOCATION_HAND)
    {
        // Duel.ConfirmCards(1-tp,g)
        duel::confirm_cards(d, 1 - tp, g);
        // return true
        return true;
    }
    false
}
