//! Big Eye
use super::prelude::*;

pub const ID: u32 = 16768387;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // sort the top deck
    let e1 = effect::create_effect(d, c);
    effect::set_description(d, e1, aux::stringid(ID, 0));
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE + EFFECT_TYPE_FLIP);
    effect::set_operation(d, e1, Some(operation(op)));
    card::register_effect(d, c, e1, false);
}
/// `s.operation(e,tp,eg,ep,ev,re,r,rp)`
fn op(d: &mut Duel, a: &EffArgs) {
    // local ct=math.min(5,Duel.GetFieldGroupCount(tp,LOCATION_DECK,0))
    let ct = duel::get_field_group_count(d, a.tp, LOCATION_DECK, 0).min(5);
    // if ct==0 then return end
    if ct == 0 {
        return;
    }
    // local ac=ct==1 and ct or Duel.AnnounceNumberRange(tp,1,ct)
    if ct != 1 {
        let _ac = duel::announce_number_range(d, a.tp, Some(1), Some(ct as u32), &[]);
    }
    // Duel.SortDecktop(tp,tp,ct)
    duel::sort_decktop(d, a.tp, a.tp, ct as u16);
}
