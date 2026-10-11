//! Crossout Designator
use super::prelude::*;
use crate::luatable::LuaTable;

pub const ID: u32 = 65681983;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate
    let e1 = effect::create_effect(d, c);
    effect::set_category(d, e1, CATEGORY_REMOVE);
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    effect::set_count_limit(d, e1, 1, ID, 0, EFFECT_COUNT_CODE_OATH as u8);
    effect::set_target(d, e1, Some(cost_target(rmtg)));
    effect::set_operation(d, e1, Some(operation(rmop)));
    card::register_effect(d, c, e1, false);
}
fn able_to_remove(d: &mut Duel, c: CardId) -> bool {
    card::is_able_to_remove(d, c, None, None, None)
}
fn rmtg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // if chk==0 then return Duel.IsExistingMatchingCard(Card.IsAbleToRemove,tp,LOCATION_DECK,0,1,nil) end
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, Some(&able_to_remove), tp, LOCATION_DECK, 0, 1, NO_EX);
    }
    // local g=Duel.GetMatchingGroup(Card.IsAbleToRemove,tp,LOCATION_DECK,0,nil)
    let g = duel::get_matching_group(d, Some(&able_to_remove), tp, LOCATION_DECK, 0, NO_EX);
    // local ids={}
    let mut ids: LuaTable<bool> = LuaTable::new();
    // for tc in aux.Next(g) do ids[tc:GetCode()]=true end
    for tc in group::iter(d, &g) {
        let code = card::get_code(d, tc)[0];
        ids.set(code as i64, Some(true));
    }
    // s.announce_filter={}
    // for code,i in pairs(ids) do ... end
    let mut announce_filter: Vec<u64> = Vec::new();
    for (code, _i) in ids.pairs() {
        if announce_filter.is_empty() {
            announce_filter.push(code as u64);
            announce_filter.push(OPCODE_ISCODE);
        } else {
            announce_filter.push(code as u64);
            announce_filter.push(OPCODE_ISCODE);
            announce_filter.push(OPCODE_OR);
        }
    }
    // local ac=Duel.AnnounceCard(tp,table.unpack(s.announce_filter))
    crate::class::set_announce_filter(d, ID, &announce_filter);
    let ac = duel::announce_card(d, tp, &announce_filter);
    // Duel.SetTargetParam(ac)
    duel::set_target_param(d, ac as u32);
    // Duel.SetOperationInfo(0,CATEGORY_REMOVE,nil,1,tp,LOCATION_DECK)
    duel::set_operation_info(d, 0, CATEGORY_REMOVE, None, 1, tp, LOCATION_DECK as i32);
    // Duel.SetOperationInfo(0,CATEGORY_ANNOUNCE,nil,0,tp,ANNOUNCE_CARD_FILTER)
    duel::set_operation_info(d, 0, CATEGORY_ANNOUNCE, None, 0, tp, ANNOUNCE_CARD_FILTER as i32);
    true
}
fn rmfilter(d: &mut Duel, c: CardId, ac: i64) -> bool {
    // return c:IsCode(ac) and c:IsAbleToRemove()
    card::is_code(d, c, &[ac as u32]) && card::is_able_to_remove(d, c, None, None, None)
}
fn rmop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // local ac=Duel.GetChainInfo(0,CHAININFO_TARGET_PARAM)
    let ac = duel::get_chain_info(d, 0, &[CHAININFO_TARGET_PARAM])[0].int();
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_REMOVE)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_REMOVE as u64);
    // local g=Duel.SelectMatchingCard(tp,s.rmfilter,tp,LOCATION_DECK,0,1,1,nil,ac)
    let g = duel::select_matching_card(d, tp, Some(&|d, c| rmfilter(d, c, ac)), tp, LOCATION_DECK, 0, 1, 1, false, NO_EX).nil();
    // local tc=g:GetFirst()
    let tc = group::get_first(d, &g);
    // if tc and Duel.Remove(tc,POS_FACEUP,REASON_EFFECT)~=0 and tc:IsLocation(LOCATION_REMOVED) then
    if let Some(tc) = tc {
        if duel::remove(d, tc, POS_FACEUP as u8, REASON_EFFECT, None, None) != 0 && card::is_location(d, tc, LOCATION_REMOVED) {
            // local c=e:GetHandler()
            let c = effect::get_handler(d, a.e).nil();
            // local code=tc:GetOriginalCodeRule()
            let code = card::get_original_code_rule(d, tc)[0];
            // local e1=Effect.CreateEffect(c)
            let e1 = effect::create_effect(d, c);
            effect::set_type(d, e1, EFFECT_TYPE_FIELD);
            effect::set_code(d, e1, EFFECT_DISABLE);
            effect::set_target_range(d, e1, LOCATION_ONFIELD, LOCATION_ONFIELD);
            effect::set_label(d, e1, &[code as i64]);
            effect::set_target(d, e1, Some(effect_card_fn(distg)));
            effect::set_reset(d, e1, RESET_PHASE | PHASE_END, 1);
            duel::register_effect(d, e1, tp);
            // local e2=Effect.CreateEffect(c)
            let e2 = effect::create_effect(d, c);
            effect::set_type(d, e2, EFFECT_TYPE_FIELD + EFFECT_TYPE_CONTINUOUS);
            effect::set_code(d, e2, EVENT_CHAIN_SOLVING);
            effect::set_label(d, e2, &[code as i64]);
            effect::set_condition(d, e2, Some(condition(discon)));
            effect::set_operation(d, e2, Some(operation(disop)));
            effect::set_reset(d, e2, RESET_PHASE | PHASE_END, 1);
            duel::register_effect(d, e2, tp);
            // local e3=e1:Clone()
            let e3 = effect::clone(d, e1);
            // e3:SetCode(EFFECT_DISABLE_TRAPMONSTER)
            effect::set_code(d, e3, EFFECT_DISABLE_TRAPMONSTER);
            // e3:SetTargetRange(LOCATION_MZONE,LOCATION_MZONE)
            effect::set_target_range(d, e3, LOCATION_MZONE, LOCATION_MZONE);
            // Duel.RegisterEffect(e3,tp)
            duel::register_effect(d, e3, tp);
        }
    }
}
fn distg(d: &mut Duel, e: EffectId, c: CardId) -> bool {
    // return c:IsOriginalCodeRule(e:GetLabel())
    let label = effect::get_label(d, e) as u32;
    card::is_original_code_rule(d, c, &[label])
}
fn discon(d: &mut Duel, a: &EffArgs) -> bool {
    // return re:GetHandler():IsOriginalCodeRule(e:GetLabel())
    let re = a.re.nil();
    let h = effect::get_handler(d, re).nil();
    let label = effect::get_label(d, a.e) as u32;
    card::is_original_code_rule(d, h, &[label])
}
fn disop(d: &mut Duel, a: &EffArgs) {
    // Duel.NegateEffect(ev)
    duel::negate_effect(d, a.ev as u8);
}
