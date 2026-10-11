//! Space Gift
use super::prelude::*;

pub const ID: u32 = 1539051;

pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_NEO_SPACIAN as u32], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate
    let e1 = effect::create_effect(d, c);
    effect::set_category(d, e1, CATEGORY_DRAW);
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    effect::set_property(d, e1, EFFECT_FLAG_PLAYER_TARGET, 0);
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    effect::set_target(d, e1, Some(cost_target(target)));
    effect::set_operation(d, e1, Some(operation(activate)));
    card::register_effect(d, c, e1, false);
}
/// `s.target(e,tp,...,chk)`
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    if a.chk == Some(0) {
        // local g=Duel.GetMatchingGroup(aux.FaceupFilter(Card.IsSetCard,SET_NEO_SPACIAN),tp,LOCATION_MZONE,0,nil)
        let g = duel::get_matching_group(d, Some(&aux::faceup_filter(|d, c| card::is_set_card(d, c, &[SET_NEO_SPACIAN as u16]))), a.tp, LOCATION_MZONE, 0, NO_EX);
        // local ct=s.count_unique_code(g)
        let ct = count_unique_code(d, &g);
        // e:SetLabel(ct)
        effect::set_label(d, a.e, &[ct as i64]);
        // return ct>0 and Duel.IsPlayerCanDraw(tp,ct)
        return ct > 0 && duel::is_player_can_draw(d, a.tp, ct as u32);
    }
    // Duel.SetTargetPlayer(tp)
    duel::set_target_player(d, a.tp);
    // Duel.SetTargetParam(e:GetLabel())
    let label = effect::get_label(d, a.e);
    duel::set_target_param(d, label as u32);
    // Duel.SetOperationInfo(0,CATEGORY_DRAW,nil,0,tp,e:GetLabel())
    duel::set_operation_info(d, 0, CATEGORY_DRAW, None, 0, a.tp, label as i32);
    true
}
/// `s.activate(e,tp,...)`
fn activate(d: &mut Duel, a: &EffArgs) {
    // local p=Duel.GetChainInfo(0,CHAININFO_TARGET_PLAYER)
    let p = duel::get_chain_info(d, 0, &[CHAININFO_TARGET_PLAYER])[0].int() as u8;
    // local g=Duel.GetMatchingGroup(aux.FaceupFilter(Card.IsSetCard,SET_NEO_SPACIAN),tp,LOCATION_MZONE,0,nil)
    let g = duel::get_matching_group(d, Some(&aux::faceup_filter(|d, c| card::is_set_card(d, c, &[SET_NEO_SPACIAN as u16]))), a.tp, LOCATION_MZONE, 0, NO_EX);
    // local ct=s.count_unique_code(g)
    let ct = count_unique_code(d, &g);
    // Duel.Draw(p,ct,REASON_EFFECT)
    duel::draw(d, p, ct as u32, REASON_EFFECT);
}
/// `s.count_unique_code(g)`: the number of different codes the cards of `g` have, counting each code once
fn count_unique_code(d: &mut Duel, g: &CardSet) -> usize {
    // local check={}
    let mut check: Vec<u32> = Vec::new();
    // local count=0
    let mut count = 0;
    // local tc=g:GetFirst()
    let _ = group::get_first(d, g);
    // for tc in aux.Next(g) do
    for tc in group::iter(d, g) {
        // for i,code in ipairs({tc:GetCode()}) do
        for code in card::get_code(d, tc) {
            // if not check[code] then check[code]=true count=count+1 end
            if !check.contains(&code) {
                check.push(code);
                count += 1;
            }
        }
    }
    // return count
    count
}
