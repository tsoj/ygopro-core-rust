//! Sword Hunter
use super::prelude::*;

pub const ID: u32 = 51345461;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // equip
    let e1 = effect::create_effect(d, c);
    effect::set_description(d, e1, aux::stringid(ID, 0));
    effect::set_category(d, e1, CATEGORY_EQUIP);
    effect::set_type(d, e1, EFFECT_TYPE_FIELD + EFFECT_TYPE_TRIGGER_F);
    effect::set_code(d, e1, EVENT_PHASE | PHASE_BATTLE);
    effect::set_range(d, e1, LOCATION_MZONE);
    effect::set_count_limit(d, e1, 1, 0, 0, 0);
    effect::set_target(d, e1, Some(cost_target(eqtg)));
    effect::set_operation(d, e1, Some(operation(eqop)));
    card::register_effect(d, c, e1, false);
    // aux.AddEREquipLimit(c,nil,aux.FilterBoolFunction(Card.IsMonster),s.equipop,e1)
    aux::add_er_equip_limit(
        d,
        c,
        None,
        lua_fn(|d, a| card::is_monster(d, a.c(0))),
        lua_fn(|d, a| equipop(d, a.c(0), a.e(1), a.p(2), a.c(3), false)),
        e1,
        None,
        None,
        None,
    );
}
/// `s.filter(c,rc,tid)`
fn filter(d: &mut Duel, c: CardId, rc: CardId, tid: i32) -> bool {
    // return c:IsReason(REASON_BATTLE) and c:GetReasonCard()==rc and c:GetTurnID()==tid and not c:IsForbidden()
    card::is_reason(d, c, REASON_BATTLE) && card::get_reason_card(d, c) == Some(rc) && card::get_turn_id(d, c) as i32 == tid && !card::is_forbidden(d, c)
}
/// `s.eqtg(e,tp,...,chk)`
fn eqtg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return true end
    if a.chk == Some(0) {
        return true;
    }
    // local g=Duel.GetMatchingGroup(s.filter,tp,LOCATION_GRAVE,LOCATION_GRAVE,nil,e:GetHandler(),Duel.GetTurnCount())
    let h = effect::get_handler(d, a.e).nil();
    let tid = duel::get_turn_count(d, None);
    let g = duel::get_matching_group(d, Some(&|d, c| filter(d, c, h, tid)), a.tp, LOCATION_GRAVE, LOCATION_GRAVE, NO_EX);
    // Duel.SetOperationInfo(0,CATEGORY_EQUIP,g,#g,0,0)
    let n = g.len() as u8;
    duel::set_operation_info(d, 0, CATEGORY_EQUIP, Some(g), n, 0, 0);
    true
}
/// `s.equipop(c,e,tp,tc,chk)`: `chk` is the fifth argument (`nil` from the equip limit, `true` from `eqop`)
fn equipop(d: &mut Duel, c: CardId, e: EffectId, tp: u8, tc: CardId, chk: bool) {
    // local eff=false or chk
    // Duel.Equip(tp,tc,c,false,eff)
    duel::equip(d, tp, tc, c, false, chk);
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
    // e1:SetProperty(EFFECT_FLAG_OWNER_RELATE)
    effect::set_property(d, e1, EFFECT_FLAG_OWNER_RELATE, 0);
    // e1:SetCode(EFFECT_EQUIP_LIMIT)
    effect::set_code(d, e1, EFFECT_EQUIP_LIMIT);
    // e1:SetReset(RESET_EVENT|RESETS_STANDARD)
    effect::set_reset(d, e1, RESET_EVENT | RESETS_STANDARD, 1);
    // e1:SetValue(Card.EquipByEffectLimit)
    effect::set_value_fn(d, e1, card::equip_by_effect_limit());
    // e1:SetLabelObject(e:GetLabelObject())
    let lo = effect::get_label_object(d, e);
    effect::set_label_object(d, e1, lo);
    // tc:RegisterEffect(e1)
    card::register_effect(d, tc, e1, false);
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetType(EFFECT_TYPE_EQUIP)
    effect::set_type(d, e2, EFFECT_TYPE_EQUIP);
    // e2:SetProperty(EFFECT_FLAG_OWNER_RELATE+EFFECT_FLAG_IGNORE_IMMUNE)
    effect::set_property(d, e2, EFFECT_FLAG_OWNER_RELATE + EFFECT_FLAG_IGNORE_IMMUNE, 0);
    // e2:SetCode(EFFECT_UPDATE_ATTACK)
    effect::set_code(d, e2, EFFECT_UPDATE_ATTACK);
    // e2:SetReset(RESET_EVENT|RESETS_STANDARD)
    effect::set_reset(d, e2, RESET_EVENT | RESETS_STANDARD, 1);
    // e2:SetValue(200)
    effect::set_value(d, e2, 200);
    // tc:RegisterEffect(e2)
    card::register_effect(d, tc, e2, false);
}
/// `s.eqop(e,tp,...)`
fn eqop(d: &mut Duel, a: &EffArgs) {
    // local ft=Duel.GetLocationCount(tp,LOCATION_SZONE)
    let ft = duel::get_location_count(d, a.tp, LOCATION_SZONE as u8, None, None, None).0;
    // if ft<=0 then return end
    if ft <= 0 {
        return;
    }
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if not c:IsRelateToEffect(e) or c:IsFacedown() then return end
    if !card::is_relate_to_effect(d, c, a.e) || card::is_facedown(d, c) {
        return;
    }
    // local g=Duel.GetMatchingGroup(s.filter,tp,LOCATION_GRAVE,LOCATION_GRAVE,nil,e:GetHandler(),Duel.GetTurnCount())
    let tid = duel::get_turn_count(d, None);
    let g = duel::get_matching_group(d, Some(&|d, cc| filter(d, cc, c, tid)), a.tp, LOCATION_GRAVE, LOCATION_GRAVE, NO_EX);
    // if #g==0 then return end
    if g.is_empty() {
        return;
    }
    // if #g>ft then return end
    if g.len() as i32 > ft {
        return;
    }
    // for tc in g:Iter() do s.equipop(c,e,tp,tc,true) end
    for tc in group::iter(d, &g) {
        equipop(d, c, a.e, a.tp, tc, true);
    }
    // Duel.EquipComplete()
    duel::equip_complete(d);
}
