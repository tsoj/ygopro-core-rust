//! Gearfried the Iron Knight
use super::prelude::*;

pub const ID: u32 = 423705;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // destroy equip
    let e1 = effect::create_effect(d, c);
    effect::set_description(d, e1, aux::stringid(ID, 0));
    effect::set_category(d, e1, CATEGORY_DESTROY);
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_F);
    effect::set_code(d, e1, EVENT_EQUIP);
    effect::set_target(d, e1, Some(cost_target(destg)));
    effect::set_operation(d, e1, Some(operation(desop)));
    card::register_effect(d, c, e1, false);
}
fn filter(d: &mut Duel, c: CardId, ec: CardId) -> bool {
    // return c:GetEquipTarget()==ec
    card::get_equip_target(d, c) == Some(ec)
}
fn destg(d: &mut Duel, a: &EffArgs) -> bool {
    // e:GetHandler()
    let h = effect::get_handler(d, a.e).nil();
    // if chk==0 then return eg:IsExists(s.filter,1,nil,e:GetHandler()) end
    if a.chk == Some(0) {
        return group::is_exists(d, &a.eg.clone().nil(), &|d, c| filter(d, c, h), 1, NO_EX);
    }
    // local dg=eg:Filter(s.filter,nil,e:GetHandler())
    let dg = group::filter(d, &a.eg.clone().nil(), &|d, c| filter(d, c, h), NO_EX);
    // Duel.SetTargetCard(dg)
    duel::set_target_card(d, dg.clone());
    // Duel.SetOperationInfo(0,CATEGORY_DESTROY,dg,#dg,0,0)
    let n = dg.len() as u8;
    duel::set_operation_info(d, 0, CATEGORY_DESTROY, Some(dg), n, 0, 0);
    true
}
fn desop(d: &mut Duel, a: &EffArgs) {
    // local tg=Duel.GetTargetCards(e)
    let tg = duel::get_target_cards(d, a.e).unwrap_or_default();
    // Duel.Destroy(tg,REASON_EFFECT)
    duel::destroy(d, tg, REASON_EFFECT, None, None);
}
