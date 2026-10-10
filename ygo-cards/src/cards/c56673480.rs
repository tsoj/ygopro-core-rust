//! Contract with Don Thousand
use super::prelude::*;

pub const ID: u32 = 56673480;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // When this card is activated: Both players lose 1000 LP, and if they do, each draws 1 card
    let e1 = effect::create_effect(d, c);
    effect::set_category(d, e1, CATEGORY_DRAW);
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    effect::set_count_limit(d, e1, 1, ID, 0, EFFECT_COUNT_CODE_OATH as u8);
    effect::set_target(d, e1, Some(cost_target(target)));
    effect::set_operation(d, e1, Some(operation(activate)));
    card::register_effect(d, c, e1, false);
    // local g=Group.CreateGroup()
    // All cards that are drawn while this card's effect is applied must remain revealed
    let e2a = effect::create_effect(d, c);
    effect::set_type(d, e2a, EFFECT_TYPE_FIELD | EFFECT_TYPE_CONTINUOUS);
    effect::set_code(d, e2a, EVENT_DRAW);
    effect::set_range(d, e2a, LOCATION_SZONE);
    effect::set_condition(d, e2a, Some(condition(drop)));
    // e2a:SetLabelObject(g): the group g is the label object of e2a
    effect::set_label_object(d, e2a, Value::Group(CardSet::new()));
    card::register_effect(d, c, e2a, false);
    let e2b = effect::create_effect(d, c);
    effect::set_type(d, e2b, EFFECT_TYPE_FIELD);
    effect::set_code(d, e2b, EFFECT_PUBLIC);
    effect::set_range(d, e2b, LOCATION_SZONE);
    effect::set_target_range(d, e2b, LOCATION_HAND, LOCATION_HAND);
    effect::set_condition(d, e2b, Some(condition(e2b_cond)));
    effect::set_target(d, e2b, Some(effect_card_fn(move |d, _e, cc| {
        // return g:IsContains(c) and c:HasFlagEffect(id+1)
        let g = label_group(d, e2a);
        group::is_contains(&g, cc) && card::has_flag_effect(d, cc, ID + 1, None)
    })));
    card::register_effect(d, c, e2b, false);
    // While a player's Spell Card in their hand is revealed by this effect, that player cannot Normal Summon/Set monsters
    let e3a = effect::create_effect(d, c);
    effect::set_type(d, e3a, EFFECT_TYPE_FIELD);
    effect::set_property(d, e3a, EFFECT_FLAG_PLAYER_TARGET, 0);
    effect::set_code(d, e3a, EFFECT_CANNOT_SUMMON);
    effect::set_range(d, e3a, LOCATION_SZONE);
    effect::set_target_range(d, e3a, 1, 0);
    effect::set_condition(d, e3a, Some(condition(move |d, a| e3a_cond(d, a, e2a))));
    card::register_effect(d, c, e3a, false);
    let e3b = effect::clone(d, e3a);
    effect::set_code(d, e3b, EFFECT_CANNOT_MSET);
    card::register_effect(d, c, e3b, false);
    let e3c = effect::clone(d, e3a);
    effect::set_target_range(d, e3c, 0, 1);
    effect::set_condition(d, e3c, Some(condition(move |d, a| e3c_cond(d, a, e2a))));
    card::register_effect(d, c, e3c, false);
    let e3d = effect::clone(d, e3c);
    effect::set_code(d, e3d, EFFECT_CANNOT_MSET);
    card::register_effect(d, c, e3d, false);
}
/// `g`: the label object of `e2a` (the group the script keeps)
fn label_group(d: &Duel, e2a: EffectId) -> CardSet {
    match effect::get_label_object(d, e2a) {
        Value::Group(g) => g,
        _ => CardSet::new(),
    }
}
fn e2b_cond(d: &mut Duel, a: &EffArgs) -> bool {
    // function(e) return e:GetHandler():HasFlagEffect(id) end
    let h = effect::get_handler(d, a.e).nil();
    card::has_flag_effect(d, h, ID, None)
}
fn e3a_cond(d: &mut Duel, a: &EffArgs, e2a: EffectId) -> bool {
    // Duel.IsExistingMatchingCard(s.cannotsumfilter,e:GetHandlerPlayer(),LOCATION_HAND,0,1,nil,g)
    let p = effect::get_handler_player(d, a.e);
    let g = label_group(d, e2a);
    duel::is_existing_matching_card(d, Some(&|d, c| cannotsumfilter(d, c, &g)), p, LOCATION_HAND, 0, 1, NO_EX)
}
fn e3c_cond(d: &mut Duel, a: &EffArgs, e2a: EffectId) -> bool {
    // Duel.IsExistingMatchingCard(s.cannotsumfilter,e:GetHandlerPlayer(),0,LOCATION_HAND,1,nil,g)
    let p = effect::get_handler_player(d, a.e);
    let g = label_group(d, e2a);
    duel::is_existing_matching_card(d, Some(&|d, c| cannotsumfilter(d, c, &g)), p, 0, LOCATION_HAND, 1, NO_EX)
}
fn cannotsumfilter(d: &mut Duel, c: CardId, pg: &CardSet) -> bool {
    // return c:IsPublic() and c:IsSpell() and pg:IsContains(c) and c:HasFlagEffect(id+1)
    card::is_public(d, c) && card::is_spell(d, c) && group::is_contains(pg, c) && card::has_flag_effect(d, c, ID + 1, None)
}
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return Duel.IsPlayerCanDraw(tp,1) and Duel.IsPlayerCanDraw(1-tp,1) end
    if a.chk == Some(0) {
        return duel::is_player_can_draw(d, a.tp, 1) && duel::is_player_can_draw(d, 1 - a.tp, 1);
    }
    // Duel.SetOperationInfo(0,CATEGORY_DRAW,nil,0,PLAYER_ALL,1)
    duel::set_operation_info(d, 0, CATEGORY_DRAW, None, 0, PLAYER_ALL as u8, 1);
    true
}
fn activate(d: &mut Duel, a: &EffArgs) {
    // Duel.SetLP(tp,Duel.GetLP(tp)-1000)
    let lp = duel::get_lp(d, a.tp) - 1000;
    duel::set_lp(d, a.tp, lp);
    // Duel.SetLP(1-tp,Duel.GetLP(1-tp)-1000)
    let lp = duel::get_lp(d, 1 - a.tp) - 1000;
    duel::set_lp(d, 1 - a.tp, lp);
    // Duel.Draw(tp,1,REASON_EFFECT)
    duel::draw(d, a.tp, 1, REASON_EFFECT);
    // Duel.Draw(1-tp,1,REASON_EFFECT)
    duel::draw(d, 1 - a.tp, 1, REASON_EFFECT);
}
/// The operation of `e2a` (its condition in Lua: it runs on each draw and returns nil).
fn drop(d: &mut Duel, a: &EffArgs) -> bool {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // local pg=e:GetLabelObject()
    let mut pg = match effect::get_label_object(d, a.e) {
        Value::Group(g) => g,
        _ => CardSet::new(),
    };
    // if not c:HasFlagEffect(id) then
    if !card::has_flag_effect(d, c, ID, None) {
        // c:RegisterFlagEffect(id,RESET_EVENT|RESETS_STANDARD_DISABLE,0,1)
        card::register_flag_effect(d, c, ID, RESET_EVENT | RESETS_STANDARD_DISABLE, 0, 1, 0, 0);
        // pg:Clear()
        pg = CardSet::new();
    }
    // for dc in eg:Iter() do
    let eg = a.eg.clone().nil();
    for dc in group::iter(d, &eg) {
        // pg:AddCard(dc)
        group::add_card(&mut pg, &CardSet::from_card(dc));
        // dc:RegisterFlagEffect(id+1,RESET_EVENT|RESETS_STANDARD,0,1)
        card::register_flag_effect(d, dc, ID + 1, RESET_EVENT | RESETS_STANDARD, 0, 1, 0, 0);
    }
    // the group changed: store it again as the label object of e2a
    effect::set_label_object(d, a.e, Value::Group(pg));
    false
}
