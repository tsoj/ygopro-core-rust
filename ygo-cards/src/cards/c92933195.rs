//! Mist Valley Windmaster
use super::prelude::*;

pub const ID: u32 = 92933195;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // hand adjust
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_TOGRAVE)
    effect::set_category(d, e1, CATEGORY_TOGRAVE);
    // e1:SetType(EFFECT_TYPE_IGNITION)
    effect::set_type(d, e1, EFFECT_TYPE_IGNITION);
    // e1:SetRange(LOCATION_MZONE)
    effect::set_range(d, e1, LOCATION_MZONE);
    // e1:SetCountLimit(1)
    effect::set_count_limit(d, e1, 1, 0, 0, 0);
    // e1:SetCondition(s.handcon)
    effect::set_condition(d, e1, Some(condition(handcon)));
    // e1:SetOperation(s.handop)
    effect::set_operation(d, e1, Some(operation(handop)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
/// `s.handcon(e,tp,eg,ep,ev,re,r,rp)`
fn handcon(d: &mut Duel, a: &EffArgs) -> bool {
    // return Duel.GetFieldGroupCount(tp,LOCATION_HAND,0)>=5 and Duel.GetFieldGroupCount(tp,0,LOCATION_HAND)>=5
    duel::get_field_group_count(d, a.tp, LOCATION_HAND, 0) >= 5 && duel::get_field_group_count(d, a.tp, 0, LOCATION_HAND) >= 5
}
/// `s.handop(e,tp,eg,ep,ev,re,r,rp)`
fn handop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // local g=Group.CreateGroup()
    let mut g = CardSet::new();
    // local ht1=Duel.GetFieldGroupCount(tp,LOCATION_HAND,0)
    let ht1 = duel::get_field_group_count(d, tp, LOCATION_HAND, 0);
    // if ht1>=5 then
    if ht1 >= 5 {
        // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_TOGRAVE)
        duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_TOGRAVE as u64);
        // local sg=Duel.SelectMatchingCard(tp,aux.TRUE,tp,LOCATION_HAND,0,ht1-4,ht1-4,nil)
        let sg = duel::select_matching_card(d, tp, None, tp, LOCATION_HAND, 0, (ht1 - 4) as u16, (ht1 - 4) as u16, false, NO_EX).nil();
        // g:Merge(sg)
        group::add_card(&mut g, &sg);
    }
    // local ht2=Duel.GetFieldGroupCount(1-tp,LOCATION_HAND,0)
    let ht2 = duel::get_field_group_count(d, 1 - tp, LOCATION_HAND, 0);
    // if ht2>=5 then
    if ht2 >= 5 {
        // Duel.Hint(HINT_SELECTMSG,1-tp,HINTMSG_TOGRAVE)
        duel::hint(d, HINT_SELECTMSG as u8, 1 - tp, HINTMSG_TOGRAVE as u64);
        // local sg=Duel.SelectMatchingCard(1-tp,aux.TRUE,1-tp,LOCATION_HAND,0,ht2-4,ht2-4,nil)
        let sg = duel::select_matching_card(d, 1 - tp, None, 1 - tp, LOCATION_HAND, 0, (ht2 - 4) as u16, (ht2 - 4) as u16, false, NO_EX).nil();
        // g:Merge(sg)
        group::add_card(&mut g, &sg);
    }
    // Duel.SendtoGrave(g,REASON_EFFECT)
    duel::send_to_grave(d, g, REASON_EFFECT, None, None);
}
