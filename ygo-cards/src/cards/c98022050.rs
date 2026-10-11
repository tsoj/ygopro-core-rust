//! Animadorned Archosaur
use super::prelude::*;

pub const ID: u32 = 98022050;

pub const INFO: ScriptInfo = ScriptInfo {
    listed_names: &[ID],
    listed_series: &[SET_EVOLUTION_PILL],
    ..ScriptInfo::DEFAULT
};

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // destroy
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_DESTROY+CATEGORY_TOHAND+CATEGORY_SEARCH)
    effect::set_category(d, e1, CATEGORY_DESTROY + CATEGORY_TOHAND + CATEGORY_SEARCH);
    // e1:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_O);
    // e1:SetCode(EVENT_SUMMON_SUCCESS)
    effect::set_code(d, e1, EVENT_SUMMON_SUCCESS);
    // e1:SetProperty(EFFECT_FLAG_DAMAGE_STEP+EFFECT_FLAG_DELAY)
    effect::set_property(d, e1, EFFECT_FLAG_DAMAGE_STEP + EFFECT_FLAG_DELAY, 0);
    // e1:SetCountLimit(1,id)
    effect::set_count_limit(d, e1, 1, ID, 0, 0);
    // e1:SetTarget(s.destg)
    effect::set_target(d, e1, Some(cost_target(destg)));
    // e1:SetOperation(s.desop)
    effect::set_operation(d, e1, Some(operation(desop)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // local e2=e1:Clone()
    let e2 = effect::clone(d, e1);
    // e2:SetCode(EVENT_SPSUMMON_SUCCESS)
    effect::set_code(d, e2, EVENT_SPSUMMON_SUCCESS);
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
}
/// `s.desfilter(c,tp)`
fn desfilter(d: &mut Duel, c: CardId, tp: u8) -> bool {
    // return c:IsRace(RACE_DINOSAUR) and (c:IsLocation(LOCATION_HAND) or c:IsFaceup()) and not c:IsCode(id)
    //     and Duel.IsExistingMatchingCard(s.thfilter,tp,LOCATION_DECK,0,1,nil,c)
    card::is_race(d, c, RACE_DINOSAUR)
        && (card::is_location(d, c, LOCATION_HAND) || card::is_faceup(d, c))
        && !card::is_code(d, c, &[ID])
        && duel::is_existing_matching_card(d, Some(&|d, tc| thfilter(d, tc, c)), tp, LOCATION_DECK, 0, 1, NO_EX)
}
/// `s.thfilter(c,oc)`
fn thfilter(d: &mut Duel, c: CardId, oc: CardId) -> bool {
    // if not c:IsAbleToHand() then return false end
    if !card::is_able_to_hand(d, c, None) {
        return false;
    }
    // if (c:IsSpell() and c:IsSetCard(SET_EVOLUTION_PILL)) then return true end
    if card::is_spell(d, c) && card::is_set_card(d, c, &[SET_EVOLUTION_PILL as u16]) {
        return true;
    }
    // return c:IsMonster() and oc:HasLevel() and c:GetOriginalLevel()==oc:GetOriginalLevel()
    //     and (c:IsRace(RACE_WINGEDBEAST) or c:IsRace(RACE_REPTILE) or c:IsRace(RACE_SEASERPENT))
    card::is_monster(d, c)
        && card::has_level(d, oc)
        && card::get_original_level(d, c) == card::get_original_level(d, oc)
        && (card::is_race(d, c, RACE_WINGEDBEAST) || card::is_race(d, c, RACE_REPTILE) || card::is_race(d, c, RACE_SEASERPENT))
}
/// `s.destg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn destg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // if chk==0 then return Duel.IsExistingMatchingCard(s.desfilter,tp,LOCATION_HAND|LOCATION_MZONE,0,1,nil,tp) end
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, Some(&|d, c| desfilter(d, c, tp)), tp, LOCATION_HAND | LOCATION_MZONE, 0, 1, NO_EX);
    }
    // local g=Duel.GetMatchingGroup(s.desfilter,tp,LOCATION_HAND|LOCATION_MZONE,0,nil,tp)
    let g = duel::get_matching_group(d, Some(&|d, c| desfilter(d, c, tp)), tp, LOCATION_HAND | LOCATION_MZONE, 0, NO_EX);
    // Duel.SetOperationInfo(0,CATEGORY_DESTROY,g,1,0,0)
    duel::set_operation_info(d, 0, CATEGORY_DESTROY, Some(g), 1, 0, 0);
    // Duel.SetOperationInfo(0,CATEGORY_TOHAND,nil,1,tp,LOCATION_DECK)
    duel::set_operation_info(d, 0, CATEGORY_TOHAND, None, 1, tp, LOCATION_DECK as i32);
    true
}
/// `s.desop(e,tp,eg,ep,ev,re,r,rp)`
fn desop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_DESTROY)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_DESTROY as u64);
    // local tc=Duel.SelectMatchingCard(tp,s.desfilter,tp,LOCATION_HAND|LOCATION_MZONE,0,1,1,nil,tp):GetFirst()
    let sel = duel::select_matching_card(d, tp, Some(&|d, c| desfilter(d, c, tp)), tp, LOCATION_HAND | LOCATION_MZONE, 0, 1, 1, false, NO_EX).nil();
    let tc = group::get_first(d, &sel);
    // if tc and Duel.Destroy(tc,REASON_EFFECT)~=0 then
    if let Some(tc) = tc {
        if duel::destroy(d, tc, REASON_EFFECT, None, None) != 0 {
            // Duel.BreakEffect()
            duel::break_effect(d);
            // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_ATOHAND)
            duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_ATOHAND as u64);
            // local g=Duel.SelectMatchingCard(tp,s.thfilter,tp,LOCATION_DECK,0,1,1,nil,tc)
            let g = duel::select_matching_card(d, tp, Some(&|d, c| thfilter(d, c, tc)), tp, LOCATION_DECK, 0, 1, 1, false, NO_EX).nil();
            // if #g>0 then
            if !g.is_empty() {
                // Duel.SendtoHand(g,nil,REASON_EFFECT)
                duel::send_to_hand(d, g.clone(), None, REASON_EFFECT, None);
                // Duel.ConfirmCards(1-tp,g)
                duel::confirm_cards(d, 1 - tp, g);
            }
        }
    }
}
