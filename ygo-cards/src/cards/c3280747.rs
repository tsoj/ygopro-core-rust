//! Sixth Sense
use super::prelude::*;

pub const ID: u32 = 3280747;
pub const INFO: ScriptInfo = ScriptInfo { roll_dice: true, ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate
    let e1 = effect::create_effect(d, c);
    effect::set_category(d, e1, CATEGORY_DRAW + CATEGORY_DECKDES + CATEGORY_DICE);
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    effect::set_target(d, e1, Some(cost_target(target)));
    effect::set_operation(d, e1, Some(operation(activate)));
    card::register_effect(d, c, e1, false);
}
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return Duel.GetFieldGroupCount(tp,LOCATION_DECK,0)>=6 end
    if a.chk == Some(0) {
        return duel::get_field_group_count(d, a.tp, LOCATION_DECK, 0) >= 6;
    }
    // Duel.SetOperationInfo(0,CATEGORY_DICE,nil,0,tp,2)
    duel::set_operation_info(d, 0, CATEGORY_DICE, None, 0, a.tp, 2);
    true
}
fn activate(d: &mut Duel, a: &EffArgs) {
    // local t={}
    // local i=1
    // local p=1
    // for i=1,6 do t[i]=i end
    let t: Vec<u64> = (1..=6).collect();
    // local a1=Duel.AnnounceNumber(tp,table.unpack(t))
    let (a1, _) = duel::announce_number(d, a.tp, &t).nil();
    // for i=1,6 do
    //     if a1~=i then t[p]=i p=p+1 end
    // end
    // t[p]=nil
    let mut t2: Vec<u64> = Vec::new();
    for i in 1..=6u64 {
        if a1 != i {
            t2.push(i);
        }
    }
    // local a2=Duel.AnnounceNumber(tp,table.unpack(t))
    let (a2, _) = duel::announce_number(d, a.tp, &t2).nil();
    // local dc=Duel.TossDice(1-tp,1)
    let dc = duel::toss_dice(d, 1 - a.tp, 1, 0)[0] as i64;
    // if dc==a1 or dc==a2 then
    if dc == a1 as i64 || dc == a2 as i64 {
        // Duel.Draw(tp,dc,REASON_EFFECT)
        duel::draw(d, a.tp, dc as u32, REASON_EFFECT);
    } else {
        // Duel.DiscardDeck(tp,dc,REASON_EFFECT)
        duel::discard_deck(d, a.tp, dc as u16, REASON_EFFECT);
    }
}
