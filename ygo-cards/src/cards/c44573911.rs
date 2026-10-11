//! Ｒ．Ｂ．ＧＡ１０パイルバンカー
//! R.B. Ga10 Pile Bunker
//! Scripted by The Razgriz
use super::prelude::*;

pub const ID: u32 = 44573911;
pub const INFO: ScriptInfo = ScriptInfo { listed_names: &[32216688], listed_series: &[SET_RB], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // If you control no face-up monsters, or the only face-up monsters you control are "R.B." monsters, you can Special Summon this card (from your hand)
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e1, EFFECT_TYPE_FIELD);
    // e1:SetProperty(EFFECT_FLAG_UNCOPYABLE)
    effect::set_property(d, e1, EFFECT_FLAG_UNCOPYABLE, 0);
    // e1:SetCode(EFFECT_SPSUMMON_PROC)
    effect::set_code(d, e1, EFFECT_SPSUMMON_PROC);
    // e1:SetRange(LOCATION_HAND)
    effect::set_range(d, e1, LOCATION_HAND);
    // e1:SetCountLimit(1,id,EFFECT_COUNT_CODE_OATH)
    effect::set_count_limit(d, e1, 1, ID, 0, EFFECT_COUNT_CODE_OATH as u8);
    // e1:SetCondition(s.spcon)
    effect::set_condition(d, e1, Some(lua_fn(spcon)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // Destroy all cards your opponent controls and this card
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e2, aux::stringid(ID, 1));
    // e2:SetCategory(CATEGORY_DESTROY)
    effect::set_category(d, e2, CATEGORY_DESTROY);
    // e2:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e2, EFFECT_TYPE_FIELD + EFFECT_TYPE_TRIGGER_O);
    // e2:SetCode(EVENT_ATTACK_ANNOUNCE)
    effect::set_code(d, e2, EVENT_ATTACK_ANNOUNCE);
    // e2:SetRange(LOCATION_MZONE)
    effect::set_range(d, e2, LOCATION_MZONE);
    // e2:SetCountLimit(1,{id,1})
    effect::set_count_limit(d, e2, 1, ID, 1, 0);
    // e2:SetCondition(s.descon)
    effect::set_condition(d, e2, Some(condition(descon)));
    // e2:SetCost(Cost.PayLP(1500))
    effect::set_cost(d, e2, Some(cost::pay_lp(cost::PayLp::Amount(1500))));
    // e2:SetTarget(s.destg)
    effect::set_target(d, e2, Some(cost_target(destg)));
    // e2:SetOperation(s.desop)
    effect::set_operation(d, e2, Some(operation(desop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
}
/// `s.spcon(e,c)`
fn spcon(d: &mut Duel, a: Args) -> bool {
    // if c==nil then return true end
    if a.card(1).is_none() {
        return true;
    }
    // local tp=e:GetHandlerPlayer()
    let tp = effect::get_handler_player(d, a.e(0));
    // local g=Duel.GetMatchingGroup(Card.IsFaceup,tp,LOCATION_MZONE,0,nil)
    let g = duel::get_matching_group(d, Some(&|d, c| card::is_faceup(d, c)), tp, LOCATION_MZONE, 0, NO_EX);
    // return Duel.GetLocationCount(tp,LOCATION_MZONE)>0 and (#g==0 or g:FilterCount(Card.IsSetCard,nil,SET_RB)==#g)
    duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 > 0
        && (g.is_empty() || group::filter_count(d, &g, &|d, c| card::is_set_card(d, c, &[SET_RB as u16]), NO_EX) == g.len())
}
/// `s.descon(e,tp,eg,ep,ev,re,r,rp)`
fn descon(d: &mut Duel, a: &EffArgs) -> bool {
    // local bc,oc=Duel.GetBattleMonster(tp)
    let (bc, oc) = duel::get_battle_monster(d, a.tp);
    // return bc and oc and bc:IsCode(32216688) and bc:GetLinkedGroup():IsContains(e:GetHandler())
    let (Some(bc), Some(_oc)) = (bc, oc) else {
        return false;
    };
    let c = effect::get_handler(d, a.e).nil();
    card::is_code(d, bc, &[32216688]) && card::get_linked_group(d, bc).contains(c)
}
/// `s.destg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn destg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // local g=Duel.GetFieldGroup(tp,0,LOCATION_ONFIELD)
    let g = duel::get_field_group(d, tp, 0, LOCATION_ONFIELD);
    // if chk==0 then return #g>0 end
    if a.chk == Some(0) {
        return !g.is_empty();
    }
    // Duel.SetOperationInfo(0,CATEGORY_DESTROY,g+e:GetHandler(),#g,tp,0)
    let mut gc = g.clone();
    let c = effect::get_handler(d, a.e).nil();
    group::add_card(&mut gc, &CardSet::from_card(c));
    duel::set_operation_info(d, 0, CATEGORY_DESTROY, Some(gc), g.len() as u8, tp, 0);
    true
}
/// `s.desop(e,tp,eg,ep,ev,re,r,rp)`
fn desop(d: &mut Duel, a: &EffArgs) {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // local g=Duel.GetFieldGroup(tp,0,LOCATION_ONFIELD)
    let mut g = duel::get_field_group(d, a.tp, 0, LOCATION_ONFIELD);
    // if c:IsRelateToEffect(e) and #g>0 then
    if card::is_relate_to_effect(d, c, a.e) && !g.is_empty() {
        // g:AddCard(c)
        group::add_card(&mut g, &CardSet::from_card(c));
        // Duel.Destroy(g,REASON_EFFECT)
        duel::destroy(d, g, REASON_EFFECT, None, None);
    }
}
