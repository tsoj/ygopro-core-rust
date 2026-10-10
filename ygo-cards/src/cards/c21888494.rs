//! Chosen One
use super::prelude::*;

pub const ID: u32 = 21888494;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetCategory(CATEGORY_SPECIAL_SUMMON)
    effect::set_category(d, e1, CATEGORY_SPECIAL_SUMMON);
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    // e1:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    // e1:SetTarget(s.target)
    effect::set_target(d, e1, Some(cost_target(target)));
    // e1:SetOperation(s.activate)
    effect::set_operation(d, e1, Some(operation(activate)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
fn spfilter(d: &mut Duel, c: CardId, e: EffectId, tp: u8) -> bool {
    // return c:IsCanBeSpecialSummoned(e,0,tp,false,false)
    card::is_can_be_special_summoned(d, c, e, 0, tp, false, false, None, None, None)
}
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return Duel.GetLocationCount(tp,LOCATION_MZONE)>0
    //     and Duel.IsExistingMatchingCard(Card.IsSpellTrap,tp,LOCATION_HAND,0,2,e:GetHandler())
    //     and Duel.IsExistingMatchingCard(s.spfilter,tp,LOCATION_HAND,0,1,nil,e,tp) end
    if a.chk == Some(0) {
        let h = effect::get_handler(d, a.e).nil();
        return duel::get_location_count(d, a.tp, LOCATION_MZONE as u8, None, None, None).0 > 0
            && duel::is_existing_matching_card(d, Some(&card::is_spell_trap), a.tp, LOCATION_HAND, 0, 2, h.into())
            && duel::is_existing_matching_card(d, Some(&|d, c| spfilter(d, c, a.e, a.tp)), a.tp, LOCATION_HAND, 0, 1, NO_EX);
    }
    true
}
fn activate(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // if Duel.GetLocationCount(tp,LOCATION_MZONE)<=0 then return end
    if duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 <= 0 {
        return;
    }
    // local g1=Duel.GetMatchingGroup(s.spfilter,tp,LOCATION_HAND,0,nil,e,tp)
    let g1 = duel::get_matching_group(d, Some(&|d, c| spfilter(d, c, a.e, tp)), tp, LOCATION_HAND, 0, NO_EX);
    // local g2=Duel.GetMatchingGroup(Card.IsSpellTrap,tp,LOCATION_HAND,0,nil)
    let g2 = duel::get_matching_group(d, Some(&card::is_spell_trap), tp, LOCATION_HAND, 0, NO_EX);
    // if #g1==0 or #g2<2 then return end
    if g1.is_empty() || g2.len() < 2 {
        return;
    }
    // Duel.Hint(HINT_SELECTMSG,tp,aux.Stringid(id,0))
    duel::hint(d, HINT_SELECTMSG as u8, tp, aux::stringid(ID, 0));
    // local sg1=g1:Select(tp,1,1,nil)
    let mut sg1 = group::select(d, &g1, tp, 1, 1, false, NO_EX).nil();
    // local sc=sg1:GetFirst()
    let _sc = group::get_first(d, &sg1);
    // Duel.Hint(HINT_SELECTMSG,tp,aux.Stringid(id,1))
    duel::hint(d, HINT_SELECTMSG as u8, tp, aux::stringid(ID, 1));
    // local sg2=g2:Select(tp,2,2,nil)
    let sg2 = group::select(d, &g2, tp, 2, 2, false, NO_EX).nil();
    // sg1:Merge(sg2)
    group::add_card(&mut sg1, &sg2);
    // Duel.ConfirmCards(1-tp,sg1)
    duel::confirm_cards(d, 1 - tp, &sg1);
    // Duel.ShuffleHand(tp)
    duel::shuffle_hand(d, tp);
    // local rg=sg1:Select(1-tp,1,1,nil)
    let rg = group::select(d, &sg1, 1 - tp, 1, 1, false, NO_EX).nil();
    // local tc=rg:GetFirst()
    let tc = group::get_first(d, &rg).nil();
    // if tc:IsCanBeSpecialSummoned(e,0,tp,false,false) then
    if card::is_can_be_special_summoned(d, tc, a.e, 0, tp, false, false, None, None, None) {
        // Duel.SpecialSummon(tc,0,tp,tp,false,false,POS_FACEUP)
        duel::special_summon(d, CardSet::from_card(tc), 0, tp, tp, false, false, POS_FACEUP as u8, None);
        // Duel.SendtoGrave(sg2,REASON_EFFECT)
        duel::send_to_grave(d, &sg2, REASON_EFFECT, None, None);
    } else {
        // Duel.SendtoGrave(sg1,REASON_EFFECT)
        duel::send_to_grave(d, &sg1, REASON_EFFECT, None, None);
    }
}
