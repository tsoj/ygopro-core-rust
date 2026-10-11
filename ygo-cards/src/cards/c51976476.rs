//! Double Trap Hole
use super::prelude::*;

pub const ID: u32 = 51976476;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetCategory(CATEGORY_REMOVE)
    effect::set_category(d, e1, CATEGORY_REMOVE);
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    // e1:SetCode(EVENT_SPSUMMON_SUCCESS)
    effect::set_code(d, e1, EVENT_SPSUMMON_SUCCESS);
    // e1:SetTarget(s.target)
    effect::set_target(d, e1, Some(cost_target(target)));
    // e1:SetOperation(s.activate)
    effect::set_operation(d, e1, Some(operation(activate)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
/// `s.filter(c,e,tp)`
fn filter(d: &mut Duel, c: CardId, e: Option<EffectId>, tp: u8) -> bool {
    // return c:IsSummonPlayer(tp) and c:IsDefensePos() and c:IsAbleToRemove()
    //     and (not e or (c:IsRelateToEffect(e) and c:IsLocation(LOCATION_MZONE)))
    card::is_summon_player(d, c, tp)
        && card::is_defense_pos(d, c)
        && card::is_able_to_remove(d, c, None, Some(POS_FACEUP as u8), Some(REASON_EFFECT))
        && match e {
            None => true,
            Some(e) => card::is_relate_to_effect(d, c, e) && card::is_location(d, c, LOCATION_MZONE),
        }
}
/// `s.target(e,tp,eg,ep,ev,re,r,rp,chk)`
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // if chk==0 then return eg and eg:IsExists(s.filter,1,nil,nil,1-tp) end
    if a.chk == Some(0) {
        return match &a.eg {
            Some(eg) => group::is_exists(d, eg, &|d, c| filter(d, c, None, 1 - tp), 1, NO_EX),
            None => false,
        };
    }
    // local g=eg:Filter(s.filter,nil,nil,1-tp)
    let eg = a.eg.clone().nil();
    let g = group::filter(d, &eg, &|d, c| filter(d, c, None, 1 - tp), NO_EX);
    // Duel.SetTargetCard(g)
    duel::set_target_card(d, g.clone());
    // Duel.SetOperationInfo(0,CATEGORY_REMOVE,g,#g,0,0)
    let n = g.len() as u8;
    duel::set_operation_info(d, 0, CATEGORY_REMOVE, Some(g), n, 0, 0);
    true
}
/// `s.activate(e,tp,eg,ep,ev,re,r,rp)`
fn activate(d: &mut Duel, a: &EffArgs) {
    let (e, tp) = (a.e, a.tp);
    // local tg=Duel.GetChainInfo(0,CHAININFO_TARGET_CARDS)
    let tg = duel::get_chain_info(d, 0, &[CHAININFO_TARGET_CARDS])[0].as_group().cloned().nil();
    // local g=tg:Filter(s.filter,nil,e,1-tp)
    let g = group::filter(d, &tg, &|d, c| filter(d, c, Some(e), 1 - tp), NO_EX);
    // if #g>0 then
    if !g.is_empty() {
        // Duel.Remove(g,POS_FACEUP,REASON_EFFECT)
        duel::remove(d, g, POS_FACEUP as u8, REASON_EFFECT, None, None);
    }
}
